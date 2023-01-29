FROM rust:1.66 AS builder
COPY . .
RUN apt-get update && apt-get install -y musl
RUN dpkg -s musl
ENV SQLX_OFFLINE true
ENV CC_x86_64_unknown_linux_musl=musl-gcc
RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get install gcc-x86-64-linux-gnu
RUN cargo build --release --target x86_64-unknown-linux-musl --bin erp

FROM alpine:3.13 AS runtime
RUN addgroup -S myuser && adduser -S myuser -G myuser
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/erp /usr/local/bin/
USER myuser
CMD ["/usr/local/bin/erp"]


