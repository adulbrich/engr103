# Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Establish the structural and component substrate (IA/sidebar, the dual-language `<Tabs syncKey>` convention, the `<WhatDiffers>` call-out, the glossary hub stub, and the weekly schedule) that every later ENGR 103 content track depends on, shipping no course content itself.

**Architecture:** The site is Astro + Starlight with a single `docs` MDX collection whose sidebar is auto-generated per directory. Foundation adds three new sidebar sections (Reference, Activities, Recitations) backed by stub pages, one new `.astro` component, and one schedule page in Overview. It standardizes on Starlight's built-in `<Tabs syncKey="lang">` (no custom component) for all future dual-language examples.

**Tech Stack:** Astro + Starlight (the `<Tabs syncKey>` prop, `autogenerate` sidebar, and `starlight-links-validator` are all present in the installed versions; do not upgrade or downgrade as part of this work), Tailwind 4 via `@astrojs/starlight-tailwind`, MDX, `.astro` components.

## Global Constraints

- **The only automated test is `npm run build`.** It runs `astro check` (type-checking) then `astro build`, and the build runs `starlight-links-validator`, which fails the build on any broken internal link. "Run the test" always means run `npm run build` and confirm it exits 0 with no type errors and no invalid-link errors.
- **Every content page (`.mdx`) needs frontmatter with at least `title:`** (a Starlight requirement). Use `sidebar.order:` to control position within a section.
- **Dual-language code convention:** every dual-language example uses `<Tabs syncKey="lang">` with `<TabItem label="Python">` first, then `<TabItem label="Rust">`, each holding a complete, paste-runnable snippet. The `syncKey` value is always the literal string `"lang"`.
- **Component styling uses Starlight CSS custom properties** (`--sl-color-gray-5`, `--sl-color-gray-6`, `--sl-color-white`, etc.) so light/dark theming is automatic. No hard-coded hex colors.
- **No emdashes in prose.** Use proper punctuation (commas, colons, parentheses).
- **Foundation ships no course content.** New stub pages carry only a `title`, a `description`, a `sidebar.order`, and a one-line note naming the track that will author them.
- **Branch:** work on `website-redesign`. Commit after each task.

---

## File Structure

- **Create** `src/components/WhatDiffers.astro` — the two-slot "what differs / what cannot differ" call-out. One responsibility: render the paired call-out; no logic.
- **Modify** `astro.config.mjs` — the `sidebar` array only: drop `Studios`, add `Reference`, `Activities`, `Recitations` in the intended order.
- **Create** stub pages (title + description + one-line note):
  - `src/content/docs/reference/glossary.mdx` (also records the term-linking convention)
  - `src/content/docs/reference/language-ladder.mdx`
  - `src/content/docs/reference/archetype-inventory.mdx`
  - `src/content/docs/reference/field-guide.mdx`
  - `src/content/docs/reference/cpp-bridge.mdx`
  - `src/content/docs/reference/matlab-bridge.mdx`
  - `src/content/docs/reference/story-bibles.mdx`
  - `src/content/docs/activities/index.mdx`
  - `src/content/docs/recitations/index.mdx`
- **Create** `src/content/docs/overview/schedule.mdx` — the per-week schedule hub.
- **Temporary** `src/content/docs/wd-demo.mdx` — created and deleted inside Task 1 to red/green the component; never committed.

## Deferred out of Foundation (do not implement here)

- **Redirects for `studios/*`.** The `studios/` files stay on disk, so their `/studios/<slug>` pages still resolve; there are no broken links to redirect. Removing `Studios` from the sidebar (Task 2) is enough to retire the framing. Redirects belong to the **Assignments track**, when those files are actually migrated or deleted. (This refines spec §7, which assumed removal.)
- **Wiring schedule rows to artifact pages.** Most target pages (per-week lectures with new titles, per-week activities and recitations) do not exist yet. The schedule ships its full structure now; each later track hyperlinks its own artifacts when their pages exist.

---

## Task 1: The `<WhatDiffers>` component

**Files:**
- Create: `src/components/WhatDiffers.astro`
- Create then delete (verification harness, never committed): `src/content/docs/wd-demo.mdx`

**Interfaces:**
- Consumes: nothing.
- Produces: a component used as `<WhatDiffers>` with two named slots, `differs` and `cannot`. Authoring form:
  ```mdx
  import WhatDiffers from '../../../components/WhatDiffers.astro'

  <WhatDiffers>
    <Fragment slot="differs">
      - Point A.
    </Fragment>
    <Fragment slot="cannot">
      - Point B.
    </Fragment>
  </WhatDiffers>
  ```

