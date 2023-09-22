use anyhow::{Context, Result};
use rusqlite::Connection;
use sqlite_backup::{
    argument,
    config::Config,
    uploader::{R2Uploader, Uploader},
    Backup, SqliteBackup,
};
use std::{env, path::Path};

#[tokio::main]
async fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();
    match argument::Argument::build(&args) {
        Ok(arg) => run(&arg).await?,

        Err(err) => eprintln!("Application Error: {}", err),
    }

    Ok(())
}

async fn run(arg: &argument::Argument) -> Result<()> {
    // load config/env
    let config = Config::load().context("load env vars")?;

    // backup data
    let src_conn = Connection::open(arg.source_path.clone()).context("create source connection")?;
    // TODO: copy to temporary path
    let dest_path = String::from("./backup.db");
    SqliteBackup::new(src_conn, dest_path.clone(), |p| {
        println!(
            "---Progress---- pagecount: {}, remaining: {}",
            p.pagecount, p.remaining
        )
    })
    .backup()
    .context("backup source to destination")?;

    // upload
    let path = Path::new(dest_path.as_str());
    let uploader = R2Uploader::new(&config).await;
    // TODO: get db_name and extension from the path
    uploader
        .upload_object(path.to_path_buf(), "test_db", "db")
        .await?;

    Ok(())
}
