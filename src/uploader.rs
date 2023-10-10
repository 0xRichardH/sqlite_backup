use std::ffi::OsString;

use anyhow::{Context, Result};
use async_trait::async_trait;
use aws_sdk_s3::{
    config::{Credentials, Region},
    primitives::ByteStream,
    types::{Delete, ObjectIdentifier},
    Client,
};
use time::format_description;

use crate::{
    argument,
    config::{self},
    encrypt,
};

#[async_trait]
pub trait Uploader {
    async fn upload_object(&self, src_path: &str, src_name: &str) -> Result<()>;
    async fn retain(&self, data_retention: u8, src_name: &str) -> Result<()>;
}

pub struct R2Uploader {
    client: Client,
    bucket: String,
    project_name: String,
    app_env: String,
    gpg_passphrase: Option<OsString>,
}

impl R2Uploader {
    pub async fn new(arg: &argument::Argument, cfg: &config::Config) -> Self {
        let endpoint = format!(
            "https://{}.r2.cloudflarestorage.com",
            cfg.account_id.clone()
        );
        let credentials = Credentials::new(
            cfg.access_key_id.clone(),
            cfg.secret_access_key.clone(),
            None,
            None,
            "Static",
        );

        let config = aws_config::from_env()
            .credentials_provider(credentials)
            .region(Region::new("auto"))
            .endpoint_url(endpoint)
            .load()
            .await;
        let client = Client::new(&config);

        Self {
            client,
            bucket: cfg.bucket_name.clone(),
            project_name: arg.project_name.clone(),
            app_env: cfg.app_env.to_string(),
            gpg_passphrase: cfg.gpg_passphrase.clone(),
        }
    }
}

#[async_trait]
impl Uploader for R2Uploader {
    async fn upload_object(&self, src_path: &str, src_name: &str) -> Result<()> {
        let key = uuid::Uuid::new_v4();
        let format = format_description::parse("[year]-[month]-[day]")?;
        let today = time::OffsetDateTime::now_utc().format(&format)?;
        let key_prefix = self.object_key_prefix(src_name);
        let mut object_key = format!("{key_prefix}/{today}__{key}");
        let mut src_filepath = src_path.to_string();

        // check gpg passphrase
        if self.gpg_passphrase.is_some() {
            object_key = encrypt::gpg_filename(&object_key);
            src_filepath = encrypt::gpg_filename(&src_filepath);
        }

        let body = ByteStream::from_path(src_filepath)
            .await
            .context("create file stream")?;

        self.client
            .put_object()
            .bucket(self.bucket.clone())
            .key(&object_key)
            .body(body)
            .send()
            .await
            .context("upload object to r2")?;
        log::info!("upload object to r2 success {}", object_key);

        Ok(())
    }

    async fn retain(&self, count: u8, src_name: &str) -> Result<()> {
        let key_prefix = self.object_key_prefix(src_name);
        let result = self
            .client
            .list_objects_v2()
            .bucket(self.bucket.clone())
            .prefix(key_prefix)
            .send()
            .await
            .context("list bojects from r2")?;

        // skip the task if the key count is less than the data_retention count
        if result.key_count() < count as i32 {
            return Ok(());
        }

        if let Some(objects) = result.contents() {
            let deleted_count = objects.len() - count as usize;
            let mut objects = objects.to_vec();
            objects.sort_by(|a, b| {
                let last_modified_a = a.last_modified().unwrap();
                let last_modified_b = b.last_modified().unwrap();
                last_modified_a.cmp(last_modified_b)
            });
            let deleted_objects = &objects[..deleted_count]
                .iter()
                .map(|obj| {
                    let key = obj.key().unwrap_or_default().to_string();
                    ObjectIdentifier::builder().set_key(Some(key)).build()
                })
                .collect::<Vec<_>>();
            self.delete_objects(deleted_objects.clone()).await?;

            log::info!("deleted {} objects from r2", deleted_objects.len());
        }

        Ok(())
    }
}

impl R2Uploader {
    fn object_key_prefix(&self, src_name: &str) -> String {
        format!("{}/{}/{}", self.app_env, self.project_name, src_name)
    }

    async fn delete_objects(&self, deleted_objects: Vec<ObjectIdentifier>) -> Result<()> {
        let delete_builder = Delete::builder().set_objects(Some(deleted_objects)).build();
        self.client
            .delete_objects()
            .bucket(self.bucket.clone())
            .delete(delete_builder)
            .send()
            .await
            .context("delete objects")?;
        Ok(())
    }
}
