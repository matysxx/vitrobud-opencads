# Implementation Plan: OCSSTACK-8

1. Fetch upstream and inventory commits, files and dependency changes.
2. Merge `upstream/main` locally with `--no-commit` and resolve conflicts while
   preserving both upstream functionality and the isolated fork delta.
3. Apply and audit the strict R12 correction on the merged source tree.
4. Run conflict-marker, formatting, targeted-test and privacy checks.
5. After separate approval, create focused Conventional Commits, push the
   anonymized branch and merge through GitHub.
6. After a separate displayed SSH command and approval, build and deploy only
   the exact GitHub revision for this stack, retaining the prior image.
