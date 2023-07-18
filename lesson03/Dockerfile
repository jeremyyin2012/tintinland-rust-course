FROM rust:1.70-bullseye as builder

WORKDIR /app
ADD . /app
RUN cargo build --release


FROM debian:bullseye

WORKDIR /app
EXPOSE 8000

COPY --from=builder /app/target/release/edu /app/edu
CMD ["./edu"]