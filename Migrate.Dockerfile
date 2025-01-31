FROM rust:latest

RUN cargo install sqlx-cli --no-default-features --features postgres

RUN apt-get update && apt-get install -y postgresql-client && rm -rf /var/lib/apt/lists/*

WORKDIR /migrate

COPY todo/migrations /migrate/todo/migrations
COPY bartender/migrations /migrate/bartender/migrations
