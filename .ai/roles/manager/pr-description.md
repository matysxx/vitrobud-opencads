# PR Description

## Purpose

Procedure for generating a pull request description from branch history and project context.

## Prerequisites

- `project/context.md` — project key (`{PROJECT_KEY}`), repository host, issue tracker type, MCP availability
- `project/tech-spec.md` — QA tools (linters, test runner), language extensions
- `manager.md` — branch naming patterns

## Trigger

User requests a PR description, e.g.:
- "Create PR description for task 2993, merge to 6.7"
- "PR for {PROJECT_KEY}-3001 into main"

## Input Parsing

1. **Task number** — extract the numeric part. The full ticket key uses the project key from `project/context.md`: `{PROJECT_KEY}-{number}`.
2. **Target branch** — the branch to merge into:
   - If the user specifies a version (e.g., `6.7`), match it to a remote branch (`git branch -r | grep release` or similar)
   - If the user specifies a branch name directly (e.g., `main`), use it
   - **If not specified — ask the user.** Do not guess.
3. **Source branch** — resolve from the task number using the branch naming pattern from `manager.md`:
   - Pattern: `feature/{PROJECT_KEY}-{number}-*` or `fix/{PROJECT_KEY}-{number}-*`
   - Verify the branch exists: `git branch -r | grep {PROJECT_KEY}-{number}`
   - If not found, check the current local branch

## Data Collection

Once source and target branches are resolved, gather all information needed for the template.

### 1. Commit History

```bash
git log --oneline {target_branch}..{source_branch}
```

Read every commit message. These drive the "What changed?" section.

### 2. Diff Summary

```bash
git diff --stat {target_branch}...{source_branch}
```

Understand the scope: which files, modules, and areas were touched.

### 3. Linter Status

Run the linting/static analysis tools listed in `project/tech-spec.md`, scoped to changed files:

```bash
git diff --name-only {target_branch}...{source_branch} -- '*.{lang_extension}'
```

If all pass, check the linter box in the template. If any fail, leave unchecked and note in Caveats.

### 4. Tests

- Check if any test files were added or modified in the diff
- Run the test runner from `project/tech-spec.md`, scoped to the relevant area
- If tests pass, check the tests box
- If no tests were written and the change is non-trivial, leave unchecked and note in Caveats

### 5. E2E Tests

- Assume E2E tests pass (check the box) unless the user states otherwise
- If E2E test files were modified in the diff, mention this in "What changed?"

### 6. Issue Tracker (Optional)

If MCP tools for the issue tracker are available, read the ticket `{PROJECT_KEY}-{number}` for:
- Summary and description (for PR title context)
- Test case section (for "How to test")
- Acceptance criteria

If MCP is not available, skip and use commit messages as the primary source.

## PR Description Template

> **MANDATORY:** Use the project's PR template if one exists (e.g., `.github/pull_request_template.md`). If no project template exists, use the default template below. Do NOT invent custom sections.

### Default Template

```markdown
Tasks list:

- [{linter_status}] I ran the linter / static analysis.
- [{test_status}] I have written the tests.
- [x] I made sure E2E tests are passing.

## What changed? (required)

{Bulleted list derived from commit messages. Group related changes.
Write in human-readable form — not raw commit messages.
Focus on WHAT and WHY, not implementation details.}

## How to test?

{Default: "Run unit tests. See the issue tracker ticket for manual
testing steps."}
{If specific manual steps are obvious from the diff, list them.}

## Caveats

{Default: "None."}
{If there are genuine caveats — incomplete coverage, known limitations,
migration requirements, env var changes — list them here.}

## Documentation (required)

{Default: "This PR and the issue tracker ticket."}
{If documentation-worthy patterns, architectural decisions, or
configuration changes exist, mention them.}

## Important References

{Default: "None."}
{If relevant external links, related PRs, or architectural references
exist, list them.}
```

## Output

Write the completed PR description to:

```
prd/{task-number}-{slug}/pr-description.md
```

Include the PR title at the top of the output file:

```markdown
# PR: {PROJECT_KEY}-{number} {Short description from commits}

> Target: `{target_branch}` | Source: `{source_branch}`

---

{template content}
```

If the task directory already exists, write into it. If not, create it.

## Post-Generation

1. Present the generated description to the user for review
2. If the user approves and requests it — create the actual PR on the repository host (defined in `project/context.md`)
3. Do **not** create the PR automatically without user confirmation

## Checklist

```
[ ] Task number and target branch resolved
[ ] Commit history and diff analyzed
[ ] Linters run and results recorded
[ ] Tests checked and results recorded
[ ] PR template located (project template or default)
[ ] PR description written to prd/{task-number}-{slug}/pr-description.md
[ ] Description reviewed with the user before creating the actual PR
[ ] PR created on repository host (if requested and approved)
```
