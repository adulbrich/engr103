# Lecture-notes Conversion, Wave 1: Skill + L2 Pilot

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the `engr103-lecture-notes` style skill, then convert one lecture (L2, Values/types/expressions) using it as a pilot, so a flawed skill is caught before it propagates across all nineteen lectures.

**Architecture:** Two tasks. Task 1 writes a repo-local skill file (`.claude/skills/engr103-lecture-notes/SKILL.md`) that is the enforceable form of VISION §9's lecture-notes house style, adapted from the existing `cs312-lecture-notes` skill. Task 2 converts the current C++ `expressions` + `operators` + `type-casting` pages into a single dual-language L2 lecture that obeys the skill. This is Wave 1 of the lecture-notes track (spec: `2026-07-02-lecture-notes-conversion-design.md`); Wave 2 (the remaining 18 lectures + the glossary body) is a separate plan written after the pilot checkpoint.

**Tech Stack:** Astro + Starlight (MDX, `<Tabs syncKey>`, `<WhatDiffers>`, `starlight-links-validator`); repo-local Claude Code skill (`.claude/skills/`).

## Global Constraints

- **The only automated test is `npm run build`** (`astro check` then `astro build`; the build runs `starlight-links-validator`). "Run the test" means run it and confirm exit 0, no type errors, no invalid links. The skill file itself is not built by Astro; its verification is structural (section presence) plus review.
- **VISION.md is the authority.** Author from it and from the named source pages. Read `/Users/ulbrical/GitHub/engr103/VISION.md`.
- **No emdashes anywhere** in the skill or the lecture. Proper punctuation only.
- **Dual-language rule (lectures):** every concept section ends with a `<Tabs syncKey="lang">` pair, `<TabItem label="Python">` then `<TabItem label="Rust">` (each a complete, paste-runnable fragment), followed by a `<WhatDiffers>` callout.
- **Language-ladder fence:** examples use ONLY constructs on the language ladder for that lecture's week, and link to `/reference/language-ladder`. L2 is a Week 1 lecture, so its examples use only Week 1 ladder constructs (literals, arithmetic and precedence, int vs float division, `print`/`println!`) and must NOT use variables (Week 2) or any later construct.
- **Plain-language rule:** first-year, never-programmed, possibly non-native-English reader. Short sentences, common words, no idioms, define every technical term in plain words at first use, one name per concept.
- **Concepts only:** no git, shell, editor, or tooling. A lecture may name the problem family a concept feeds, never a specific assignment, recitation, or activity.
- **Lecture is the canonical definition site:** each new term gets its plain-language definition and a stable heading anchor in the lecture; the glossary (Wave 2) links in.
- **Length band (lectures):** roughly 1,800 to 3,500 words, a diagnostic not a target (`wc -w`).
- **Writers:** Sonnet 5 implementer subagents.
- **Branch:** `website-redesign`, never `main`. Commit after each task.

---

## File Structure

- **Create** `.claude/skills/engr103-lecture-notes/SKILL.md` (Task 1): the lecture-notes style contract, repo-local so it is version-controlled on the branch, reviewable, and readable by every Wave 2 implementer.
- **Create** `src/content/docs/lectures/values-types-expressions.mdx` (Task 2): the L2 lecture. New slug, `sidebar.order: 2`. The old `expressions`, `operators`, and `type-casting` pages are left in place for now (Wave 2 retires them and reorders the section); the L2 page coexisting with them during the pilot is an accepted transitional state.

Note on skill location: repo-local `.claude/skills/` is chosen over user-global `~/.claude/skills/` (where the cs312 skills live) so the skill travels with the course on this branch. If you prefer user-global, it moves with one `git mv` / relocation later.

---

## Task 1: Author the `engr103-lecture-notes` skill

**Files:** Create `.claude/skills/engr103-lecture-notes/SKILL.md`

**Interfaces:**
- Produces: the style contract Task 2 and all Wave 2 lecture tasks read and obey. Its section headings are referenced by reviewers.

The skill is adapted from `~/.claude/skills/cs312-lecture-notes/SKILL.md` (read it for structure and tone of a mature lecture-notes skill) and encodes VISION §9 plus this track's spec §2. It MUST contain these sections, with the rules stated here made concrete and exemplified:

