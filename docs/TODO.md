# TODO

## Backup your Sqlite database to Cloudflare R2

- [x] Backup SQLite database
  - https://github.com/rusqlite/rusqlite
  - https://docs.rs/rusqlite/0.29.0/rusqlite/backup/index.html
- [x] Update backup file to ~Google Drive~ **Cloudflare R2**
- [x] Add prefix to the R2 file `backup/prod/project_name`
- [ ] keep recent `n` backups
- [ ] Restore SQLite backup (low priority, because we could run `sqlite_backup` backward to restore the data)
