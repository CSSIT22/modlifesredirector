FROM rust as builder

RUN USER=root cargo new --bin modlifesredirector
WORKDIR /modlifesredirector

COPY ./Rocket.toml .

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/modlifesredirector*
RUN cargo build --release


FROM debian:buster-slim

COPY --from=builder /modlifesredirector/target/release/modlifesredirector .

ENV ROCKET_ADDRESS=0.0.0.0


EXPOSE 8000

ENTRYPOINT [ "./modlifesredirector" ]