- [ ] **Step 1: Write the failing test (a demo page that imports the not-yet-existing component)**

Create `src/content/docs/wd-demo.mdx`:

```mdx
---
title: WhatDiffers demo (temporary)
description: Temporary page to verify the WhatDiffers component and the Tabs syncKey convention. Deleted before commit.
---

import { Tabs, TabItem } from '@astrojs/starlight/components';
import WhatDiffers from '../../components/WhatDiffers.astro';

## First example

<Tabs syncKey="lang">
  <TabItem label="Python">
    ```python
    def area(r):
        return 3.14159 * r * r
    ```
  </TabItem>
  <TabItem label="Rust">
    ```rust
    fn area(r: f64) -> f64 {
        3.14159 * r * r
    }
    ```
  </TabItem>
</Tabs>

<WhatDiffers>
  <Fragment slot="differs">
    - Python infers the type; Rust needs `: f64` and `-> f64`.
    - Rust returns the last expression with no `return` keyword.
  </Fragment>
  <Fragment slot="cannot">
    - The formula is identical and produces the same value.
    - Both take one number and give one number back.
  </Fragment>
</WhatDiffers>

## Second example (proves the tabs sync)

<Tabs syncKey="lang">
  <TabItem label="Python">
    ```python
    print("hello")
    ```
  </TabItem>
  <TabItem label="Rust">
    ```rust
    fn main() {
        println!("hello");
    }
    ```
  </TabItem>
</Tabs>
```

- [ ] **Step 2: Run the build to verify it fails**

Run: `npm run build`
Expected: FAIL. `astro check`/`astro build` reports it cannot resolve `../../components/WhatDiffers.astro` (module not found).

- [ ] **Step 3: Create the component**

Create `src/components/WhatDiffers.astro`:

```astro
---
// A two-part call-out that pairs with a dual-language (Python/Rust) tabbed
// example: what differs between the two languages, and what cannot differ.
// Static, no client JS. Styled with Starlight CSS variables so it adapts to
// light and dark themes automatically.
---

<aside class="what-differs" aria-label="What differs, what cannot differ">
  <div class="wd-section">
    <p class="wd-title">&#8644; What differs</p>
    <slot name="differs" />
  </div>
  <div class="wd-section">
    <p class="wd-title">&#8801; What cannot differ</p>
    <slot name="cannot" />
  </div>
</aside>

<style>
  .what-differs {
    border: 1px solid var(--sl-color-gray-5);
    border-radius: 0.5rem;
    background-color: var(--sl-color-gray-6);
    padding: 1rem 1.25rem;
    margin: 1.5rem 0;
  }
  .wd-section + .wd-section {
    margin-top: 0.75rem;
    padding-top: 0.75rem;
    border-top: 1px solid var(--sl-color-gray-5);
  }
  .wd-title {
    font-weight: 600;
    color: var(--sl-color-white);
    margin: 0 0 0.25rem;
  }
</style>
```

- [ ] **Step 4: Run the build to verify it passes**

Run: `npm run build`
Expected: PASS. Exit 0, `astro check` reports 0 errors, no invalid-link errors, build completes.

- [ ] **Step 5: Visual check in the dev server**

Run: `npm run dev`
Open `http://localhost:4321/wd-demo`. Confirm:
- The call-out renders as one bordered card with two labeled sections ("What differs", "What cannot differ"), each showing its bullet list.
- Toggle the site theme (light/dark control in the header): border, background, and title text stay legible in both.
- Click the Rust tab on the first example; the second example switches to Rust too (they share `syncKey="lang"`). Reload the page; the choice persists.
Stop the dev server (Ctrl+C).

- [ ] **Step 6: Remove the temporary demo page and re-verify the build**

Run: `rm src/content/docs/wd-demo.mdx`
Run: `npm run build`
Expected: PASS, exit 0.

- [ ] **Step 7: Commit (component only; the demo page is already deleted)**

```bash
git add src/components/WhatDiffers.astro
git commit -m "feat(foundation): add WhatDiffers dual-language call-out component"
```

---

## Task 2: IA/sidebar restructure and stub pages

**Files:**
- Modify: `astro.config.mjs` (the `sidebar` array)
- Create: the seven `reference/*.mdx` stubs, `activities/index.mdx`, `recitations/index.mdx` (paths listed in File Structure)

