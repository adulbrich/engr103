# ENGR 103 Activities Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ship the Activities track's foundation: a `<Reveal>` component, the `engr103-activities` authoring skill, and one pilot activity paired with L3, so activities work both in class (instructor resolves live) and for an absent student alone.

**Architecture:** A small Astro `<Reveal>` component (styled native `<details>`, closed by default) holds each activity block's resolution. A governing skill (`engr103-activities/SKILL.md`, modeled on the existing `engr103-lecture-notes` skill) defines the VISION §7 activity shape. A pilot MDX activity paired with the Expressions and Operators lecture proves the shape end to end. Verification is `npm run build` (astro check + link validation), browser checks for interactivity, and a code-accuracy pass in `python3`/`rustc`.

**Tech Stack:** Astro 7 / Starlight 0.41, MDX content collections, Tailwind 4 with Starlight CSS variables, `<Tabs syncKey="lang">`, native `<details>`. Verification: `npm run build`, Playwright browser check, `python3` 3.14, `rustc` 1.88.

## Global Constraints

- Work on branch `website-redesign`. NEVER commit on `main`.
- No emdashes anywhere (neither `—` nor `&mdash;` nor `--` used as a dash). Use proper punctuation.
- The design source of truth is `docs/superpowers/specs/2026-07-06-engr103-activities-design.md`.
- Components use Tailwind utility classes with Starlight CSS variables (e.g. `border-[var(--sl-color-gray-5)]`), NOT a `<style>` block. Readable in light and dark.
- Activities are authored for one individual ("you..."). **No pairing mention on any activity page.**
- Activities carry **no story** (bare concept). Dual-language via `<Tabs syncKey="lang">`.
- `<LanguageVersions />` is the first body element of an activity page (same convention as lectures).
- New activity files are `draft: true`. NEVER flip `draft` to false or remove the key; only the instructor publishes.
- Accuracy pass: every code block and every claimed output, including planted bugs and their error messages, is run in `python3` (3.14) and `rustc` (1.88) and confirmed before use.
- The only automated test is `npm run build`; it must pass (green, links valid) before a task is complete.

---

## File Structure

- `src/components/Reveal.astro`: the collapsible resolution panel. One responsibility: hide a block's resolution until opened. Consumed by activity pages.
- `.claude/skills/engr103-activities/SKILL.md`: the authoring style guide for activities. One responsibility: define how an activity is written. References `<Reveal>` and existing components.
- `src/content/docs/activities/expressions-and-operators.mdx`: the pilot activity (draft). One responsibility: a complete L3 activity that exercises the skill and the component.

---

### Task 1: The `<Reveal>` component

**Files:**
- Create: `src/components/Reveal.astro`
- Temp (create then delete): `src/content/docs/reveal-demo.mdx`

**Interfaces:**
- Produces: `Reveal.astro`, an Astro component with one prop `label` (string, optional, default `"Reveal the answer"`). Renders its MDX children (slot) inside a `<details>` that is closed by default. Imported by activity pages as `import Reveal from '/src/components/Reveal.astro';` and used as `<Reveal label="Reveal the real output"> ...children... </Reveal>`.

- [ ] **Step 1: Write the component**

Create `src/components/Reveal.astro` with exactly this content:

```astro
---
// A collapsible "hold your answer" panel for activities. In class the
// instructor resolves the block live and this stays closed; a student working
// alone opens it to self-check. Wraps native <details> for zero-JS
// accessibility and keyboard operation. The body is forced visible when
// printing so a printed activity still shows its resolutions.
const { label = 'Reveal the answer' } = Astro.props;
---

<details class="group my-4 rounded-md border border-[var(--sl-color-gray-5)] bg-[var(--sl-color-bg)]">
  <summary
    class="flex cursor-pointer list-none items-center gap-2 rounded-md px-3 py-2 text-sm font-semibold text-[var(--sl-color-white)] marker:hidden"
  >
    <span class="inline-block transition-transform group-open:rotate-90" aria-hidden="true">&rsaquo;</span>
    <span>{label}</span>
  </summary>
  <div class="border-t border-[var(--sl-color-gray-5)] px-3 py-2 print:!block">
    <slot />
  </div>
</details>
```

