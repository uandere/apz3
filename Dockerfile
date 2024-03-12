FROM rust:1.75.0-alpine3.19 as builder
RUN apk update && apk upgrade
RUN apk add pkgconfig openssl openssl-dev musl-dev gcc

WORKDIR /usr/src/app
COPY .. .
RUN rustup target add x86_64-unknown-linux-musl

RUN export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-gnu-gcc
RUN export CC=aarch64-linux-gnu-gcc
RUN cargo build -p prqlc --release --target=aarch64-unknown-linux-musl

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM rust:alpine3.19 as facade-service
RUN apk update && apk upgrade
RUN apk add pkgconfig openssl openssl-dev musl-dev gcc
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/facade-service /usr/local/bin/facade-service
CMD ["facade-service"]
