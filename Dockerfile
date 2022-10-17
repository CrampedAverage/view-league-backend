FROM rust AS builder

COPY . /app

WORKDIR /app

RUN cargo builder --release

FROM debian:buster-slim

COPY --from=builder /app/target/release/view-league-backend /app/view-league-backend 
WORKDIR /app

CMD ["./view-league-backend"]