# Implementation plan: OCSSTACK-22

1. Start an isolated worktree at verified fork `3822cd68`; review upstream
   Cargo, web build, renderer, file IO, and workflow changes.
2. Merge exact upstream revision with both ancestry parents preserved. Resolve
   conflicts while retaining container/R12/privacy behavior.
3. Adapt fork-only codec imports and ensure R12 regression tests are registered
   despite upstream's explicit integration-test target.
4. Update `.ai/project/tech-spec.md`, `docs/upstream-maintenance.md`, and affected
   README/export documentation to match verified code and build inputs.
5. Run shell/YAML/privacy/integration audits, publish reviewed main, and verify
   full native and WASM CI including R12 regressions.
6. Present exact server rollout command for approval, then rebuild and verify
   checkout, image, service, HTTP, and HTTPS browser startup.
7. Complete local task summary, observations, heartbeat, handoff, and reflection.

## Risks

The upstream dependency rename requires adapting fork-only imports. Large
updates may create compile/API conflicts: CI must pass before deployment.
Keep R12 handlers outside the central update stack frame to preserve the
previous stack-overflow fix. Keep production configuration outside Git.
