# TODO

## Backup your Sqlite database to Cloudflare R2

- [x] Backup SQLite database
  - https://github.com/rusqlite/rusqlite
  - https://docs.rs/rusqlite/0.29.0/rusqlite/backup/index.html
- [x] Update backup file to ~Google Drive~ **Cloudflare R2**
- [x] Add prefix to the R2 file `backup/prod/project_name`
- [x] Keep recent `n` backups
- [x] Add GPG encryption support
  - encrypt: `add env variable GPG_PASSPHRASE`
  - decrypt: `gpg -o backup.tar.gz -d backup.tar.gz.gpg`
- [x] Beauty the output log
