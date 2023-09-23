## SQLite Backup

- run the example

```
cargo run  ./source.db
```

- build the docker image and run it

```bash
docker build -f ./build/Dockerfile -t sqlite_backup:latest .
```

```
docker run -ti --rm -e APP_ENV=prod --env-file .env -v "$PWD:/app/file" sqlite_backup:latest ./file/source.db
```

```

```
