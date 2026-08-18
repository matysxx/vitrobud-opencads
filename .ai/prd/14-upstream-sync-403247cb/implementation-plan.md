# OCSSTACK-14 Implementation Plan

1. Fetch and record upstream `403247cb` and GitHub `main`.
2. Integrate upstream in an isolated worktree based on the verified local
   rollout branch, leaving OCSSTACK-12 untouched.
3. Resolve conflicts narrowly: prefer current upstream architecture while
   reapplying only the fork-specific R12 command wiring where required.
4. Update the technical baseline and container builder pins from the merged
   manifest and lock file without introducing a second version source.
5. Audit command registration, web/native export paths, exporter tests,
   anonymization, conflict markers, shell scripts, Compose, and formatting.
6. Commit with Conventional Commits, fast-forward local `main`, and push the
   exact anonymized revision to GitHub `main`.
7. Show the exact SSH rollout command and wait for explicit approval.
8. On Debian, back up private rollout configuration, fetch GitHub `main`,
   rebuild only the Open CAD Studio image, recreate only its web service, and
   verify revision, health, version, and HTTPS behavior.
