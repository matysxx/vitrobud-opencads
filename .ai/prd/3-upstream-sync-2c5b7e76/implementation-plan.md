# Implementation Plan: OCSSTACK-3

## Overview

Merge the reviewed upstream delta into a dedicated local branch, verify that
automatic integration preserved the fork-only R12 feature and container build,
then deliver it through `local -> anonymized GitHub -> server`.

## Steps

1. Fetch `origin` and `upstream`; record exact heads, merge base, commits, files,
   dependency changes, and a synthetic merge result.
2. Create `chore/OCSSTACK-3-upstream-2c5b7e76` from local `main` and merge exact
   `2c5b7e76` with `--no-ff --no-commit` for review.
3. Review shared application integration points and confirm `EXPORTDXFR12` /
   `EXPORTR12`, exporter module, tests, and normal save isolation remain intact.
4. Run the supported web build and, if upstream gates a helper still required
   by the no-default-features path, remove only the invalid feature gate and
   rerun the exact-revision verification.
5. Update the reviewed upstream baseline in `.ai/project/tech-spec.md` and run
   privacy, whitespace, shell, Compose/static, ancestry, and command audits.
6. After separate approval, create a Conventional Commit merge and push the
   branch; verify the exact public GitHub revision and merge it to `main`.
7. With a separately displayed and approved SSH command, build/test the exact
   GitHub revision in isolation: targeted exporter tests, web OCI build, and
   disposable `/healthz` smoke test.
8. After a final separately approved SSH command, update only this stack,
   retain the prior image/private `.env`, and verify health and user systemd.

## Rollback

Before activation retain the current `ad018a8` image and private `.env`. On any
build or health failure, leave or restore only `vitrobud-opencads-web`; do not
touch reverse proxy, firewall, other stacks, or host-wide Podman state.
