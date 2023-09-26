use anyhow::{Context, Result};
use clap::Parser;
use rusqlite::Connection;
use sqlite_backup::{
    argument::{self, Argument},
    backup::{Backup, SqliteBackup, SqliteSourceFile},
    config::Config,
    uploader::{R2Uploader, Uploader},
};

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Config::load().context("load env vars")?;
    let args = Argument::parse();
    run(&args, &cfg).await?;

    println!("Done");

    Ok(())
}

async fn run(arg: &argument::Argument, cfg: &Config) -> Result<()> {
    // create temp dir
    let tmp_dir = tempfile::tempdir()?;

    // backup data
    let src_file = SqliteSourceFile::from(arg.db.as_str()).context("parse source path")?;
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
    let uploader = R2Uploader::new(arg, cfg).await;
    uploader.upload_object(dest, src_file.filename).await?;

    // close temp dir
    tmp_dir.close()?;

    Ok(())
}
