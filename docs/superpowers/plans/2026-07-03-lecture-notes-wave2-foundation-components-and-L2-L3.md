# Lecture-notes Wave 2: foundation components + L2/L3 split

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the two interactive/visual components the data-representation lecture needs (a binary/bits visualizer and a memory-box diagram), then split the grounded pilot into a dedicated L2 "Data representation and memory" (embedding those components) and L3 "Values, types, expressions."

**Architecture:** Two components first (an interactive Svelte binary visualizer and a static `.astro` memory-box diagram), then the two foundational lectures that use them, per the v7 schedule (`docs/superpowers/specs/2026-07-02-lecture-schedule-redesign.md`) and the `engr103-lecture-notes` skill. The current pilot `values-types-expressions.mdx` already contains the storage foundation; this wave moves that content into a new L2 and trims the pilot to L3.

**Tech Stack:** Astro + Starlight (MDX), Svelte 5 (runes) for the interactive component, Tailwind for styling; Python 3.14 and Rust 1.88 for the accuracy pass.

## Global Constraints

- **The only automated test is `npm run build`** (`astro check` then `astro build`; the build runs `starlight-links-validator`). No unit-test runner; do not add one.
- **No emdashes anywhere.** Proper punctuation only.
- **Component styling uses Tailwind utility classes** (Starlight CSS variables via arbitrary values, e.g. `text-[var(--sl-color-gray-3)]`), never `<style>` blocks, matching `RubricTable.astro` and the updated `WhatDiffers.astro`.
- **Svelte components** are Svelte 5 (runes: `$state`, `$derived`, `$props`); the implementer MUST use the `svelte-code-writer` skill (and follow `svelte-core-bestpractices`) when writing or editing any `.svelte` file. Hydrate with `client:visible` when embedded in MDX.
- **Lectures follow the `engr103-lecture-notes` skill** at `.claude/skills/engr103-lecture-notes/SKILL.md`: read it first. In particular: `<LanguageVersions />` is the first body element; every concept section ends with a `<Tabs syncKey="lang">` Python-then-Rust pair and a `<WhatDiffers>` callout that fills only the `differs` slot (there is no "what cannot differ"); show at least one commented bad-code example where it teaches; the legibility rule (no forward references; ground `i32`/`f64` before decoding the bit-widths); the authoritative accuracy pass (execute every example in both languages, including bad code); every fenced code block carries a `title` attribute (Expressive Code, for accessibility); plain-language; story-agnostic.
- **Language-ladder fence:** L2 and L3 are Week 1 lectures. Use only Week 1 ladder constructs (literals, arithmetic, `print`/`println!`); NO variables.
- **Writers:** Sonnet 5 implementer subagents for the lecture (Task 3, Task 4); the Svelte component (Task 1) also Sonnet 5 with the svelte skill; the `.astro` component (Task 2) may use a cheaper tier (transcription of provided structure).
- **Branch:** `website-redesign`, never `main`. Commit after each task.

## Out of scope (later waves)

- The dual-language memory stepper (first needed at L4; its own focused build).
- Lectures L4 through L16 and the two week-10 extras.
- The glossary body.
- Relocating `git`/`development-environment` (Activities/Practicalities tracks).

---

## File Structure

- **Create** `src/components/BinaryVisualizer.svelte` (Task 1): interactive decimal-to-binary/bits visualizer.
- **Create** `src/components/MemoryBoxes.astro` (Task 2): static memory-as-addressable-boxes diagram, configurable via a `boxes` prop.
- **Create** `src/content/docs/lectures/data-representation.mdx` (Task 3): the new L2, holding the storage foundation and embedding both components. `sidebar.order: 2`.
- **Modify** `src/content/docs/lectures/values-types-expressions.mdx` (Task 4): the pilot becomes L3; remove the storage-foundation content (now in L2), keep values/types/expressions/operators, reference L2 for sizes, set `sidebar.order: 3`.

---

## Task 1: The binary/bits visualizer (interactive)

**Files:** Create `src/components/BinaryVisualizer.svelte`

**Interfaces:**
- Produces: `<BinaryVisualizer client:visible />` (no required props), usable in MDX.

**Behavior spec** (author idiomatic Svelte 5 with the `svelte-code-writer` skill; this is a spec, not verbatim code):
- A number input where the reader types a non-negative whole number (default something like `65`).
- Shows, reactively:
  - the decimal value;
  - its binary digits, grouped into bytes of 8 bits, with leading zeros shown to fill the current width;
  - a width selector (8, 16, 32 bits) so the reader sees how a fixed width holds the value;
  - a short, plain-language note when the value does not fit the selected width ("this number needs more than N bits, so a fixed N-bit integer cannot hold it"), which is the intuition behind `i32`'s range.
