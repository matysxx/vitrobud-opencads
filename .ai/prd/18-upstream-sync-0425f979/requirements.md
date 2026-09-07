# OCSSTACK-18 — Requirements

## Goal

Synchronize the maintained fork with the reviewed upstream Open CAD Studio
revision `0425f979f9ed93433a3f23234d4bff970358e148` while preserving the reusable
rootless Podman stack and all intentional fork integrations.

## Requirements

- Merge the exact upstream revision without rewriting upstream history.
- Preserve the `EXPORTDXFR12` compatibility exporter and its regression tests.
- Preserve the explicit OpenGL/WebGL backend used by the web build.
- Preserve the root-path web build and the repository-first container workflow.
- Keep `ROLLOUT_REVISION` as the sole production image-version input.
- Keep host-specific values, credentials, runtime state, and private infrastructure
  outside the public repository.
- Do not include unrelated local OCSSTACK-12 or Nextcloud planning changes.
- Record the new upstream version, tag distance, and dependency revisions.
- Validate locally, publish the anonymized revision to GitHub, and only then deploy
  that exact revision to the Debian rootless Podman runtime.

## Acceptance criteria

- The merge has upstream `0425f979f9ed93433a3f23234d4bff970358e148`
  as a parent.
- The fork-specific R12 export command remains registered and tested.
- The web application still selects OpenGL/WebGL explicitly.
- Container and operational files pass static validation.
- The public tree contains no private infrastructure or secrets.
- GitHub `main`, local `main`, and the deployed server revision resolve to the same
  verified commit after rollout.
