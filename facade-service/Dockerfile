FROM rust:1.75.0-alpine3.19

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

WORKDIR /facade-service
COPY . .

RUN apk add --no-cache openssl-dev musl-dev linux-headers
ENV OPENSSL_DIR=/usr

RUN rustup default nightly
RUN cargo build

CMD ["cargo", "run"]