**Interfaces:**
- Consumes: nothing.
- Produces: the routes `/reference/glossary`, `/reference/language-ladder`, `/reference/archetype-inventory`, `/reference/field-guide`, `/reference/cpp-bridge`, `/reference/matlab-bridge`, `/reference/story-bibles`, `/activities`, `/recitations`. Later tracks (and Task 3) may link to these.

- [ ] **Step 1: Update the sidebar array in `astro.config.mjs`**

Replace the existing `sidebar` array (currently Overview, Practicalities, Lecture Notes, Studios, Assignments, About) with this exact block:

```js
      sidebar: [
        {
          label: "Overview",
          autogenerate: { directory: "overview" },
        },
        {
          label: "Reference",
          autogenerate: { directory: "reference" },
        },
        {
          label: "Practicalities",
          autogenerate: { directory: "practicalities" },
        },
        {
          label: "Lecture Notes",
          autogenerate: { directory: "lectures" },
        },
        {
          label: "Activities",
          autogenerate: { directory: "activities" },
        },
        {
          label: "Assignments",
          autogenerate: { directory: "assignments" },
        },
        {
          label: "Recitations",
          autogenerate: { directory: "recitations" },
        },
        {
          label: "About",
          autogenerate: { directory: "about" },
        },
      ],
```

- [ ] **Step 2: Run the build to observe the pre-stub state**

Run: `npm run build`
Expected: the build errors because `autogenerate` now points at `reference`, `activities`, and `recitations`, which have no pages yet. (If a given Starlight version instead renders those groups empty and the build passes, that is acceptable; the definitive gate is the green build in Step 5 after the stubs exist.)

- [ ] **Step 3: Create the two Activities/Recitations index stubs**

Create `src/content/docs/activities/index.mdx`:

```mdx
---
title: Activities
description: In-lecture activities for ENGR 103, the guided first altitude of each problem family.
sidebar:
  order: 0
---

Activity prompt sheets are authored in the Activities sub-project.
```

Create `src/content/docs/recitations/index.mdx`:

```mdx
---
title: Recitations
description: Proctored, in-person recitations for ENGR 103, the performance tier of each problem family.
sidebar:
  order: 0
---

Recitation flight specs are authored in the Recitations sub-project.
```

- [ ] **Step 4: Create the seven Reference stubs**

Every Reference stub uses this template, substituting the per-file values from the table below:

```mdx
---
title: <TITLE>
description: <DESCRIPTION>
sidebar:
  order: <ORDER>
---

This reference page is authored in the Reference sub-project (see the project roadmap). It is a placeholder so the section and its links resolve.
```

| File | `<TITLE>` | `<ORDER>` | `<DESCRIPTION>` |
|---|---|---|---|
| `reference/glossary.mdx` | Glossary | 0 | One plain-language definition per course term, each linking to the lecture that explains it in full. |
| `reference/language-ladder.mdx` | Language Ladder | 1 | The Python and Rust constructs permitted each week, so examples and take-home work stay inside the course subset. |
| `reference/archetype-inventory.mdx` | Archetype Inventory | 2 | The ten published exam question archetypes practiced all term. |
| `reference/field-guide.mdx` | Language Field Guide | 3 | One-page concept maps for languages beyond the two course languages (for example Fortran and R). |
| `reference/cpp-bridge.mdx` | C++ Bridge | 4 | The course subset of Rust mapped onto C++ syntax, for students continuing into C++ courses. |
| `reference/matlab-bridge.mdx` | MATLAB Bridge | 5 | The course subset mapped onto MATLAB, for students heading into MATLAB-based courses. |
| `reference/story-bibles.mdx` | Story Bibles | 6 | The Mission Ares and Rubber Duck Robotics story lines and their recurring technical conventions. |

For `reference/glossary.mdx` only, append this convention note under the placeholder line, so the linking rule is recorded where authors will look for it:

```mdx

## How this glossary works (author note)

Each term below is an `##` heading (giving it a stable anchor such as
`#function`), one plain-language sentence of definition, and a link to the
lecture section where the term is first introduced and explained in full. Other
artifacts reference a term by linking to its entry here (for example
`/reference/glossary#function`); this page routes the reader on to the lecture.
Terms are authored in the Reference sub-project, after lecture-note anchors exist.
```

- [ ] **Step 5: Run the build to verify it passes**

Run: `npm run build`
Expected: PASS, exit 0, `astro check` reports 0 errors, no invalid-link errors. The three new sidebar groups now have entries.

- [ ] **Step 6: Confirm the sidebar in the dev server**

Run: `npm run dev`
Open `http://localhost:4321/`. Confirm the sidebar shows, in order: Overview, Reference, Practicalities, Lecture Notes, Activities, Assignments, Recitations, About. Confirm "Studios" is gone. Confirm an old studio URL such as `http://localhost:4321/studios/loops` still loads (pages remain on disk, just off the sidebar).
Stop the dev server.

