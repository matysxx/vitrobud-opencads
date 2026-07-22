# Container quick start

## Prerequisites

- rootless Podman, Python 3, `curl`, and `sha256sum`; `./dev-ops/setup`
  installs the checksum-verified Compose provider privately under
  `dev-ops/.runtime/`
- Git
- a user session capable of running `systemd --user`

## Private configuration

```bash
cp .env.dist .env
cp src/.env.dist src/.env
chmod 600 .env src/.env
```

Set the bind address, published port, Compose project name, and full verified
rollout SHA in `.env`. Never commit either `.env` file.

## Start and verify

```bash
./dev-ops/setup
./dev-ops/compose ps
curl -fsS http://127.0.0.1:8088/healthz
curl -fsSI http://127.0.0.1:8088/ | grep -Ei 'cross-origin-(opener|embedder)-policy'
```

Open the published URL in a WebGL2/WebGPU-capable browser. Test a disposable
DWG/DXF file using the browser picker and confirm that Save creates a browser
download. Do not use confidential production drawings for the first test.
