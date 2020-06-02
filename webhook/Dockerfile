FROM rust:slim AS bin

USER root

RUN mkdir -p /usr/src/app/webhook/src /usr/src/app/config/src
RUN echo 'fn main() {}' > /usr/src/app/webhook/src/main.rs
RUN true > /usr/src/app/config/src/lib.rs

ADD webhook/Cargo.toml /usr/src/app/webhook/Cargo.toml
ADD config/Cargo.toml /usr/src/app/config/Cargo.toml

WORKDIR /usr/src/app/webhook
RUN cargo build --release

ADD webhook/src /usr/src/app/webhook/src
ADD config/src /usr/src/app/config/src

RUN cargo install --path .

FROM php:cli-alpine
COPY --from=bin /usr/local/cargo/bin/webhook /usr/bin/webhook

USER root
ADD webhook /usr/src/app
WORKDIR /usr/src/app

RUN addgroup -S poggit && adduser -S poggit -G poggit
RUN chown poggit:poggit . -R
USER poggit

CMD /usr/bin/webhook
