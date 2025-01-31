FROM rust:latest AS builder
WORKDIR /usr/src/project

COPY Cargo.toml Cargo.lock ./
COPY domain/ domain/
COPY middlewares/ middlewares/
COPY bartender/ bartender/
COPY todo/ todo/
COPY todo-cli/ todo-cli/

COPY bartender.docker.config.yaml bartender.config.yaml

RUN SQLX_OFFLINE=true cargo install --path bartender/ --locked

FROM rust:latest

RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/bartender /usr/local/bin/bartender
COPY --from=builder /usr/src/project/bartender.config.yaml /etc/bartender/config.yaml

CMD ["bartender", "--config", "/etc/bartender/config.yaml"]
