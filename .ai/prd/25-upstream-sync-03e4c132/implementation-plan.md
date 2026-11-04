# OCSSTACK-25 — Implementation plan

1. Work in an isolated branch from origin/main; preserve the dirty original
   checkout. Merge reviewed exact upstream SHA without squashing.
2. Resolve conflicts for upstream features plus maintained fork changes.
   Inspect dependencies, R12 integration, web entry and publication workflows.
3. Refresh project tech spec and upstream/privacy/rollout documentation.
4. Run packaging tests, shell checks, ancestry and private-data audits.
5. Publish to GitHub main only after local review; require required CI green
   for the exact SHA. Diagnose failures rather than skip tests.
6. Present exact SSH rollout command. After approval, guard expected server
   baseline, back up private env, set sole rollout SHA, build on Debian, replace
   only web service, verify health/autostart/cache and HTTPS startup.
7. Record outcome and rollback in ignored local task wiki.

Risks: large upstream delta and codec API changes can cause non-textual
regressions. CI plus manual importer acceptance are distinct gates.
