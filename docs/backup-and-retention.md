# Backup and retention

The basic web runtime is stateless. Host-side backup protects only untracked
operational configuration: `.env`, `src/.env`, and an optional
`compose.override.yaml`.

- schedule source: `BACKUP_ON_CALENDAR` in root `.env`
- default: daily at 03:30
- retention source: `BACKUP_RETENTION_DAYS` in root `.env`
- default: 30 days
- destination: `dev-ops/storage/backup`
- permissions: directory `0700`, archives `0600`

Install the user timer with:

```bash
./dev-ops/install-systemd-user-backup
systemctl --user enable --now vitrobud-opencads-backup.timer
```

Backups contain private values and must never be committed or uploaded to the
public repository. CAD drawings are not part of this backup model.
