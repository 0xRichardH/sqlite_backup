use anyhow::{Context, Result};
use clap::Parser;
use rusqlite::Connection;
use sqlite_backup::{
    argument::{self, Argument},
    backup::{Backup, SqliteBackup, SqliteSourceFile},
    config::Config,
    encrypt,
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
    SqliteBackup::new(cfg, src_conn, dest.display().to_string(), |p| {
        println!(
            "---Progress---- pagecount: {}, remaining: {}",
            p.pagecount, p.remaining
        )
    })
    .backup()
    .context("backup source to destination")?;

    // get source file name
    let src_filename = if cfg.gpg_passphrase.is_some() {
        encrypt::gpg_filename(src_file.filename)
    } else {
        src_file.filename.to_string()
    };

    // upload
    let uploader = R2Uploader::new(arg, cfg).await;
    let (upload_res, retain_res) = tokio::join!(
        uploader.upload_object(dest, src_filename.as_str()),
        uploader.retain(arg.data_retention, src_filename.as_str())
    );
    upload_res?;
    retain_res?;

    // close temp dir
    tmp_dir.close()?;

    Ok(())
}
