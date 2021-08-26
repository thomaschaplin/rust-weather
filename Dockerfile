FROM rust:latest as build

USER root

WORKDIR /rust-weather

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./.env ./.env

RUN cargo build --release

FROM rust:latest

COPY --from=build /rust-weather/target/release/rust-weather .

CMD ["./rust-weather"]
