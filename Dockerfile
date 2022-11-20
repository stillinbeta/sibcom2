FROM rust:1.65-alpine as builder
RUN mkdir /build
RUN apk add pkgconfig openssl openssl-dev musl-dev
COPY assets /build/assets
COPY bmon /build/bmon
COPY generator /build/generator
COPY minify /build/minify
COPY server /build/server
COPY updater /build/updater
COPY site.yaml /build/
COPY Cargo.toml Cargo.lock /build/
WORKDIR /build
RUN cargo build --release

FROM alpine

COPY --from=builder /build/target/release/sibcom2 /
COPY --from=builder /build/target/release/updater /
ENTRYPOINT ["/sibcom2"]