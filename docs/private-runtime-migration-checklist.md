# Private runtime migration checklist

- [ ] GitHub commit SHA is recorded and verified locally.
- [ ] Server checkout matches that exact SHA.
- [ ] `.env` and `src/.env` are created locally on the server and untracked.
- [ ] Bind address and port do not conflict with another stack.
- [ ] No certificates or reverse-proxy configuration were added to this repo.
- [ ] Rootless Podman build succeeds for the selected revision.
- [ ] Health and COOP/COEP headers pass.
- [ ] Disposable DWG/DXF open and save pass.
- [ ] Container runs read-only with all capabilities dropped.
- [ ] User service and backup timer are enabled only for the runtime user.
- [ ] Reverse proxy targets only the selected backend port.
- [ ] Final local-domain endpoint uses a trusted HTTPS certificate.
- [ ] Backend port access is restricted to the reverse-proxy host/admin network.
- [ ] COOP/COEP headers survive the external reverse proxy.
- [ ] Rollback revision is recorded before cutover.
