FROM --platform=linux/arm64 alpine:latest

# if it fails to build inside WSL, running the following commands is necessary:
# $ sudo apt install qemu-user-static
# $ sudo podman run --rm --privileged docker.io/multiarch/qemu-user-static --reset -p yes
#
# source:
#   - https://blog.differentpla.net/blog/2025/04/30/multiarch-containers-podman-docker-1/
RUN apk add --no-cache pkgconf openssl openssl-dev

COPY --chmod=770 target/aarch64-unknown-linux-musl/release/cai-reverse-proxy-example /home/cai-reverse-proxy


# the reverse proxy writes logs into a `/logs` folder
VOLUME [ "/home/logs", "etc/letsencrypt/live/" ]
USER root
CMD [ "/home/cai-reverse-proxy" ]
