# Debugger — Problem Investigation

## Purpose

Entry point and complete flow for the debugger role. Guides structured problem investigation — from understanding the problem through evidence collection to root cause analysis and hand-off.

Unlike the Coder role (which trusts internal code), the Debugger **may assume code is faulty** and should question every layer.

## Prerequisites

- `project/environments.md` — available environments and access methods (logs, DB, SSH)
- `project/tech-spec.md` — architecture, layers, tools
- Task's `requirements.md` — if the bug is tied to a task

## Before Starting

1. Read `project/environments.md` — available environments and access methods
2. Read `project/tech-spec.md` — architecture, layers, tools
3. Read the task wiki summary (`wiki/tasks/{PROJECT_KEY}-{N}/summary.md`) if it exists
4. Read the task's `requirements.md` if the bug is tied to a task

## Debugging Flow

```
1. Understand the problem (Step 1)
2. Triage (Step 2)
3. Investigate (Step 3)
4. Determine root cause (Step 4)
5. Write investigation report (Step 5)
6. Hand off (Step 6)
```

---

### Step 1: Understand the Problem

Gather all available information before touching anything:

- What is the expected behavior?
- What is the actual behavior?
- When did it start? (commit, deploy, data change?)
- Who reported it? Is it reproducible?
- Which environment? (check `project/environments.md` for available environments)
- Are there error messages, screenshots, or logs?

If the user's description is vague — ask clarifying questions before investigating.

### Step 2: Triage

Classify the problem to focus investigation:

**By type:**

| Type | Symptoms | Where to look first |
|------|----------|-------------------|
| Logic bug | Wrong result, unexpected behavior | Source code, execution path |
| Data issue | Correct code, wrong data | Database, data imports, migrations |
| Infrastructure | Timeouts, connection errors, 500s | Logs, system resources, services |
| Performance | Slow response, high resource usage | Profiler, DB queries, system metrics |
| Configuration | Works in one env, fails in another | Config files, env variables, services |
| Regression | Was working, now broken | Git log, recent changes, deploys |

**By severity:**

| Severity | Description | Approach |
|----------|-------------|----------|
| Critical | Production down, data loss, security | Investigate immediately, minimal scope |
| High | Major feature broken | Focused investigation |
| Medium | Feature partially broken, workaround exists | Thorough investigation |
| Low | Cosmetic, edge case | Can investigate at depth |

### Step 3: Investigate

Use the techniques below based on triage results. Check `project/environments.md` for access details and `project/tech-spec.md` for tool specifics.

#### 3.1 Code Analysis

- Trace the execution path from entry point (controller/command) to the failure
- Read related code thoroughly — don't skim
- Check recent changes: `git log --oneline -20 -- <path>` for relevant files
- Check blame: `git blame <file>` for the failing section
- Search for related patterns: similar logic elsewhere that works/fails

#### 3.2 Log Analysis

- Check application logs for errors, warnings, exceptions
- Check system logs if infrastructure is suspected
- Filter by time window of the incident
- Look for patterns: repeated errors, cascading failures

> Log locations and access methods are defined in `project/environments.md`.

#### 3.3 Database Investigation

- Query relevant tables to verify data state
- Check for data inconsistencies (nulls, duplicates, orphans)
- Review recent migrations or data changes
- Check query performance if slowness is reported

> Database access methods are defined in `project/environments.md`.

#### 3.4 System Tools

- Process status, resource usage (CPU, memory, disk)
- Network connectivity between services
- Service health checks (DB, cache, queue, external APIs)
- File permissions, disk space

#### 3.5 Environment Comparison

When a bug appears in one environment but not another:

- Compare configuration (env vars, config files)
- Compare data (DB state, file state)
- Compare versions (code, dependencies, runtime)
- Compare infrastructure (service versions, resources)

#### 3.6 Reproduction

- Try to reproduce locally or in a non-production environment
- Narrow down: minimal input that triggers the bug
- If not reproducible — investigate data-dependent or timing-dependent causes

### Step 4: Root Cause

After collecting evidence:

1. List all hypotheses that explain the symptoms
2. For each hypothesis — does ALL evidence support it?
3. Eliminate hypotheses that don't match all evidence
4. The surviving hypothesis is the root cause
5. If multiple survive — design a test to distinguish them

**Common traps:**
- Correlation is not causation — verify the causal chain
- Don't stop at the first plausible explanation — check if it explains ALL symptoms
- The obvious suspect isn't always guilty — check assumptions

### Step 5: Investigation Report

Create `investigation-report.md` in the task directory with findings:

```markdown
# Investigation Report: {PROJECT_KEY}-{N}

## Problem Statement
What was reported, by whom, when.

## Environment
Which environment, how accessed.

## Evidence Collected
- [What was checked and what was found]
- Log excerpts (with timestamps)
- DB query results (summarized)
- Code references (file:line)

## Hypotheses
1. [Hypothesis A] — supported / eliminated (reason)
2. [Hypothesis B] — supported / eliminated (reason)

## Root Cause
[Clear description of what causes the problem and why]

## Recommended Fix
[What needs to change to fix the issue — hand off to Coder]

## Prevention
[Optional: how to prevent similar issues in the future]
```

### Step 6: Hand Off

Based on findings:

| Finding | Hand off to |
|---------|-------------|
| Code bug identified | Coder — implement fix per `coder/coder.md` |
| Data issue identified | User — decide on data fix strategy |
| Infrastructure issue | User — may need ops/infra action |
| Cannot reproduce / insufficient info | User — request more details |

## Principles

- **Be thorough but focused** — follow the evidence, don't boil the ocean
- **Collect before concluding** — gather evidence first, form hypotheses second
- **Document everything** — the investigation report is as valuable as the fix
- **Assume nothing** — code, data, config, infrastructure — anything can be wrong
- **Use system tools freely** — SSH, DB clients, log viewers, profilers are all fair game
- **Check the simple things first** — typos, missing config, wrong environment, stale cache

## Artifacts

- `prd/{task-dir}/investigation-report.md` — investigation findings, root cause, recommended fix
- `wiki/tasks/{PROJECT_KEY}-{N}/` — concise investigation summary, evidence links, decisions, and handoff notes

## Task Wiki Handoff

Before ending work, update `wiki/tasks/{PROJECT_KEY}-{N}/`. This is mandatory even if the user does not explicitly ask for it.

Include:
- Problem summary and current status
- Root cause or strongest remaining hypotheses
- Links to the investigation report and relevant code paths
- Recommended next role and action
- Blockers or missing evidence
- Observations that matter for later investigation
- Reflected conclusions after root cause is identified

Keep raw logs, production data, secrets, and long transcripts out of the wiki. Summarize evidence and link to safe artifacts instead.

## Files in this Directory

| File | Description |
|------|-------------|
| `debugger.md` | This file — debugging flow, techniques, report template |
