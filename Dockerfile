FROM clux/muslrust:stable as builder
WORKDIR /app
RUN USER=root cargo new blog
WORKDIR /app/blog
COPY Cargo.toml Cargo.lock ./
RUN echo 'fn main() { println!("Dummy") }' > ./src/main.rs
RUN cargo build --release
RUN rm -r target/x86_64-unknown-linux-musl/release/.fingerprint/blog-*

COPY src src/
COPY migrations migrations/
COPY .env .env
COPY diesel.toml diesel.toml

RUN cargo build --release --frozen --bin blog

FROM alpine:latest
RUN addgroup -g 1000 blog
RUN adduser -D -s /bin/sh -u 1000 -G blog blog

COPY --from=builder /app/blog/migrations /application/migrations
COPY --from=builder /app/blog/.env /application/.env
COPY --from=builder /app/blog/diesel.toml /application/diesel.toml
COPY --from=builder /app/blog/target/x86_64-unknown-linux-musl/release/blog /application/blog
RUN chown blog:blog blog
USER blog
WORKDIR /application
CMD ["./blog"]
