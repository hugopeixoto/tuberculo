FROM rust:1.80-alpine as builder
RUN apk add build-base
WORKDIR /usr/src/tuberculo
COPY . .
RUN cargo install --path .

FROM alpine:latest
RUN apk add yt-dlp
COPY --from=builder /usr/local/cargo/bin/tuberculo /usr/local/bin/tuberculo
WORKDIR /app/
CMD ["tuberculo"]
