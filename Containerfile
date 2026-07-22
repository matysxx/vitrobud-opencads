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
COPY --from=builder /build/dist /srv
COPY container/Caddyfile /etc/caddy/Caddyfile
USER 1000:1000
EXPOSE 8080
