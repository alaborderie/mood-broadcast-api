FROM rust as build

RUN apt-get update && apt-get install -y --no-install-recommends libpq-dev

WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml .

RUN printf "[global.databases]\npostgres_database = { url = \"postgres://$DB_USER:$DB_PASSWORD@$DB_HOST/$DB_NAME\" }\n" > Rocket.toml; \
    head -c16 /dev/urandom > src/secret.key; \
    cargo build --release

FROM debian:bookworm-slim

RUN mkdir app
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends libpq-dev

COPY --from=build /app/target/release/mood-broadcast-api .
COPY --from=build /app/Rocket.toml .
COPY --from=build /app/diesel.toml .

EXPOSE 8000

ENTRYPOINT ["/app/mood-broadcast-api"]
