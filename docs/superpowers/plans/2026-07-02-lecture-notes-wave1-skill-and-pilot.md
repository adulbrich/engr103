# Lecture-notes Conversion, Wave 1: Skill + Version Component + L2 Pilot

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the `engr103-lecture-notes` style skill and a shared language-version component, then convert one lecture (L2, Values/types/expressions) using them as a pilot, so a flawed skill is caught before it propagates across all nineteen lectures.

**Architecture:** Three tasks. Task 1 writes a repo-local skill file (`.claude/skills/engr103-lecture-notes/SKILL.md`), the enforceable form of VISION §9's lecture-notes house style. Task 2 adds a shared `<LanguageVersions>` component and its single-source data file, shown at the top of every content page. Task 3 converts the current C++ `expressions` + `operators` + `type-casting` pages into a single dual-language L2 lecture that obeys the skill, carries the version component, and passes the authoritative accuracy pass (every claim verified against official docs and by executing the code in both languages). This is Wave 1 of the lecture-notes track (spec: `2026-07-02-lecture-notes-conversion-design.md`); Wave 2 (the remaining 18 lectures + the glossary body) is a separate plan written after the pilot checkpoint.

**Tech Stack:** Astro + Starlight (MDX, `<Tabs syncKey>`, `<WhatDiffers>`, `starlight-links-validator`); repo-local Claude Code skill (`.claude/skills/`); Python 3.14 and Rust 1.88 toolchains for the accuracy pass.

## Global Constraints

