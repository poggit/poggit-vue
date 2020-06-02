FROM rust:slim
RUN apt-get update && apt-get install -y libpq-dev
RUN cargo install diesel_cli --no-default-features --features postgres
ADD migrations /migrations

WORKDIR /migrations
CMD bash -c 'diesel migration run --database-url=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost/${POSTGRES_DB}'
