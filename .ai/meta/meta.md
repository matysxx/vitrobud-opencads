# Meta — AI Instructions Management

This directory contains tools for initializing, auditing, and maintaining the `.ai/` instruction structure.

These files are **portable** — they work across any project.

The installed `.ai/wiki/` directory provides local-only task context exchange for agents. Its policy and templates are portable, but `wiki/tasks/` entries are operational memory and must not be committed.

## Files

| File | When to use |
|------|-------------|
| `init.md` | First-time setup of `.ai/` on a new project, or re-initialization |
| `audit.md` | Auditing an existing `.ai/` structure for gaps and inconsistencies |
| `discovery.md` | Auto-detection rules and question catalogue for building project context |

## Templates

| File | Purpose |
|------|---------|
| `templates/context.md` | Template for `project/context.md` |
| `templates/tech-spec.md` | Template for `project/tech-spec.md` |
| `templates/environments.md` | Template for `project/environments.md` |

## Examples

| File | Purpose |
|------|---------|
| `guidelines-example-shopware.md` | Example `guidelines.md` for a Shopware 6 e-commerce project |

## Typical Usage

1. User says: "Set up AI instructions for this project" (or similar)
2. Agent reads `init.md` → determines mode (full scaffold vs audit)
3. Agent runs discovery (`discovery.md`) → auto-detects what it can, asks user the rest
4. Agent generates/updates project-specific files from templates
5. Agent verifies local task wiki ignore rules so cross-role context remains private
