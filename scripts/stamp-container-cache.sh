#!/bin/sh
# Mirror upstream Pages cache versioning for the container's root-path bundle.
set -eu
cd "${1:?Usage: stamp-container-cache.sh BUNDLE_DIRECTORY}"
test -f index.html
test -f sw.js
grep -q '^const CACHE_VERSION = "[^"]*";' sw.js
# Hash only bundle contents, never the previous stamp or a private runtime input.
hash=$(find . -type f ! -name sw.js -print0 | LC_ALL=C sort -z | xargs -0 sha256sum | sha256sum | cut -c 1-12)
temporary=$(mktemp ./sw.js.stamp.XXXXXX)
trap 'test ! -f "$temporary" || rm -f "$temporary"' EXIT HUP INT TERM
sed "s/^const CACHE_VERSION = \"[^\"]*\";/const CACHE_VERSION = \"$hash\";/" sw.js > "$temporary"
grep -q "^const CACHE_VERSION = \"$hash\";" "$temporary"
cat "$temporary" > sw.js
