---
name: engr103-recitations
description: Use when creating or editing recitation files (MDX in src/content/docs/recitations/). Enforces the in-lab, dual-language, Rubber Duck Robotics structure of the ENGR 103 recitations: Gary's ticket, the warm-up plus two core problems plus stretch, bare requirements, and Gary's subtly wrong helper. Always load this skill before writing or editing any recitation file.
---

# Recitation Style Guide

This skill governs how recitations are written and revised for the ENGR 103 course website (Astro/Starlight, MDX format). Recitations are the **test set**: the proctored, in-lab performance where a student demonstrates, alone and without AI, that they can do the work they practiced at home on the paired assignment. VISION.md section 5 is the authority for this tier; this skill operationalizes it. When this skill and VISION.md disagree, VISION.md wins and this skill is the bug.

A recitation is a **bare specification**. It states what to build, the contract each piece must satisfy, and the cases it will be checked against, and it stops there. It carries none of the assignment tier's scaffolding: no hints, no worked sub-steps, no suggested order of attack, and none of the assignment's test-first steps. The student reads the requirements and writes the code, in the room, in 110 minutes.

## What a Recitation Is

A recitation is paired with exactly one assignment and its problem family, and it assesses the **same skeleton the assignment practiced**, in fresh costume. Under the course's one-week lag, recitation N assesses the two lectures of week N-1, which the student has had a weekend and the paired assignment to practice. The recitation exists to certify that the practiced work transferred: a student who did the assignment honestly should walk in expecting to succeed.

Recitations are **not harder than the assignment that prepares them.** The only variable a recitation adds is the conditions: locked down, alone, docs only, time-boxed. Difficulty rises in exactly one disclosed place, the stretch problem, and only as extra credit. The graded core never ambushes: every core problem is a fresh variation of a practiced skeleton, with new values and at most one small structural twist, never new territory.

Like the assignment, a recitation is **dual-language**: every problem is solvable in Python or Rust against one shared contract, and the student chooses per problem. The starter repository carries twin scaffolds and identical test cases; students submit to Gradescope from their seats, continuously, exactly as they do for assignments.

## The Story: Rubber Duck Robotics

Recitations wear the **Rubber Duck Robotics** story line: a mediocre but lovable novelty-gadget company (self-stirring mugs, motivational bathtub ducks, duck-race timers, the occasional smart hot tub). In the room the student is the new firmware intern, and each recitation arrives as a **ticket from Gary**, the senior engineer, who is enthusiastic, overcommitted, and reliably wrong about exactly one thing. The tone is comic where the assignment's Mission Ares is heroic, and that is deliberate: the comedy lightens the test anxiety a proctored room otherwise breeds. The company name is a planted joke that pays off when rubber-duck debugging is introduced.

The recurring beat is **Gary's subtly wrong helper**: on one core problem, Gary hands the intern a small provided function he wrote, and it looks convenient but has a quiet defect. This is the recitation's rung of the red herring ladder and its find-the-bug archetype (O8). Unlike the assignment tier, where HAB states a wrong claim in prose, the recitation keeps the wrong helper as **code the student must verify before use**, because verifying provided code under the documentation-only conditions is exactly the skill this tier certifies. Gary's helper is a **separate small utility**, never the whole solution: the student can ignore it and write their own, and a correct solution never depends on noticing the defect. Copying it uncritically costs a visible test.

The story is a spine, not a cage: every requirement is numbered, testable, and fully understandable with the Rubber Duck plot skipped.

## Structure of a Session

Every recitation page authors these parts, in this order:

1. **Warm-up (25% of points).** One direct application of the current problem family, at assignment difficulty, that everyone should finish. One function (or one short program before the functions lecture).
2. **Core problem 1 (part of the 75%).** A fresh Rubber Duck variation of a practiced skeleton, at assignment difficulty.
3. **Core problem 2 (part of the 75%).** A second fresh variation, at assignment difficulty. **Exactly one core problem carries Gary's subtly wrong helper.**
4. **Stretch problem (extra credit, up to +10).** The one place difficulty genuinely rises; it may test composure in unpracticed territory. It carries, verbatim, the disclosed label: **"This is beyond what you practiced. Attempting it can only help you."** Give complex math it needs in LaTeX (never make the student derive it), the same calibration as the assignment tier.

