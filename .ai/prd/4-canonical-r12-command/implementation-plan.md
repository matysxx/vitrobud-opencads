# Implementation Plan: OCSSTACK-4

1. Remove `EXPORTR12` from the command registry and dispatch match.
2. Update the application message comment, README, and R12 export guide to name
   only `EXPORTDXFR12`.
3. Audit references, whitespace, privacy, and the unchanged fail-closed path.
4. After separate approval, commit and push an anonymized integration branch.
5. Verify the exact public revision with R12 tests and a web image build before
   a separately approved repository-first rollout.

## Out of Scope

Conversion or omission of `DIMENSION_LINEAR`, `HATCH`, `INSERT`, `LEADER`,
`MTEXT`, or `TEXT`. Those require an explicit geometry-normalization design and
consumer tests.
