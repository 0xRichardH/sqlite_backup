use dotenvy::{dotenv, from_filename_override};
use std::{env, ffi::OsString, fmt::Display};

use anyhow::Result;

#[derive(PartialEq, Debug)]
pub enum AppEnv {
    Prod,
    Dev,
    Test,
    CI,
}

impl Display for AppEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppEnv::Prod => write!(f, "prod"),
            AppEnv::Dev => write!(f, "dev"),
            AppEnv::Test => write!(f, "test"),
            AppEnv::CI => write!(f, "ci"),
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
    pub gpg_passphrase: Option<OsString>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let app_env = match env::var("APP_ENV") {
            Ok(v) => match v.as_str() {
                "prod" => AppEnv::Prod,
                "test" => AppEnv::Test,
                "ci" => AppEnv::CI,
                _ => AppEnv::Dev,
            },
            _ => AppEnv::Dev,
        };

        println!("Running in {app_env} mode");

        match app_env {
            AppEnv::Dev => {
                // load environment variables from .env file
                dotenv().expect(".env file not found");
            }

            AppEnv::Test => {
                from_filename_override(".env.test").expect(".env.test file not found");
            }

            _ => {
                // load environment variables from the system env
            }
        }

        let config = Self {
            app_env,
            bucket_name: env::var("BUCKET_NAME")?,
            account_id: env::var("ACCOUNT_ID")?,
            access_key_id: env::var("ACCESS_KEY_ID")?,
            secret_access_key: env::var("SECRET_ACCESS_KEY")?,
            gpg_passphrase: env::var_os("GPG_PASSPHRASE"),
        };
        Ok(config)
    }
}
