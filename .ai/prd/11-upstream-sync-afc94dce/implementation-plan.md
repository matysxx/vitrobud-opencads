# OCSSTACK-11 Implementation Plan

1. Fetch and record the exact upstream revision and tag.
2. Merge upstream locally without committing, resolve only real conflicts and
   retain fork-specific stack and exporter behavior.
3. Apply OCSSTACK-10 on top of the merged tree.
4. Run privacy, conflict-marker and formatting audits locally.
5. After separate approval, commit and push a review branch; build and test the
   exact revision on Debian before merging to `main` and rolling out.
