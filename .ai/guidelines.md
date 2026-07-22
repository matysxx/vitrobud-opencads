# AI Agent Guidelines

This repository is the public, reusable container stack and maintained fork for
Open CAD Studio.

## Required context

Before changing the stack, read:

1. `.ai/project/context.md`
2. `.ai/project/tech-spec.md`
3. the active task in `.ai/prd/`

## Operating rules

- Use the delivery path `local -> GitHub -> server`.
- Use `main` as the rollout branch.
- Show the exact SSH command and wait for explicit approval before every remote
  operation.
- Do not edit production directly or affect unrelated stacks.
- Keep the public repository anonymized and reusable.
- Never commit secrets, private hostnames, private addresses, certificates,
  runtime state, CAD files, or host-specific configuration.
- Keep image/version and host-side settings in root `.env`; keep only container
  runtime secrets in `src/.env`.
- Use Conventional Commits.
