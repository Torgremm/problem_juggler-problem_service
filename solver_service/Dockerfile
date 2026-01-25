FROM rust:1.91 AS builder
WORKDIR /workspace
COPY solver_service ./solver_service
COPY contracts ./contracts

WORKDIR /workspace/solver_service
RUN cargo build --release --package solver_service

FROM debian:bookworm-slim
WORKDIR /service
COPY --from=builder /workspace/solver_service/target/release/solver_service .
CMD ["./solver_service"]
