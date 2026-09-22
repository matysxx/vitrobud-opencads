# OCSSTACK-21 — Implementation plan

1. Review the exact upstream head, tag distance, dependency pins, renderer
   changes, web build entry point, and workflow changes.
2. Merge upstream in an isolated branch/worktree and resolve conflicts in favor
   of current upstream architecture while restoring deliberate fork features.
3. Update project technical metadata and task documentation.
4. Audit R12 integration, browser renderer selection, release guard, container
   build, scripts, YAML, privacy, and Git ancestry.
5. Fast-forward local `main` and publish the anonymized revision to GitHub.
6. Show the exact SSH command and wait for approval before rebuilding the Debian
   runtime at the verified GitHub revision.
7. Verify server Git SHA, image tag, container health, and HTTP endpoints.
8. Complete the local task-wiki context dump.
