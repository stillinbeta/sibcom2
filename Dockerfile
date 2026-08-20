FROM rust:1.97-alpine3.24 AS builder
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
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target \
    cargo build --release && \
    cp /build/target/release/updater / && \
    cp /build/target/release/sibcom2 /


FROM alpine:3.24
RUN apk add openssl libgcc

COPY --from=builder /sibcom2 /
COPY --from=builder /updater /
CMD ["/sibcom2"]
