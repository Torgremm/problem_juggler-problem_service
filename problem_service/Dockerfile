FROM rust:1.91 AS builder
WORKDIR /workspace
COPY problem_service ./problem_service
COPY contracts ./contracts

ENV PROBLEM_PORT=4001
ENV PROBLEM_HOST=0.0.0.0

WORKDIR /workspace/problem_service
RUN cargo build --release --package problem_service

FROM debian:bookworm-slim
WORKDIR /service
COPY --from=builder /workspace/problem_service/target/release/problem_service .
EXPOSE 4001
CMD ["./problem_service"]
