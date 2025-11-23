#!/usr/bin/env bash
set -e

# Base URL for pages
BASE="https://engr103.alexulbrich.com"

# Lecture topics
lecture_topics=(
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
    "error-handling"
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

# Practicalities
practicalities=(
    "debugging"
    "demos"
    "polya"
    "style"
    "vim"
)

echo "Generating individual PDFs with Node.js..."

for topic in "${lecture_topics[@]}"; do
  node print-clean.js "$BASE/lectures/$topic" "../pdf/temp/lecture-notes-$topic.pdf"
  node print.js "$BASE/lectures/$topic" "../pdf/lecture-notes-$topic.pdf"
done

for topic in "${assignments[@]}"; do
  node print-clean.js "$BASE/assignments/$topic" "../pdf/temp/assignment-$topic.pdf"
  node print.js "$BASE/assignments/$topic" "../pdf/assignment-$topic.pdf"
done

for topic in "${studios[@]}"; do
  node print-clean.js "$BASE/studios/$topic" "../pdf/temp/studio-$topic.pdf"
  node print.js "$BASE/studios/$topic" "../pdf/studio-$topic.pdf"
done

for topic in "${practicalities[@]}"; do
  node print-clean.js "$BASE/practicalities/$topic" "../pdf/temp/practicalities-$topic.pdf"
  node print.js "$BASE/practicalities/$topic" "../pdf/practicalities-$topic.pdf"
done

echo "Combining PDFs with Node.js..."
node combine.js

echo "PDF generation complete!"
