FROM --platform=linux/amd64 rust:1.75.0-alpine3.19

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
ENV CGO_ENABLED=1
ENV GOOS=linux
ENV GOARCH=amd64

WORKDIR /facade-service
COPY . .

RUN apk add --no-cache openssl-dev musl-dev linux-headers
ENV OPENSSL_DIR=/usr

RUN rustup default nightly
RUN cargo build

CMD ["cargo", "run"]
