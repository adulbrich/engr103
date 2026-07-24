---
name: engr103-recitations
description: Use when creating or editing recitation files (MDX in src/content/docs/recitations/). Enforces the ENGR 103 recitations as private, printable, paper-answerable problem handouts in the Rubber Duck Robotics story: Gary's ticket, a warm-up, a couple of multiple-choice quick checks, two core problems mixing code-reading and short production items, and an extra-credit problem, with no distractors or trick helpers, every item gradeable from an answer key, and no extensive code writing. Always load this skill before writing or editing any recitation file.
---

# Recitation Style Guide

This skill governs how recitations are written and revised for the ENGR 103 course. Recitations are the **test set**: the in-class problem set a student works alone, on paper, having practiced the same problem family at home on the paired assignment. VISION.md section 5 is the authority for this tier; this skill operationalizes it. When this skill and VISION.md disagree, VISION.md wins and this skill is the bug.

A recitation is a **printable handout of problems, and nothing else.** It is authored as MDX so it lives beside the rest of the course, but the instructor prints it and hands it out in class; students answer on paper. The page carries the problems only: a warm-up, a couple of multiple-choice quick checks, two core problems, and one extra-credit problem. It carries no mechanics, no submission instructions, no rubric, and no worked solutions.

## Two Rules That Override Everything

1. **Recitations are private. They stay `draft: true` at all times and are never published.** They are printed by the instructor and used only in the room. If a recitation were published, students could find and pre-solve it, which defeats the point of a proctored assessment. Never flip `draft` to `false` on any graded recitation, ever. `schedule.mdx` names recitations by topic but never links to a graded recitation page. **The one exception is Recitation 1**, the ungraded onboarding session: it has nothing to pre-solve, so it may be published (`draft: false`).

2. **Recitations are answerable on paper, first and foremost, and gradeable from an answer key.** Every item's answer form is one of: a selected option, a numeric value or short printed output, a filled trace table, a short written answer of a sentence or two, or a short code fragment of a handful of lines. Never ask for a full program; extensive code writing is assignment work. Present **production items** language-neutrally: name the function and its parameters inline, describe in plain words what it must compute, and give a small table of example results, with no typed signatures and no language names. Present **code-reading items** (a trace, a predict-output, a find-the-bug, a complete-the-code) as the same short program printed twice, a ```python block then a ```rust block, one after the other so both print, and tell the student to read whichever they prefer; the question and the answer are the same for both. Do not mention any tool, autograder, submission system, or the word "paper" itself.

## What a Recitation Is

A recitation is paired with exactly one assignment and its problem family, and it assesses the **same skeleton the assignment practiced**, in fresh Rubber Duck costume. It exists to certify that the practiced work transferred: a student who did the assignment honestly should be able to work the recitation.

Recitations are **not harder than the assignment that prepares them.** The only variable a recitation adds is doing the work alone, in the room, from memory. Difficulty rises in exactly one place, the extra-credit problem. The core never ambushes: every core problem is a fresh variation of a practiced skeleton, with new values and at most one small twist, never new territory.

## The Story: Rubber Duck Robotics

Recitations wear the **Rubber Duck Robotics** story line: a mediocre but lovable novelty-gadget company (self-stirring mugs, motivational bathtub ducks, duck-race timers, the occasional smart hot tub). In the room the student is the new firmware intern, and each recitation is a **ticket from Gary**, the senior engineer: he hands the intern the gadget work for the week. The tone is comic where the assignment's Mission Ares is heroic, which lightens the test-day nerves. The company name is a planted joke that pays off when rubber-duck debugging is introduced.

Recitations carry **no distractors, no red herrings, and no trick helpers.** They are straightforward problem sheets: the intern reads each ticket and solves it. Do not plant a subtly wrong helper to catch, an unused quantity to notice, or any other misdirection. The judging-provided-code skill lives on the assignment tier (HAB); the recitation is an honest test of whether the practiced work transferred.

The story is a spine, not a cage: every problem is fully understandable with the Rubber Duck plot skipped.

## The Parts of a Handout

Author these parts, in this order, and nothing else:

1. **Gary's ticket** (no heading, 2 to 4 sentences): the Rubber Duck beat that frames the session and says, in a sentence, what the intern is working on. No mechanics.
2. **Warm-up** (`## Warm-up`): one direct, easy application of the week's family that everyone should finish. It may be a tiny production item or a short reading item, whichever the week's material makes easiest.
3. **Quick Checks** (`## Quick Checks`): one or two **multiple-choice questions** that probe the week's concepts (a predicted result, a distinction like return-versus-display or whole-number-versus-ordinary division). Each is a stem and four options labelled `(a)` to `(d)`, with exactly one correct answer and distractors drawn from real misconceptions. Do not mark the answer on the page. A quick check must never restate the rule or structure a core problem turns on; if the stem gives away a core problem's logic, move the probe to a different misconception.
4. **Core Problem 1** (`## Core Problem 1`): a fresh Rubber Duck variation of a practiced skeleton, at assignment difficulty.
5. **Core Problem 2** (`## Core Problem 2`): a second fresh variation, at assignment difficulty.
6. **Extra Credit** (`## Extra Credit`): the one harder problem, which may reach into unpracticed territory. It carries, verbatim, the disclosed label in an `<Aside>`: **"This one is beyond what you practiced. Attempting it can only help you."** Give any complex math it needs in LaTeX (never make the student derive it), the same calibration as the assignment tier.

