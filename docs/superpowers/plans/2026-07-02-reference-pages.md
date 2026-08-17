# Reference Pages Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the six student-facing Reference pages (language ladder, archetype inventory, story bibles, C++ bridge, MATLAB bridge, field guide) from VISION, filling the stubs Foundation created.

**Architecture:** Each page is one MDX file that already exists as a stub under `src/content/docs/reference/`. Each task replaces one stub's body with real content derived faithfully from the named VISION section(s), keeping the stub's frontmatter. Reference content is the Reference genre: terse tables and lists, dual-language and target-language forms shown side by side in table columns (never `<Tabs>`).

**Tech Stack:** Astro + Starlight (MDX, `autogenerate` sidebar, `starlight-links-validator`), no unit-test runner.

## Global Constraints

- **The only automated test is `npm run build`** (runs `astro check` then `astro build`; the build runs `starlight-links-validator`). "Run the test" means run `npm run build` and confirm it exits 0 with no type errors and no invalid-link errors. Do not add a unit-test runner.
- **VISION.md is the authority.** Author each page from its named VISION section(s); do not invent course content. Read `/Users/ulbrical/GitHub/engr103/VISION.md` for the cited sections.
- **Reference genre (VISION §9):** terse tables and lists, minimal connecting prose, one consistent name per concept, plain-language rule (short sentences, common words, no idioms, define any term at first use).
- **No emdashes anywhere.** Use proper punctuation (commas, colons, parentheses).
- **Side by side, never tabs.** Where a page shows Python and Rust, or maps a course construct onto another language, both forms appear in adjacent table columns. Do NOT import or use `<Tabs>` / `<TabItem>` on any Reference page. Small code fragments may appear inside table cells; no standalone runnable program listings.
- **Keep each stub's frontmatter** (`title`, `sidebar.order`); the `description` may be refined to fit the finished page. The sidebar order is fixed: language-ladder 1, archetype-inventory 2, field-guide 3, cpp-bridge 4, matlab-bridge 5, story-bibles 6.
- **Branch:** work on `website-redesign`. NEVER switch to or commit on `main`. Commit after each task.
- **Out of scope (do not author):** the glossary body; the story-bibles season arc and recurring technical conventions; any use of `<Tabs>`; retiring the old `cpp-basics.mdx` lecture.

---

## File Structure

All six files already exist as Foundation stubs; each task rewrites one file's body and keeps its frontmatter.

- `src/content/docs/reference/language-ladder.mdx` (Task 1)
- `src/content/docs/reference/archetype-inventory.mdx` (Task 2)
- `src/content/docs/reference/story-bibles.mdx` (Task 3)
- `src/content/docs/reference/cpp-bridge.mdx` (Task 4)
- `src/content/docs/reference/matlab-bridge.mdx` (Task 5)
- `src/content/docs/reference/field-guide.mdx` (Task 6)

Ordering: Task 1 (language ladder) first, because Tasks 4, 5, and 6 map the subset it codifies. Tasks 2 and 3 are independent.

## Shared per-task verification

Every task ends with the same gate. After authoring the page:

