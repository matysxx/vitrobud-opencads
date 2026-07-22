# First parallel start plan

1. Select an unused loopback or private bind port in the untracked `.env`.
2. Build only this stack with `./dev-ops/setup`.
3. Verify health, COOP/COEP headers, logs, and container hardening.
4. Test disposable DWG and DXF open/save flows directly on the test endpoint.
5. Add an external HTTPS reverse-proxy route only after the direct test passes.
   Verify the final local-domain URL and COOP/COEP headers through the proxy.
6. Keep any existing CAD workflow unchanged during observation.

The stack is stateless and has no database migration. Parallel operation is the
default first-run strategy.
