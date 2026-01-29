#!/usr/bin/env bash
set -e

# Resolve this script's directory and the repository root so the script can be
# invoked from anywhere in the filesystem and still find the node helpers and
# output files reliably.
# Works on macOS and Linux because the script runs under bash (shebang above).
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]:-$0}")" >/dev/null 2>&1 && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." >/dev/null 2>&1 && pwd)"

# Output directories (absolute)
PDF_DIR="$REPO_ROOT/pdf"
TMP_DIR="$PDF_DIR/temp"

# Ensure output directories exist
mkdir -p "$TMP_DIR" "$PDF_DIR"

# Base URL for pages
BASE="https://engr103.alexulbrich.com"

# Lecture topics
lectures=(
    "introduction"
    "development-environment"
    "git"
    "cpp-basics"
    "comments"
    "expressions"
    "operators"
    "variables"
    "constants"
    "type-casting"
    "shorthand-operators"
    "functions"
    "scope"
    "booleans"
    "if-statements"
    "loops"
    "characters"
    "strings"
    "string-functions"
    "arrays"
    "references"
    "error-handling" # not used
)

# Studios
studios=(
    "introduction"
    "environment-setup"
    "git-and-github"
    "expressions-operators-variables"
    "functions"
    "if-statements"
    "rng-and-errors"
    "loops"
    "strings"
    "arrays"
    "file-io"
)

# Assignments
assignments=(
  "introduction"
  "getting-started"
  "linear-equations"
  "financial-planner"
  "dictionary"
  "calculator"
)

# Practicalities
practicalities=(
    "debugging"
    "demos"
    "gen-ai"
    "polya"
    "style"
    "vim"
)

echo "Generating individual PDFs with Node.js..."

for topic in "${lectures[@]}"; do
  node "$SCRIPT_DIR/print-clean.js" "$BASE/lectures/$topic" "$TMP_DIR/lecture-notes-$topic.pdf"
  node "$SCRIPT_DIR/print.js" "$BASE/lectures/$topic" "$PDF_DIR/lecture-notes-$topic.pdf"
done

for topic in "${studios[@]}"; do
  node "$SCRIPT_DIR/print-clean.js" "$BASE/studios/$topic" "$TMP_DIR/studio-$topic.pdf"
  node "$SCRIPT_DIR/print.js" "$BASE/studios/$topic" "$PDF_DIR/studio-$topic.pdf"
done

for topic in "${assignments[@]}"; do
  node "$SCRIPT_DIR/print-clean.js" "$BASE/assignments/$topic" "$TMP_DIR/assignment-$topic.pdf"
  node "$SCRIPT_DIR/print.js" "$BASE/assignments/$topic" "$PDF_DIR/assignment-$topic.pdf"
done

for topic in "${practicalities[@]}"; do
  node "$SCRIPT_DIR/print-clean.js" "$BASE/practicalities/$topic" "$TMP_DIR/practicalities-$topic.pdf"
  node "$SCRIPT_DIR/print.js" "$BASE/practicalities/$topic" "$PDF_DIR/practicalities-$topic.pdf"
done

echo "Combining PDFs with Node.js..."

# Helper to join array elements with a separator
join_by() { local IFS="$1"; shift; echo "$*"; }

LECTURES_CSV=$(join_by , "${lectures[@]}")
STUDIOS_CSV=$(join_by , "${studios[@]}")
ASSIGNMENTS_CSV=$(join_by , "${assignments[@]}")
PRACTICALITIES_CSV=$(join_by , "${practicalities[@]}")

node "$SCRIPT_DIR/combine.js" \
  --lectures="$LECTURES_CSV" \
  --studios="$STUDIOS_CSV" \
  --assignments="$ASSIGNMENTS_CSV" \
  --practicalities="$PRACTICALITIES_CSV"

echo "PDF generation complete!"
