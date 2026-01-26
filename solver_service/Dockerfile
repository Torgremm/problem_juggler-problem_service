FROM rust:1.91 AS builder
WORKDIR /workspace
COPY solver_service ./solver_service
COPY contracts ./contracts

ENV SOLVER_PORT=4000
ENV SOLVER_HOST=0.0.0.0

WORKDIR /workspace/solver_service
RUN cargo build --release --package solver_service

FROM debian:bookworm-slim
WORKDIR /service
COPY --from=builder /workspace/solver_service/target/release/solver_service .
EXPOSE 4000
CMD ["./solver_service"]