Warm-up plus the two core problems is the full 100%; a prepared student earns a perfect session without touching the stretch. The **oral check** is an in-room event run by the TA, not authored on the page; do not write it into the recitation.

## Genre: Requirements, Bare

Recitations and assignments share one spec template; the recitation strips the scaffolding. Each problem is written with this structure, in this fixed order, and nothing more:

- **Context** (one or two sentences): the Rubber Duck ticket beat for this problem, skippable.
- **Requirements** (numbered, testable statements): exactly what each function must compute, in plain language. State units and rounding or truncation explicitly.
- **Interface** (a `<Tabs syncKey="lang">` pair, functions era): the exact function signature in Python and Rust, with a title on each block. Before the functions lecture, state the exact printed-output format instead.
- **Acceptance tests** (a short table): visible cases as call-to-value pairs (or input-to-printed-output before functions), including at least one boundary case. Hidden cases run at grading time.
- **Assumptions and edge cases**: what the input is guaranteed to be.

There is **no scaffolding section**, no hint `<Aside>`, no `<Steps>`, no `<Reveal>`, and no test-first steps. The only `<Aside>` a recitation uses is the one presenting Gary's wrong helper (a `danger`/`caution` callout naming it as Gary's and asking the intern to sign off or reject), and the callout carries no verdict, no defect name, and no fix. The stretch's disclosed label is a plain `<Aside>` too.

## Reused Components

Recitations import only what they use, and only from this list:

```mdx
import { Aside, Tabs, TabItem } from '@astrojs/starlight/components';
import LanguageVersions from '/src/components/LanguageVersions.astro';
import WhatDiffers from '/src/components/WhatDiffers.astro';
import Latex from '/src/components/Latex.astro';
import RubricTable from '/src/components/RubricTable.astro';
```

- **`Tabs` / `TabItem`**: the dual-language pair, always `syncKey="lang"`, Python first. Use it for every interface signature and for Gary's helper (shown wrong in both languages).
- **`LanguageVersions`**: at the very top of the body.
- **`WhatDiffers`**: when a shown snippet differs between the languages beyond the standard `fn main`/`println!` boilerplate (the `f64` print display, integer-division split, `True`/`true`).
- **`Latex`**: for a formula the stretch (or a core problem) hands the student; give complex math, never make the student derive it.
- **`Aside`**: only for Gary's wrong-helper callout and the stretch's disclosed label. Never for hints.
- **`RubricTable`**: required at the end, in a `## Rubric` section, loading the recitation's Canvas rubric TSV.

Do not import `Reveal`, `Steps`, a simulator, or any lecture-only or assignment-only component. A recitation has no worked solutions on the page, no scaffolding, and no simulator.

## Frontmatter and the `ai-summary` Block

### Frontmatter

```yaml
---
title: "Recitation N: Short Rubber-Duck Ticket Title"
description: "One sentence naming the ticket and the family the session assesses."
sidebar:
  order: <the recitation number>
draft: true
---
```

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
story_beat: <Gary's Rubber Duck Robotics ticket, one clause>
graded: <true, or false for the week-1 onboarding recitation>
*/}
````

## Page Structure

1. **`<LanguageVersions />`** at the top of the body.
2. **Gary's ticket** (no heading, 2 to 4 sentences): the Rubber Duck Robotics beat that frames the session and names, in one sentence, what the intern is building today. No requirements, no concept teaching.
3. **How this session works** (a short `##` section): one or two sentences naming the parts (warm-up, two core, stretch), the per-problem language choice, that the grader calls the functions directly, and that submission is continuous to Gradescope. Link [Running and Submitting Your Code](/practicalities/starter-repo-and-check/).
4. **Warm-up** (`## Warm-up`): one problem, the bare template.
5. **Core Problem 1** (`## Core Problem 1`): the bare template.
6. **Core Problem 2** (`## Core Problem 2`): the bare template, carrying Gary's wrong helper.
7. **Stretch** (`## Stretch Problem (extra credit)`): the bare template plus the disclosed label; complex math in LaTeX.
8. **Rubric** (`## Rubric`, last): `<RubricTable src="canvas/recitations/<slug>-rubrics.tsv" caption="..." />`.