**The two cores mix archetypes.** One core is a **code-reading problem**: a short listing (printed in both languages, see rule 2) with the archetype's question, which is a trace (fill a variable-value table), a predict-output (write exactly what it prints), a find-the-bug (name the wrong line and give the one-line fix), a complete-the-code (write the line that fills a labeled blank), or, for the memory weeks, a draw-memory item (draw names, values, and references after a marked line). The other core is a **short production problem**: write a short function or fragment, a handful of lines, against a language-neutral statement with an example table. Never make both cores production items, and never make either core a full program. Pick the reading archetype that best fits the week's material, and vary it across the term so the tier as a whole rehearses all the reading archetypes the exams draw on.

Every problem is stated straight, with no distractor or trick to catch (see the story section). There is no "how this session works" section, no rubric section, no submission section, and no oral-check text. Just Gary's ticket and the problems.

## How Each Problem Reads

Bare, and shaped by its kind.

A **production problem** is language-neutral:

- **One or two sentences of Rubber Duck context**, skippable.
- **A plain-language statement of what to write**: name the function and its parameters inline (for example, "Write `heatup_minutes(current_c, target_c, minutes_per_degree, margin_percent)`, which returns ..."), then describe exactly what it must compute, stating units and any rounding or truncation in words ("how many **whole** servings", "discarding any partial serving"). Say the word **write** explicitly; a problem must always state its deliverable, never leave it implied by an example.
- **A small example table** of calls and their results, including at least one boundary case (a zero, an exact division, a "not even one" case). The example table illustrates the contract; it is never fully worked to the point that nothing is left to produce. Write results as their plain mathematical value: `44`, not `44.0`, and `500`, not `500.0`, since the handout names no language.

A **code-reading problem** shows the code and asks the archetype's question:

- **One or two sentences of Rubber Duck context**, skippable.
- **The listing, twice**: the same short program (roughly 5 to 15 lines) as a ```python block, then as a ```rust block, both printed in full, with one sentence telling the student to read whichever language they prefer. The two versions must be genuinely the same program with the same output; verify both.
- **The question and the answer form**: "trace the loop by filling this table" (give the empty table with its column headers), "write exactly what this program prints", "one line makes this program wrong; name it and give the fix", or "write the line that goes in the labeled blank". The expected answer is short and objective.

No hints, no suggested steps, no worked solutions. State what to compute or what to answer, never how.

## Components

Recitations import only what they use, and only from this short list:

```mdx
import { Aside } from '@astrojs/starlight/components';
import Latex from '/src/components/Latex.astro';
```

- **`Aside`**: only for the extra-credit disclosed label (a `note`). Never for hints.
- **`Latex`**: for a formula the extra-credit problem (or a core problem) hands the student. Give complex math; never make the student derive it.
- **Fenced code blocks** appear only in code-reading items: the same short program as a ```python block then a ```rust block, both printed in full (never `Tabs`, which would print only one). A multiple-choice quick check may use a neutral ```text``` snippet if it genuinely needs one. Production problems carry no code blocks.

Do not import `Tabs`, `TabItem`, `LanguageVersions`, `WhatDiffers`, `RubricTable`, `Reveal`, `Steps`, a simulator, or any lecture-only or assignment-only component. A recitation has no dual-language tabs (printed twin listings instead), no version banner, no rubric, no worked solutions, no scaffolding, and no simulator.

## Frontmatter and the `ai-summary` Block

### Frontmatter

```yaml
---
title: "Short Rubber-Duck Handout Name"
description: "One sentence naming Gary's ticket and the family the session assesses."
sidebar:
  order: <the recitation number>
draft: true
---
```

The `title` is the handout's name only, with **no `Recitation N:` prefix** (for example `Gadget Firmware Math`, not `Recitation 4: Gadget Firmware Math`). The number lives in `sidebar.order`. `draft: true` is mandatory and permanent.

### The `ai-summary` block

Immediately after the imports:

