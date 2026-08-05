# OCSSTACK-10 Implementation Plan

1. Replace only the private R12 binary serializer with a deterministic ASCII
   group/value serializer using CRLF line endings.
2. Replace the embedded binary verifier with an ASCII parser and retain all
   structural, version, entity-count and numeric checks.
3. Rewrite the application regression tests around an independent ASCII DXF
   parser and explicit CRLF assertions.
4. Update runtime comments, README and the machine-export runbook.
5. Run local static checks; build and test the exact pushed revision on the
   Debian builder before merging and rolling out through `main`.

The standard DXF writer and `SAVE`/`SAVEAS` remain untouched. Historical PRD
records stay unchanged because they document the evidence that led here.
