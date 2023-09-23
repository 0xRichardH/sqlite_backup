use anyhow::{Context, Result};
use rusqlite::Connection;
use sqlite_backup::{
    argument,
    backup::{Backup, SqliteBackup, SqliteSourceFile},
    config::Config,
    uploader::{R2Uploader, Uploader},
};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Config::load().context("load env vars")?;
    let args = env::args().collect::<Vec<String>>();
    match argument::Argument::build(&args) {
        Ok(arg) => run(&arg, &cfg).await?,

        Err(err) => eprintln!("Application Error: {}", err),
    }

    println!("Done");

    Ok(())
}

async fn run(arg: &argument::Argument, cfg: &Config) -> Result<()> {
    // create temp dir
    let tmp_dir = tempfile::tempdir()?;

    // backup data
    let src_file = SqliteSourceFile::from(arg.source_path.as_str()).context("parse source path")?;
    let src_conn = Connection::open(src_file.path).context("create source connection")?;
    let dest = tmp_dir.path().join(src_file.filename);
    SqliteBackup::new(src_conn, dest.display().to_string(), |p| {
        println!(
            "---Progress---- pagecount: {}, remaining: {}",
            p.pagecount, p.remaining
        )
    })
    .backup()
    .context("backup source to destination")?;

    // upload
    let uploader = R2Uploader::new(cfg).await;
    uploader
        .upload_object(
            dest,
            format!("sqlite__{}", src_file.db_name).as_str(),
            src_file.db_extension,
        )
        .await?;

    // close temp dir
    tmp_dir.close()?;

    Ok(())
}