- **The only automated test is `npm run build`** (`astro check` then `astro build`; the build runs `starlight-links-validator`). "Run the test" means run it and confirm exit 0, no type errors, no invalid links. The skill file itself is not built by Astro; its verification is structural (section presence) plus review.
- **VISION.md is the authority.** Author from it and from the named source pages. Read `/Users/ulbrical/GitHub/engr103/VISION.md`.
- **No emdashes anywhere** in the skill or the lecture. Proper punctuation only.
- **Dual-language rule (lectures):** every concept section ends with a `<Tabs syncKey="lang">` pair, `<TabItem label="Python">` then `<TabItem label="Rust">` (each a complete, paste-runnable fragment), followed by a `<WhatDiffers>` callout.
- **Language-ladder fence:** examples use ONLY constructs on the language ladder for that lecture's week, and link to `/reference/language-ladder`. L2 is a Week 1 lecture, so its examples use only Week 1 ladder constructs (literals, arithmetic and precedence, int vs float division, `print`/`println!`) and must NOT use variables (Week 2) or any later construct.
- **Plain-language rule:** first-year, never-programmed, possibly non-native-English reader. Short sentences, common words, no idioms, define every technical term in plain words at first use, one name per concept.
- **Concepts and dual-language only, fully story-agnostic:** no git, shell, editor, or tooling, and no reference to stories, problem families, assignments, activities, or recitations. Teach the concept in Python and Rust and nothing about where it is later exercised. (This overrides VISION §9's "names the problem family" clause per instructor decision.)
- **Authoritative accuracy pass:** before a lecture (or any content) is considered done, verify every factual claim against official documentation (docs.python.org, doc.rust-lang.org) and execute every code example in both languages, comparing the real output and behavior to what the text states. Nothing ships unverified. Toolchains available here: Python 3.14 (`python3`), Rust 1.88 (`rustc`).
- **Language versions on the page:** a shared `<LanguageVersions>` component sits at the top of every content page, reading the single source `src/data/languageVersions.ts` (Python 3.14, Rust 1.88). Versions are updated in that one file only.
- **Lecture is the canonical definition site:** each new term gets its plain-language definition and a stable heading anchor in the lecture; the glossary (Wave 2) links in.
- **Length band (lectures):** roughly 1,800 to 3,500 words, a diagnostic not a target (`wc -w`).
- **Writers:** Sonnet 5 implementer subagents.
- **Branch:** `website-redesign`, never `main`. Commit after each task.

---

## File Structure

- **Create** `.claude/skills/engr103-lecture-notes/SKILL.md` (Task 1): the lecture-notes style contract, repo-local so it is version-controlled on the branch, reviewable, and readable by every Wave 2 implementer.
- **Create** `src/data/languageVersions.ts` and `src/components/LanguageVersions.astro` (Task 2): the single-source version data and the small badge component shown at the top of every content page. Shared infrastructure, reused later by activities, assignments, and recitations.
- **Create** `src/content/docs/lectures/values-types-expressions.mdx` (Task 3): the L2 lecture. New slug, `sidebar.order: 2`, carrying `<LanguageVersions />` at the top. The old `expressions`, `operators`, and `type-casting` pages are left in place for now (Wave 2 retires them and reorders the section); the L2 page coexisting with them during the pilot is an accepted transitional state.

Note on skill location: repo-local `.claude/skills/` is chosen over user-global `~/.claude/skills/` (where the cs312 skills live) so the skill travels with the course on this branch. If you prefer user-global, it moves with one `git mv` / relocation later.

---

## Task 1: Author the `engr103-lecture-notes` skill

**Files:** Create (or, if it exists from a prior pass, revise) `.claude/skills/engr103-lecture-notes/SKILL.md`

**Interfaces:**
- Produces: the style contract Tasks 2 and 3 and all Wave 2 lecture tasks read and obey. Its section headings are referenced by reviewers.

The skill is adapted from `~/.claude/skills/cs312-lecture-notes/SKILL.md` (read it for structure and tone of a mature lecture-notes skill) and encodes VISION §9 plus this track's spec §2. It MUST contain these sections, with the rules stated here made concrete and exemplified:

1. **Frontmatter:** `name: engr103-lecture-notes`; a `description` that says to use it when creating or editing lecture notes (MDX in `src/content/docs/lectures/`) and that it enforces the concept-first, dual-language, plain-language ENGR 103 style.
2. **What lecture notes are for:** explanation genre (Diataxis), concept-first, read before a flipped class; deepen understanding and make connections; not tutorials, not how-to, not reference.
3. **Audience and the plain-language rule:** first-year, never-programmed, possibly non-native-English reader; short sentences, common words, no idioms; define every term in plain words at first use; one name per concept forever.
4. **The dual-language rule:** every concept section ends with a `<Tabs syncKey="lang">` Python-then-Rust example pair (self-contained, paste-runnable) and a `<WhatDiffers>` callout with `differs` and `cannot` slots; show the concept in both languages and name what changed and what could not change.
5. **The language-ladder fence:** examples use only constructs on the ladder for that week; link `/reference/language-ladder`; never use a construct before the ladder introduces it.
6. **Glossary discipline:** the lecture is where a term gets its canonical plain-language definition and a stable `##`/`###` heading anchor; the glossary links in and does not restate the definition.
7. **Concepts and dual-language only, fully story-agnostic:** no git/shell/editor/tooling, and no reference to stories, problem families, assignments, activities, or recitations. The skill and every lecture it governs stay agnostic of the course's narrative and assessment structure: teach the concept in Python and Rust, and say nothing about where it is later exercised. No tutorial walkthroughs; no single running scenario threaded through the whole lecture (use independent examples). (This overrides VISION §9's "names the problem family" clause, per instructor decision.)
8. **Structure:** opening problem statement (why this matters, no "in this lecture we will"); every section leads with prose before any code/table/component; concept depth (define, explain at course level, concrete example, why it matters); prose over lists; a penultimate **Takeaways** prose synthesis; an optional final **Resources** list of external links.
9. **Lecture frontmatter + `ai-summary` block:** frontmatter `title`, `description`, `sidebar.order`. Immediately after imports, an `ai-summary` MDX comment with fields: `type: lecture`, `slug`, `order`, `covers` (one-line, semicolon-separated), `prereq_lectures`, `followup_lectures`, and `glossary_terms` (the terms this lecture defines canonically, for the Wave 2 glossary). Every slug/term must be real. The block carries no story, family, or assignment fields.
10. **Imports:** the available components are `LanguageVersions` (`/src/components/LanguageVersions.astro`), `WhatDiffers` (`/src/components/WhatDiffers.astro`), `Latex`, `AsciiTable`, `MemoryPoolDiagram`, `VariableLifecycleDiagram`, and Starlight's `Tabs`, `TabItem`, `Aside`. Import only what the lecture uses. Do NOT import assignment/recitation components.
11. **Writing style:** no emdashes (name both the emdash character and the double-hyphen convention as forbidden); active voice; second person to the student; present tense for always-true concepts; define acronyms at first use; a gentle, encouraging beginner tone (calmer and plainer than cs312's expert-colleague voice), while still always explaining the why.
12. **Length calibration:** roughly 1,800 to 3,500 words, a diagnostic not a target; `wc -w src/content/docs/lectures/<file>.mdx` to sanity-check.
13. **Validation:** run `npm run build`; fix any MDX/import/link errors before considering the work done.
14. **What lecture notes must NOT contain:** tooling/setup, references to assignments/recitations/activities, tutorial walkthroughs, a single running scenario across sections, or any construct not yet on the ladder.
15. **The authoritative accuracy pass:** a required workflow section (replacing cs312's lighter "Factual Currency" section with a stronger rule). Before a lecture is done, verify every factual claim against official documentation (docs.python.org, doc.rust-lang.org) AND execute every code example in both languages (`python3` for Python, `rustc` for a single Rust file), comparing the real output, value, type, or error to what the text states. Any mismatch is fixed. State plainly that no claimed output or behavior ships unverified.
16. **Language versions on the page:** a required section stating that `<LanguageVersions />` (imported from `/src/components/LanguageVersions.astro`) sits at the top of every lecture body, and that the versions come from the single source `src/data/languageVersions.ts` (never hard-coded in the prose).

If a prior draft of this skill exists, also correct the reviewer-flagged Writing Style "Good" example: the dynamic-typing exemplar must NOT imply a name has a fixed type. Use the name-versus-value framing, for example: "In Python a value carries its type; the name does not. `x = 10` makes `x` refer to an integer; a later `x = "hi"` rebinds `x` to a string. The type travels with the value, which is why Python is dynamically typed." (This preserves the course's foundational "names are not values" distinction.)

The skill drops cs312's build-time draft-exclusion assumption: this site does not exclude drafts from the build, so do not require `draft: true` (adding true draft exclusion is a separate site-config change, out of scope). New lectures are simply added and reviewed.

- [ ] **Step 1: Read the sources.** Read `~/.claude/skills/cs312-lecture-notes/SKILL.md` (structure/tone reference) and VISION §9 and §10 in `/Users/ulbrical/GitHub/engr103/VISION.md`, plus this track's spec §2 (`docs/superpowers/specs/2026-07-02-lecture-notes-conversion-design.md`).
- [ ] **Step 2: Author (or revise) the skill** at `.claude/skills/engr103-lecture-notes/SKILL.md` with all sixteen sections above, each made concrete with a short ENGR 103 example where the section warrants one (for example a tiny `<Tabs syncKey="lang">` + `<WhatDiffers>` block in the dual-language section). Apply the Writing Style "Good" example correction.
- [ ] **Step 3: Structural self-check.**
  - Run `grep -nE "^#|^name:|ai-summary|syncKey|WhatDiffers|language-ladder|Plain-language|accuracy|LanguageVersions|Takeaways|1,800|Length" .claude/skills/engr103-lecture-notes/SKILL.md` and confirm the key sections and rules (including the accuracy pass and LanguageVersions) are present.
  - Run `grep -n "—" .claude/skills/engr103-lecture-notes/SKILL.md` and confirm no matches.
- [ ] **Step 4: Commit.**

```bash
git add .claude/skills/engr103-lecture-notes/SKILL.md
git commit -m "feat(lectures): engr103-lecture-notes skill with accuracy pass and version rule"
```

---

## Task 2: The `<LanguageVersions>` component and version source

**Files:**
- Create: `src/data/languageVersions.ts`
- Create: `src/components/LanguageVersions.astro`

**Interfaces:**
- Produces: `<LanguageVersions />` (no props), placed at the top of every content page; it reads the single source `src/data/languageVersions.ts`. Later tracks (activities, assignments, recitations) reuse it.

- [ ] **Step 1: Create the version source** `src/data/languageVersions.ts`:

```ts
export const LANGUAGE_VERSIONS = { python: "3.14", rust: "1.88" } as const;
```

- [ ] **Step 2: Create the component** `src/components/LanguageVersions.astro`. It imports `LANGUAGE_VERSIONS` and renders one small, unobtrusive line, for example "Targets Python 3.14 and Rust 1.88", styled with Starlight CSS custom properties (for example `--sl-color-gray-2`/`--sl-color-gray-6`), static, no client JS. Suggested shape:

```astro
---
import { LANGUAGE_VERSIONS } from '../data/languageVersions.ts';
const { python, rust } = LANGUAGE_VERSIONS;
---
<p class="language-versions">Targets Python {python} and Rust {rust}</p>
<style>
  .language-versions {
    font-size: var(--sl-text-xs);
    color: var(--sl-color-gray-3);
    border: 1px solid var(--sl-color-gray-5);
    border-radius: 0.375rem;
    padding: 0.25rem 0.6rem;
    display: inline-block;
    margin: 0 0 1rem;
  }
</style>
```

- [ ] **Step 3: Red/green via a temporary use.** Create a throwaway `src/content/docs/lv-demo.mdx` (frontmatter `title: LV demo`) that imports and renders `<LanguageVersions />`. Run `npm run build`; expect green (component compiles and renders). Then delete `lv-demo.mdx` and run `npm run build` again; expect green.
- [ ] **Step 4: Commit** (component + data only; the demo page is already deleted):

```bash
git add src/data/languageVersions.ts src/components/LanguageVersions.astro
git commit -m "feat(lectures): add shared LanguageVersions component and version source"
```

---

## Task 3: Convert L2 (Values, types, expressions) as the pilot

**Files:** Create `src/content/docs/lectures/values-types-expressions.mdx`

**Interfaces:**
- Consumes: the `engr103-lecture-notes` skill (Task 1); the `<LanguageVersions />` component (Task 2); the `/reference/language-ladder` page (for the Week 1 fence and the link).
- Produces: the route `/lectures/values-types-expressions`, the pilot proving the skill.

**Source:** the current C++ pages `src/content/docs/lectures/expressions.mdx`, `operators.mdx`, and `type-casting.mdx` (read them for the concept coverage to carry over), converted to dual-language per the skill. VISION §10 L2 scope: integers, floats, booleans, strings as values; static versus dynamic typing; operators, precedence, integer versus float division; evaluating expressions by hand.

**Ladder note:** L2 is a Week 1 lecture. Use only Week 1 ladder constructs. In particular, do NOT use variables (Week 2): show values and expressions evaluated directly and printed, not stored in names. This is the sharpest test of whether the skill's fence is being honored.

- [ ] **Step 1: Read the skill and sources.** Read `.claude/skills/engr103-lecture-notes/SKILL.md` (the contract you must follow, including the accuracy pass and the version-component rule), the three source pages named above, VISION §10 (the L2 row), and `src/content/docs/reference/language-ladder.mdx` (Week 1 row).
- [ ] **Step 2: Author the lecture** at `src/content/docs/lectures/values-types-expressions.mdx`:
  - Frontmatter `title: "Values, Types, and Expressions"`, a one-sentence `description`, `sidebar.order: 2`.
  - Import `LanguageVersions` from `/src/components/LanguageVersions.astro` and the Starlight `Tabs`/`TabItem` plus `WhatDiffers`.
  - The `ai-summary` block (`type: lecture`, `slug: values-types-expressions`, `order: 2`, `covers`, `prereq_lectures` and `followup_lectures` left empty for the pilot, `glossary_terms`: value, type, expression, operator, operator precedence, integer division, static typing, dynamic typing). No family or story fields.
  - `<LanguageVersions />` as the first element of the body, before the opening problem statement.
  - An opening problem statement, then concept sections (values and types; static versus dynamic typing; operators and precedence; integer versus float division; evaluating by hand), each leading with prose and ending with a `<Tabs syncKey="lang">` Python/Rust pair and a `<WhatDiffers>` callout.
  - Define each new term in plain words at first use (this is the glossary's future link target).
  - A **Takeaways** prose synthesis; an optional **Resources** list.
- [ ] **Step 3: Authoritative accuracy pass.** For every code example, run it and compare the real result to what the text claims:
  - Python: save each snippet and run `python3 <file>.py`; confirm the printed output/value/type matches.
  - Rust: wrap each snippet in a minimal `fn main() { ... }` if needed, save as `<file>.rs`, run `rustc <file>.rs -o /tmp/lv && /tmp/lv`; confirm output. (Use the scratchpad dir for these temp files.)
  - Verify any stated fact (precedence, integer-vs-float division behavior, type names) against docs.python.org and doc.rust-lang.org.
  - Fix any mismatch in the lecture, then re-run. Record the commands and results in the report.
- [ ] **Step 4: Verify the page.**
  - Run `npm run build`. Expected: green, 0 errors, all internal links valid (the `/reference/language-ladder` link resolves).
  - Run `grep -n "—" src/content/docs/lectures/values-types-expressions.mdx`. Expected: no matches.
  - Run `grep -c "syncKey=\"lang\"" src/content/docs/lectures/values-types-expressions.mdx` and confirm one per concept section (at least three); confirm a `<WhatDiffers>` accompanies each.
  - Confirm `<LanguageVersions />` is present as the first body element.
  - Run `wc -w src/content/docs/lectures/values-types-expressions.mdx` and confirm roughly 1,800 to 3,500 words.
  - Read the page once to confirm no variables or other post-Week-1 constructs appear in any example.
- [ ] **Step 5: Commit.**

```bash
git add src/content/docs/lectures/values-types-expressions.mdx
git commit -m "feat(lectures): convert L2 Values, types, expressions (dual-language pilot)"
```

---

## Pilot checkpoint (after Task 3)

Stop here and present the skill, the version component, and the L2 lecture to the user for sign-off before Wave 2. Wave 2 (the remaining 18 lectures, the section reorder and retirement of superseded pages, and the glossary body) is written as a separate plan once the skill is confirmed.

## Self-review notes (author)

- **Spec coverage (Wave 1 slice):** the skill including the accuracy pass and version rule (spec §2) is Task 1; the shared version component (spec §2, cross-cutting) is Task 2; the pilot lecture (spec §6) is Task 3. The full 19-lecture mapping (spec §3), the glossary body (spec §5), and page retirement/reorder are Wave 2, out of this plan by design.
- **Red steps where they fit:** the component (Task 2) has a real red/green via a throwaway import page; the skill (Task 1) and the lecture (Task 3) are content, verified by the green build plus structural greps, the word count, the accuracy-pass execution results, and a read-through for ladder fidelity.
- **Model:** Sonnet 5 implementers per the spec.
