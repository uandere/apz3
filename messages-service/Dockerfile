FROM rust:1.75.0-alpine3.19

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8002

WORKDIR /messages-service
COPY . .

RUN apk add --no-cache openssl-dev musl-dev linux-headers

RUN rustup default nightly
RUN cargo build

CMD ["cargo", "run"]