- Optionally, for a value 0 to 127, show the ASCII character it maps to (ties to the lecture's ASCII point). Keep this secondary and clearly labeled.
- Styling: Tailwind utilities with Starlight CSS variables (no `<style>` block); readable in light and dark; monospace for the bits.
- Pure client-side; no network; no external libraries.

- [ ] **Step 1: Read the skills.** Read the `svelte-code-writer` skill and `svelte-core-bestpractices`, and look at `src/components/CppMemoryStepper.svelte` and `ArraysStdDevForm.svelte` for the existing Svelte 5 patterns in this repo.
- [ ] **Step 2: Author the component** at `src/components/BinaryVisualizer.svelte` per the behavior spec, Svelte 5 runes, Tailwind styling.
- [ ] **Step 3: Red/green via a throwaway page.** Create `src/content/docs/bv-demo.mdx` (frontmatter `title: BV demo`) importing and rendering `<BinaryVisualizer client:visible />`. Run `npm run build`; expect green (component type-checks and renders). Then delete `bv-demo.mdx` and run `npm run build` again; expect green.
- [ ] **Step 4: Commit** (component only; demo deleted):

```bash
git add src/components/BinaryVisualizer.svelte
git commit -m "feat(lectures): add interactive binary/bits visualizer component"
```

(The controller performs the interactive visual check in a browser during acceptance; the implementer confirms the build is green and the component compiles.)

---

## Task 2: The memory-box diagram (static)

**Files:** Create `src/components/MemoryBoxes.astro`

**Interfaces:**
- Consumes: nothing.
- Produces: `<MemoryBoxes boxes={[...]} />`. The `boxes` prop is an array of objects `{ address?: string, value?: string, name?: string }`; the component renders a horizontal row of labeled boxes (a byte/cell each), showing the address under each box, the stored value inside, and an optional variable name pointing at the box. Follows the static `.astro` pattern of `MemoryPoolDiagram.astro`.

**Structure spec:**
- Read the `boxes` prop in the frontmatter; render a flex row of boxes.
- Each box: a bordered cell (Tailwind, Starlight CSS vars) with the value centered inside and the address as small text beneath; if `name` is set, show it above the box with a small down-arrow or label indicating the name refers to that box.
- Static, no client JS, no `<style>` block.

- [ ] **Step 1: Look at the pattern.** Read `src/components/MemoryPoolDiagram.astro` and `RubricTable.astro` for the `.astro` + Tailwind pattern.
- [ ] **Step 2: Author the component** at `src/components/MemoryBoxes.astro` per the structure spec.
- [ ] **Step 3: Red/green via a throwaway page.** Create `src/content/docs/mb-demo.mdx` (frontmatter `title: MB demo`) importing `MemoryBoxes` and rendering it with a small example, for example `<MemoryBoxes boxes={[{address: "0x00", value: "65"}, {address: "0x01", value: "66"}]} />`. Run `npm run build`; expect green. Then delete `mb-demo.mdx` and run `npm run build`; expect green.
- [ ] **Step 4: Commit** (component only; demo deleted):

```bash
git add src/components/MemoryBoxes.astro
git commit -m "feat(lectures): add static memory-box diagram component"
```

---

## Task 3: L2 "Data representation and memory"

**Files:** Create `src/content/docs/lectures/data-representation.mdx`

**Interfaces:**
- Consumes: `BinaryVisualizer` (Task 1), `MemoryBoxes` (Task 2), `LanguageVersions`, `WhatDiffers`, Starlight `Tabs`/`TabItem`.
- Produces: the route `/lectures/data-representation`, `sidebar.order: 2`.

**Source:** the storage-foundation content currently in `src/content/docs/lectures/values-types-expressions.mdx` (the "How Values Are Stored" material: bits, bytes, sizes, Python arbitrary-precision `int` vs Rust fixed `i32`/`f64`, ASCII), moved here and expanded into a full lecture, plus the `engr103-lecture-notes` skill and VISION §10's L2 row.

**Required shape (per the skill):**
- Frontmatter `title: "Data Representation and Memory"`, a one-sentence `description`, `sidebar.order: 2`.
- `<LanguageVersions />` as the first body element.
- An `ai-summary` block (`type: lecture`, `slug: data-representation`, `order: 2`, `covers`, `prereq_lectures: how-programs-run`, `followup_lectures: values-types-expressions`, `glossary_terms`: bit, byte, binary, ASCII, memory address, integer size, arbitrary precision).
- An opening problem statement, then concept sections, each leading with prose and ending with a `<Tabs syncKey="lang">` pair and a `differs`-only `<WhatDiffers>`:
  - Binary and bits (embed `<BinaryVisualizer client:visible />`).
  - Bytes and memory as addressable boxes (embed `<MemoryBoxes boxes={...} />`).
  - ASCII (characters as numbers).
  - Integer sizes: Python's arbitrary-precision `int` vs Rust's fixed `i32`/`f64`; here, and only here, decode what the `32`/`64` mean. Include a commented bad-code example (a Rust integer literal out of `i32` range that will not compile).
- A **Takeaways** synthesis; optional **Resources**.
- Week 1 ladder only: NO variables.

- [ ] **Step 1: Read.** Read `.claude/skills/engr103-lecture-notes/SKILL.md`, the current `values-types-expressions.mdx` (for the storage content to move), VISION §10 L2 row, and the Week 1 ladder `src/content/docs/reference/language-ladder.mdx`.
- [ ] **Step 2: Author** `data-representation.mdx` per the shape, embedding both components.
- [ ] **Step 3: Accuracy pass.** Execute every code example (including the bad Rust one) in Python 3.14 and Rust 1.88; confirm real output and the real compile failure match the prose; record commands/outputs in the report.
- [ ] **Step 4: Verify.** `npm run build` green; `grep -n "—" <file>` empty; `grep -n 'slot="cannot"' <file>` empty; `<LanguageVersions />` first; a `<Tabs syncKey="lang">` + `<WhatDiffers>` per concept section; no variables; at least one commented bad-code example; `wc -w` in the 1,800 to 3,500 band.
- [ ] **Step 5: Commit.**

```bash
git add src/content/docs/lectures/data-representation.mdx
git commit -m "feat(lectures): author L2 Data representation and memory"
```

---

## Task 4: Re-scope L3 "Values, types, expressions"

**Files:** Modify `src/content/docs/lectures/values-types-expressions.mdx`

**Interfaces:**
- Consumes: L2 `/lectures/data-representation` (referenced in prose for the storage foundation).

**Changes:**
- Remove the storage-foundation content (bits/bytes/sizes/arbitrary-precision/ASCII) now living in L2; where L3 needs a type size, reference L2 in prose ("as covered in the Data Representation and Memory lecture, a Rust `i32` holds a fixed 32-bit range").
- Keep values and types, static vs dynamic typing, operators and precedence, integer vs float division and truncation, coercion, evaluating by hand, each as a concept section ending in `<Tabs syncKey="lang">` + `differs`-only `<WhatDiffers>`, with the existing bad-code examples retained.
- Frontmatter: keep `title: "Values, Types, and Expressions"`; set `sidebar.order: 3`.
- `ai-summary`: `slug: values-types-expressions`, `order: 3`, `prereq_lectures: data-representation`, `followup_lectures: variables-and-state`, keep `glossary_terms` for the values/types terms this lecture still owns (drop the storage terms, now owned by L2).
- Keep `<LanguageVersions />` first; no emdashes; no `slot="cannot"`; Week 1 ladder (no variables).

- [ ] **Step 1: Read** the skill and the current `values-types-expressions.mdx`; note what moved to L2 in Task 3.
- [ ] **Step 2: Edit** the file: remove the storage sections, add the prose reference to L2, set `sidebar.order: 3`, update the `ai-summary`.
- [ ] **Step 3: Accuracy pass** on the remaining examples (both languages); record in the report.
- [ ] **Step 4: Verify.** `npm run build` green (the L2 reference link, if any, resolves); `grep -n "—"` empty; `grep -n 'slot="cannot"'` empty; no variables; `<LanguageVersions />` first; word count in band.
- [ ] **Step 5: Commit.**

```bash
git add src/content/docs/lectures/values-types-expressions.mdx
git commit -m "feat(lectures): re-scope L3 Values, types, expressions (storage moved to L2)"
```

---

## Acceptance (controller)

After Task 4: run `npm run build` (green), and in the browser verify the interactive `BinaryVisualizer` works (typing a number updates the bits; the width selector shows the fit/overflow note) and the `MemoryBoxes` diagram renders in L2, in light and dark. Confirm the sidebar shows Data Representation (order 2) before Values/Types/Expressions (order 3).

## Self-review notes (author)

- **Spec coverage (this slice):** the two L2 components (schedule redesign §2, spec §1 interactive sub-track) are Tasks 1 to 2; the L2/L3 split (v7 grid) is Tasks 3 to 4. The stepper, L4 to L16, week-10 extras, and glossary are later waves, out of this plan by design.
- **Component code is spec'd, not transcribed:** the Svelte binary visualizer is a creative interactive component, so the plan gives a precise behavior/interface spec and defers idiomatic code to the implementer (with the svelte-code-writer skill), the same way content tasks are handled. The `.astro` memory-box has a concrete prop interface.
- **Model:** Sonnet 5 implementers.
