ARG RUST_IMAGE=docker.io/library/rust:1.88.0-bookworm
ARG CADDY_IMAGE=docker.io/library/caddy:2.10.0-alpine

FROM ${RUST_IMAGE} AS builder
ARG TRUNK_VERSION=0.21.14
RUN rustup target add wasm32-unknown-unknown \
    && cargo install trunk --version "${TRUNK_VERSION}" --locked
WORKDIR /build
COPY . .
RUN trunk build --release --public-url /

FROM ${CADDY_IMAGE} AS runtime
LABEL org.opencontainers.image.title="Open CAD Studio Web" \
      org.opencontainers.image.description="Rootless static web runtime for Open CAD Studio" \
      org.opencontainers.image.source="https://github.com/matysxx/vitrobud-opencads" \
      org.opencontainers.image.licenses="GPL-3.0-only"
# The official Caddy binary carries cap_net_bind_service. Rootless Podman
# correctly refuses to exec that file when the runtime drops every capability
# and enables no-new-privileges. A normal copy has no file capability and is
# sufficient because this stack listens on unprivileged port 8080.
RUN cp /usr/bin/caddy /usr/local/bin/caddy && chmod 0755 /usr/local/bin/caddy
COPY --from=builder /build/dist /srv
COPY container/Caddyfile /etc/caddy/Caddyfile
USER 1000:1000
EXPOSE 8080
