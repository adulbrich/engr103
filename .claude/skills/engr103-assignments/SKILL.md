---
name: engr103-assignments
description: Use when creating or editing assignment files (MDX in src/content/docs/assignments/). Enforces the take-home, dual-language, Mission Ares specification structure of the ENGR 103 assignments: the four parts, the functions-against-a-harness model, HAB's wrong snippet, and alignment to the recitation the assignment prepares. Always load this skill before writing or editing any assignment file.
---

# Assignment Style Guide

This skill governs how assignments are written and revised for the ENGR 103 course website (Astro/Starlight, MDX format). Assignments are the take-home **training set**: the tier where a student practices a problem family, with any tool including AI, before the proctored recitation tests a fresh variation of the same skeleton. VISION.md sections 6, 8, and 9 are the authority for this tier; this skill operationalizes them. When this skill and VISION.md disagree, VISION.md wins and this skill is the bug.

An assignment is a **specification**, not a tutorial. It states what to build, the contract each piece must satisfy, and the cases it will be checked against. It does not walk the student through building it step by step; the lecture explains, the activity gives a first guided rep, and the assignment hands the student a goal and lets them reach it.

## What an Assignment Is

Each assignment is paired with exactly one upcoming recitation and covers the lecture material that recitation will assess. Under the course's one-week lag (see `schedule.mdx`), an assignment is released right after a recitation and due the Monday evening before the next one, covering the previous week's two lectures. It exists to make the student ready for that recitation and, cumulatively, for the exams.

Assignments are **low-stakes**: ten percent of the grade and openly tool-permissive. An assignment's grade combines completeness and correctness: the auto-graded problem set is scored for correctness, while the paper practice and reflection are graded for completeness only. The course says so in week 1: use any tool you like here, because the recitation will test the same skeletons in the locked-down room without them, and a student who outsourced the practice walks in unprepared. That framing is the whole integrity model for this tier; an assignment never polices tool use, it makes tool-only work self-defeating at the recitation, where the real points are.

Like a lecture, an assignment is **dual-language**: every problem is solvable in Python or Rust, against one shared contract, and the student may even mix languages across problems within a set.

## The Story: Mission Ares

Assignments wear the **Mission Ares** story line. The student is a member of the flight software crew of a crewed Mars habitat, and the assignments follow a small plot, one episode per assignment, building from launch preparation through the surface phase to the Sol 100 status report as the capstone assignment. Each assignment's opening narrative is a mission beat that sets the scene and hands the student their task; the exact episode per week follows the family schedule.

The recurring character is **HAB**, the habitat's onboard AI assistant: helpful, tireless, and confidently wrong at a convenient moment. HAB is how the judge-the-code work arrives in story ("HAB drafted this; sign off or reject it"). Placing HAB on the assignment tier is deliberate: it is the one tier where students may use AI, so a confidently wrong AI whose code the student must judge is the right training partner here.

The story is a **spine, not a cage**. Every requirement is numbered, testable, and fully understandable with the plot skipped. The mission never obscures the technical ask, and a student who ignores the lore entirely can still earn full marks. The upcoming recitation assesses the same skeletons in fresh Rubber Duck Robotics costume, so a memorized or generated solution transfers only if the understanding did.

## Trains the Recitation, Harder Than the Activity

An assignment sits between the activity and the recitation on one problem family's escalator.

- It is **harder than the paired activity.** The activity is a guided first rep at one lecture's concepts; the assignment is an independent, more complete program that combines the family's pieces into something worth 2.5 to 3 hours of work. If a student who did the activity could finish the assignment without thinking, the assignment is too thin.
- It is the **same level as its recitation sibling.** The recitation problem is a fresh variation of the assignment's skeleton: new mission beat, new values, at most one small structural twist (an extra edge case, one more parameter). It is never a harder problem. So the assignment must not be *easier* than the recitation either, or the recitation would ambush a student who practiced honestly. Difficulty headroom belongs to the optional summit problem alone.

Read that as a two-sided constraint: strictly above the activity, level with the recitation. The variable the recitation adds is the conditions (locked down, alone, docs only, time-boxed), not the difficulty.

## The Four Parts

Every assignment has these parts, in this order. The problem set is always present; the paper practice and reflection are non-negotiable in every regular assignment; the summit is optional and uncounted.

