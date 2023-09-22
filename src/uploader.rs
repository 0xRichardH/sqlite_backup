use std::path::PathBuf;

use anyhow::{Context, Result};
use async_trait::async_trait;
use aws_sdk_s3::{config::Region, primitives::ByteStream, Client};

#[async_trait]
pub trait Uploader {
    async fn upload_object(&self, path: PathBuf, db_name: &str, extension: &str) -> Result<()>;
}

const BUCKET_NAME: &str = "backup-sqlite";

pub struct R2Uploader {
    client: Client,
    bucket: String,
}

impl R2Uploader {
    pub async fn new() -> Self {
        let endpoint = "https://#{cloudflare_account_id}.r2.cloudflarestorage.com".to_string();
        let sdk_config = aws_config::load_from_env().await;
        let config = aws_sdk_s3::config::Builder::from(&sdk_config)
            .region(Region::new("auto"))
            .endpoint_url(endpoint)
            .build();
        let client = Client::from_conf(config);

        Self {
            client,
            bucket: String::from(BUCKET_NAME),
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
        let object_key = format!("{db_name}/{key}.{extension}");

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
