FROM rust:latest AS builder
WORKDIR /usr/src/project

COPY Cargo.toml Cargo.lock ./
COPY domain/ domain/
COPY middlewares/ middlewares/
COPY bartender/ bartender/
COPY todo/ todo/
COPY todo-cli/ todo-cli/

COPY todo.docker.config.yaml todo.config.yaml

RUN SQLX_OFFLINE=true cargo install --path todo/ --locked

FROM rust:latest

RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/todo /usr/local/bin/todo
COPY --from=builder /usr/src/project/todo.config.yaml /etc/todo/config.yaml

CMD ["todo", "--config", "/etc/todo/config.yaml"]
