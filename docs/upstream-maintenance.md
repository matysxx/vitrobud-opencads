# Upstream maintenance

Remotes:

- `origin` — maintained deployment fork
- `upstream` — `HakanSeven12/OpenCADStudio`

Update procedure:

```bash
git fetch upstream main
git log --oneline --decorate main..upstream/main
git merge --no-ff upstream/main
```

Review upstream web/native behavior, Cargo changes, Trunk configuration, and
release workflows before merging. Re-run stack validation and build testing.
Never force-push rewritten upstream history to `main`.
