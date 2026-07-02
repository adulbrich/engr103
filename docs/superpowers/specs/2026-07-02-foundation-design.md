# Foundation: Design Spec

First sub-project of the ENGR 103 website redesign (see
`2026-07-02-website-roadmap.md`). Realizes the structural and component
substrate that every later content track depends on, per `VISION.md`.

`VISION.md` is the authority. Where this spec and VISION disagree, VISION wins.

## 1. Purpose

Foundation makes the site *able to hold* dual-language, VISION-conformant
content. It ships **no course content** itself. Its job is to settle the
IA and the shared components so that every later track (reference pages,
lecture conversion, activities, assignments, recitations) builds on a fixed,
audited substrate rather than reworking structure mid-flight.

Success means: the new sidebar renders; a reader's Python/Rust choice syncs
across all example blocks and persists site-wide; `<WhatDiffers>` renders in
light and dark; the glossary hub page exists with a fixed linking convention;
retired `studios/` URLs redirect; and `npm run build` (which runs
`astro check` + `starlight-links-validator`) passes green.

## 2. Information architecture

The `sidebar` array in `astro.config.mjs` is updated to the sections below.
Organization stays by artifact type (matches Starlight autogenerate and
VISION §9's one-house-style-per-genre rule).

| Section | Directory | Change in Foundation |
|---|---|---|
| Overview | `overview/` | add `schedule.mdx` (the VISION §10 table) beside `introduction`, `learning-outcomes` |
| Reference | `reference/` | **new** section; stub pages: `glossary`, `language-ladder`, `archetype-inventory`, `field-guide`, `cpp-bridge`, `matlab-bridge`, `story-bibles` |
| Practicalities | `practicalities/` | unchanged (content edits are their own track) |
| Lecture Notes | `lectures/` | unchanged (conversion is its own track) |
| Activities | `activities/` | **new** section; stub index |
| Assignments | `assignments/` | unchanged dir (studios migration is the Assignments track) |
| Recitations | `recitations/` | **new** section; stub index |
| About | `about/` | unchanged |

`studios/` is removed from the sidebar (retired as a framing per VISION §11).
Its files stay on disk until the Assignments track migrates their mechanics;
redirects cover the old URLs in the meantime (section 6).

Stub pages are minimal valid MDX (title + a one-line "authored in the
`<track>` sub-project" note) so the sidebar renders and internal links from
later tracks validate against real routes from day one.

## 3. The schedule page

`overview/schedule.mdx` is the navigation hub for the weekly escalator. It is
organized as **one section (H2 header) per week** (`## Week 1` … `## Week 10`,
plus `## Finals Week`), and each week's section holds a **two-column table**
mapping that week's artifacts to their pages:

| | |
|---|---|
| **Lecture 1 — How programs run** | link to the lecture page |
| **Assignment 0** | link |
| **Recitation 1** | link |
| **Lecture 2 — Values, types, expressions** | link |

Rows are listed in chronological within-week order following VISION's weekly
rhythm (Lecture A, recitation, Lecture B), with the assignment placed where it
is due (before the recitation). The **midterm** appears as a row in the Week 6
section and the **final** in the Finals Week section. Activities, once authored,
may appear as their own rows under the lecture they belong to.

The single source of truth for what each week contains is VISION §10 (now
corrected: L17 "Testing", L18 "Program design"); the schedule page is its
rendered, link-carrying form. In Foundation the page is authored with the full
per-week structure, linking to the stub routes created for every section, so
`starlight-links-validator` passes and links resolve to real content as later
tracks fill the stubs in.

## 4. Dual-language code component (Tabs)

Standardize on Starlight's built-in tabs. No custom component.

- Every dual-language example uses `<Tabs syncKey="lang">` with exactly two
  items: `<TabItem label="Python">` and `<TabItem label="Rust">`, in that
  order, each containing a complete, paste-runnable snippet (VISION §9:
  examples must be self-contained, no project scaffolding).
- The `syncKey` value is the literal string `"lang"` everywhere, so all
  example blocks stay in lockstep across the whole site.
- Persistence: `syncKey` writes the choice to `localStorage`, so the selection
  follows the reader across pages and between visits. A reader may still flip
  any single block without changing the global default (Starlight re-syncs on
  the next same-key interaction). This satisfies VISION's "pick a language but
  you may mix within a week."

Foundation fixes the pattern; the human-readable authoring convention for it is
written in the Author-tooling track. The pattern is proven by the build-time
smoke check in section 8 (a temporary scratch page with two `syncKey="lang"`
blocks, deleted before merge). No course content is authored in Foundation.

## 5. `<WhatDiffers>` component

`src/components/WhatDiffers.astro`. Static, zero client JS, following the
existing `AssignmentRequirements` / `RubricTable` `.astro` pattern.

- Two named slots: `differs` and `cannot`.
- Renders a single card with two labeled regions: "What differs" and
  "What cannot differ", visually matching the approved mockup (a bordered card,
  a distinct marker per region, bullet lists inside each).
- Styling uses Starlight's CSS custom properties so it inherits light/dark
  themes automatically; no hard-coded colors.
- Authoring shape:

  ```mdx
  <WhatDiffers>
    <Fragment slot="differs">
      - Python infers the type; Rust needs `: f64`.
      - `mut` vs. plain rebinding.
    </Fragment>
    <Fragment slot="cannot">
      - Same formula, same result.
      - The loop runs the same number of times.
    </Fragment>
  </WhatDiffers>
  ```

The component ships in Foundation but is exercised by content in later tracks.

## 6. Glossary hub page

`reference/glossary.mdx` is a **hub index**, not a parallel store of
definitions. Each entry is:

1. the term as an `##` heading (so it gets a stable anchor: `#function`),
2. one plain-language sentence defining it, and
3. a link out to the lecture-note section where the term is first introduced
   and explained in full (the authoritative source).

Other artifacts that need to reference a term link to the glossary entry
(`/reference/glossary#function`); the glossary then routes the reader to the
detailed lecture explanation. This keeps one authoritative definition (in the
lecture) and prevents two definitions drifting apart.

Consequence recorded for planning: the glossary's outbound links depend on
lecture-note anchors existing, so the glossary's **content** is authored in the
Reference track after/alongside lecture conversion. In Foundation the page is a
stub. `starlight-links-validator` guards both the inbound anchors and, once
authored, the outbound lecture links, so term references cannot rot.

## 7. Migration, redirects, and language persistence

- **Redirects:** add Astro `redirects` in `astro.config.mjs` for the retired
  `studios/*` routes. Until the Assignments track defines exact targets, point
  them at a sensible existing page (e.g. the assignments index) so external
  links and search results survive.
- **Language persistence:** entirely handled by `syncKey` → `localStorage`
  (section 4). No custom state, no header switcher, no new JS.

## 8. Verification

- `npm run build` passes (`astro check` type-check + `starlight-links-validator`
  with zero broken links).
- Manual smoke check: on a page with two `<Tabs syncKey="lang">` blocks,
  switching one switches the other, and the choice survives a reload and a
  navigation to another page.
- Visual check: `<WhatDiffers>` renders correctly in both light and dark modes.
- Sidebar shows the eight sections in the intended order with no empty/broken
  section.

## 9. Explicitly out of scope (later sub-projects)

Converting lecture notes to dual-language; the dual-language memory stepper;
authoring any reference content (ladder, archetype inventory, field guide,
bridges, glossary body, story bibles); authoring activities, assignments, or
recitations; the gen-ai practicalities rewrite; retiring the demos page;
author-tooling skills. These are tracked in the roadmap and each gets its own
spec.
