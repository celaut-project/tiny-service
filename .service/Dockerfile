FROM rust:latest AS builder
COPY SRC
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder /tiny-service/target/release/tiny-service /tiny-service
