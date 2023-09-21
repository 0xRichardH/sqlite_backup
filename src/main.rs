use anyhow::{Context, Result};
use rusqlite::Connection;
use sqlite_backup::{Backup, SqliteBackup};
use std::env;

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();
    if let Some(source_path) = args.get(1) {
        let src_conn = Connection::open(source_path).context("create source connection")?;
        let dest_path = String::from("./backup.db");
        SqliteBackup::new(src_conn, dest_path, |p| {
            println!(
                "---Progress---- pagecount: {}, remaining: {}",
                p.pagecount, p.remaining
            )
        })
        .backup()
        .context("backup source to destination")?;
    } else {
        eprintln!("No source path provided");
    }

    Ok(())
}
