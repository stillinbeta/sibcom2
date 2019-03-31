FROM gcr.io/distroless/cc

COPY /target/release/sibcom2 /
ENTRYPOINT ["/sibcom2"]