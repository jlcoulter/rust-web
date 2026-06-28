# Build stage
FROM rust:1.87-alpine AS builder
RUN apk add --no-cache musl-dev
ENV SQLX_OFFLINE=true
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
RUN cargo build --release

# Final stage
FROM scratch
COPY --from=builder /app/target/release/rust-web-template /rust-web-template
EXPOSE 3000
ENTRYPOINT ["/rust-web-template"]