## The Rubric

Every recitation ends with a `## Rubric` section rendering `<RubricTable src="canvas/recitations/<slug>-rubrics.tsv" caption="..." />`, and you author the TSV alongside the page. It follows VISION's recitation weighting: **provided (visible) tests 50, hidden tests 30, style and decomposition 20**, summing to 100, plus the stretch as a separate extra-credit row (up to 15). The style share is scored by the course linters (ruff, clippy) with TAs spot-checking only what linters cannot see. The TSV is headerless Canvas format, the same shape as the assignment rubric TSVs in `canvas/assignments/`; put recitation TSVs in `canvas/recitations/`. Within the 100, the warm-up carries 25 and the two core problems 75, split across the visible/hidden/style criteria as fits the session.

## Dual-Language Rule

Every problem works identically in both languages against one shared contract. Show each signature in both, Python first. Where a shown snippet (an acceptance-test demo, Gary's wrong helper) differs beyond the standard boilerplate, add a `<WhatDiffers>` naming the real difference. Python signatures carry no type annotations; Rust signatures carry explicit parameter types and an explicit `-> ReturnType` with an explicit `return`.

## Difficulty and the Ladder

- **Core problems sit at assignment difficulty, never above.** If a core problem is harder than the paired assignment's problem, it is wrong; move that difficulty to the stretch.
- **Solvable within what has been taught.** Use only constructs the recitation's week has reached on the [language ladder](/reference/language-ladder/). A recitation before the functions lecture cannot ask for student-written functions; check the ladder first.
- **Fresh variation, not new territory.** The core is the assignment's skeleton in new Rubber Duck costume with new values and at most one small twist (an extra edge case, one more parameter). Name that twist in the `ai-summary` `new_twist` field.

## The Week-1 Onboarding Recitation

Recitation 1 is ungraded onboarding (environment, editor, terminal, git, a "hello" run in both languages), not the warm-up/core/stretch shape. Author it as a short guided session like the `systems-checkout` assignment, mark `graded: false` in the `ai-summary`, and give it no stretch and no Gary's-helper rung. Every other recitation (2 to 10) uses the full structure above.

## Draft Policy

Set `draft: true` while a recitation is in development. **Never flip it to `false` yourself and never remove the key**; only the instructor publishes. A draft recitation is not a source of truth, and `schedule.mdx` must not link to it while it is a draft.

## Style

**Bare specification voice.** Direct and imperative. State what to build, not how. Use "must" for hard requirements. No hints, ever: if a sentence starts to help the student decide *how*, cut it.

**Plain language.** Write for a first-year reader who may not speak English as a first language: short sentences, common words, every technical term already defined in the lecture. The Rubber Duck flavor never obscures the ask.

**No emdashes.** Never use the emdash character or a double hyphen as a dash. Use a colon, a semicolon, a comma, or a period.

## The Accuracy Pass

A recitation is not done until every function and every claimed value on the page has been run and confirmed in Python (`python3`, 3.14) and Rust (`rustc`, 1.88). Run **Gary's wrong helper** too, and confirm it really produces the wrong result on a visible case, and that a correct version passes. Confirm each language's exact display, including the `f64` versus Python-float difference. Fix any mismatch before shipping.

## Validation

After writing or editing a recitation, run `npm run build` (this runs `astro check` and the link validator) and fix every error.

## What Recitations Must NOT Contain

- No scaffolding: no hints, no `<Steps>`, no worked sub-steps, no suggested order of attack, no test-first steps. That support belongs to the assignment tier.
- No `<Reveal>` and no worked solutions on the page; a recitation is a test.
- No simulator.
- No core problem harder than the paired assignment; difficulty above assignment level belongs only in the stretch.
- No construct beyond the recitation's week on the language ladder.
- No Mission Ares or HAB; recitations are Rubber Duck Robotics and Gary.
- No Gary's-helper giveaway: never state the helper is wrong, name the defect, or hand the fix; show it in both languages and ask for sign-off.
- No point values in prose; points live only in the rubric TSV. The stretch is labeled `(extra credit)` with the disclosed label.
- No oral-check text on the page; the oral check is an in-room TA event.
- No emdashes, and no pairing language; author for one individual intern.
