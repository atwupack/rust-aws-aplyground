FROM rust:1.61 as builder
WORKDIR /usr/src/rust-aws-ecs
COPY Cargo.* ./
COPY src/ src/
RUN cargo install --path .

FROM gcr.io/distroless/cc
COPY --from=builder /usr/local/cargo/bin/rust-aws-ecs /
COPY application.yaml /
EXPOSE 8080
CMD ["./rust-aws-ecs"]