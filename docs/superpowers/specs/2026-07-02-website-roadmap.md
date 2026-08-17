# ENGR 103 Website Redesign: Roadmap

Companion to `VISION.md`. This document decomposes the website-and-author-tooling
work required to realize the VISION into independently-specifiable sub-projects,
records their dependency order, and marks which one is being brainstormed first.

`VISION.md` is the authority. Where this roadmap and VISION disagree, VISION wins.

## Scope of this program

In scope: the Astro/Starlight website (content restructure, dual-language
components, reference pages, glossary) **plus** the author-facing tooling that
keeps future content conformant to VISION's house styles (§9).

Out of scope: the physical recitation-lab infrastructure described in
`ARCHITECTURE.md` (Raspberry Pi fleet, on-device network filter, Gradescope
Docker autograder, GitLab starter-repo pipeline, `check`/`pack` scripts). Those
are real deliverables tracked separately and are not the website.

## Current site (as analyzed)

- **Framework:** Astro 7, Starlight 0.41 docs theme, a single `docs` content
  collection of MDX files; sidebar auto-generated per directory.
- **Rendering:** Tailwind 4, KaTeX, `astro-mermaid`, Expressive Code with line
  numbers, Plausible analytics, sitemap, `starlight-links-validator` (broken
  links fail the build).
- **Components:** `.astro` for static illustrations (`Latex`, `AsciiTable`,
  `RubricTable`, `AssignmentRequirements`, `AssignmentAICritique`,
  `MemoryPoolDiagram`, `VariableLifecycleDiagram`); `.svelte` (Svelte 5) for
  interactive islands (`CppMemoryStepper`, `FinancialPlannerForm`,
  `LinearEquationsForm`, `ArraysStdDevForm`).
- **Two facts that drive the redesign:** the entire course is currently C++
  (VISION wants dual-language Python + Rust everywhere), and the site uses no
  `<Tabs>` today (VISION wants tabbed Python/Rust example pairs throughout).

## Sub-projects (dependency order)

Foundation gates everything. After it, Reference defines the vocabulary and
constraints content must obey; the four content genres can then proceed largely
in parallel; author tooling is extractable once one genre's style is proven.

1. **Foundation** *(first; spec: `2026-07-02-foundation-design.md`)* — IA/sidebar
   restructure; the `<Tabs syncKey="lang">` convention; the `<WhatDiffers>`
   component; the glossary hub page + linking convention; migration/redirects.
   Ships no course content.
2. **Reference pages** — language ladder, the 10-archetype inventory, language
   field guide (Fortran/R one-pagers), C++ bridge (from `cpp-basics.mdx`),
   MATLAB bridge, glossary content, and the Mission Ares / Rubber Duck Robotics
   story bibles. Glossary content depends on lecture-note anchors (see below).
3. **Lecture-notes conversion** — convert the ~19 existing C++ notes to tabbed
   Python/Rust with `<WhatDiffers>` call-outs; port `CppMemoryStepper` to a
   dual-language memory stepper (VISION §11 flags this reuse).
4. **Activities** — new genre: thin, story-free prompt sheets (a listing, a
   question, room to predict), one per family altitude-1; worked resolutions
   published after class. Also owns a standalone **git/GitHub extra-credit
   activity** (the onboarding tooling, pulled out of the lectures).
5. **Assignments** — Rubber Duck Robotics tickets (Gary), spec-to-tests
   problems (from week 3), paper practice, reflection line, summit problem.
   Absorbs the migrated `studios/` mechanics.
6. **Recitations** — Mission Ares flight specs (bare template), stretch problem,
   oral-check items, paper variant.
7. **Practicalities** — rewrite `gen-ai` around the training-set/test-set
   framing; retire `demos`; add how-tos (Gradescope, the recitation lab, the
   `check` workflow).
8. **Author tooling** *(cross-cutting)* — skills/templates encoding the four
   house styles from VISION §9 so future content stays genre-pure.

## Cross-track dependencies worth flagging early

- **Glossary → lectures.** The glossary (Reference) links each term out to the
  lecture section where it is first defined. Those outbound links need
  lecture-note anchors to exist, so glossary *content* is authored after or
  alongside the lecture conversion. Its stub page is created in Foundation.
- **Everything → Foundation.** Every content page renders through
  `<Tabs syncKey="lang">` and may reference the glossary, so Foundation's
  component and IA decisions must be settled before content work begins.
- **Reference ladder → all content.** The language ladder fences which
  constructs may appear in which week; content authors must obey it, so the
  ladder should be authored early in the Reference track.

## Status

- [x] Program decomposed and scoped (this document)
- [x] **Foundation** — specced, built, reviewed (on `website-redesign`)
- [x] **Reference pages** — specced, built, reviewed (glossary deferred to lecture track; story arc + technical conventions deferred to content tracks)
- [ ] Lecture-notes conversion
- [ ] Activities
- [ ] Assignments
- [ ] Recitations
- [ ] Practicalities
- [ ] Author tooling
