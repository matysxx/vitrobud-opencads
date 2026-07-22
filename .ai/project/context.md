# Project Context

## Business overview

This repository maintains an anonymized, reusable fork of Open CAD Studio and
packages its browser build as a private, repository-driven container stack.
Development and verification happen locally, changes are published to GitHub,
and the Debian runtime pulls only a verified revision.

## Project identifiers

- **Project key:** `OCSSTACK`
- **Local directory:** `vitrobud-opencads`
- **Public repository name:** `vitrobud-opencads`
- **Main and rollout branch:** `main`
- **Container name prefix:** `vitrobud-opencads`

Private runtime paths, project/pod names, addresses, and hostnames are
deployment inputs. They must not be copied into public examples or defaults.

## Source of truth

1. This repository for source, image build, and operational behavior.
2. GitHub `main` for rollout revisions.
3. Private, untracked server configuration for runtime values and secrets.

## Collaboration and release rules

- Follow `local -> GitHub -> server`.
- Keep the upstream relationship explicit and auditable.
- Pin deployment to an immutable, locally verified Git revision and image
  digest where available.
- Show every exact SSH command and wait for approval before execution.
- Use Conventional Commits.
