# Init — AI Instructions Setup

## Purpose

Initialize or update the `.ai/` instruction structure on any project.

## Prerequisites

- Access to the project root filesystem
- Git repository initialized
- User available for answering discovery questions

## Instructions

### Step 1: Determine Mode

Check the current state of the project:

```
IF .ai/ directory does not exist OR is empty:
  → Full Scaffold Mode (Step 2)
ELSE IF .ai/ exists but is incomplete or outdated:
  → Audit Mode (Step 3)
```

How to decide:
- No `.ai/` directory → Full Scaffold
- `.ai/` exists but missing key files (guidelines.md, project/, roles/, etc.) → Full Scaffold
- `.ai/` exists with structure in place → Audit Mode

---

### Step 2: Full Scaffold Mode

Create the complete `.ai/` structure from scratch.

#### 2.1 Create directory structure

```
.ai/
├── guidelines.md
├── project/
│   ├── context.md
│   ├── tech-spec.md
│   └── environments.md
├── roles/
│   ├── coder/
│   │   ├── coder.md
│   │   ├── coding-standards.md
│   │   ├── testing-rules.md
│   │   └── code-quality.md
│   ├── debugger/
│   │   └── debugger.md
│   ├── designer/
│   │   ├── designer.md
│   │   ├── gather-requirements.md
│   │   └── design-principles.md
│   └── manager/
│       ├── manager.md
│       ├── create-task.md
│       ├── close-task.md
│       └── commit-message.md
├── prd/
│   ├── prd.md
│   └── task-status.local.md
├── wiki/
│   ├── README.md
│   ├── context-policy.md
│   ├── task-summary-template.md
│   ├── observations-template.md
│   ├── reflection-template.md
│   ├── heartbeat-template.md
│   └── tasks/
└── meta/
    ├── meta.md
    ├── init.md
    ├── audit.md
    └── discovery.md
```

Optional directories (add when needed):
- `roles/e2e-tester/` — E2E testing instructions
- `flows/` — multi-role workflow definitions

#### 2.2 Copy portable files

Copy these directories **as-is** from the reference structure (they are project-independent):

- `roles/` — all role directories and files
- `meta/` — all files
- `wiki/` — local task context policy, templates, and documentation
- `guidelines.md` — use an appropriate example as starting point (e.g., `meta/guidelines-example-shopware.md`), then customize for the project

#### 2.3 Set up agent entry points

Each AI tool needs an entry point that redirects to `.ai/guidelines.md`. Create the relevant files based on which tools the project uses:

**Claude Code** (`.claude/CLAUDE.md`):
```markdown
Read and follow all instructions from `.ai/guidelines.md` before starting any task.
```

**Codex** (`.codex/AGENTS.md`):
```markdown
Read and follow all instructions from `.ai/guidelines.md` before starting any task.
```

**Junie** (`.junie/guidelines.md`):
```markdown
Read and follow all instructions from `.ai/guidelines.md` before starting any task.
```

> Ask the user which AI tools the project uses. Only create entry points for tools in use.

#### 2.4 Set up .gitignore

Add these rules to the project's `.gitignore` (check if they already exist first):

```gitignore
# AI agent instructions — local-only files
.ai/prd/*
!.ai/prd/prd.md

# AI task context wiki — local-only files
.ai/wiki/tasks/
```

> This ensures task directories (requirements, plans) stay local, while the task index is committed. The `task-status.local.md` is covered by the `prd/*` wildcard.
> This also ensures task wiki entries stay local and cannot be committed accidentally.
> Do not ignore the whole `.ai/wiki/` directory. Only `tasks/` is local-only; policy and template files are reusable procedure.

#### 2.5 Run Discovery

Go to `meta/discovery.md` → run auto-detection and ask the user questions.

#### 2.6 Generate project-specific files

Based on discovery results, generate:

- `project/context.md` — from discovery answers (project identity, workflow tools, task management)
- `project/tech-spec.md` — from discovery answers (tech stack, QA tools, infrastructure)
- `project/environments.md` — from discovery answers (environments, access, logs, services)
- `prd/prd.md` — empty task index with correct `{PROJECT_KEY}`
- `prd/task-status.local.md` — empty status file
- `wiki/tasks/` — empty local-only task wiki directory; task files are created from wiki templates when a task starts

Before committing generated project files, verify that `project/` files contain no private IPs, customer data, hostnames, credentials, production paths, service maps, logs, or host-local configuration. If a project snapshot is needed in Git, store it as a sanitized project document such as `project/context-snapshot.md`, not inside `wiki/tasks/`.

#### 2.7 Verify

Run the audit (`meta/audit.md`) to confirm everything is in place.

---

### Step 3: Audit Mode

The `.ai/` structure already exists. Run the audit to find gaps.

1. Read `meta/audit.md` and execute the full audit checklist
2. Present findings to the user: what's OK, what's missing, what's inconsistent
3. Run discovery (`meta/discovery.md`) **only for missing information**
4. Update/create files as needed
5. Re-run audit to confirm

---

### Notes

- Agent entry point files (`.claude/CLAUDE.md`, `.codex/AGENTS.md`, `.junie/guidelines.md`) are created in step 2.3. If they already exist with different content, ask the user before overwriting.
- Portable files (`roles/`, `meta/`) must not contain project-specific content. If they do after init, something went wrong.
- `project/` and `prd/` are project-specific and generated per-project.
- `wiki/tasks/` is local operational memory. Never commit task-specific wiki entries.
- GitHub should contain reusable procedure and templates; local-only files should contain session memory, heartbeat, observations, handoffs, status, and deployment details.

## Artifacts

- Complete `.ai/` directory structure
- Agent entry point files (`.claude/CLAUDE.md`, `.codex/AGENTS.md`, `.junie/guidelines.md`)
- `.gitignore` rules for AI instruction files