````mdx
{/* ai-summary
type: recitation
slug: <filename without .mdx>
order: <sidebar.order>
family: <the problem family, by description>
paired_assignment: <the assignment slug this recitation certifies>
assesses_lectures: <comma-separated lecture slugs from week N-1>
core_archetypes: <the two cores' archetypes, comma-separated, e.g. trace, write-short-function>
new_twist: <the at-most-one small twist over the assignment skeleton, or none>
story_beat: <Gary's Rubber Duck ticket, one clause>
graded: <true, or false for the week-1 onboarding recitation>
private: instructor-only print handout, never published
*/}
````

## Difficulty and the Ladder

- **Core problems sit at assignment difficulty, never above.** If a core problem is harder than the paired assignment's problem, move that difficulty to the extra-credit problem.
- **Solvable within what has been taught.** Use only ideas the recitation's week has reached on the [language ladder](/reference/language-ladder/). A recitation before the functions lecture asks the student to complete a computation and state its printed result rather than to write a named function; check the ladder first.
- **Fresh variation, not new territory.** The core is the assignment's skeleton in new Rubber Duck costume with new values and at most one small twist. Name that twist in the `ai-summary` `new_twist` field.

## The Week-1 Onboarding Recitation

Recitation 1 is ungraded onboarding, and it is a **hands-on environment-setup walkthrough**, not the warm-up/checks/core/extra-credit shape and not a paper problem sheet: the intern creates an OSU engineering account, connects to the ENGR servers over SSH, runs a first "Hello, World!" in both languages, and adds SSH keys to skip Duo (adapted from the retired `environment-setup` studio). Because it is a guided tutorial rather than a problem handout, it is the **one recitation that may use `Steps`, `Tabs`/`TabItem`, and real code blocks**, and the language-neutral and no-mechanics rules do not apply to it. Mark `graded: false` in the `ai-summary`, and give it no core or extra-credit problems. It is also the **only recitation that may be published** (`draft: false`), since onboarding has nothing to pre-solve. Every other recitation (2 to 10) uses the full structure above, stays language-neutral, and stays `draft: true` forever.

## Style

**Bare specification voice.** Direct and imperative. State what to build, not how. Use "must" for hard requirements. No hints, ever.

**Plain language.** Write for a first-year reader who may not speak English as a first language: short sentences, common words, every term already defined in the lecture. The Rubber Duck flavor never obscures the ask.

**No emdashes.** Never use the emdash character or a double hyphen as a dash. Use a colon, a semicolon, a comma, or a period.

## The Accuracy Pass

A recitation is not done until every example result, every multiple-choice answer, and every code listing has been confirmed by actually computing or running it. Because a student may answer in either language, confirm each example holds in both Python (`python3`, 3.14) and Rust (`rustc`, 1.88), and write the result on the page as the plain shared value (no trailing `.0`, no language-specific display). For a code-reading item, run both printed listings and confirm they produce identical output (or the identical planted bug); a twin listing that diverges between the languages is a defect. Confirm each multiple-choice question has exactly one correct option, and confirm the answer key for every item (the filled trace table, the exact printed output, the buggy line and its fix) by execution. This verification lives in your work log, never on the page.

## Validation

After writing or editing a recitation, run `npm run build` (this runs `astro check` and the link validator) and fix every error. The recitation stays `draft: true`, so it is not itself built into a route; the build still compiles its MDX and catches syntax errors.

## What Recitations Must NOT Contain

- No published state: `draft: true` always, never flipped. Recitations are instructor-only print handouts.
- No mechanics: no mention of any autograder, submission system, grading tool, lab machine, or the word "paper". No "how this session works" section and no rubric section.
- No full programs: never ask the student to write more than a short function or fragment; extensive code writing lives on the assignment tier.
- No item a TA cannot grade quickly from an answer key: every answer is a selection, a value, a table, a sentence or two, or a short fragment.
- No `Tabs`, no typed signatures, no version banner: production problems are language-neutral (name the function, describe what it computes, give an example table); code-reading problems print the same listing twice, ```python then ```rust.
- No two production cores: one core reads code, one core produces a short fragment.
- No quick check that gives away a core problem's rule or structure.
- No fully worked example table on a production problem: the deliverable must remain to be produced, and the word "write" must appear in the ask.
- No scaffolding: no hints, no `Steps`, no worked sub-steps, no suggested order of attack.
- No `Reveal` and no worked solutions or marked answers on the page; a recitation is a test.
- No `Recitation N:` prefix in the title; the number lives in `sidebar.order`.
- No core problem harder than the paired assignment; extra difficulty belongs only in the extra-credit problem.
- No construct beyond the recitation's week on the language ladder.
- No Mission Ares or HAB; recitations are Rubber Duck Robotics and Gary.
- No distractors, red herrings, or trick helpers (no Gary's-wrong-helper, no unused quantities); recitations are straightforward problem sheets.
- No emdashes, and no pairing language; author for one individual intern.
