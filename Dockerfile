FROM rust:1.80-alpine as builder
WORKDIR /usr/src/tuberculo
COPY . .
RUN apk add build-base
RUN cargo install --path .

FROM alpine:latest
RUN apk add yt-dlp
COPY --from=builder /usr/local/cargo/bin/tuberculo /usr/local/bin/tuberculo
