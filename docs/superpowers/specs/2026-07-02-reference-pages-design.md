# Reference Pages: Design Spec

Second sub-project of the ENGR 103 website redesign (see
`2026-07-02-website-roadmap.md`), building on Foundation
(`2026-07-02-foundation-design.md`). Authors the student-facing Reference
section that defines the vocabulary and constraints all later content must obey.

`VISION.md` is the authority. Where this spec and VISION disagree, VISION wins.

## 1. Purpose and scope

The Reference section is the "rules of the game" students consult all term: what
language constructs are allowed when, what the exam question types are, and how
the course subset maps onto other languages. Foundation already created the
seven Reference stub pages and the sidebar section; this track fills six of them
with content.

**In scope (six pages):** language ladder, archetype inventory, field guide,
C++ bridge, MATLAB bridge, story bibles.

**Deferred (kept as their Foundation stubs):**

- **Glossary** goes to the lecture-notes track. By design it links each term out
  to the lecture section that explains it (VISION §9); those anchors do not exist
  until the lectures are converted to dual-language, so authoring the links now
  means redoing them.
- **Story bibles: the season arc and the recurring technical conventions**
  (data formats, function names, provided helpers) go to the content tracks that
  define them. This track ships only the stable orientation material.

The sidebar order set by Foundation is unchanged: glossary (0), language-ladder
(1), archetype-inventory (2), field-guide (3), cpp-bridge (4), matlab-bridge
(5), story-bibles (6).

## 2. House style and presentation

All six pages are the **Reference genre** (VISION §9): terse tables and lists,
minimal connecting prose, every technical term used consistently, and the
plain-language rule throughout (short sentences, common words, no idioms). No
emdashes anywhere; use proper punctuation.

**Dual-language content is shown side by side, not in tabs.** Where a page shows
both Python and Rust (the ladder) or maps a course construct onto another
language (the bridges and field guide), both forms are always visible at once,
in adjacent table columns. The `<Tabs syncKey="lang">` component is for lecture
examples, where a reader picks one language and reads it; a reference page exists
so the reader can compare the two at a glance, so hiding one behind a tab defeats
the page. Small code fragments may appear inside table cells; the pages carry no
runnable, standalone program listings.

## 3. The six pages

Each page is authored from VISION, not invented. Content that VISION states
(the ten archetypes, the family-to-costume table) is reproduced faithfully in
the Reference genre; content VISION implies (the week-by-week subset) is derived
mechanically from the stated rules and reviewed.

### 3.1 Language ladder (`reference/language-ladder.mdx`)

The fence that grows week by week. Derived from the lecture schedule (VISION §10)
and the subset rules (VISION §1: early weeks have no comprehensions, no f-string
format specs beyond basics, no lambdas in Python; no borrowing, no traits, no
lifetimes, everything in `main` plus free functions in Rust).

Shape: one cumulative row per course week (Week 1 through Week 10), with a Python
column and a Rust column naming the constructs that become permitted that week,
and a short note on what remains fenced off. Cumulative means a construct
introduced in an earlier week stays available. The page states plainly that
anything not yet on the ladder must not appear in take-home work, and that
experienced students may use anything already introduced.

This page is authored first: the bridges and the field guide map the subset it
defines.

### 3.2 Archetype inventory (`reference/archetype-inventory.mdx`)

The ten published exam question archetypes (VISION §4), reproduced as a numbered
list with one terse entry each: the archetype name and a one-line description of
what the student does (for example "Trace: fill a variable-value table for a loop
or a call, line by line"). Independent of the other pages. Faithful to VISION §4's
list and numbering.

### 3.3 Field guide (`reference/field-guide.mdx`)

One-page concept maps for languages beyond the two course languages, for students
whose major uses them. Languages: **Fortran and R only** (VISION §1 names these;
the format extends cheaply later). Fixed format per language: the course subset
mapped construct by construct (course concept in one column, that language's form
in another) plus a short "what will surprise you" list (for example 1-indexing in
both R and Fortran). Links to the language ladder. Depends on the ladder's subset.

### 3.4 C++ bridge (`reference/cpp-bridge.mdx`)

The course subset of Rust mapped onto C++ syntax, for students continuing into
C++-based courses (VISION §1, §11). Authored **fresh** as a construct-by-construct
mapping table (Rust course-subset form, C++ form) plus a "what will surprise you"
note. It may salvage orientation fragments from the existing `cpp-basics.mdx`
lecture (for example the build pipeline or hello-world), but it is not a light
edit of that lecture, which is intro-C++ content, not a bridge. Links to the
language ladder. Depends on the ladder's subset.

Retiring or redirecting the old `cpp-basics.mdx` lecture is out of scope here (a
lecture-track concern).

### 3.5 MATLAB bridge (`reference/matlab-bridge.mdx`)

The course subset mapped onto MATLAB, for students heading into MATLAB-based major
courses (VISION §1). Same shape as the C++ bridge: a construct-by-construct
mapping table plus a "what will surprise you" list (1-indexing, matrix-first,
weak typing). Links to the language ladder. Depends on the ladder's subset.

### 3.6 Story bibles (`reference/story-bibles.mdx`)

Lean student-facing orientation, stable parts of VISION §8 only:

- The two settings: Mission Ares (the class as flight software crew of a crewed
  Mars habitat, the face worn by recitations and exams) and Rubber Duck Robotics
  (the student as firmware intern at a novelty-gadget company, the face worn by
  assignments).
- The recurring characters: HAB (the habitat AI, helpful and confidently wrong,
  how find-the-bug and judge-this-code problems arrive) and Gary (the
  overcommitted senior engineer whose tickets are reliably wrong about one thing).
- The family-to-costume table (VISION §8): each problem family and its Rubber Duck
  and Mission Ares faces.

Explicitly omitted here: the week-by-week season arc and the recurring technical
conventions (data formats, function names, provided helpers), both deferred to
the content tracks.

## 4. Ordering within the track

1. Language ladder (defines the subset the bridges and field guide map).
2. Archetype inventory and story bibles (independent; any time).
3. C++ bridge, MATLAB bridge, field guide (map the subset; after the ladder).

## 5. Verification

Same gate as Foundation, the site's only automated test: `npm run build` passes
(`astro check` type-check plus `starlight-links-validator`), with no broken
links and no emdashes. Internal links (bridges and field guide to the language
ladder) must resolve. Each page is additionally reviewed for fidelity to its
VISION source and for the plain-language rule.

## 6. Execution note

Page content is authored by opus implementer subagents (higher-capability model),
per the user's instruction, because the pages require faithful derivation from
VISION rather than mechanical transcription. Reviews may use a mid-tier model
scaled to each page.

## 7. Out of scope

The glossary body; the story bibles' season arc and recurring technical
conventions; retiring or redirecting the old `cpp-basics.mdx` lecture; any
language-subset decisions beyond what the ladder codifies; and any use of the
`<Tabs syncKey>` component (reference content is side-by-side, section 2).