1. Run `npm run build`. Expected: exit 0, `astro check` 0 errors, `starlight-links-validator` reports all links valid.
2. Run `grep -n "—" <file>`. Expected: no matches (no emdashes).
3. Run `grep -nE "<Tabs|<TabItem|starlight/components" <file>`. Expected: no matches (reference pages are side by side, not tabbed).
4. Confirm any required internal link resolves (the build's link validator enforces this; a green build is the proof).
5. Commit.

---

## Task 1: Language ladder

**Files:** Modify `src/content/docs/reference/language-ladder.mdx` (replace body, keep frontmatter `title: Language Ladder`, `sidebar.order: 1`).

**Source:** VISION §10 (the lecture schedule, for which constructs are taught which week) and VISION §1 (the restricted-subset rules).

**Interfaces:**
- Produces: the route `/reference/language-ladder`, linked to by Tasks 4, 5, 6.

**Required structure:**
- A short lead (2 to 3 sentences): the ladder lists the Python and Rust constructs allowed each week; it grows cumulatively; anything not yet on the ladder must not appear in take-home work; experienced students may use anything already introduced.
- One table, one row per course week (Week 1 through Week 10), with columns: **Week** | **Newly allowed in Python** | **Newly allowed in Rust** | **Still fenced off**. Cumulative: a construct listed in an earlier week stays allowed.
- A closing short list of constructs fenced off for the whole course: Python comprehensions, lambdas, and f-string format specs beyond the basics; Rust traits, lifetimes, and generics; Rust borrowing before week 7.

**Derivation skeleton (refine against VISION §10 and §1; keep cells terse):**

| Week | Python (new) | Rust (new) | Still fenced |
|---|---|---|---|
| 1 | `print`, int/float/bool/str literals, arithmetic operators and precedence, int vs float division | `println!`, `i32`/`f64`/`bool`/`&str`, arithmetic and precedence | variables, functions |
| 2 | variables and assignment, constants | `let`/`let mut`, shadowing, `const`, free functions with typed params and return, `fn` | control flow |
| 3 | comparisons, `and`/`or`/`not`, `if`/`elif`/`else` | comparisons, `&&`/`||`/`!`, `if`/`else if`/`else`, `match` | loops |
| 4 | `while`, `for` over `range` | `while`, `for` over ranges | collections |
| 5 | local scope, `list`, indexing, iteration, `append` | scope and the call stack, `Vec`, indexing, iteration, `push` | strings-as-sequences detail |
| 6 | strings as sequences, characters, slicing, searching, building | `String`/`&str` methods subset, chars, slicing, building | memory model |
| 7 | aliasing (two names, one list) | `&` references, moves, borrowing-lite | traits, lifetimes, generics (always) |
| 8 | `input`, `try`/`except`, `int()`/`float()` parsing | `read_line`, `parse`, `Result`, `?` | none new fenced |
| 9 | `dict`, `assert`, test functions | `HashMap`, `assert!`, test functions | none new fenced |
| 10 | (synthesis, no new constructs) | (synthesis, no new constructs) | none new fenced |

- [ ] **Step 1: Read the source.** Read VISION §10 (lecture schedule table) and VISION §1 (the "Restricted subsets" paragraph and the two-language rationale) in `/Users/ulbrical/GitHub/engr103/VISION.md`.
- [ ] **Step 2: Author the page.** Replace the stub body (keep frontmatter) with the lead, the per-week table (using the skeleton above, refined against the source and kept terse), and the whole-course fenced list.
- [ ] **Step 3: Run the shared verification** (build green, no emdashes, no Tabs).
- [ ] **Step 4: Commit.**

```bash
git add src/content/docs/reference/language-ladder.mdx
git commit -m "feat(reference): author the language ladder"
```

---

## Task 2: Archetype inventory

**Files:** Modify `src/content/docs/reference/archetype-inventory.mdx` (keep `title: Archetype Inventory`, `sidebar.order: 2`).

**Source:** VISION §4, "The public archetype inventory" (the numbered list of ten archetypes).

**Interfaces:**
- Produces: the route `/reference/archetype-inventory` (linked to by later tracks; no inbound links to create now).

**Required structure:**
- A short lead (2 to 3 sentences): every exam is built from these ten question types; students practice all ten in activities and assignments and meet fresh variations on exam day.
- A numbered list, exactly ten items in VISION §4's order, one terse entry each: the archetype name in bold, then one plain-language sentence for what the student does. The ten, in order: 1 Evaluate, 2 Trace, 3 Predict output, 4 Find the bug, 5 Write a function, 6 Draw memory, 7 Explain, 8 Choose tests, 9 Complete the code, 10 Concept short answer. Keep each description faithful to VISION §4.

- [ ] **Step 1: Read the source.** Read VISION §4, "The public archetype inventory".
- [ ] **Step 2: Author the page.** Replace the stub body with the lead and the ten-item numbered list.
- [ ] **Step 3: Run the shared verification.**
- [ ] **Step 4: Commit.**

```bash
git add src/content/docs/reference/archetype-inventory.mdx
git commit -m "feat(reference): author the archetype inventory"
```

---

## Task 3: Story bibles

**Files:** Modify `src/content/docs/reference/story-bibles.mdx` (keep `title: Story Bibles`, `sidebar.order: 6`).

**Source:** VISION §8, the stable orientation parts only: "Mission Ares", "Rubber Duck Robotics", the recurring characters, and the family-to-costume table. **Do not** include the week-by-week season arc (the R2 to R10 plot) or the recurring technical conventions (data formats, function names, provided helpers).

**Required structure:**
- A one-sentence lead: the course wears two story lines, one for take-home practice and one for the supervised tiers; the story is always skippable and never changes the technical task.
- `## Mission Ares (recitations and exams)`: a short paragraph. The class is the flight software crew of a crewed Mars habitat. Introduce **HAB**, the habitat AI: helpful, tireless, and confidently wrong at convenient moments, which is how find-the-bug and judge-this-code problems arrive ("HAB drafted this; sign off or reject it"). State the flight rule: no HAB code runs unverified. Do not narrate the weekly plot.
- `## Rubber Duck Robotics (assignments)`: a short paragraph. The student is the new firmware intern at a lovable, mediocre novelty-gadget company. Every assignment arrives as a ticket from **Gary**, the overcommitted senior engineer who is reliably wrong about one thing per week (his provided helper is subtly wrong; copying it uncritically costs a test).
- `## The problem families and their two faces`: reproduce VISION §8's family table with columns **Family** | **Rubber Duck face (assignments)** | **Mission Ares face (recitations, exams)** | **Core topics**, for the six families: Launch Window, Airlock, Rover, Comms, Telemetry, Manifest. (Omit the "Weeks" column to keep this page about story, not schedule.)

- [ ] **Step 1: Read the source.** Read VISION §8, sections "Mission Ares", "Rubber Duck Robotics", and the family table.
- [ ] **Step 2: Author the page** per the structure, omitting the arc and technical conventions.
- [ ] **Step 3: Run the shared verification.**
- [ ] **Step 4: Commit.**

```bash
git add src/content/docs/reference/story-bibles.mdx
git commit -m "feat(reference): author the story bibles orientation page"
```

---

## Task 4: C++ bridge

**Files:** Modify `src/content/docs/reference/cpp-bridge.mdx` (keep `title: C++ Bridge`, `sidebar.order: 4`).

**Source:** VISION §1 (the "C++ bridge" rationale) and the **language ladder from Task 1** (the Rust course subset this page maps). Author fresh; do not copy the old `cpp-basics.mdx` lecture, though a one-line pointer to running C++ is fine.

**Interfaces:**
- Consumes: `/reference/language-ladder` (Task 1) for the subset and for a link.

**Required structure:**
- A short lead: this page maps the course subset of Rust onto C++ syntax for students continuing into C++ courses; it covers only constructs on the [language ladder](/reference/language-ladder).
- One mapping table with columns **Concept** | **Rust (course subset)** | **C++**, one row per construct the ladder introduces, grouped in ladder order: values and types, variables and `mut`, functions, comparisons and boolean operators, `if`/`else if`/`match`, `while` and `for`, `Vec`/arrays, `String`, references, console input. Cells hold short code fragments, not full programs.
- A `## What will surprise you` short list: C++ has uninitialized variables and undefined behavior where Rust rejects them; manual headers and build steps; `std::vector` and `std::string` versus `Vec`/`String`; no move checker.

- [ ] **Step 1: Read the source.** Read VISION §1 (C++ bridge rationale) and the finished `src/content/docs/reference/language-ladder.mdx`.
- [ ] **Step 2: Author the page** per the structure, mapping only ladder constructs.
- [ ] **Step 3: Run the shared verification** (the link to `/reference/language-ladder` must resolve; a green build confirms it).
- [ ] **Step 4: Commit.**

```bash
git add src/content/docs/reference/cpp-bridge.mdx
git commit -m "feat(reference): author the C++ bridge"
```

---

## Task 5: MATLAB bridge

**Files:** Modify `src/content/docs/reference/matlab-bridge.mdx` (keep `title: MATLAB Bridge`, `sidebar.order: 5`).

**Source:** VISION §1 (the MATLAB bridge rationale) and the **language ladder from Task 1**.

**Interfaces:**
- Consumes: `/reference/language-ladder` (Task 1).

**Required structure:**
- A short lead: this page maps the course subset onto MATLAB for students heading into MATLAB-based major courses; it covers only constructs on the [language ladder](/reference/language-ladder).
- One mapping table with columns **Concept** | **Course subset (Python or Rust)** | **MATLAB**, grouped in ladder order (values and types, variables, functions, comparisons and logicals, `if`/`elseif`/`else`, `while` and `for`, arrays/lists, strings, input). Short fragments only.
- A `## What will surprise you` short list: MATLAB is 1-indexed; it is matrix-first; it is weakly typed; blocks close with `end`; `=` versus `==`; scripts versus functions.

- [ ] **Step 1: Read the source.** Read VISION §1 (MATLAB bridge rationale) and the finished `language-ladder.mdx`.
- [ ] **Step 2: Author the page** per the structure.
- [ ] **Step 3: Run the shared verification.**
- [ ] **Step 4: Commit.**

```bash
git add src/content/docs/reference/matlab-bridge.mdx
git commit -m "feat(reference): author the MATLAB bridge"
```

---

## Task 6: Field guide

**Files:** Modify `src/content/docs/reference/field-guide.mdx` (keep `title: Language Field Guide`, `sidebar.order: 3`).

**Source:** VISION §1 (the "language field guide" paragraph: one page per language, Fortran and R only for now) and the **language ladder from Task 1**.

**Interfaces:**
- Consumes: `/reference/language-ladder` (Task 1).

**Required structure:**
- A short lead: one-page concept maps for languages some majors use downstream; each maps the course subset construct by construct and lists what will surprise you; see the [language ladder](/reference/language-ladder) for the subset. Two languages for now: Fortran and R.
- `## Fortran`: a mapping table **Concept** | **Course subset (Python or Rust)** | **Fortran**, covering the core ladder constructs (values and types, variables, functions/subroutines, conditionals, loops, arrays, strings), then a `### What will surprise you` short list (1-indexing, fixed declarations and types, `do` loops, array-first culture).
- `## R`: a mapping table with the same columns for R, then a `### What will surprise you` short list (1-indexing, vectorized operations, `<-` assignment, everything is a vector).

- [ ] **Step 1: Read the source.** Read VISION §1 (the field guide paragraph) and the finished `language-ladder.mdx`.
- [ ] **Step 2: Author the page** with the Fortran and R sections only.
- [ ] **Step 3: Run the shared verification.**
- [ ] **Step 4: Commit.**

```bash
git add src/content/docs/reference/field-guide.mdx
git commit -m "feat(reference): author the language field guide"
```

---

## Self-review notes (author)

- **Spec coverage:** all six in-scope pages have a task (ladder 1, archetype inventory 2, story bibles 3, C++ bridge 4, MATLAB bridge 5, field guide 6). Glossary and the story arc/technical conventions are intentionally out of scope per the spec. Side-by-side (no Tabs) is enforced in Global Constraints and the shared verification (Step 3). Ladder-first ordering is honored (Tasks 4 to 6 consume Task 1).
- **No manufactured red step:** these are content pages; a stub already builds green, so there is no natural failing build to show. Verification is the green build plus the structural greps (emdashes, Tabs) and the link validator. This is intentional and correct for content authoring.
- **Execution model:** page content is authored by opus implementer subagents (user instruction); the ladder task especially benefits from the stronger model because the per-week subset must be derived and kept consistent with VISION §10.
