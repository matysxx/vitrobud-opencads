# External reverse proxy

The private runtime is intended to be accessed through an externally managed
reverse proxy using a trusted HTTPS certificate and a local DNS name. The proxy
and certificates remain outside this repository.

## Traffic model

```text
browser -- HTTPS --> external reverse proxy -- HTTP/private network --> web:8080
```

The backend serves static WASM assets and has no application credentials. HTTP
on a trusted, access-controlled private segment is therefore the initial model.
If the network threat model requires encryption on the backend hop, implement
that in the external proxy/runtime override without committing certificates.

## Requirements

- Set `OPENCADS_BIND_ADDRESS` privately to the Debian interface reachable by the
  reverse-proxy host; do not publish the private address in Git.
- Restrict the published backend port to the reverse-proxy host and designated
  administration network.
- Forward the original host and scheme headers.
- Preserve these response headers from the backend:
  - `Cross-Origin-Opener-Policy: same-origin`
  - `Cross-Origin-Embedder-Policy: require-corp`
  - `Cross-Origin-Resource-Policy: same-origin`
- Allow WASM MIME types and large static responses without content rewriting.
- Verify `/healthz` directly and the application root through final HTTPS.

No WebSocket, sticky session, or application upload route is required for the
basic stateless web edition.
