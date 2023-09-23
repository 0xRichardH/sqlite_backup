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

    // tempfile
    // TODO: Remove unwrap
    let src_path = Path::new(arg.source_path.as_str());
    let file_name = src_path.file_name().unwrap();
    let db_name = src_path.file_stem().unwrap().to_str().unwrap();
    let db_extension = src_path.extension().unwrap().to_str().unwrap();
    let tmp_dir = tempfile::tempdir()?;
    let dest = tmp_dir.path().join(file_name);

    // backup data
    let src_conn = Connection::open(arg.source_path.clone()).context("create source connection")?;
    SqliteBackup::new(src_conn, dest.display().to_string(), |p| {
        println!(
            "---Progress---- pagecount: {}, remaining: {}",
            p.pagecount, p.remaining
        )
    })
    .backup()
    .context("backup source to destination")?;

    // upload
    let uploader = R2Uploader::new(&config).await;
    uploader.upload_object(dest, db_name, db_extension).await?;

    // close temp dir
    tmp_dir.close()?;

    Ok(())
}
