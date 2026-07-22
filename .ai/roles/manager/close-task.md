# Close Task

## Purpose

Procedure for closing a task after implementation is complete — verify acceptance criteria, run QA, commit, and optionally create a PR and update the issue tracker.

## Prerequisites

- `project/context.md` — project key, issue tracker type, commit format, MCP availability
- `project/tech-spec.md` — QA tools (linters, static analysis, test runner)
- Task's `requirements.md` — acceptance criteria to verify

## Important

> Before performing any step that references another procedure file, read that file first. Do not rely on memory or assumptions about the procedure content.

## Steps

1. **Verify AC** — confirm that all acceptance criteria in `requirements.md` are fulfilled (all checkboxes checked)
2. **Run QA** — use QA tools defined in `project/tech-spec.md`:
   - Scope QA to changed files: `git status --porcelain` (exclude deleted files)
   - Pass file list to QA tools (linters, static analysis, tests)
3. **Commit** — read the commit format file specified in `project/context.md` (`conventional-commits.md` or `custom-commits.md`), then create the commit following the format exactly
4. **Update status** — set to `Done` in `prd/task-status.local.md`
5. **(Optional) Create PR** — if the user requests it:
   - Read `pr-description.md` — follow the procedure exactly
   - Write the PR description to `prd/{task-dir}/pr-description.md` first
   - Present to the user for review before creating the actual PR
6. **(Optional) Update ticket** — if a PR was created in step 5 and an issue tracker is configured:
   - Read `update-ticket.md` — determine the correct scenario
   - Compose the full comment text following the structure from `update-ticket.md`
   - Present both the comment and the proposed transition to the user for approval
   - Do NOT transition the ticket or post comments without explicit user approval
   - If the user declines or MCP is unavailable, skip

## Checklist

```
[ ] All AC checkboxes checked in requirements.md
[ ] QA passed (linters, static analysis, tests)
[ ] Commit format file read before committing
[ ] Commit created with proper message
[ ] Status set to Done in prd/task-status.local.md
[ ] pr-description.md read before creating PR (if requested)
[ ] PR description written to prd/{task-dir}/pr-description.md
[ ] PR description reviewed by user before PR creation
[ ] PR created (if requested and approved)
[ ] update-ticket.md read before updating ticket (if applicable)
[ ] Ticket comment composed and presented for user approval
[ ] Ticket updated (only after user approval)
```
