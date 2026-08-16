#!/usr/bin/env bash
#
# Assemble every assignment starter into zip/<slug>/ and zip each one
# for upload to Canvas. Each starter's own files live in starters/<slug>/; the
# check and pack tools live once in starters/_lib/ and are copied in here, so
# there is exactly one copy of the harness to maintain.
#
#   ./scripts/build-starters.sh              build every starter
#   ./scripts/build-starters.sh mix-slug ... build only the named ones
#
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SRC="$ROOT/starters"
LIB="$SRC/_lib"
OUT="$ROOT/zip"

if [ ! -d "$LIB" ]; then
	echo "build-starters: missing $LIB" >&2
	exit 1
fi

slugs=()
if [ "$#" -gt 0 ]; then
	slugs=("$@")
else
	for dir in "$SRC"/*/; do
		name="$(basename "$dir")"
		[ "$name" = "_lib" ] && continue
		slugs+=("$name")
	done
fi

rm -rf "$OUT"
mkdir -p "$OUT"

for slug in "${slugs[@]}"; do
	src="$SRC/$slug"
	if [ ! -d "$src" ]; then
		echo "build-starters: no such starter: $slug" >&2
		exit 1
	fi
	if [ ! -f "$src/tests/visible.json" ]; then
		echo "build-starters: $slug has no tests/visible.json" >&2
		exit 1
	fi

	dest="$OUT/$slug"
	mkdir -p "$dest"
	cp -R "$src"/. "$dest"/

	# The shared harness, copied rather than linked so the zip works on any
	# platform and a student can read every file they were given.
	cp "$LIB/check" "$dest/check"
	cp "$LIB/pack" "$dest/pack"
	mkdir -p "$dest/tests"
	cp "$LIB/tests/_py_driver.py" "$dest/tests/_py_driver.py"
	chmod +x "$dest/check" "$dest/pack"

	# Never ship an author's build leftovers.
	find "$dest" -name '__pycache__' -type d -prune -exec rm -rf {} + 2>/dev/null || true
	find "$dest" -name '*.pyc' -delete 2>/dev/null || true
	rm -f "$dest/submission.zip"

	( cd "$OUT" && zip -qr "$slug.zip" "$slug" )
	echo "built $slug -> zip/$slug.zip"
done

echo
echo "Upload each zip/<slug>.zip to its Canvas assignment."