1. **Frontmatter:** `name: engr103-lecture-notes`; a `description` that says to use it when creating or editing lecture notes (MDX in `src/content/docs/lectures/`) and that it enforces the concept-first, dual-language, plain-language ENGR 103 style.
2. **What lecture notes are for:** explanation genre (Diataxis), concept-first, read before a flipped class; deepen understanding and make connections; not tutorials, not how-to, not reference.
3. **Audience and the plain-language rule:** first-year, never-programmed, possibly non-native-English reader; short sentences, common words, no idioms; define every term in plain words at first use; one name per concept forever.
4. **The dual-language rule:** every concept section ends with a `<Tabs syncKey="lang">` Python-then-Rust example pair (self-contained, paste-runnable) and a `<WhatDiffers>` callout with `differs` and `cannot` slots; show the concept in both languages and name what changed and what could not change.
5. **The language-ladder fence:** examples use only constructs on the ladder for that week; link `/reference/language-ladder`; never use a construct before the ladder introduces it.
6. **Glossary discipline:** the lecture is where a term gets its canonical plain-language definition and a stable `##`/`###` heading anchor; the glossary links in and does not restate the definition.
7. **Concepts only:** no git/shell/editor/tooling; may name the problem family a concept feeds (Launch Window and so on), never a specific assignment/recitation/activity; no tutorial walkthroughs; no single running scenario threaded through the whole lecture (use independent examples).
8. **Structure:** opening problem statement (why this matters, no "in this lecture we will"); every section leads with prose before any code/table/component; concept depth (define, explain at course level, concrete example, why it matters); prose over lists; a penultimate **Takeaways** prose synthesis; an optional final **Resources** list of external links.
9. **Lecture frontmatter + `ai-summary` block:** frontmatter `title`, `description`, `sidebar.order`. Immediately after imports, an `ai-summary` MDX comment with fields: `type: lecture`, `slug`, `order`, `covers` (one-line, semicolon-separated), `prereq_lectures`, `followup_lectures`, `families` (the problem families this lecture feeds), and `glossary_terms` (the terms this lecture defines canonically, for the Wave 2 glossary). Every slug/term must be real.
10. **Imports:** the available components are `WhatDiffers` (`/src/components/WhatDiffers.astro`), `Latex`, `AsciiTable`, `MemoryPoolDiagram`, `VariableLifecycleDiagram`, and Starlight's `Tabs`, `TabItem`, `Aside`. Import only what the lecture uses. Do NOT import assignment/recitation components.
11. **Writing style:** no emdashes (state both `—` and `--`); active voice; second person to the student; present tense for always-true concepts; define acronyms at first use; a gentle, encouraging beginner tone (calmer and plainer than cs312's expert-colleague voice), while still always explaining the why.
12. **Length calibration:** roughly 1,800 to 3,500 words, a diagnostic not a target; `wc -w src/content/docs/lectures/<file>.mdx` to sanity-check.
13. **Validation:** run `npm run build`; fix any MDX/import/link errors before considering the work done.
14. **What lecture notes must NOT contain:** tooling/setup, references to assignments/recitations/activities, tutorial walkthroughs, a single running scenario across sections, or any construct not yet on the ladder.

The skill drops cs312's build-time draft-exclusion assumption: this site does not exclude drafts from the build, so do not require `draft: true` (adding true draft exclusion is a separate site-config change, out of scope). New lectures are simply added and reviewed.

- [ ] **Step 1: Read the sources.** Read `~/.claude/skills/cs312-lecture-notes/SKILL.md` (structure/tone reference) and VISION §9 and §10 in `/Users/ulbrical/GitHub/engr103/VISION.md`, plus this track's spec §2 (`docs/superpowers/specs/2026-07-02-lecture-notes-conversion-design.md`).
- [ ] **Step 2: Author the skill** at `.claude/skills/engr103-lecture-notes/SKILL.md` with all fourteen sections above, each made concrete with a short ENGR 103 example where the section warrants one (for example a tiny `<Tabs syncKey="lang">` + `<WhatDiffers>` block in the dual-language section).
- [ ] **Step 3: Structural self-check.**
  - Run `grep -nE "^#|^name:|ai-summary|syncKey|WhatDiffers|language-ladder|Plain-language|Takeaways|1,800|Length" .claude/skills/engr103-lecture-notes/SKILL.md` and confirm the key sections and rules are present.
  - Run `grep -n "—" .claude/skills/engr103-lecture-notes/SKILL.md` and confirm no matches.
- [ ] **Step 4: Commit.**

```bash
git add .claude/skills/engr103-lecture-notes/SKILL.md
git commit -m "feat(lectures): author the engr103-lecture-notes style skill"
```

---

## Task 2: Convert L2 (Values, types, expressions) as the pilot

**Files:** Create `src/content/docs/lectures/values-types-expressions.mdx`

**Interfaces:**
- Consumes: the `engr103-lecture-notes` skill (Task 1); the `/reference/language-ladder` page (for the Week 1 fence and the link).
- Produces: the route `/lectures/values-types-expressions`, the pilot proving the skill.

**Source:** the current C++ pages `src/content/docs/lectures/expressions.mdx`, `operators.mdx`, and `type-casting.mdx` (read them for the concept coverage to carry over), converted to dual-language per the skill. VISION §10 L2 scope: integers, floats, booleans, strings as values; static versus dynamic typing; operators, precedence, integer versus float division; evaluating expressions by hand.

**Ladder note:** L2 is a Week 1 lecture. Use only Week 1 ladder constructs. In particular, do NOT use variables (Week 2): show values and expressions evaluated directly and printed, not stored in names. This is the sharpest test of whether the skill's fence is being honored.

- [ ] **Step 1: Read the skill and sources.** Read `.claude/skills/engr103-lecture-notes/SKILL.md` (the contract you must follow), the three source pages named above, VISION §10 (the L2 row), and `src/content/docs/reference/language-ladder.mdx` (Week 1 row).
- [ ] **Step 2: Author the lecture** at `src/content/docs/lectures/values-types-expressions.mdx`:
  - Frontmatter `title: "Values, Types, and Expressions"`, a one-sentence `description`, `sidebar.order: 2`.
  - The `ai-summary` block (`type: lecture`, `slug: values-types-expressions`, `order: 2`, `covers`, `prereq_lectures: how-programs-run` if referenced by name only, `followup_lectures: variables-and-state`, `families: launch-window`, `glossary_terms`: the terms this lecture defines, for example value, type, expression, operator, operator precedence, integer division, static typing, dynamic typing).
  - An opening problem statement, then concept sections (values and types; static versus dynamic typing; operators and precedence; integer versus float division; evaluating by hand), each leading with prose and ending with a `<Tabs syncKey="lang">` Python/Rust pair and a `<WhatDiffers>` callout.
  - Define each new term in plain words at first use (this is the glossary's future link target).
  - A **Takeaways** prose synthesis; an optional **Resources** list.
- [ ] **Step 3: Verify.**
  - Run `npm run build`. Expected: green, 0 errors, all internal links valid (the `/reference/language-ladder` link resolves).
  - Run `grep -n "—" src/content/docs/lectures/values-types-expressions.mdx`. Expected: no matches.
  - Run `grep -c "syncKey=\"lang\"" src/content/docs/lectures/values-types-expressions.mdx` and confirm one per concept section (at least three); confirm a `<WhatDiffers>` accompanies each.
  - Run `wc -w src/content/docs/lectures/values-types-expressions.mdx` and confirm roughly 1,800 to 3,500 words.
  - Read the page once to confirm no variables or other post-Week-1 constructs appear in any example.
- [ ] **Step 4: Commit.**

```bash
git add src/content/docs/lectures/values-types-expressions.mdx
git commit -m "feat(lectures): convert L2 Values, types, expressions (dual-language pilot)"
```

---

## Pilot checkpoint (after Task 2)

Stop here and present the skill and the L2 lecture to the user for sign-off before Wave 2. Wave 2 (the remaining 18 lectures, the section reorder and retirement of superseded pages, and the glossary body) is written as a separate plan once the skill is confirmed.

## Self-review notes (author)

- **Spec coverage (Wave 1 slice):** the skill (spec §2) is Task 1; the pilot lecture (spec §6 execution, the L2 choice) is Task 2. The full 19-lecture mapping (spec §3), the glossary body (spec §5), and page retirement/reorder are Wave 2, out of this plan by design.
- **No manufactured red step:** the skill file is not built, and a new lecture page builds green immediately, so verification is the green build plus structural greps (emdashes, `syncKey`, `WhatDiffers`), the word count, and a read-through for ladder fidelity. This is intentional for content authoring.
- **Model:** Sonnet 5 implementers per the spec.
