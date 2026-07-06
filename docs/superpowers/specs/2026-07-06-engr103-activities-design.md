# ENGR 103 Activities: Skill, Component, and Pilot Design

**Goal:** Establish how in-class activities are authored for ENGR 103, as a governing skill (`engr103-activities`), one small supporting component (`<Reveal>`), and a first pilot activity, so every activity works both as an instructor-led in-class session and as a self-contained exercise an absent student can complete alone.

**Status:** Approved shape (brainstorming), pending spec review before planning.

---

## 1. Context and constraints

**The reference skill (`cs312-activity`, at `~/.agents/skills/cs312-activity/`).** A Diátaxis tutorial skill: one activity paired with one lecture, a single continuous hands-on walkthrough filling an 80-minute session. The teacher bears responsibility; the student follows directions and sees results at every step; explanation is ruthlessly minimized (the lecture owns exposition). It already assumes a lone student can complete it. It uses `<Steps>`, `<ActivityQuestion>`, `<Aside>`, an `ai-summary` with `paired_lecture`, and a `draft: true` instructor-only publish gate.

**What VISION §7 requires for ENGR 103 (different from cs312).** ENGR 103 activities are the guided first altitude of each problem family, and a session is:

- **50 minutes**, not 80: a 5-minute misconception opener, **three rotating ~13-minute blocks**, a 2-minute bridge naming the recitation family fed.
- **Typed, rotating blocks**, some on paper and some in the browser: predict-then-run, same-program-two-languages, live debugging, peer instruction, paper tracing.
- **Dual-language** (Python + Rust) and **no story** (bare concept, fastest possible read; assignments and recitations carry the story lines, activities do not).
- Attempted **"solo or in pairs"** and resolved **live** by the instructor.

So an ENGR 103 activity is a set of short, mixed-type guided exercises, not one linear tutorial.

**The tension to resolve.** Activities are designed for class, but a student who misses class must be able to do them independently, so the content must stand alone. Two sub-tensions, resolved:

1. **Pairing.** VISION already says "solo or in pairs," so pairing is a *delivery mode*, not a content dependency. Every activity is authored for one individual ("you predict…", "you run…"), with **no pairing mention on the page at all**. An in-class pair and a solo at-home student read the exact same individual-facing exercise; whether to pair up is the instructor's in-the-room call, not something the page states.
2. **Live resolution.** Several block types are resolved live by the instructor ("then resolve it live"; peer-instruction voting; debugging as a class). An absent student misses that. **Resolution: hidden reveals.** Each block's resolution lives on the page behind a collapsible closed by default. In class the instructor resolves live and reveals stay shut, preserving the commit-before-you-see pedagogy; at home the student attempts, then opens the reveal to self-resolve. One artifact serves both audiences.

---

## 2. The `<Reveal>` component

A small Astro component wrapping native `<details>`/`<summary>` (zero-JS, accessible, keyboard-operable, prints in an open state).

**Props:**
- `label` (string, optional): the summary text. Default: `"Reveal the answer"`. Authors set pedagogically apt labels, e.g. `"Reveal what the class worked out"`, `"Reveal the real output"`, `"Reveal the fix"`.

**Behavior and style:**
- Closed by default. Styled with Starlight CSS variables to read as a "hold your answer" panel (a muted, bordered summary with a disclosure caret; the body indented on open). No emdashes in any built-in copy.
- Renders arbitrary MDX children (prose, code blocks, `<Tabs>`, `<WhatDiffers>`, `<Latex>`), so a resolution can itself be dual-language.
- Print fallback: because `<details>` prints closed in some engines, the component forces the body visible under `print:` so a printed activity shows resolutions.

**Everything else reuses existing components:** `<Tabs syncKey="lang">`, `<TabItem>`, `<LanguageVersions />`, `<WhatDiffers>`, Starlight `<Aside>` and `<Steps>`, and `<Latex>`.

---

## 3. Activity page structure

One page per lecture that has an in-class session, paired via `paired_lecture`.

- **Frontmatter:** `title` (short, action-oriented), `description` (one sentence), `sidebar.order` matching the paired lecture, `draft: true` while developing.
- **`ai-summary` block** (MDX comment after imports):
  - `type: activity`, `slug`, `order`, `paired_lecture` (one real lecture slug), `practices` (concepts as student actions), `prereq_activities` (comma-separated slugs or empty), `block_types` (the three chosen), `output` (what the student ends up having predicted, fixed, or traced).