1. **Problem set (auto-graded).** Three to five problems in the same family and at the same level as the upcoming recitation. Sized to a **median completion time of about 2.5 to 3 hours** for a student who did the reading, tests included; the three-to-five count bends to that time target and never past it. Each problem is a set of functions written against a provided harness (see below). From the functions week on, **one problem per set is the spec-to-tests problem** (see below).
2. **Paper practice (self-checked).** One tracing or memory-diagram exercise in exam-archetype format, done by hand and photographed. This is the only regular rehearsal of the paper modality outside lectures, so it is required in every regular assignment. The worked solution sits in a closed `<Reveal>` on the page: the student attempts the trace by hand, then opens it to self-check. Graded on completion (the attached photograph), not on correctness.
3. **Reflection line (two sentences).** The student must **explicitly disclose whether they used AI, and for what** (and state plainly when they did not), then name any other help used (peers, docs, the lecture notes) and one thing they still cannot do without help. Graded on completion, not content; the explicit AI disclosure normalizes honest tool talk and feeds the instructor's misconception review.
4. **Summit problem (optional, worth nothing).** One problem at stretch altitude, the take-home training ground for the recitation's stretch problem. It carries no points on purpose: extra credit at the take-home tier would reward spare time rather than mastery. What it earns is preparation.

Every assignment is a full assignment: there is no reduced exam-adjacent variant, and no assignment carries exam-rehearsal material. Exam rehearsal is provided separately from the assignment track (for example as a practice exam or a Canvas quiz), so keep even the week-6 and week-10 assignments sized to the same weekly budget as any other week, never heavier.

## The Functions-Against-a-Harness Model

From the functions lecture (week 3) onward, every problem is a **set of functions written against a provided harness**: the surrounding `main`, all console input and output, and the local test runner are given, so the student writes only the named functions and the automated grader calls them directly with arguments it chooses, checking what comes back. This is what the functions lecture promises the student, so honor it: the grader reads only return values, never printed text, and a function that tries to print its answer or mutate a caller's value fails.

Consequences the spec must respect:

- **State the contract in the Interface, always in both languages.** Give each function's exact signature: name, parameter names and (for Rust) types, and the return type. Python signatures carry no type annotations, matching the course style; Rust signatures carry explicit parameter types and an explicit `-> ReturnType`, with an explicit `return` in the body.
- **Acceptance tests are call-to-value, not printed output.** Write each visible case as a call and the value it must return (`whole_sols(200000) -> 2`), because that is what the grader checks. Show a printed run only to illustrate the harness, and when you do, show each language's real display faithfully (a Rust `f64` of `1150.0` prints as `1150`, Python's prints as `1150.0`).
- **Twin scaffolds, one contract.** The same acceptance tests bind both languages. The spec page shows the signatures in a `<Tabs syncKey="lang">` pair; the full starter repository (twin scaffolds, visible tests, `check`/`pack`) is separate infrastructure and is not part of the MDX page.

**The pre-functions exception.** The first assignments, before the functions lecture, cannot use this model, because the student has no function syntax yet. Those assignments give a complete program with a single clearly marked region to fill, checked by comparing the program's output to expected output. Do not write a functions-against-a-harness problem for an assignment due before the functions lecture; check the language ladder first. Even showing a function skeleton counts as using functions: a skeleton, a signature, or `todo!()` may appear only once the functions lecture has been taught.

**Introduce the harness conventions before the first harness assignment.** The provided `main` and the local test runner use conventions the student reads but never writes, in particular Python's `if __name__ == "__main__": main()` entry-point idiom and the test-runner's structure. The lecture notes and activities do not currently teach these, so they must be introduced in the week-1 onboarding (the setup assignment and its how-to) before the first functions-against-a-harness assignment; a harness assignment authored before that onboarding exists is not ready to ship. The same onboarding is the place to show students where to find the official documentation, the provided helper functions, and any libraries they may use (outcome O7). An assignment must never be the first place a student meets the `__main__` idiom, the test runner, or the documentation.

## The Red Herring Ladder

Real specifications carry noise, so an assignment plants two calibrated distractors and no more:

- **One mild spec distractor:** a quantity mentioned in the narrative that is never needed for any function. It adds noise, never ambiguity.
- **HAB's snippet:** one provided function that looks convenient but is subtly wrong or unnecessary, presented as HAB's draft awaiting sign-off. The student must judge it, spot the defect (a precedence slip, an integer division where a float was meant, an off-by-one), and write the correct version. Copying HAB's snippet uncritically costs a visible test.

