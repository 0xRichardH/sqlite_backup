use anyhow::{Context, Result};
use rusqlite::Connection;
use sqlite_backup::{argument, Backup, SqliteBackup};
use std::env;

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();
    match argument::Argument::build(&args) {
        Ok(arg) => run(&arg)?,

        Err(err) => eprintln!("Application Error: {}", err),
    }

    Ok(())
}

fn run(arg: &argument::Argument) -> Result<()> {
    let src_conn = Connection::open(arg.source_path.clone()).context("create source connection")?;
    let dest_path = String::from("./backup.db");
    SqliteBackup::new(src_conn, dest_path, |p| {
        println!(
            "---Progress---- pagecount: {}, remaining: {}",
            p.pagecount, p.remaining
        )
    })
    .backup()
    .context("backup source to destination")?;

    Ok(())
}
