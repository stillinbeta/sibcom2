FROM gcr.io/distroless/cc

COPY /target/release/sibcom2 /
COPY /target/release/updater /
ENTRYPOINT ["/sibcom2"]