- **Opening paragraph** (no heading, 2 to 3 sentences): names the paired lecture with a link, says what the student will do across the session, states what they will be able to do by the end. No concept exposition.
- **Three block sections** (`##` each, `---` between), one per chosen block type. Each: 1 to 2 sentences of prose framing the problem, the prompt (a `<Steps>` sequence, a code block, or a `<Tabs>` pair), then a `<Reveal>` holding the resolution. Authored for the individual.
- **Bridge** (final short `##`): one or two sentences naming the recitation problem family this session fed (the escalator VISION describes).

**No penultimate "submittable" section and no "Going Further"** (those are cs312 tutorial conventions; ENGR 103 activities are participation-only bare-concept sessions, not build-an-artifact tutorials).

---

## 4. The five block types

An activity picks **three**. Each is authored as *prompt, then `<Reveal>` resolution*:

- **Predict-then-run.** Show a program; the student commits to its output (on paper or in a comment). `<Reveal>` shows the real output and reconciles the gap. Best where the result surprises (precedence, integer vs float division, float equality).
- **Same program, two languages.** The same concept in Python and Rust in a `<Tabs syncKey="lang">` pair; the student names what changed and what could not. `<Reveal>` gives the answer, usually with a `<WhatDiffers>`.
- **Live debugging.** A planted-bug program. The student diagnoses using the systematic method (read the error, form a hypothesis, test). `<Reveal>` shows the diagnosis and the one-line fix.
- **Peer instruction.** A concept question with distractors mined from likely misconceptions. `<Reveal>` gives the correct answer and why each distractor is wrong.
- **Paper tracing.** Trace an exam archetype by hand (a variable table, a call stack, an expression reduction). `<Reveal>` shows the completed trace.

---

## 5. Conventions (adapted from cs312, retuned)

- **Word band:** roughly **1,200 to 2,500 words** (a 50-minute three-block session, about half a cs312 tutorial). Diagnostic, not a target.
- **Ruthless minimal explanation:** the lecture owns exposition; an activity visits concepts hands-on with at most a one-sentence context clue. No lecture repetition.
- **Tutorial voice:** second person, present tense, short active sentences. **No emdashes** (proper punctuation instead), per the global rule.
- **Ladder-respecting:** an activity uses only constructs the paired lecture's week allows.
- **Accuracy pass (required):** every code block and every claimed output, including planted bugs and their error messages, is run in `python3` 3.14 and `rustc` 1.88 and confirmed. Reveals must show real output.
- **Draft policy:** `draft: true` while developing; only the instructor flips it. Activities are not listed individually in `schedule.mdx`; they inherit their week from the paired lecture's order.
- **Validation:** `npm run build` (astro check + build + link validation) must pass.

---

## 6. Scope

- **Pilot first.** Build the `<Reveal>` component, the `engr103-activities` skill, and **one pilot activity** paired with **L3 Expressions and Operators** (predict-then-run fits the `10 / 4 + 3 * 2` precedence and integer-vs-float-division surprises; same-program-two-languages fits the Python `//` vs Rust `/` division contrast; live debugging fits a precedence bug). Get instructor sign-off on the rendered pilot.
- **Then propagate.** After sign-off, plan activities for the remaining in-class lectures (L4 through L18; L1 and L2 are the onboarding walkthroughs with no separate activity), each at the pilot's standard.

---

## 7. Deliverables

1. `src/components/Reveal.astro` (the collapsible resolution component), browser-verified.
2. `.claude/skills/engr103-activities/SKILL.md` (the governing style guide, structured like `engr103-lecture-notes`).
3. `src/content/docs/activities/expressions-and-operators.mdx` (the pilot, `draft: true`), accuracy-verified.

## 8. Non-goals

- Not building assignments or recitations (separate tracks).
- Not retiring the old C++ studios/assignments here (separate cleanup).
- Not authoring activities beyond the pilot until the pilot is signed off.
- The `<Reveal>` component is intentionally minimal (a styled `<details>`), not an interactive quiz engine.
