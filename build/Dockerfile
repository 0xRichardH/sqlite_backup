####################################################################################################
## Builder
####################################################################################################
FROM lukemathwalker/cargo-chef:latest-rust-1.74.0-buster AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

# Ensure working C compile setup (not installed by default in arm64 images)
RUN apt update && apt install build-essential -y

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin sqlite_backup

####################################################################################################
## Final image
####################################################################################################
FROM debian:bullseye-slim

RUN useradd -c 'app user' app

RUN apt update && apt install ca-certificates sqlite3 cron -y && rm -rf /var/lib/apt/lists/*

# resolve cron permission issue
RUN rm -rf /etc/cron.*/* && chmod u+s /usr/sbin/cron

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/release/sqlite_backup ./

# Use an unprivileged user.
USER app:app

ENV TZ=Etc/UTC

# Set the entrypoint to run the installed binary in /workdir
# Example: docker run -ti --rm -e APP_ENV=prod --env-file .env -v "$PWD:/app/file" sqlite_backup:latest ./file/source.db
ENTRYPOINT ["./sqlite_backup"]
