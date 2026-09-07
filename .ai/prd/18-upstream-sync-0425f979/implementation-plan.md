# OCSSTACK-18 — Implementation plan

1. Fetch and identify the exact upstream revision and its release context.
2. Work in an isolated branch/worktree created from the verified fork `main`.
3. Merge upstream `0425f979f9ed93433a3f23234d4bff970358e148`
   with a merge commit and resolve conflicts in favor of the current upstream
   architecture while restoring deliberate fork integrations.
4. Update project technical metadata and task documentation.
5. Audit the R12 exporter, WebGL backend, container build, scripts, privacy, YAML,
   shell syntax, and Git graph.
6. Fast-forward local `main` and push the anonymized result to GitHub.
7. After explicit approval of the exact SSH command, update the Debian runtime to
   the verified GitHub revision, rebuild, and verify health and image identity.
8. Write the final local task-wiki context dump.
