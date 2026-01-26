
FROM rust:1.91 AS builder
WORKDIR /workspace
COPY user_service ./user_service
COPY contracts ./contracts

ENV USER_PORT=4002
ENV USER_HOST=0.0.0.0

WORKDIR /workspace/user_service
RUN cargo build --release --package user_service

FROM debian:bookworm-slim
WORKDIR /service
COPY --from=builder /workspace/user_service/target/release/user_service .
EXPOSE 4002
CMD ["./user_service"]
