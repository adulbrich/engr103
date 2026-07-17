---
name: engr103-recitations
description: Use when creating or editing recitation files (MDX in src/content/docs/recitations/). Enforces the ENGR 103 recitations as private, printable, paper-answerable problem handouts in the Rubber Duck Robotics story: Gary's ticket, a warm-up, a couple of multiple-choice quick checks, two core problems, and an extra-credit problem, with no distractors or trick helpers. Always load this skill before writing or editing any recitation file.
---

# Recitation Style Guide

This skill governs how recitations are written and revised for the ENGR 103 course. Recitations are the **test set**: the in-class problem set a student works alone, on paper, having practiced the same problem family at home on the paired assignment. VISION.md section 5 is the authority for this tier; this skill operationalizes it. When this skill and VISION.md disagree, VISION.md wins and this skill is the bug.

A recitation is a **printable handout of problems, and nothing else.** It is authored as MDX so it lives beside the rest of the course, but the instructor prints it and hands it out in class; students answer on paper. The page carries the problems only: a warm-up, a couple of multiple-choice quick checks, two core problems, and one extra-credit problem. It carries no mechanics, no submission instructions, no rubric, and no worked solutions.

## Two Rules That Override Everything

1. **Recitations are private. They stay `draft: true` at all times and are never published.** They are printed by the instructor and used only in the room. If a recitation were published, students could find and pre-solve it, which defeats the point of a proctored assessment. Never flip `draft` to `false` on any graded recitation, ever. `schedule.mdx` names recitations by topic but never links to a graded recitation page. **The one exception is Recitation 1**, the ungraded onboarding session: it has nothing to pre-solve, so it may be published (`draft: false`).

2. **Recitations are answerable on paper, first and foremost.** Present every problem language-neutrally: name the function and its parameters, describe in plain words what it must compute, and give a small table of example results. Do **not** write dual-language Python and Rust code blocks, do not show function signatures with types, and do not mention any language, tool, autograder, submission system, or the word "paper" itself. A student reads the problem and writes their answer by hand, in whatever form the room calls for.

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
2. **Warm-up** (`## Warm-up`): one direct, easy application of the week's family that everyone should finish. One function.
3. **Quick Checks** (`## Quick Checks`): one or two **multiple-choice questions** that probe the week's concepts (a predicted result, a distinction like return-versus-display or whole-number-versus-ordinary division). Each is a stem and four options labelled `(a)` to `(d)`, with exactly one correct answer and distractors drawn from real misconceptions. Do not mark the answer on the page.
4. **Core Problem 1** (`## Core Problem 1`): a fresh Rubber Duck variation of a practiced skeleton, at assignment difficulty.
5. **Core Problem 2** (`## Core Problem 2`): a second fresh variation, at assignment difficulty.
6. **Extra Credit** (`## Extra Credit`): the one harder problem, which may reach into unpracticed territory. It carries, verbatim, the disclosed label in an `<Aside>`: **"This one is beyond what you practiced. Attempting it can only help you."** Give any complex math it needs in LaTeX (never make the student derive it), the same calibration as the assignment tier.

Every problem is stated straight, with no distractor or trick to catch (see the story section). There is no "how this session works" section, no rubric section, no submission section, and no oral-check text. Just Gary's ticket and the problems.

## How Each Problem Reads

Bare and language-neutral. Each problem is:

- **One or two sentences of Rubber Duck context**, skippable.
- **A plain-language statement of what to write**: name the function and its parameters inline (for example, "Write `heatup_minutes(current_c, target_c, minutes_per_degree, margin_percent)`, which returns ..."), then describe exactly what it must compute, stating units and any rounding or truncation in words ("how many **whole** servings", "discarding any partial serving").
- **A small example table** of calls and their results, including at least one boundary case (a zero, an exact division, a "not even one" case). Write results as their plain mathematical value: `44`, not `44.0`, and `500`, not `500.0`, since the handout names no language.

No hints, no suggested steps, no worked solutions. State what to compute, never how.

## Components

Recitations import only what they use, and only from this short list:

```mdx
import { Aside } from '@astrojs/starlight/components';
import Latex from '/src/components/Latex.astro';
```

- **`Aside`**: only for the extra-credit disclosed label (a `note`). Never for hints.
- **`Latex`**: for a formula the extra-credit problem (or a core problem) hands the student. Give complex math; never make the student derive it.
- A plain fenced code block (use a neutral ```text``` block, not a language) only if a multiple-choice question genuinely needs a small snippet, kept language-neutral. Most recitations need no code blocks at all.

Do not import `Tabs`, `TabItem`, `LanguageVersions`, `WhatDiffers`, `RubricTable`, `Reveal`, `Steps`, a simulator, or any lecture-only or assignment-only component. A recitation has no dual-language tabs, no version banner, no rubric, no worked solutions, no scaffolding, and no simulator.

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

A recitation is not done until every example result and every multiple-choice answer has been confirmed by actually computing it. Because a student may answer in either language, confirm each example holds in both Python (`python3`, 3.14) and Rust (`rustc`, 1.88), and write the result on the page as the plain shared value (no trailing `.0`, no language-specific display). Confirm each multiple-choice question has exactly one correct option. This verification lives in your work log, never on the page.

## Validation

After writing or editing a recitation, run `npm run build` (this runs `astro check` and the link validator) and fix every error. The recitation stays `draft: true`, so it is not itself built into a route; the build still compiles its MDX and catches syntax errors.

## What Recitations Must NOT Contain

- No published state: `draft: true` always, never flipped. Recitations are instructor-only print handouts.
- No mechanics: no mention of any autograder, submission system, grading tool, lab machine, or the word "paper". No "how this session works" section and no rubric section.
- No language names and no dual-language code: present every problem language-neutrally, by naming the function and describing what it computes, with an example table. No `Tabs`, no typed signatures, no version banner.
- No scaffolding: no hints, no `Steps`, no worked sub-steps, no suggested order of attack.
- No `Reveal` and no worked solutions or marked answers on the page; a recitation is a test.
- No `Recitation N:` prefix in the title; the number lives in `sidebar.order`.
- No core problem harder than the paired assignment; extra difficulty belongs only in the extra-credit problem.
- No construct beyond the recitation's week on the language ladder.
- No Mission Ares or HAB; recitations are Rubber Duck Robotics and Gary.
- No distractors, red herrings, or trick helpers (no Gary's-wrong-helper, no unused quantities); recitations are straightforward problem sheets.
- No emdashes, and no pairing language; author for one individual intern.
