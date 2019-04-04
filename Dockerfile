FROM gcr.io/distroless/static

COPY /target/release/sibcom2 /
COPY /target/release/updater /
ENTRYPOINT ["/sibcom2"]