# Lecture-notes Conversion: Design Spec

Third sub-project of the ENGR 103 website redesign (see
`2026-07-02-website-roadmap.md`), building on Foundation and the Reference
pages. Converts the existing C++ lecture notes into VISION §10's nineteen
dual-language (Python + Rust) concept lectures, and authors an ENGR 103
lecture-notes style skill to govern that conversion and all future edits.

`VISION.md` is the authority. Where this spec and VISION disagree, VISION wins.

## 1. Shape of the track

Three deliverables, in order:

1. **The `engr103-lecture-notes` skill**, the style contract every lecture
   conversion follows (section 2).
2. **Nineteen dual-language concept lectures** matching VISION §10, built by
   converting and restructuring the current C++ pages (section 3).
3. **The glossary body**, deferred here by the Reference track because each
   term links out to the lecture section that defines it; those anchors exist
   only once the lectures are converted (section 5).

Deferred out of this track: the interactive dual-language memory stepper (its
own component-engineering effort), and relocating the tooling pages
`git.mdx` and `development-environment.mdx` from lectures to Practicalities
(that track's job).

## 2. The `engr103-lecture-notes` skill

Adapted from the existing `cs312-lecture-notes` skill (a mature lecture-notes
style guide for a sysadmin course). The skill is authored first and is the
detailed, enforceable form of VISION §9's lecture-notes house style.

**Kept from cs312 (course-agnostic and valuable):** the no-emdash rule;
concept-first and prose-over-lists; the `ai-summary` block (cross-page
alignment, even more useful here given glossary and schedule links); the
draft-flag policy (only the instructor publishes); a Takeaways synthesis
section; build validation; length treated as a diagnostic, not a target.

**Changed or added for ENGR 103:**

- **Dual-language spine.** Every concept section ends with a
  `<Tabs syncKey="lang">` Python then Rust example pair (each self-contained
  and paste-runnable) and a `<WhatDiffers>` callout naming what differs and
  what cannot differ between the two renderings.
- **Plain-language audience.** The reader is a first-year who has never
  programmed and may not speak English as a first language: short sentences,
  common words, no idioms, every technical term defined in plain words at first
  use, one name per concept forever.
- **Language-ladder fence.** Examples use only constructs on the language ladder
  for that week (link to `/reference/language-ladder`). No construct appears
  before the ladder introduces it.
- **Glossary discipline.** The lecture is where a term gets its canonical
  plain-language definition and a stable heading anchor; the glossary links in
  (it does not restate the definition).
- **Concepts and dual-language only, fully story-agnostic.** No git, shell,
  editor, or other tooling, and no reference to stories, problem families,
  assignments, activities, or recitations. A lecture teaches the concept in
  Python and Rust and says nothing about how or where it is later exercised.
  This deliberately **overrides** VISION §9's "names the problem family where
  the concept will be exercised" clause, per instructor decision: the narrative
  and assessment structure lives entirely in the activity, assignment, and
  recitation tiers, never in the notes.
- **ENGR imports.** The available components are `WhatDiffers`, `Latex`,
  `AsciiTable`, and the static memory diagrams (`MemoryPoolDiagram`,
  `VariableLifecycleDiagram`), plus Starlight's `Tabs`/`TabItem`/`Aside`.
  Schedule Alignment points at `overview/schedule.mdx`.
- **Tone.** Gentler and plainer than cs312's confident-expert voice, calibrated
  to a nervous beginner, while still explaining the "why" behind each concept.

## 3. Old to new lecture mapping

The nineteen new lectures and the current pages each is built from. VISION §11
is the authority for intent ("most existing pages map directly; the work is
converting C++ examples to tabbed Python/Rust pairs and adding the what-differs
call-outs").

| New (VISION §10) | Built from current pages |
|---|---|
| L1 How programs run | `introduction` plus the conceptual parts of `cpp-basics` (source, interpret, compile); new two-language, ladder, systematic-debugging, and modeling-versus-analysis threads |
| L2 Values, types, expressions | `expressions` + `operators` + `type-casting` |
| L3 Variables and state | `variables` + `constants` + `shorthand-operators` |
| L4 Functions I | `functions` (mechanics) |
| L5 Booleans and conditionals | `booleans` + `if-statements` |
| L6 Decision structures | `if-statements` (chains and `match`); new decision tables |
| L7 Loops | `loops` (while and for) |
| L8 Loop patterns | `loops` (patterns and nesting); new pattern material |
| L9 Functions II: scope and the call stack | `scope` + `functions` (call stack) |
| L10 Collections I | `arrays` |
| L11 Strings and characters | `strings` + `characters` + `string-functions` |
| L12 The memory model | `references` + the `memory-stepper` page (static diagrams) |
| L13 Sharing and mutation | `references` (passing collections); new |
| L14 Errors, input, and validation | `error-handling`; new input parsing |
| L15 Numbers in machines | new (integer overflow, floating point, units) |
| L16 Collections II | new (dictionaries and hashmaps) |
| L17 Testing | new |
| L18 Program design | new (from the Polya practicalities page) |
| L19 Computing with judgment | new (outcome O8) |

Disposition of current pages not carried forward as their own lecture:

- `comments` folds into L1 or L2 as a brief note, not its own lecture.
- `cpp-basics` is retired from the lectures section; its bridge role now lives
  in the Reference C++ bridge.
- `development-environment` stays in place for the Practicalities track to
  relocate as a how-to guide.
- `git` / GitHub is neither a lecture nor a Practicalities how-to: it becomes its
  own **extra-credit activity** (owned by the Activities track). It stays in
  place until that track authors it.
- The `memory-stepper` page's content folds into L12; the interactive component
  is deferred (section 1).

The new lecture files use slugs matching their concepts and `sidebar.order`
values 1 through 19 in schedule order. Retiring or renaming old files that no
longer map (for example `cpp-basics`) is done with redirects so external links
survive, mirroring the Foundation approach.

## 4. Length

A diagnostic band of roughly **1,800 to 3,500 words** per lecture: these are
flipped notes read in the roughly 45-minute pre-lecture budget by novices, with
code examples included, so they are much shorter than cs312's 6,000 to 10,000
word band. The band is a diagnostic, not a target; a concept that genuinely
needs more space may use it, and a short one should stop.

## 5. The glossary body

`reference/glossary.mdx` (a Foundation stub) is authored last in this track,
after the lectures exist. Each entry is the term as an `##` heading, one
plain-language sentence, and a link to the lecture section where the term is
first defined and explained in full. Authored from the finished lectures so
every outbound link resolves; `starlight-links-validator` enforces this.

## 6. Execution

- **Writers:** Sonnet 5 implementer subagents (per the user's instruction for
  this track).
- **Skill-first with a pilot checkpoint:** author the skill, convert one
  lecture (L2, Values/types/expressions, a meaty early concept) using it,
  review both, and get the user's sign-off before converting the remaining
  eighteen and authoring the glossary. This keeps the work a single track while
  ensuring a flawed skill does not propagate nineteen times.
- Per-task reviews throughout (spec fidelity plus the plain-language and
  dual-language rules), a final whole-branch review, on branch
  `website-redesign`, never `main`.

## 7. Verification

The site's only automated test, `npm run build` (`astro check` plus
`starlight-links-validator`), must stay green. Per-lecture checks: no emdashes;
every concept section carries a `<Tabs syncKey="lang">` Python/Rust pair and a
`<WhatDiffers>` callout; examples stay within the week's ladder; every new term
is defined in plain words at first use; length within the band; internal links
resolve.

## 8. Out of scope

The interactive dual-language memory stepper; relocating `git` and
`development-environment` to Practicalities; the Numbers-in-machines,
Collections-II, Testing, Program-design, and Computing-with-judgment problem
sets (content tracks); and any change to the language ladder or archetype
inventory (Reference track, already done).