The rule that keeps this honest: **a correct solution never depends on noticing the herring.** Distractors punish only uncritical copying; they never gate the right answer behind a catch. This trains extracting the computational core from a noisy spec, and judging code the student did not write.

**Never give away HAB's defect or its fix.** Judging the draft is the exercise, so the page must not do the judging for the student. Present HAB's draft in both languages, state that it is HAB's work and must be signed off or rejected, and stop there. Do **not** state whether the draft is wrong, do **not** name the defect or the line it is on, do **not** hand a failing input or the wrong output it produces, do **not** explain the cause, and do **not** put any of this behind a `<Reveal>`. There is no hint attached to HAB's snippet at all: the student has the full requirements, and testing HAB's draft against them is exactly the skill being trained. The one thing the page may say is the neutral instruction to check it: "HAB drafted this. Sign off on it, or reject it and write your own." (The accuracy pass still runs HAB's snippet behind the scenes to confirm it really is wrong; that verification lives in your work log, never on the page.)

## Reused Components

Assignments import only what they use, and only from this list:

```mdx
import { Aside, Steps, Tabs, TabItem } from '@astrojs/starlight/components';
import Reveal from '/src/components/Reveal.astro';
import LanguageVersions from '/src/components/LanguageVersions.astro';
import WhatDiffers from '/src/components/WhatDiffers.astro';
import Latex from '/src/components/Latex.astro';
```

- **`Tabs` / `TabItem`**: the dual-language pair, always `syncKey="lang"`, Python first and Rust second. Use it for every interface signature and every shown snippet that exists in both languages.
- **`LanguageVersions`**: at the very top of the body, before the opening narrative, exactly as a lecture does.
- **`Aside`**: a hint, a caution, or the HAB sign-off callout, always with a `title`. This is the assignment tier's scaffolding voice; use it for the open hints VISION allows, not to smuggle in lecture exposition.
- **`Steps`**: for a suggested order of attack or a worked sub-step sequence, when the scaffolding is genuinely an ordered list.
- **`Reveal`**: for the paper practice's worked solution (always), and otherwise sparingly for an *optional* hint on a problem the student can choose to open ("Stuck on the precedence? Open for a nudge"), where the nudge is a hint, never the answer. **Never attach a `<Reveal>` to HAB's snippet:** judging HAB's draft carries no hint of any kind (see The Red Herring Ladder).
- **`WhatDiffers`**: inside a shown dual-language snippet when a real Python/Rust difference needs naming, beyond the standard `fn main`/`println!` boilerplate.
- **`Latex`**: for a formula the spec references (a delta-v budget, a conversion), when plain text would be ambiguous.

Do not import a lecture-only component (`MemoryStepper`, the visualizers); the assignment tier does not use PDF or video deliverables. A TSV-driven rubric table may be added to this tier later (as in other courses); it is out of scope for now, neither required nor forbidden, so do not add one unprompted and do not write anything that assumes one.

## Frontmatter and the `ai-summary` Block

### Frontmatter

```yaml
---
title: "Short Mission-Flavored Name"
description: "One sentence naming the mission task and what the student will build."
sidebar:
  order: <number, the assignment's position; keep distinct from lecture/activity orders>
draft: true
---
```

The `title` is the assignment's name only, with no `Assignment N:` prefix (for example `The Injection Burn`, not `Assignment 3: The Injection Burn`). The assignment's number lives in `sidebar.order` and in the schedule's label, not in the page title.

### The `ai-summary` block

Immediately after the imports, every assignment carries an `ai-summary` MDX comment, hidden from students:

````mdx
{/* ai-summary
type: assignment
slug: <filename without .mdx>
order: <sidebar.order>
family: <the problem family, by description>
paired_recitation: <the recitation this assignment prepares, e.g. R4>
prereq_lectures: <comma-separated lecture slugs whose concepts are used>
prereq_activities: <comma-separated activity slugs, or empty>
prereq_assignment: <previous assignment slug, or none>
new_scope: <the new capability this assignment adds, one clause>
persistent_requirements: <carried-forward requirements; semicolon-separated>
story_beat: <the Mission Ares mission episode, one clause>
output: <what the student submits: the functions, the paper trace, the reflection>
*/}
````

Rules:

- `paired_recitation` and `family` are required; the family is named by description, never by a code name.
- `prereq_lectures` and `prereq_activities` name only directly load-bearing prerequisites.
- Leave a field blank rather than guessing.

