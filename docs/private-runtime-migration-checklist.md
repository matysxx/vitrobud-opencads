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
- [ ] Container autostart is enabled only through the runtime user's systemd
      service.
- [ ] Host-side backup timer is either enabled for the runtime user or its
      intentional deferral is recorded privately; no container cron is used.
- [ ] Reverse proxy targets only the selected backend port.
- [ ] Final local-domain endpoint uses a trusted HTTPS certificate.
- [ ] Backend exposure model is recorded privately: reverse-proxy/admin only or
      intentional trusted-LAN direct access.
- [ ] Host firewall configuration passes its syntax/configuration check before
      reload.
- [ ] Effective firewall rules allow the selected backend port only according
      to that exposure model and expose no management sockets.
- [ ] COOP/COEP headers survive the external reverse proxy.
- [ ] Container and final HTTPS endpoint remain healthy after a host restart.
- [ ] No unrelated stack or global Podman service was restarted or modified.
- [ ] Rollback revision is recorded before cutover.
- [ ] Private migration record contains the deployed full SHA, image identity,
      systemd state, proxy route, firewall scope, and rollback reference.

Passing the network/firewall items closes infrastructure acceptance. It does
not replace the separate SHA/image alignment check required by the
`local -> GitHub -> server` rollout model.
