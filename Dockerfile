FROM rust as build
ARG DB_USER
ARG DB_PASSWORD
ARG DB_HOST
ARG DB_NAME
ENV DB_USER $DB_USER
ENV DB_PASSWORD $DB_PASSWORD
ENV DB_HOST $DB_HOST
ENV DB_NAME $DB_NAME
RUN apt-get update && apt-get install -y --no-install-recommends libpq-dev

WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml .

RUN echo "[global.databases]\npostgres_database = { url = \"postgres://$DB_USER:$DB_PASSWORD@$DB_HOST/$DB_NAME\" }\n" > Rocket.toml; \
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
