# SQLite Backup

## Run the example from the code

```shell
cargo run -- --db ./source.db --project-name test-project --data-retention 7
```

## Build the docker image and run it

- build the docker image

```shell
docker build -f ./build/Dockerfile -t sqlite_backup:latest .
```

- run the docker image

```shell
docker run -ti --rm -e APP_ENV=prod --env-file .env -v "$PWD:/app/file" sqlite_backup:latest --db ./file/source.db --project-name test-project --data-retention 7
```

## Run the remote docker image

```shell
docker run -ti --rm -e APP_ENV=prod --env-file .env -v "$PWD:/app/file" ghcr.io/0xrichardh/sqlite_backup:main --db ./file/source.db --project-name test-project --data-retention 7
```

## Use Cron job inside Docker Container to run sqlite_backup

- `config/crontab` file

```shell
# * * * * * command to be executed
# | | | | |
# | | | | |
# | | | | |
# | | | | |_______________ Day of the Week (0 - 6)(Sunday to Saturday)
# | | | |
# | | | |_______________ Month of the Year (1 - 12)
# | | |
# | | |_______________ Day of the Month (1 - 31)
# | |
# | |_______________ Hour (0 - 23)
# |
# |_______________ Minute (0 - 59)
#
12 0 * * * bash -c ". /app/config/container.env; /app/sqlite_backup --db /app/test-project/db.sqlite3 --project-name test-project --data-retention 7" >> /app/config/backup.log 2>&1
```

- docker compose file

```yaml
version: "3"
services:
  test-backup:
    container_name: test-backup
    image: ghcr.io/0xrichardh/sqlite_backup:latest
    restart: unless-stopped
    volumes:
      - type: bind
        source: /vw-data
        target: /app/test-project
      - type: bind
        source: ./test-project/backup/
        target: /app/config
    environment:
      - APP_ENV=prod
      - BUCKET_NAME=backup
    env_file:
      - .env
    entrypoint: bash -c "declare -p | grep -E 'APP_ENV|BUCKET_NAME|ACCOUNT_ID|ACCESS_KEY_ID|SECRET_ACCESS_KEY|GPG_PASSPHRASE' > config/container.env && cat config/crontab | crontab - && cron -f"
```
