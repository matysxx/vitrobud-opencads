# OCSSTACK-20 — Implementation plan

1. Add a canonical-repository condition to the weekly release `prepare` job.
2. Keep the schedule, permissions, scripts, and dependent jobs otherwise intact.
3. Validate workflow YAML and assert the guard is present once.
4. Audit the diff and public-repository privacy boundary.
5. Commit with Conventional Commits, fast-forward local `main`, and push GitHub
   `main`.
6. Confirm the published revision; do not roll out to the Debian runtime because
   no build or runtime artifact changed.
