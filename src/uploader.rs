use std::path::PathBuf;

use anyhow::{Context, Result};
use async_trait::async_trait;
use aws_sdk_s3::{
    config::{Credentials, Region},
    primitives::ByteStream,
    Client,
};
use time::format_description;

use crate::config::{self};

#[async_trait]
pub trait Uploader {
    async fn upload_object(&self, path: PathBuf, db_name: &str, extension: &str) -> Result<()>;
}

pub struct R2Uploader {
    client: Client,
    bucket: String,
}

impl R2Uploader {
    pub async fn new(cfg: &config::Config) -> Self {
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
        }
    }
}

#[async_trait]
impl Uploader for R2Uploader {
    async fn upload_object(&self, path: PathBuf, db_name: &str, extension: &str) -> Result<()> {
        let body = ByteStream::from_path(path)
            .await
            .context("create file stream")?;
        let key = uuid::Uuid::new_v4();
        let format = format_description::parse("[year]-[month]-[day]")?;
        let today = time::OffsetDateTime::now_utc().format(&format)?;
        let object_key = format!("{db_name}/{today}__{key}.{extension}");

        self.client
            .put_object()
            .bucket(self.bucket.clone())
            .key(object_key)
            .body(body)
            .send()
            .await
            .context("upload object to r2")?;

        Ok(())
    }
}
