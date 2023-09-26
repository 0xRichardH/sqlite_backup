use dotenvy::dotenv;
use std::{env, fmt::Display};

use anyhow::Result;

#[derive(PartialEq, Debug)]
pub enum AppEnv {
    Prod,
    Dev,
    Test,
}

impl Display for AppEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppEnv::Prod => write!(f, "prod"),
            AppEnv::Dev => write!(f, "dev"),
            AppEnv::Test => write!(f, "test"),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub app_env: AppEnv,
    pub bucket_name: String,
    pub account_id: String,
    pub access_key_id: String,
    pub secret_access_key: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let app_env = match env::var("APP_ENV") {
            Ok(v) => match v.as_str() {
                "prod" => AppEnv::Prod,
                "test" => AppEnv::Test,
                _ => AppEnv::Dev,
            },
            _ => AppEnv::Dev,
        };

        println!("Running in {app_env} mode");

        if app_env == AppEnv::Dev {
            // load environment variables from .env file
            dotenv().expect(".env file not found");
        }

        let config = Self {
            app_env,
            bucket_name: env::var("BUCKET_NAME")?,
            account_id: env::var("ACCOUNT_ID")?,
            access_key_id: env::var("ACCESS_KEY_ID")?,
            secret_access_key: env::var("SECRET_ACCESS_KEY")?,
        };
        Ok(config)
    }
}
