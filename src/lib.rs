pub mod argument;

use std::time::Duration;

use anyhow::{Context, Result};
use rusqlite::{
    backup::{self, Progress},
    Connection,
};

pub trait Backup {
    fn backup(&self) -> Result<()>;
}

pub struct SqliteBackup {
    src_conn: rusqlite::Connection,
    dest: String,
    progress: fn(Progress),
}

impl SqliteBackup {
    pub fn new(src_conn: rusqlite::Connection, dest: String, progress: fn(Progress)) -> Self {
        Self {
            src_conn,
            dest,
            progress,
        }
    }
}

impl Backup for SqliteBackup {
    fn backup(&self) -> Result<()> {
        let mut dest_conn =
            Connection::open(self.dest.clone()).context("open backup destination")?;
        let online_backup =
            backup::Backup::new(&self.src_conn, &mut dest_conn).context("create online backup")?;
        online_backup
            .run_to_completion(5, Duration::from_millis(250), Some(self.progress))
            .context("run backup")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[derive(Clone)]
    struct Person {
        id: i32,
        name: String,
        age: i32,
    }

    #[test]
    fn it_should_backup_db() -> Result<()> {
        // create source connection
        let src_conn = Connection::open_in_memory()?;

        // seed
        seed_person_table(&src_conn)?;

        // create temp destination path
        let tmp_dir = tempdir()?;
        let dest = tmp_dir.path().join("backup.db").display().to_string();

        // backup
        let backup = SqliteBackup::new(src_conn, dest.clone(), |p| {
            println!(
                "---Progress---- pagecount: {}, remaining: {}",
                p.pagecount, p.remaining
            )
        });
        backup.backup()?;

        // verify the backup
        let dest_conn = Connection::open(dest)?;
        let people = get_people(&dest_conn)?;
        assert_eq!(people.len(), 1);
        let person = people[0].clone();
        assert_eq!(person.id, 1);
        assert_eq!(person.name, "Richard");
        assert_eq!(person.age, 18);

        // close temp dir
        tmp_dir.close()?;

        Ok(())
    }

    fn seed_person_table(conn: &Connection) -> Result<()> {
        // create table
        conn.execute(
            "
            CREATE TABLE person (
                id    INTEGER PRIMARY KEY,
                name  TEXT NOT NULL,
                age   INTEGER
            )
            ",
            (),
        )?;

        // seed data
        let me = Person {
            id: 0,
            name: "Richard".to_string(),
            age: 18,
        };
        conn.execute(
            "INSERT INTO person (name, age) VALUES (?1, ?2)",
            (&me.name, &me.age),
        )?;

        Ok(())
    }

    fn get_people(conn: &Connection) -> Result<Vec<Person>> {
        let mut stmt = conn.prepare("SELECT * FROM person")?;
        let people = stmt
            .query_map([], |row| {
                let person = Person {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    age: row.get(2)?,
                };
                Ok(person)
            })?
            .flat_map(|f| if let Ok(p) = f { Some(p) } else { None })
            .collect::<Vec<_>>();

        Ok(people)
    }
}