- [ ] **Step 2: Write a demo page to verify it renders and toggles**

Create `src/content/docs/reveal-demo.mdx`:

````mdx
---
title: Reveal demo
---

import Reveal from '/src/components/Reveal.astro';

Predict the output of `print(10 / 4 + 3 * 2)`, then check.

<Reveal label="Reveal the real output">
Python prints `8.5`. Precedence does `3 * 2` first (6), then `10 / 4` is `2.5`, then `2.5 + 6`.

```python title="in the interpreter"
print(10 / 4 + 3 * 2)
```
</Reveal>
````

- [ ] **Step 3: Build with the demo (must be green)**

Run: `npm run build`
Expected: build completes, links valid. If the slot's code block or text fails to render, fix the component (e.g. ensure children are not stripped) and rebuild.

- [ ] **Step 4: Browser-verify the component**

Start the dev server (`npm run dev`), open `http://localhost:4321/reveal-demo`, and confirm with a screenshot and/or DOM check:
- The panel is **closed** on load (the code block inside is not visible).
- Clicking the summary **opens** it and reveals the code block; the chevron rotates.
- It is legible in dark mode (the default), the border and text use the Starlight variables.
- Keyboard: focusing the summary and pressing Enter toggles it.

Record the result. Stop the dev server when done.

- [ ] **Step 5: Delete the demo and rebuild**

Delete `src/content/docs/reveal-demo.mdx`. Run `npm run build`. Expected: green (one fewer page).

- [ ] **Step 6: Commit (component only)**

```bash
git add src/components/Reveal.astro
git commit -m "feat(activities): add Reveal component for hidden activity resolutions"
```

---

### Task 2: The `engr103-activities` skill

**Files:**
- Create: `.claude/skills/engr103-activities/SKILL.md`
- Read first: `.claude/skills/engr103-lecture-notes/SKILL.md` (structural model), `~/.agents/skills/cs312-activity/SKILL.md` (reference), `docs/superpowers/specs/2026-07-06-engr103-activities-design.md` (source of truth).

**Interfaces:**
- Consumes: the `<Reveal>` component from Task 1 (its `label` prop and children behavior) when documenting activity structure.
- Produces: a skill file that governs activity authoring. No code interface; other tasks (the pilot, and future activities) follow it.

- [ ] **Step 1: Write the skill file**

