FROM rust:1.81-bullseye AS builder

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

WORKDIR /app

COPY . .

RUN cargo build --release

FROM rust:1.81-bullseye

WORKDIR /app
ENV ROCKET_ADDRESS=0.0.0.0

ENV ROCKET_PORT=8080

COPY --from=builder /app/target/release/backend .

CMD ["./backend"]
