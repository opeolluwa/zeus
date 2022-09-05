FROM rust:1.63.0 as builder
WORKDIR /app
COPY . .


RUN cargo build --release
RUN rm ./target/release/deps/*


FROM debian:buster-slim as runner
WORKDIR /app
COPY --from=builder /app/target/release/zeus /app/zeus
COPY --from=builder /app/*.toml /app/

CMD ["/app/zeus"]