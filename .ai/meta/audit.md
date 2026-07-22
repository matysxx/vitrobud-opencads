# Audit — AI Instructions Health Check

## Purpose

Audit an existing `.ai/` structure to find gaps, inconsistencies, and outdated content.

## Prerequisites

- Existing `.ai/` directory in the project

## Instructions

Execute each check. Record results as PASS / FAIL / MISSING.

---

### 1. Directory Structure

Verify these directories exist:

```
[ ] .ai/
[ ] .ai/project/
[ ] .ai/roles/coder/
[ ] .ai/roles/debugger/
[ ] .ai/roles/designer/
[ ] .ai/roles/manager/
[ ] .ai/prd/
[ ] .ai/wiki/
[ ] .ai/wiki/tasks/
[ ] .ai/meta/
```

### 2. Required Files

Verify these files exist and are non-empty:

**Root:**
```
[ ] .ai/guidelines.md
```

**Project-specific:**
```
[ ] .ai/project/context.md
[ ] .ai/project/tech-spec.md
[ ] .ai/project/environments.md
```

**Coder (portable):**
```
[ ] .ai/roles/coder/coder.md
[ ] .ai/roles/coder/coding-standards.md
[ ] .ai/roles/coder/testing-rules.md
[ ] .ai/roles/coder/code-quality.md
```

**Debugger (portable):**
```
[ ] .ai/roles/debugger/debugger.md
```

**Designer (portable):**
```
[ ] .ai/roles/designer/designer.md
[ ] .ai/roles/designer/gather-requirements.md
[ ] .ai/roles/designer/design-principles.md
```

**Manager (portable):**
```
[ ] .ai/roles/manager/manager.md
[ ] .ai/roles/manager/create-task.md
[ ] .ai/roles/manager/close-task.md
[ ] .ai/roles/manager/commit-message.md
```

**PRD:**
```
[ ] .ai/prd/prd.md
[ ] .ai/prd/task-status.local.md
```

**Wiki:**
```
[ ] .ai/wiki/README.md
[ ] .ai/wiki/context-policy.md
[ ] .ai/wiki/task-summary-template.md
[ ] .ai/wiki/observations-template.md
[ ] .ai/wiki/reflection-template.md
[ ] .ai/wiki/heartbeat-template.md
```

**Meta:**
```
[ ] .ai/meta/meta.md
[ ] .ai/meta/init.md
[ ] .ai/meta/audit.md
[ ] .ai/meta/discovery.md
```

### 3. Index Consistency

For each role directory, verify that every `.md` file in the directory is listed in the index file:

```
[ ] roles/coder/coder.md lists all files in roles/coder/
[ ] roles/debugger/debugger.md lists all files in roles/debugger/
[ ] roles/designer/designer.md lists all files in roles/designer/
[ ] roles/manager/manager.md lists all files in roles/manager/
[ ] meta/meta.md lists all files in meta/
```

### 4. Cross-Reference Validation

Verify that file references between documents point to existing files:

```
[ ] All references in guidelines.md point to existing files
[ ] All references in role indexes point to existing files
[ ] All references in procedures point to existing files
```

### 5. Portability Check

Portable files (`roles/`, `meta/`) must NOT contain:

```
[ ] No hardcoded project keys (e.g., "ALF", "PROJ") — only {PROJECT_KEY}
    Exception: examples clearly marked as examples with a note about project/context.md
[ ] No hardcoded tool names (e.g., "Symfony", "React", "PHPUnit")
    → should reference project/tech-spec.md instead
[ ] No hardcoded platform names (e.g., "GitHub", "Jira")
    → should reference project/context.md instead
```

### 6. Project Context Completeness

Verify `project/context.md` contains:

```
[ ] Project description
[ ] Project key
[ ] Version control platform and main branch
[ ] Task management method and location
[ ] Task statuses definition
[ ] Task status file location
```

### 7. Tech Spec Completeness

Verify `project/tech-spec.md` contains:

```
[ ] Backend technology stack (language, framework, versions)
[ ] Frontend technology stack (if applicable)
[ ] Database / infrastructure
[ ] QA tools (linter, static analysis, tests)
[ ] Project directory structure
```

### 8. Environments Completeness

Verify `project/environments.md` contains:

```
[ ] Environment matrix (name, purpose, access method, app path)
[ ] Log locations per environment
[ ] Database access methods per environment
[ ] Service addresses per environment (cache, queue, etc.)
```

### 9. Gitignore Rules

Verify `.gitignore` contains rules for AI instruction files:

```
[ ] .ai/prd/* is ignored
[ ] .ai/prd/prd.md is excluded from ignore (committed)
[ ] task-status.local.md is effectively ignored (covered by prd/* or explicit rule)
[ ] .ai/wiki/tasks/ is ignored
[ ] .ai/wiki/ itself is not ignored, so reusable wiki policy/templates can be committed
```

### 10. PRD Consistency

```
[ ] prd/prd.md task table matches existing task directories
[ ] prd/task-status.local.md has entries for all tasks in prd.md
[ ] No orphan task directories (directory exists but not in index)
[ ] No phantom tasks (in index but directory missing)
```

### 11. Task Wiki Hygiene

```
[ ] wiki/tasks/ exists for local agent handoff context
[ ] For active tasks, wiki/tasks/{PROJECT_KEY}-{N}/summary.md exists when cross-role handoff has occurred
[ ] For active tasks, heartbeat.md is current when blockers, dependencies, or ownership changed
[ ] Observations have been reflected when observations.md or summary.md grew beyond policy limits
[ ] Task wiki entries are concise and link to artifacts instead of duplicating long content
[ ] No secrets, credentials, raw production data, or long chat transcripts are stored in wiki/tasks/
[ ] wiki/tasks/ is local-only and not staged for commit
[ ] No files under wiki/tasks/** are staged for commit
[ ] Any committed project/context snapshot is sanitized and stored outside wiki/tasks/
```

---

## Audit Report Format

After running all checks, present results as:

```
## Audit Results

### PASS (N items)
- [list of passing checks]

### FAIL (N items)
- [list of failing checks with explanation]

### MISSING (N items)
- [list of missing files/sections with recommendation]

### Recommended Actions
1. [action 1]
2. [action 2]
...
```

## Artifacts

- Audit report presented to the user (not saved as file)
