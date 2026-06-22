FROM alpine:3.24.1

COPY --chmod=770 target/aarch64-unknown-linux-gnu/release/cai-reverse-proxy-example .

# the reverse proxy writes logs into a `/logs` folder
VOLUME [ "/logs" ]
CMD [ "cai-reverse-proxy" ]