## Page Structure

Every assignment follows this order.

1. **`<LanguageVersions />`** at the very top of the body.
2. **Opening narrative** (no heading, 2 to 4 sentences): the Mission Ares mission beat. It names the situation, gives the student their crew role and constraint, and ends with what they are being asked to build in one sentence. It states no requirements and explains no concept.
3. **How this assignment works** (a short `##` section): one or two sentences naming the four parts present, the language choice, and that the grader calls the student's functions directly. Link the relevant how-to for setup and submission rather than restating mechanics.
4. **Problem set** (a `##` section, then one `###` per problem). Each problem uses the per-problem template below.
5. **Paper practice** (a `##` section): the archetype trace or diagram to do by hand and photograph, with its worked solution in a closed `<Reveal>` on the page for self-checking. Graded on completion.
6. **Reflection** (a short `##` section): the two-sentence prompt.
7. **Summit problem** (a final `##` section, optional): one stretch problem, labeled as optional and worth nothing.

### The Per-Problem Template

Each problem in the set is written with this structure, drawn from VISION section 9. The order is fixed:

- **Context** (one or two sentences): the mission beat for this specific problem, skippable.
- **Requirements** (numbered, testable statements): exactly what each function must compute, in plain language. Use "must"; state the units and the rounding or truncation behavior explicitly, because the family turns on it.
- **Interface** (a `<Tabs syncKey="lang">` pair): the exact function signature(s) in Python and Rust, with a title on each code block.
- **Acceptance tests** (a short list or table): visible cases as call-to-value pairs, including at least one boundary case. These are the cases the student can see; hidden cases run at grading time.
- **Assumptions and edge cases**: what the input is guaranteed to be, and which cases the function must handle.
- **Scaffolding** (assignment tier only): open hints, a suggested order of attack, worked sub-steps, or mission commentary, as an `<Aside>`, a `<Steps>`, or an optional `<Reveal>`. Recitations strip this; assignments keep it. Do not let a hint become a step-by-step tutorial.

Exactly one problem in the set carries **HAB's snippet** (the red-herring rung). Introduce it in story, show the wrong draft in both languages, and require the student to reject or fix it. Give away nothing about the defect or its fix, and attach no `<Reveal>` to it: see the no-giveaway rule in The Red Herring Ladder.

Every problem **labels its modeling step and its analysis step**: which sentence builds the representation of the mission situation, and which runs it and interprets the result, so the modeling-versus-analysis distinction is rehearsed continuously.

## The Spec-to-Tests Problem

From the functions week on, one problem per set is the spec-to-tests problem: the student receives a function's requirements and signature and writes only its test cases, not its body. This makes "did you understand the spec" auto-gradable, because a test suite is an executable statement of the requirements.

Until the testing lecture (week 6), assertion syntax is not on the ladder, so early sets use a **plain data format**: the student lists cases as input-and-expected-output rows (a small table or a list of tuples), not `assert` calls. State the format explicitly, give one worked example row, and ask for the boundary and error cases by name. From the testing lecture on, the spec-to-tests problem may use `assert`.

## Dual-Language Rule

Every problem works identically in both languages against one shared contract. Show each function's signature in both, Python first. Where a shown snippet (an acceptance-test demo, HAB's wrong draft) differs between the languages beyond the standard `fn main`/`println!` boilerplate, add a `<WhatDiffers>` naming the real difference: the integer-division split, the `True`/`true` spelling, the `f64` print display. Omit it when boilerplate is the only difference.

## Difficulty, Sizing, and the Ladder

