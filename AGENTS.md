# Guidance for coding agents — ENGR 103

This repository is the public ENGR 103 course website: an Astro + Starlight site
whose content lives in `src/content/docs/` as MDX. The course is taught in
**Python and Rust simultaneously**; every concept appears in both languages and
students may answer in either.

`VISION.md` is the authority for course design. A skill, a page, or this file
disagreeing with VISION is the bug.

## Before editing any content file

Load the matching skill in `.claude/skills/`. The five kinds of page are
deliberately different genres, and mixing them is the most common authoring
mistake.

| Directory | What it is | Genre | Skill | Story line |
|---|---|---|---|---|
| `lectures/` | Read before class, flipped | Explanation, prose | `engr103-lecture-notes` | none |
| `activities/` | Run live in lecture | Guided problem sets with closed reveals | `engr103-activities` | none |
| `assignments/` | Take-home, AI allowed | One open problem with a fixed contract | `engr103-assignments` | Willamette Resource Office |
| `recitations/` | Coverage pages for graded sessions | Premise and skills, never a problem | `engr103-recitation-coverage` | Rubber Duck Robotics |
| `reference/`, `practicalities/` | Language ladder, glossary, archetypes, how-tos | Terse reference and numbered how-to | none | none |

The skills also encode the plain-language rule the whole site is written to:
first year, never programmed, may not be a first-language English speaker.

## Two binding invariants

- **The language ladder** (`src/content/docs/reference/language-ladder.mdx`) is
  binding. Every graded item must be solvable with the constructs listed for its
  week, and nothing above that row may appear.
- **Graded, on paper, and offline lives in the private `engr103-assessments`
  repository. Everything else lives here.** This repository is public, so a
  problem authored here is a published problem. Never write a recitation
  problem, an exam question, an answer key, or a hidden test case in this
  repository. `draft: true` hides a page from the built site and does nothing
  else. Recitation 1 (`intern-orientation.mdx`) is ungraded and is the one
  session authored here in full.

## Commands

This repository uses **npm**, not bun.

```bash
npm install
npm run dev      # localhost:4321
npm run build    # astro check && astro build
```

`npm run build` is the gate for any content change. The
`starlight-links-validator` integration runs during the build and fails it on a
broken internal link, so a passing build is the check that cross-references
still resolve. Run it after every content edit.

## Where things are

- `astro.config.mjs`: Starlight config. New pages need a sidebar entry here.
- `src/content.config.ts`: collection schemas.
- `src/components/`: Astro and Svelte components. New interactive pieces go
  here as Svelte 5 (runes); use `Latex.astro` for math rather than importing
  KaTeX directly.
- `starters/<slug>/`: per-assignment starter code. The `check` and `pack` tools
  live once in `starters/_lib/` and are copied in by
  `./scripts/build-starters.sh`. Edit the harness there, never in a copy. The
  script writes to `zip/`, which is generated output.
- `canvas/`: TSV rubrics and HTML assignment snippets for Canvas.
- `dist/`, `pdf/`, `.astro/`: generated output. Never edit as source.

## Working style

- Keep changes small and self-contained, with a commit message saying what
  changed and why.
- Do not introduce new build tooling without asking. The stack is Astro +
  Starlight + Tailwind + Svelte on npm.
- Do not take over manual maintainer tasks (PDF generation, printing,
  deployment). Improve the instructions instead.
