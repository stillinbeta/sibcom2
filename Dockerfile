FROM rust:1.65-alpine3.16 as builder
RUN mkdir /build
RUN apk add pkgconfig openssl openssl-dev musl-dev
COPY assets /build/assets
COPY bmon /build/bmon
COPY generator /build/generator
COPY minify /build/minify
COPY server /build/server
COPY updater /build/updater
COPY site.yaml /build/
COPY .cargo /build/.cargo
COPY Cargo.toml Cargo.lock /build/
WORKDIR /build
RUN cargo build --release

FROM alpine:3.16
RUN apk add openssl libgcc

COPY --from=builder /build/target/release/sibcom2 /
COPY --from=builder /build/target/release/updater /
CMD ["/sibcom2"]