Create `.claude/skills/engr103-activities/SKILL.md` with YAML frontmatter (`name: engr103-activities`; `description:` naming when to load it, matching the lecture skill's style: "Use when creating or editing activity files (MDX in src/content/docs/activities/)..."). The body must cover, in clearly headed sections, all of the following, drawn verbatim in intent from the design spec:

1. **What an activity is.** The guided first altitude of a problem family; a 50-minute in-class session shaped as VISION §7 prescribes: a misconception opener, **three rotating ~13-minute blocks**, a bridge naming the recitation family fed. Dual-language, no story (bare concept), authored for one individual. Every activity is paired with exactly one lecture.
2. **Works in class and alone.** In class the instructor resolves each block live with reveals closed; an absent student opens the `<Reveal>` to self-check. State plainly: no pairing is ever mentioned on the page.
3. **The `<Reveal>` component.** How to import (`import Reveal from '/src/components/Reveal.astro';`) and use it: each block is *prompt, then a `<Reveal>` holding the resolution*, closed by default, with a pedagogically apt `label`. Reveals hold the real output, the fix, or the answer plus a one-line why.
4. **Reused components.** `<Tabs syncKey="lang">`/`<TabItem>`, `<LanguageVersions />` (first body element), `<WhatDiffers>`, Starlight `<Aside>` and `<Steps>`, `<Latex>` for math. Do NOT import lecture-only or assignment-only components.
5. **The five block types**, each authored as prompt then `<Reveal>`: predict-then-run, same-program-two-languages, live debugging, peer instruction, paper tracing. An activity picks three. Give a one-paragraph description of each with what its reveal contains.
6. **Frontmatter and `ai-summary`.** `title`, `description`, `sidebar.order` (matching the paired lecture), `draft: true`. The `ai-summary` MDX comment carries: `type: activity`, `slug`, `order`, `paired_lecture` (one real lecture slug), `practices`, `prereq_activities`, `block_types` (the three chosen), `output`.
7. **Page structure.** Opening paragraph (2 to 3 sentences, links the paired lecture, says what the student will do and end up able to do; no concept exposition). Then three `##` block sections separated by `---`, each opening with 1 to 2 sentences of framing, then the prompt, then the `<Reveal>`. Then a final short `##` bridge naming the recitation family. NO "submittable" section and NO "Going Further" (those are cs312 conventions that do not apply here).
8. **Draft policy.** `draft: true` while developing; only the instructor removes it; never link to a draft activity as a source of truth. Activities are not listed in `schedule.mdx`; they inherit their week from the paired lecture's order.
9. **Style.** Ruthless minimal explanation (the lecture owns exposition; at most a one-sentence context clue; no lecture repetition). Tutorial voice: second person, present tense, short active sentences. No emdashes. Ladder-respecting: use only constructs the paired lecture's week allows.
10. **Word band.** Roughly 1,200 to 2,500 words; a diagnostic, not a target.
11. **Accuracy pass.** Every code block and claimed output, including planted bugs and their exact error messages, is run in `python3` (3.14) and `rustc` (1.88) and confirmed. Reveals must show real output.
12. **Validation.** `npm run build` must pass.

- [ ] **Step 2: Self-check the skill for accuracy and consistency**

Verify by inspection: every component the skill names exists (`grep -l` in `src/components/` for `Reveal.astro`, `LanguageVersions.astro`, `WhatDiffers.astro`, `Latex.astro`); the `ai-summary` fields match what the pilot (Task 3) will use; there are no emdashes (`grep -n "—" .claude/skills/engr103-activities/SKILL.md` is empty); the described page structure has no "submittable"/"Going Further" leftovers from cs312.

- [ ] **Step 3: Commit (skill only)**

```bash
git add .claude/skills/engr103-activities/SKILL.md
git commit -m "docs(activities): add engr103-activities authoring skill"
```

---

### Task 3: The pilot activity (Expressions and Operators)

**Files:**
- Create: `src/content/docs/activities/expressions-and-operators.mdx`
- Read first: `.claude/skills/engr103-activities/SKILL.md` (Task 2), `src/content/docs/lectures/expressions-and-operators.mdx` (the paired lecture, for scope and ladder), `docs/superpowers/specs/2026-07-06-engr103-activities-design.md`.

**Interfaces:**
- Consumes: the `<Reveal>` component (Task 1) and the `engr103-activities` skill (Task 2). `paired_lecture: expressions-and-operators`.

- [ ] **Step 1: Verify the code examples and their outputs first**

Before writing, run each intended example and record exact output. Use these three blocks (all within L3's ladder: literals, arithmetic operators, precedence, integer vs float division, printing; no variables of the student's own beyond what L3 uses):

- **Predict-then-run:** `print(10 / 4 + 3 * 2)` and `print(2 + 3 * 2)`.
  Verify (Python 3.14): `python3 -c "print(10 / 4 + 3 * 2); print(2 + 3 * 2)"` → expect `8.5` then `8`.
- **Same program, two languages:** the same `7 / 2` in both.
  Verify Python: `python3 -c "print(7 / 2); print(7 // 2)"` → expect `3.5` then `3`.
  Verify Rust: a `fn main` printing `7 / 2` for integers → expect `3` (truncation); and `7.0 / 2.0` → `3.5`. Run via `rustc`.
- **Live debugging:** an average computed with a precedence bug, `first + second / 2` instead of `(first + second) / 2`, for `first = 8`, `second = 4` (use literals per the ladder, e.g. `print(8 + 4 / 2)` expecting the student to want the average 6 but getting 10).
  Verify Python: `python3 -c "print(8 + 4 / 2)"` → expect `10.0`; `python3 -c "print((8 + 4) / 2)"` → expect `6.0`.

Record every command and its real output. Do NOT claim any output you did not run.

- [ ] **Step 2: Write the pilot activity**

Create `src/content/docs/activities/expressions-and-operators.mdx` following the `engr103-activities` skill exactly:
- Frontmatter: `title: "Predicting and Fixing Expressions"` (or similar action-oriented title), one-sentence `description`, `sidebar.order: 3`, `draft: true`.
- `ai-summary` with `paired_lecture: expressions-and-operators`, `order: 3`, `block_types: predict-then-run; same-program-two-languages; live-debugging`, `practices`, `prereq_activities:` (empty), `output`.
- `<LanguageVersions />` as the first body element.
- Opening paragraph linking `[Expressions and Operators](/lectures/expressions-and-operators/)`.
- Three `##` blocks separated by `---`, each with framing prose, a prompt, and a `<Reveal>` holding the verified resolution (real output / the fix / the why). The two-language block uses `<Tabs syncKey="lang">` and may use `<WhatDiffers>` inside the reveal. The debugging block uses the systematic method in its reveal.
- A final `## Bridge` naming the recitation family this feeds (expressions and types, the Launch Window arithmetic family per VISION).
- No pairing mention. No emdashes. Word count roughly 1,200 to 2,500.

- [ ] **Step 3: Build (must be green)**

Run: `npm run build`
Expected: green, links valid, new route `/activities/expressions-and-operators`. Fix any MDX or link errors.

- [ ] **Step 4: Verify accuracy and mechanics**

Confirm: `grep -c "—" <file>` is 0; `<LanguageVersions />` is the first body element; every code block has a title; the outputs shown match Step 1's recorded runs exactly; no pairing words (`grep -niE "pair|partner|neighbor" <file>` is empty); `draft: true` present.

- [ ] **Step 5: Browser-verify the reveals on the activity page**

Start dev, open `/activities/expressions-and-operators`, confirm each `<Reveal>` is closed on load and opens to show the correct resolution (including the `<Tabs>` inside the two-language reveal). Screenshot one open reveal. Stop dev.

- [ ] **Step 6: Commit (pilot only)**

```bash
git add src/content/docs/activities/expressions-and-operators.mdx
git commit -m "feat(activities): add pilot activity for Expressions and Operators (draft)"
```

---

## Self-Review

**Spec coverage:**
- `<Reveal>` component (spec §2) → Task 1.
- `engr103-activities` skill (spec §7 deliverable 2, conventions §5, structure §3, block types §4) → Task 2.
- Pilot activity paired with L3 (spec §6, §7 deliverable 3) → Task 3.
- Hidden-reveals independence model (spec §1.2) → Tasks 1 and 3 (Reveal closed by default; instructor resolves live, student opens).
- No pairing on page (spec §1.2, §3) → Global Constraints + Task 3 Step 4 grep.
- Accuracy pass (spec §5) → Task 3 Steps 1 and 4.
- Draft policy (spec §5) → Global Constraints + Task 3 frontmatter.
- Propagation to L4 to L18 is explicitly out of scope (spec §6, §8) → not planned here; follows sign-off.

**Placeholder scan:** No TBD/TODO. The component code is given in full; the skill's required sections are enumerated; the pilot's three blocks and their verified outputs are specified with exact commands.

**Type consistency:** The component prop `label` and import path `/src/components/Reveal.astro` are identical across Tasks 1, 2, and 3. The `ai-summary` field set (including `block_types`) is identical in Task 2's spec and Task 3's usage. `paired_lecture: expressions-and-operators` matches the real lecture slug.

**Note on verification model:** this is a content/component site with no unit-test harness, so tasks verify via `npm run build`, browser checks, and the `python3`/`rustc` accuracy pass rather than pytest-style tests. That is intentional and matches the lecture track's established process.