- **Sizing is by time, not word count.** The problem set targets a 2.5 to 3 hour median for a prepared student, tests included. Let that set the three-to-five count. As a secondary diagnostic, a full assignment page runs roughly **2,000 to 4,000 words**; landing well under usually means the problems are under-specified or too thin to be harder than the activity, and well over usually means tutorial contamination that belongs in a hint or a how-to.
- **Solvable within what has been taught.** Every assignment must be fully solvable using only the constructs the course has introduced by its due date, so a student who followed the course is never blocked. The [language ladder](/reference/language-ladder/) is a recommendation for students (they may use anything they already know), but for you, the author, it is a hard requirement in this one direction: the intended solution, the provided starter code, and every required step must stay inside the taught subset. Check the ladder before writing any code: an assignment due before the functions lecture cannot require student-written functions; one due before conditionals cannot require branching; one due before loops cannot require iteration. A problem may be solvable in more than one way, and a student is free to reach for a feature they already know, but the taught subset must always suffice. Raise difficulty by combining and extending what has been taught, not by requiring something from a later week.
- **Exercise the week's full range, honestly.** An assignment should practice the breadth of what its week newly makes available, not the narrowest slice of it. If the week introduces four literal types, the problems should touch more than one; if it introduces the remainder operator alongside integer division, use both. This is what keeps an assignment from feeling thin. But respect the week's ceiling honestly: some constructs a week introduces can only be exercised trivially given what else is missing (before comparisons exist, a boolean can only be printed, not computed), and padding a problem with a trivially-used construct to claim "variety" is worse than leaving it out. Cover what the week can genuinely exercise, and when the week's ceiling is simply low (the earliest assignments), max out what it does allow and say plainly, in your work log, that the gentleness is structural rather than over-building to hide it.
- **Mine the retired assignments for challenges.** Before writing or strengthening an assignment, look at the retired course's assignments (in `src/content/docs/assignments/`, the non-Ares drafts such as `linear-equations`, `financial-planner`, `calculator`, `dictionary`) for reusable problem structures: a break-even or linear-system solve, compound growth by accumulation, input validation with a menu and dispatch, evaluating an expression, the "call these provided functions correctly without knowing their internals" pattern. Reuse the *structure*, not the C++ or the off-ladder parts: reframe it into a Mission Ares beat, fit it to the due week's ladder (a network API becomes provided harness functions the student orchestrates; a `ceil()` call becomes an integer-ceiling expression), and verify it in both languages. A strong reused challenge, placed at the right week, is the fastest way to lift an assignment above thin.

## Draft Policy

Set `draft: true` while an assignment is in development. **Never flip it to `false` yourself and never remove the key**; only the instructor publishes. A draft assignment is not a source of truth, and `schedule.mdx` must not link to it (a link to a draft page 404s). The schedule may name the assignment in prose while it is a draft; converting that name to a link happens only when the instructor publishes.

## Style

**Specification voice.** Direct and imperative. Use "must" for hard requirements and "you may use any defensible approach" where the implementation is the student's choice. State what to build, not how to build it.

**No tutorial contamination.** Requirements say *what*, not *how*. "First do X, then do Y" belongs in a hint or an activity, never in a requirement.

**Plain language.** Write for a first-year reader who may not speak English as a first language: short sentences, common words, every technical term already defined in the lecture and used consistently. The story flavor never obscures the ask.

**No emdashes.** Never use the emdash character or a double hyphen as a dash. Use a colon, a semicolon, a comma, or a period.

## The Accuracy Pass

An assignment is not done until every piece of code and every claimed value on the page has actually been run and confirmed, in both languages.

- Run every function shown, and every acceptance-test call, in Python (`python3`, version 3.14) and Rust (`rustc`, version 1.88), and confirm each returns the value the page claims.
- Run **HAB's wrong snippet** too, and confirm it really produces the wrong result the problem says it does, and that the corrected version produces the right one.
- For any printed run shown to illustrate the harness, confirm each language's exact display, including the `f64` versus Python-float difference.
- Fix any mismatch before shipping. No claimed value ships unverified.

## Validation

After writing or editing an assignment, run:

```bash
npm run build
```

This runs `astro check` and the link validator. Fix every error before considering the assignment done.

## What Assignments Must NOT Contain

- No tutorial walkthrough: requirements state what, not how.
- No assignment that *requires* a construct the course has not introduced by its due-date week; it must be solvable with the taught subset (a student may still use more if they know it).
- No functions-against-a-harness problem in an assignment due before the functions lecture.
- No acceptance test written as printed output when the grader checks return values.
- No Rubber Duck Robotics or bare-exam framing: assignments are Mission Ares.
- No paper practice whose worked solution is withheld from the page; it goes in a closed `<Reveal>` for self-checking.
- No giveaway on HAB's snippet: never state that it is wrong, name its defect, hand a failing case, explain the fix, or attach a `<Reveal>` to it.
- No `Assignment N:` prefix in the page title; the number lives in `sidebar.order`.
- No reduced exam-adjacent assignment and no exam-rehearsal material in an assignment.
- No points attached to the summit problem.
- No PDF or video deliverables.
- No lecture-only or other-course component imports.
- No emdashes, and no pairing language; author for one individual crew member.
