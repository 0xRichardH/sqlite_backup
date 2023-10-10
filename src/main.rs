use anyhow::{Context, Result};
use clap::Parser;
use sqlite_backup::{
    argument::{self, Argument},
    backup::{Backup, SqliteBackup, SqliteSourceFile},
    config::Config,
    uploader::{R2Uploader, Uploader},
};

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    log::info!("================================================================================");
    if let Err(err) = execute().await {
        log::error!("{:?}", err)
    }
    log::info!("================================================================================");
}

async fn execute() -> Result<()> {
    log::info!("Starting backup...");

    let cfg = Config::load().context("load env vars")?;
    let args = Argument::parse();
    run(&args, &cfg).await?;

    log::info!("Ending backup...");

    Ok(())
}

async fn run(arg: &argument::Argument, cfg: &Config) -> Result<()> {
    // create temp dir
    let tmp_dir = tempfile::tempdir()?;

    // backup data
    let src_file = SqliteSourceFile::from(arg.db.as_str()).context("parse source path")?;
    let dest = tmp_dir.path().join(src_file.filename).display().to_string();
    SqliteBackup::new(
        cfg,
        src_file.path.display().to_string(),
        dest.clone(),
        |p| {
            log::info!(
                "---Progress---- pagecount: {}, remaining: {}",
                p.pagecount,
                p.remaining
            )
        },
    )
    .backup()
    .context("backup source to destination")?;

    // upload
    let uploader = R2Uploader::new(arg, cfg).await;
    let (upload_res, retain_res) = tokio::join!(
        uploader.upload_object(&dest, src_file.filename),
        uploader.retain(arg.data_retention, src_file.filename),
    );
    upload_res?;
    retain_res?;

    // close temp dir
    tmp_dir.close()?;

    Ok(())
}