- [ ] **Step 7: Commit**

```bash
git add astro.config.mjs src/content/docs/reference src/content/docs/activities src/content/docs/recitations
git commit -m "feat(foundation): restructure sidebar and add Reference/Activities/Recitations stubs"
```

---

## Task 3: The schedule page

**Files:**
- Create: `src/content/docs/overview/schedule.mdx`

**Interfaces:**
- Consumes: nothing (links are unwired for now).
- Produces: the route `/overview/schedule`, linked from the Activities and Recitations index stubs.

Design: one `##` header per week (plus a Finals Week section). Each week holds a two-column table. The left column is the bold artifact label (it becomes a hyperlink when that artifact's page is authored in a later track). The right column is a one-line topic/family note, which stays useful even after the label is linked. Rows are in within-week order: Lecture A, the assignment due before the recitation, the recitation, then Lecture B. The single source of truth for contents is VISION §10 (with L17 "Testing" and L18 "Program design").

- [ ] **Step 1: Create the schedule page**

Create `src/content/docs/overview/schedule.mdx`:

```mdx
---
title: Schedule
description: The ten-week ENGR 103 schedule. Each week lists its lectures, the assignment due that week, and the recitation, showing how each problem family escalates from lecture to assignment to recitation to exam.
sidebar:
  order: 2
---

This is the weekly map of the course. Each week lists its two lectures, the
assignment due that week, and the recitation. Artifact names become links as
each page is authored; the right column names the topic or problem family.

The rhythm each week is Lecture A, then the recitation (with that week's
assignment due just before it), then Lecture B.

## Week 1

| | |
|---|---|
| **Lecture 1: How programs run** | Computation, interpreters and compilers, errors as messages, the systematic debugging method, modeling versus analysis, the two-language philosophy |
| **Assignment 0** | Setup, toolchains, hello in both languages |
| **Recitation 1 (ungraded)** | Onboarding: environment, terminal, editor, git, the `check` harness |
| **Lecture 2: Values, types, expressions** | Integers, floats, booleans, strings; static versus dynamic typing; operators and precedence |

## Week 2

| | |
|---|---|
| **Lecture 3: Variables and state** | Names versus values, assignment and rebinding, `let`/`let mut`, tracing with a variable table |
| **Assignment 1** | Launch Window arithmetic families and a paper trace |
| **Recitation 2** | Launch Window: expressions, types, variables |
| **Lecture 4: Functions I** | Defining and calling, parameters and return values, signatures, how the autograder calls your functions |

## Week 3

| | |
|---|---|
| **Lecture 5: Booleans and conditionals** | Comparisons, logical operators, truth tables, `if`/`else` from specifications |
| **Assignment 2** | Launch Window function families and a paper trace |
| **Recitation 3** | Launch Window: writing and testing functions |
| **Lecture 6: Decision structures** | `elif`/`else if` chains and `match`, decision tables, guard clauses, boundary bugs |

## Week 4

| | |
|---|---|
| **Lecture 7: Loops** | `while` for unknown counts, counted loops, loop variables, termination, tracing loops |
| **Assignment 3** | Airlock rule families and a paper trace |
| **Recitation 4** | Airlock: conditionals and decision tables |
| **Lecture 8: Loop patterns** | Accumulate, count, search, sentinel, validate-until-correct, nested loops |

## Week 5

| | |
|---|---|
| **Lecture 9: Functions II, scope and the call stack** | Local scope, the call stack by hand, decomposing a program into functions |
| **Assignment 4** | Rover drive families and call-stack paper practice |
| **Recitation 5** | Rover: loops and functions synthesis |
| **Lecture 10: Collections I** | Lists and vectors, indexing, iteration, growing and mutating, out-of-bounds as the canonical run-time error |

## Week 6

| | |
|---|---|
| **Lecture 11: Strings and characters** | Strings as sequences, characters and encodings, slicing, searching, building strings |
| **Assignment 5** | Cumulative families and a midterm-format paper practice set |
| **Recitation 6** | Mission Readiness Review: cumulative synthesis, doubles as midterm rehearsal |
| **Midterm exam** | On paper, lectures 1 to 9, held in the second lecture slot |

## Week 7

| | |
|---|---|
| **Lecture 12: The memory model** | Names point at values, aliasing in Python and ownership/borrowing-lite in Rust, memory diagrams |
| **Assignment 6** | Comms encode/decode families and a paper trace |
| **Recitation 7** | Comms: lists and strings, ciphers and checksums |
| **Lecture 13: Sharing and mutation** | Passing collections to functions, when the caller sees your changes, defensive copying |

## Week 8

| | |
|---|---|
| **Lecture 14: Errors, input, and validation** | Kinds of errors, console input at last, exceptions versus `Result`, validating input |
| **Assignment 7** | Memory and sharing families and memory-diagram paper practice |
| **Recitation 8** | Comms: memory model and functions over collections, packet buffers |
| **Lecture 15: Numbers in machines** | Integer overflow, floating-point representation, comparison with tolerance, units and magnitudes |

## Week 9

| | |
|---|---|
| **Lecture 16: Collections II** | Dictionaries and hashmaps, key-value thinking, choosing between list and map, frequency counting |
| **Assignment 8** | Telemetry watchdog families and paper practice |
| **Recitation 9** | Telemetry: robust numeric programs, validation and floats |
| **Lecture 17: Testing** | Test cases from specifications, boundary and error cases, assertions and test functions |

## Week 10

| | |
|---|---|
| **Lecture 18: Program design** | Decomposition top-down, inputs/outputs/assumptions, from word problem to program; synthesis only |
| **Assignment 9** | Manifest map families, capstone prep, accessibility/equity critique, final-format paper practice |
| **Recitation 10** | Capstone: the Sol 100 mission-status program with tests |
| **Lecture 19: Computing with judgment** | Evaluating code you did not write, limitations and failure modes, course synthesis |

## Finals Week

| | |
|---|---|
| **Final exam** | On paper, cumulative, weighted to weeks 6 to 10 |
```

- [ ] **Step 2: Run the build to verify it passes**

Run: `npm run build`
Expected: PASS, exit 0, no invalid-link errors.

- [ ] **Step 3: Visual check**

Run: `npm run dev`
Open `http://localhost:4321/overview/schedule`. Confirm ten week sections plus Finals Week, each with a two-column table, and that "Schedule" appears in the Overview sidebar group after "Introduction" and "Course Learning Outcomes".
Stop the dev server.

- [ ] **Step 4: Commit**

```bash
git add src/content/docs/overview/schedule.mdx
git commit -m "feat(foundation): add weekly schedule page to Overview"
```

---

## Task 4: Foundation acceptance

**Files:** none (verification only).

- [ ] **Step 1: Full clean build**

Run: `npm run build`
Expected: PASS, exit 0, `astro check` reports 0 errors, no invalid-link errors from `starlight-links-validator`.

- [ ] **Step 2: End-to-end dev check**

Run: `npm run dev`
- Sidebar order (any page): Overview, Reference, Practicalities, Lecture Notes, Activities, Assignments, Recitations, About; no Studios.
- `/overview/schedule` renders all ten week sections plus Finals Week.
- Foundation authors no `<Tabs>` content, so cross-page `syncKey` persistence is not re-checked here; it was spot-checked within a page (and across a reload) in Task 1 Step 5, and cross-page persistence uses the same `localStorage` mechanism Starlight provides.
Stop the dev server.

- [ ] **Step 3: Confirm the working tree is clean and on-branch**

Run: `git status`
Expected: on `website-redesign`, nothing to commit (no stray `wd-demo.mdx`, no uncommitted changes from Tasks 1 to 3).

---

## Self-review notes (author)

- **Spec coverage:** IA/sidebar (Task 2), `<Tabs syncKey>` convention (Global Constraints + verified in Tasks 1 and 4), `<WhatDiffers>` (Task 1), glossary hub stub + linking convention (Task 2 Step 4), schedule page (Task 3), language persistence (Global Constraints, verified Task 4). Migration/redirects: intentionally deferred with rationale (see "Deferred out of Foundation").
- **Task independence:** Tasks 1, 2, and 3 have no cross-dependencies (the stubs do not link to the schedule), so they may be implemented and reviewed in any order; Task 4 is the final acceptance gate.
