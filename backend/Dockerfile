FROM rust:slim

USER root
ADD backend /usr/src/app/backend
ADD config /usr/src/app/config
WORKDIR /usr/src/app/backend

RUN addgroup -S poggit && adduser -S poggit -G poggit
RUN chown poggit:poggit . -R
USER poggit

RUN cargo install --path .

CMD backend
