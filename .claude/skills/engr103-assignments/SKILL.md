---
name: engr103-assignments
description: Use when creating or editing assignment files (MDX in src/content/docs/assignments/). Enforces the take-home, dual-language, Mission Ares specification structure of the ENGR 103 assignments: one open problem whose contract is fixed and whose decomposition is the student's, HAB's wrong claim to judge, and alignment to the recitation the assignment prepares. Always load this skill before writing or editing any assignment file.
---

# Assignment Style Guide

This skill governs how assignments are written and revised for the ENGR 103 course website (Astro/Starlight, MDX format). Assignments are the take-home **training set**: the tier where a student practices a problem family, with any tool including AI, before the proctored recitation tests a fresh variation of the same skeleton. VISION.md sections 6, 8, and 9 are the authority for this tier; this skill operationalizes them. When this skill and VISION.md disagree, VISION.md wins and this skill is the bug.

An assignment is a **specification**, not a tutorial, and not a worksheet of pre-decomposed one-liners. It hands the student a mission situation and a goal, fixes only what the grader can observe, and leaves every decision about how to get there to the student. It does not walk the student through building the solution, and it does not pre-break the solution into named helpers for the student to fill in; the lecture explains, the activity gives a first guided rep, and the assignment makes the student work out how to use what they have learned.

## The Core Principle: Fix the Contract, Not the Solution

The single most important rule of this tier, and the one the earlier assignments got wrong: **specify only the observable contract, and let the student own everything behind it.**

The student must decide, themselves, what intermediate values to compute, whether to write helper functions and what to call them, which variables represent the situation, and which language construct is the right tool. Those decisions are the learning. An assignment that pre-names the helper functions, states "the modeling step is deciding to represent time as whole sols and leftover seconds," or tells the student "join these with `and`" has already done the thinking and left the student to transcribe. That is the failure this rewrite exists to end.

What the page is allowed to pin down is exactly what the grader checks, and no more:

- **Before the functions lecture (weeks 1 to 2):** the contract is the program's **printed output**, in an exact stated format. The page says what to read in, what to print, and in what format; grading compares the printed output. How the program computes it is entirely the student's.
- **From the functions lecture (week 3) on:** the contract is **one named entry-point function** (see [Up to Two Entry Points](#up-to-two-entry-points) for the rare second), with an exact signature and an exact return. The grader calls that one function with values it chooses and reads only what it returns. Every helper the student writes inside is theirs to name, structure, and choose; the page never names a helper, never prescribes a decomposition, and never dictates a construct.

Everything else in this skill serves that principle.

## Give the Math to Match Its Complexity

The challenge of an assignment is always **implementation**: turning requirements into working, well-structured code. It is never deriving mathematics, and never inventing from nothing an algorithm the student has no way to reach. So calibrate how much of the math you hand over to how hard that math is.

- **Simple math** (a unit conversion, a percentage, the whole hours within a day): state the requirement in plain language and let the student work out the operations. Do **not** give the formula. Figuring out that you need a remainder and then an integer division is programming, and it is exactly the skill the week trains.
- **Complex math** (a system of equations, an involved closed-form expression): **give the student the answer, the equations, rendered in LaTeX with `<Latex>`.** The student does not derive or solve anything. The challenge is implementing the given formula well: reading the notation, mapping each symbol to an input, breaking a long expression into their own intermediate variables, and getting the float arithmetic right. This is what "let students break down the given equations in their own set of variables" means.

The line between the two is judgment, but the test is clear: if a prepared first-year could reasonably work the math out as part of learning to program, make them; if it is real mathematics they would have to derive or solve, hand it over and let the coding be the work. Either way, never pre-decompose the *program*. Giving a formula is fine; giving the student their variable names, their helper functions, or their operator choices is not.

## What an Assignment Is

Each assignment is paired with exactly one upcoming recitation and covers the lecture material that recitation will assess. Under the course's one-week lag (see `schedule.mdx`), an assignment is released right after a recitation and due the Monday evening before the next one, covering the previous week's two lectures. It exists to make the student ready for that recitation and, cumulatively, for the exams.

Assignments are **low-stakes**: ten percent of the grade and openly tool-permissive. An assignment's grade combines completeness and correctness: the auto-graded problem is scored for correctness, while the paper practice and reflection are graded for completeness only. The course says so in week 1: use any tool you like here, because the recitation will test the same skeletons in the locked-down room without them, and a student who outsourced the practice walks in unprepared. That framing is the whole integrity model for this tier; an assignment never polices tool use, it makes tool-only work self-defeating at the recitation, where the real points are.

Like a lecture, an assignment is **dual-language**: the problem is solvable in Python or Rust against one shared contract, and the student may work in whichever they prefer.

## Choosing the Problem So the Week's Construct Is the Natural Tool

Total freedom over the solution creates one risk: if the page never tells the student which construct to use, a student could solve a "loops week" problem without a loop and arrive at the recitation unprepared for the thing it assesses. You resolve this not by directing the student, but by **choosing a problem where the week's target construct is the natural or necessary tool.** This is a hard constraint on you, the author, and it is invisible to the student.

- A loops-week problem takes input of unknown or large length, or repeats an operation a data-dependent number of times, so that iteration is the obvious way through and writing it out by hand is absurd.
- A conditionals-week problem has several distinct outcomes that turn on the inputs, so a decision is unavoidable.
- A functions-week problem is naturally expressed as a computation with a clear input and output, so the entry-point function contract fits the work rather than fighting it.

The test: could a prepared student who ignored the week's lecture still hit the contract easily? If yes, the problem is wrong for the week; reshape the situation until the taught construct is the path of least resistance. The student still gets to discover that path; you guarantee the path exists and leads through this week's material.

## The Story: Mission Ares

Assignments wear the **Mission Ares** story line. The student is a member of the flight software crew of a crewed Mars habitat, and the assignments follow a small plot, one episode per assignment, building from launch preparation through the surface phase to the Sol 100 status report as the capstone assignment. Each assignment's opening narrative is a mission beat that sets the scene and hands the student their task; the exact episode per week follows the family schedule.

The recurring character is **HAB**, the habitat's onboard AI assistant: helpful, tireless, and confidently wrong at a convenient moment. HAB is how the judge-the-work skill arrives in story. In the new format HAB does not hand the student a code draft to fix, because handing over a draft is itself a pre-decomposition. Instead **HAB states a claim**: a confident assertion about how to approach the problem, or a fact about it, that is subtly wrong. The student has to catch that it is wrong before building on it. Placing HAB on the assignment tier is deliberate: it is the one tier where students may use AI, so a confidently wrong AI whose advice the student must judge is the right training partner here. See [HAB's Wrong Claim](#habs-wrong-claim).

The story is a **spine, not a cage**. The requirement is testable and fully understandable with the plot skipped. The mission never obscures the technical ask, and a student who ignores the lore entirely can still earn full marks. The upcoming recitation assesses the same skeleton in fresh Rubber Duck Robotics costume, so a memorized or generated solution transfers only if the understanding did.

## Trains the Recitation, Harder Than the Activity

An assignment sits between the activity and the recitation on one problem family's escalator.

- It is **harder than the paired activity.** The activity is a guided first rep at one lecture's concepts; the assignment is an independent, more complete program that combines the family's pieces into something worth 2.5 to 3.5 hours of work, with the decomposition left to the student. If a student who did the activity could finish the assignment without making any decisions of their own, the assignment is too thin.
- It is the **same level as its recitation sibling.** The recitation problem is a fresh variation of the assignment's skeleton: new mission beat, new values, at most one small structural twist. It is never a harder problem, so the assignment must not be easier than the recitation either. Difficulty headroom belongs to the optional summit problem alone.

The variable the recitation adds is the conditions (locked down, alone, docs only, time-boxed), not the difficulty, and not the amount of hand-holding removed. If the only thing that makes your assignment easier than its recitation sibling is that the assignment pre-decomposed the solution, you have written the assignment wrong: the decomposition is the student's work at both tiers.

## The Parts, and the Test-Driven Flow

The assignment is one problem, worked **test first**: the student reads the problem, then designs test cases, then writes pseudocode, then writes code, in that fixed order. This mirrors how real software is written and makes the "understand the spec" work visible and gradable. The page presents the problem and then walks the student through the three steps, followed by the extra-credit summit, the paper practice, and the reflection.

The parts, in page order:

1. **The problem (read it).** The situation, any given equations, the contract (what each function returns), the function signatures, and **one** worked example test case for format. It fixes only the observable contract; the decomposition is the student's.
2. **Step 1: Acceptance tests (the student writes).** Before any code, the student designs a set of test cases, including boundary and edge cases, each with the expected return values worked out by hand. We give exactly one example; the student produces the rest. This is the test-first rung, and it is where "did you understand the spec" is graded. It goes in the PDF (see [The Submission Split](#the-submission-split)).
3. **Step 2: Pseudocode (the student writes).** A plain-language plan of how each function turns its inputs into its return, naming the intermediate values the student will create. Still no code. It goes in the PDF.
4. **Step 3: Code (the student writes).** The functions themselves, implemented from the student's own pseudocode, in Python or Rust. Autograded on Gradescope.
5. **Summit problem (extra credit).** One problem at stretch altitude, worth extra credit and the take-home training ground for the recitation's stretch problem. Same rules as the main problem: give complex math, fix only the contract, never pre-decompose. Its code goes to Gradescope; the page labels it `(extra credit)` and states no point values.
6. **Paper practice (self-checked).** One tracing or memory-diagram exercise in exam-archetype format, done by hand and photographed, with the worked solution in a closed `<Reveal>` for self-checking. Graded on completion, not correctness. The photo goes in the PDF.
7. **Reflection.** The AI-use disclosure (two sentences, stating plainly when no AI was used) plus the modeling line: which intermediate variables the student chose, and why. This is where the modeling-versus-analysis distinction is rehearsed now that the spec no longer labels it. Graded on completion. It goes in the PDF.

The problem, all three steps, the paper practice, and the reflection are non-negotiable in every regular assignment; the summit is optional extra credit. Two further page elements close every assignment: an optional **simulator** and the **rubric** (see [The Simulator](#the-simulator) and [The Rubric](#the-rubric)).

Every assignment is a full assignment: there is no reduced exam-adjacent variant, and no assignment carries exam-rehearsal material. Exam rehearsal is provided separately, so keep even the week-6 and week-10 assignments sized to the same weekly budget as any other week, never heavier.

## The Entry-Point Contract (functions era)

From the functions lecture (week 3) onward, the assignment fixes **one named entry-point function** written against a provided harness: the surrounding `main`, all console input and output, and the local test runner are given, so the student writes the entry-point function (and any helpers they choose) and the automated grader calls the entry point directly with arguments it chooses, checking what it returns. The grader reads only the return value, never printed text, and a function that tries to print its answer or mutate a caller's value fails.

Consequences the spec must respect:

- **Name only the entry point, in both languages.** Give the entry-point function's exact signature: name, parameter names and (for Rust) types, and the return type. Do **not** name, sketch, or imply any helper function; helpers are the student's decision. Python signatures carry no type annotations, matching the course style; Rust signatures carry explicit parameter types and an explicit `-> ReturnType`, with an explicit `return` in the body.
- **Acceptance tests are call-to-value, not printed output.** Write each visible case as a call to the entry point and the value it must return (`whole_sols(200000) -> 2`), because that is what the grader checks. Show a printed run only to illustrate the harness, and when you do, show each language's real display faithfully (a Rust `f64` of `1150.0` prints as `1150`, Python's prints as `1150.0`).
- **One contract, two languages.** The same acceptance tests bind both languages. The spec page shows the entry-point signature in a `<Tabs syncKey="lang">` pair; the full starter repository (twin scaffolds, visible tests, `check`/`pack`) is separate infrastructure and is not part of the MDX page.

### Up to Two Entry Points

Fix a **second** named entry-point function only when the family is genuinely two independent outputs that a single function cannot naturally return together (for example two distinct predicates the recitation family treats as separate). This is the exception, not the default; most assignments fix one entry point. When you do fix two, they are two independent contracts, never a decomposition of one another: never name one function as a helper the other must call. If you find yourself wanting a third named function, you are decomposing the solution for the student. Stop, and fold the extra structure back inside the entry point as the student's choice.

### The Pre-Functions Contract (weeks 1 to 2)

The first assignments, before the functions lecture, cannot use the entry-point model, because the student has no function syntax yet. Those assignments fix the **printed output**: the page states what to read in, what to compute, and the exact format to print, and grading compares the program's printed output to the expected output. Do not show a function skeleton, a signature, or `todo!()` in a pre-functions assignment: even a skeleton counts as using functions, and those may appear only once the functions lecture has been taught. The decomposition inside the program is still the student's; the page fixes only the printed result and its format.

**Introduce the harness conventions before the first entry-point assignment.** The provided `main` and the local test runner use conventions the student reads but never writes, in particular Python's `if __name__ == "__main__": main()` idiom and the test-runner's structure. These are introduced in the week-1 onboarding (the setup assignment and its how-to) before the first entry-point assignment; the same onboarding shows students where to find the official documentation, the provided helper functions, and any libraries they may use (outcome O7). An assignment must never be the first place a student meets the `__main__` idiom, the test runner, or the documentation.

## Provided Code

An assignment may ship **provided code** beyond the standard harness, shown on the page in a `<Tabs syncKey="lang">` pair: a module the student's function must plug into, a parser that feeds it its values, or a provided function the student must write tests against or find the fault in. Provided code widens what a week can practice: reading, testing, judging, and extending code the student did not write are the O5 and O8 skills, and they need real code to act on.

The one rule: **provided code never decomposes the student's own solution.** It stands on the other side of the contract (it calls the student's function, or the student's tests call it); it is never a skeleton of the function the student is writing, never a named helper the student's solution must call into existence, and never a draft to finish. If removing the provided code would leave the student's own task unchanged in shape, the provided code is fine; if removing it would hand the student back a design decision, it was a decomposition in disguise.

Provided code follows the same accuracy pass as everything else: it must run as shown, in both languages, and stay within the taught subset (or gloss, in one sentence, any construct it uses that the student only reads and never writes).

## HAB's Wrong Claim

HAB is the assignment tier's rung of the red herring ladder, reframed for the new format. HAB does not hand over a code draft; a draft pre-decomposes the solution, which is exactly what this tier no longer does. Instead **HAB makes a claim in prose**: a confident, plausible, and subtly wrong statement about how to approach the problem or a fact it turns on. For example, HAB might assert that a fifteen-percent margin is added by adding the percent straight onto the mass, that a ceiling can be gotten by plain division, that two floats can be compared for exact equality, or that a sol boundary case rounds a particular way. The student must judge the claim against the requirements, and a student who takes it on faith arrives at a wrong answer that a visible test catches.

The rules that keep this honest:

- **A correct solution never depends on noticing the claim.** A student who ignores HAB entirely and reasons from the requirements gets the right answer. The claim punishes only uncritical trust; it never gates the right answer behind a catch.
- **Never give away the verdict or the fix.** Judging the claim is the exercise. State the claim as HAB's, in story, and ask the student to decide whether to trust it. Do **not** say whether it is right or wrong, do **not** name the flaw, do **not** hand a case that exposes it, do **not** explain the correction, and do **not** attach a `<Reveal>` to it. The one thing the page may add is the neutral instruction to check it before relying on it. (The accuracy pass still verifies the claim really is wrong behind the scenes; that verification lives in your work log, never on the page.)
- **The claim is prose, not code.** Present it as something HAB says (an `<Aside>` in HAB's voice), not as a function draft. If you catch yourself writing a `def` or `fn` for HAB, you are back in the old format.

Also plant **one mild spec distractor**: a quantity mentioned in the narrative that is never needed to hit the contract. It adds noise, never ambiguity. Plant it and then **stop talking about it**: never defuse it on the page ("though the panel never prints that rating", "a detail for the log and nothing your functions need"). A defused distractor trains no noise-filtering; the student's own reading of the requirements is what establishes the quantity is unused. Two calibrated distractors (HAB's claim and the spec distractor), and no more.

One more fairness calibration: the requirements must make HAB's claim refutable, but **never phrase a requirement as a direct rebuttal of the claim**. State the governing rule once, in its own terms, where it naturally belongs; do not restate it beside HAB, do not quote HAB's boundary back at it ("still counts as agreeing, so it returns 0, not 3"), and do not state it twice. If judging the claim reduces to matching HAB's sentence against an adjacent spec sentence, the judgment is gone; the student should have to reason from the rule to the verdict.

## The Submission Split

An assignment is submitted in two places, and the page must say which part goes where, both in the top **What to Submit** section and again in each step:

- **A single PDF on Canvas** holds everything that is not code: the acceptance tests (Step 1), the pseudocode (Step 2), the paper-trace photo, and the reflection.
- **The code goes to Gradescope**: the functions, plus the summit function if attempted. The autograder calls the functions and checks return values.

### The Acceptance-Tests Step

Step 1 is the test-first rung. Ask the student to write a set of cases as a plain table of rows (the inputs and the expected return values), not `assert` calls, until the testing lecture (week 6); from the testing lecture on they may use `assert`. Give exactly **one** worked example in the problem so the student knows the format and has a first check; the student writes the rest. Require, by name, the cases that matter for this problem: a normal case, the boundary or edge cases the spec has, and at least one the student computes fully by hand. For a pure calculation with no true boundary (positive floats, no zero or empty input), ask instead for cases that stress a different regime, such as inputs that make a shared denominator come out negative. The tests are collected in the PDF and graded by the TA against the rubric's acceptance-tests criterion (full coverage, correct expected values); the autograder never runs them, because it only sees the code.

## Reused Components

Assignments import only what they use, and only from this list:

```mdx
import { Aside, Steps, Tabs, TabItem } from '@astrojs/starlight/components';
import Reveal from '/src/components/Reveal.astro';
import LanguageVersions from '/src/components/LanguageVersions.astro';
import WhatDiffers from '/src/components/WhatDiffers.astro';
import Latex from '/src/components/Latex.astro';
import RubricTable from '/src/components/RubricTable.astro';
```

A straightforward-calculation assignment also imports its own simulator, for example `import InjectionBurnForm from '/src/components/InjectionBurnForm.svelte';`.

- **`Tabs` / `TabItem`**: the dual-language pair, always `syncKey="lang"`, Python first and Rust second. Use it for the entry-point signature and any shown snippet that exists in both languages.
- **`LanguageVersions`**: at the very top of the body, before the opening narrative.
- **`Aside`**: a hint, a caution, or HAB's claim, always with a `title`. This is the assignment tier's scaffolding voice; use it for optional hints, not to smuggle in a decomposition or lecture exposition.
- **`Steps`**: for a *suggested* order of attack the student may ignore, when the scaffolding is genuinely an ordered list. Never a mandated build sequence, and never a decomposition of the solution into named pieces.
- **`Reveal`**: for the paper practice's worked solution (always), and otherwise sparingly for an *optional* hint the student can choose to open, where the nudge points at a way in, never the answer and never the decomposition. **Never attach a `<Reveal>` to HAB's claim:** judging it carries no hint.
- **`WhatDiffers`**: inside a shown dual-language snippet when a real Python/Rust difference needs naming, beyond the standard `fn main`/`println!` boilerplate.
- **`Latex`**: for a formula or a system of equations the problem hands the student. When the math is complex enough that a student could not reasonably be expected to derive it, **give the solved equations in LaTeX** and let the student implement them; deriving the math is never the assignment's challenge. See [Give the Math to Match Its Complexity](#give-the-math-to-match-its-complexity).
- **`RubricTable`**: required at the end of every assignment, in a `## Rubric` section, loading the assignment's Canvas rubric TSV. See [The Rubric](#the-rubric).
- **A per-assignment simulator** (optional): when the assignment is a straightforward calculation, a small Svelte component the student can plug numbers into, placed in a `## Simulators` section and hidden from print. See [The Simulator](#the-simulator).

Do not import a lecture-only component (`MemoryStepper`, the visualizers). The assignment tier collects its written artifacts (the acceptance tests, pseudocode, paper trace, and reflection) in a **Canvas PDF** and its code in the **Gradescope** autograder; every assignment ends with a `RubricTable`, and a straightforward-calculation assignment may add one small Svelte simulator.

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

The `title` is the assignment's name only, with no `Assignment N:` prefix (for example `The Injection Burn`, not `Assignment 3: The Injection Burn`). The number lives in `sidebar.order` and in the schedule's label, not in the page title.

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
target_construct: <the week's construct the problem is shaped to make the natural tool>
persistent_requirements: <carried-forward requirements; semicolon-separated>
story_beat: <the Mission Ares mission episode, one clause>
output: <what the student submits: the code to Gradescope; the tests, pseudocode, paper trace, and reflection in a Canvas PDF>
*/}
````

Rules:

- `paired_recitation`, `family`, and `target_construct` are required; the family is named by description, never by a code name.
- `prereq_lectures` and `prereq_activities` name only directly load-bearing prerequisites.
- Leave a field blank rather than guessing.

## Page Structure

Every assignment follows this order.

1. **`<LanguageVersions />`** at the very top of the body.
2. **Opening narrative** (no heading, 2 to 4 sentences): the Mission Ares mission beat. It names the situation, gives the student their crew role and constraint, and ends with what they are being asked to build in one sentence. It states no requirements and explains no concept.
3. **How this assignment works** (a short `##` section): a sentence or two naming that the assignment is worked test first (design tests, then pseudocode, then code), the language choice, and that the grader checks the printed output (pre-functions) or calls the entry point directly (functions era). It does not restate submission mechanics; that is the next section's job.
4. **What to submit** (a short `##` section): the submission split, as two bullets, the Canvas PDF (tests, pseudocode, paper trace, reflection) and the Gradescope code (functions, plus the summit function). Link the setup-and-submission how-to, and say each later section repeats its own ask.
5. **The problem** (a `##` section, using the template below): the situation, the given equations, the contract, the signatures, and one worked example test case. It is the "read it" step; it does not contain the tests, pseudocode, or code, which are the steps that follow.
6. **Step 1: Write your acceptance tests** (a `##` section): the student designs the test set, including boundary or regime-stressing cases, ending with the reminder that it goes in the PDF.
7. **Step 2: Write your pseudocode** (a `##` section): the plain-language plan, ending with the PDF reminder.
8. **Step 3: Write your code** (a `##` section): implement the functions from the pseudocode, ending with the reminder to submit to Gradescope. The `<WhatDiffers>` on the float display, if any, lives here.
9. **Summit problem** (a `##` section), labeled `(extra credit)`, written to the same rules as the problem, ending with a one-line reminder to submit its function to Gradescope. States no point values.
10. **Paper practice** (a `##` section): the archetype trace to do by hand, worked solution in a closed `<Reveal>`, ending with the reminder to photograph it and include it in the PDF. Graded on completion.
11. **Reflection** (a short `##` section): the reminder to include it in the PDF, the two-sentence disclosure prompt, plus the modeling line.
12. **Simulators** (a `##` section, only for a straightforward-calculation assignment): the Svelte simulator with `client:load`, followed by the print-only note `<div class="print:block hidden italic">The simulator is not printed to PDF. Consult the online textbook to use it.</div>`.
13. **Rubric** (a `##` section, always last): `<RubricTable src="canvas/assignments/<slug>-rubrics.tsv" caption="..." />`.

Every step and graded section carries a one-line reminder of what to submit for it and where; the **What to submit** section near the top summarizes the whole submission.

### The Problem Template

The single problem is written with this structure. The order is fixed:

- **The situation** (a short paragraph, plus given math): the mission beat and the goal, in plain language, and any formula or equations the student needs. Give complex math in LaTeX and leave only simple arithmetic for the student to work out (see [Give the Math to Match Its Complexity](#give-the-math-to-match-its-complexity)). It sets the scene, the target, and the given math; it does not decompose the implementation.
- **What your program must do / What your function must return** (numbered, testable statements about the *contract only*): exactly what the printed output must be and in what format (pre-functions), or exactly what the entry-point function must return for its inputs (functions era). State units and the rounding or truncation behavior explicitly, because the family turns on it. Do **not** state intermediate steps, name helper values, or prescribe a construct. "Must return the number of whole sols elapsed" is a contract; "must use integer division after taking the remainder" is a decomposition and is banned.
- **Interface** (functions era only, a `<Tabs syncKey="lang">` pair): the exact entry-point signature in Python and Rust, with a title on each code block. One signature (rarely two). No helpers.
- **One worked example** (a short table or a call-to-value pair): exactly one visible case, so the student knows the format and has a first check. The student designs the rest of the tests in Step 1; the remaining hidden cases run at grading time. Do not print a full table of acceptance cases here.
- **Assumptions and edge cases**: what the input is guaranteed to be, and which cases the solution must handle.
- **HAB's claim**: as its own `<Aside>` in HAB's voice (see below).
- **Optional scaffolding**: at most a short `<Aside>` hint or an optional `<Reveal>` nudge the student may ignore. Never a mandated build order, never a decomposition, never a construct directive. The tests, pseudocode, and code each have their own step section after the problem; do not fold them into the problem.

The problem carries **HAB's claim** (the red-herring rung) as a prose `<Aside>` in HAB's voice: introduce it in story, state the confident wrong claim, and ask the student to judge it before relying on it. Give away nothing about the verdict or the fix, and attach no `<Reveal>`; see [HAB's Wrong Claim](#habs-wrong-claim).

Do **not** write a "modeling step / analysis step" paragraph on the page. Labeling those steps for the student is the spoonfeeding this rewrite removes; the distinction is now drawn out of the student in the reflection.

## Dual-Language Rule

The problem works identically in both languages against one shared contract. Show the entry-point signature in both, Python first. Where a shown snippet (an acceptance-test demo, or a printed run) differs between the languages beyond the standard `fn main`/`println!` boilerplate, add a `<WhatDiffers>` naming the real difference: the integer-division split, the `True`/`true` spelling, the `f64` print display. Omit it when boilerplate is the only difference.

## Difficulty, Sizing, and the Ladder

- **Sizing is by time, not word count.** The whole flow, designing the tests, writing the pseudocode, and writing the code, targets a 2.5 to 3.5 hour median for a prepared student. The upper half of that band is legitimate, not a stretch: recitations and exams ask for no extensive code, so the assignment is the one tier where all substantial programs get written, and a problem that lands at 3.5 hours of honest implementation work is doing its job. As a secondary diagnostic, a full assignment page runs roughly **2,000 to 4,000 words** including the step sections and the rubric; landing well under usually means the situation is under-specified or the problem is too thin to sit above the activity, and well over usually means either tutorial contamination or a decomposition creeping back in that belongs to the student. A page that is long because it lists many named functions to fill in is the specific failure to avoid.
- **Solvable within what has been taught.** Every assignment must be fully solvable using only the constructs the course has introduced by its due date, so a student who followed the course is never blocked. The [language ladder](/reference/language-ladder/) is a recommendation for students (they may use anything they already know), but for you, the author, it is a hard requirement in this one direction: at least one intended solution, the provided starter code, and the fixed contract must all stay inside the taught subset. Raise difficulty by choosing a richer situation that demands more decisions, never by requiring a construct from a later week.
- **Shape the problem to the week's construct.** See [Choosing the Problem](#choosing-the-problem-so-the-weeks-construct-is-the-natural-tool). The problem must be one where the week's target construct is the natural or necessary tool, so that a student solving it freely still practices exactly what the recitation assesses. Record the target construct in the `ai-summary` `target_construct` field.
- **Mine the retired assignments for situations.** Before writing an assignment, look at the retired course's assignments (in `src/content/docs/assignments/`, the non-Ares drafts such as `linear-equations`, `financial-planner`, `calculator`, `dictionary`) for reusable *situations* that demand real implementation work: a linear system whose solved formulas the student implements, a compound-growth accumulation, an input-validation-and-dispatch loop, an expression evaluator. Reuse the situation and the coding decisions it forces, never the C++ and never the pre-decomposed program. The retired linear-equation solver is the archetype: give the equations in LaTeX and let the student write the code that computes them, breaking each formula into their own intermediate variables. Never make the student derive or solve the math. Reframe into a Mission Ares beat, fit it to the due week's ladder, and verify it in both languages.

## The Rubric

Every assignment ends with a `## Rubric` section rendering `<RubricTable src="canvas/assignments/<slug>-rubrics.tsv" caption="..." />`. You author the TSV alongside the page.

- **File and format.** The TSV lives at `canvas/assignments/<slug>-rubrics.tsv`. It is **headerless**, tab-delimited, in Canvas rubric order per row: criterion title, description, `true` (use range), then repeating rating groups of three cells (points, rating title, rating description). A two-level criterion is `Full Marks` then `No Marks (0)`; a design artifact that deserves partial credit adds a middle `Partial` group. Look at the retired `canvas/assignments/linear-equations-rubrics.tsv` for the exact shape.
- **Shape it to the test-driven parts.** Give real weight to the design artifacts, so the test-first work is rewarded, and keep autograded code correctness the largest single block. A workable default for a two-function problem: acceptance tests (about 20, with a partial level), pseudocode (about 15, with a partial level), each function's correctness (autograded, together about 40), the implementation criterion (about 5, see below), paper practice (about 10, completion), reflection (about 10, completion), summing to about 100. The summit is a separate extra-credit criterion (autograded).
- **Always include an implementation criterion** that makes the intended practice non-optional and blocks hardcoding. Its description states, in student-readable terms, that the solution must compute its answer generally rather than special-case the visible tests (hidden tests vary sizes and values, so a hardcoded or lookup-table answer fails), and, where the week's construct is the point, names that construct as the graded expectation: a loops-week value must be computed by iteration, not a pasted closed form; a conditionals-week status must branch on its inputs. This is the one place the course names the construct: the spec stays open about how to solve the problem, and the rubric closes the door on not solving it. The TA checks this criterion by reading the submitted code.
- **Grade the PDF artifacts on the page's terms.** Tests are graded on coverage and correct expected values; pseudocode on clarity and completeness; paper practice and reflection on completion. Code correctness is autograded on Gradescope and imported.
- The component prints "Total: N pts" by summing every criterion, so the extra-credit criterion is included in that total; that is expected.

## The Simulator

When the assignment is a **straightforward calculation** (the student computes a value from inputs), add a small Svelte simulator the student can plug numbers into, so they can build intuition and check their own answers. This is optional and belongs only where the calculation is simple enough to mirror in a reactive form; a problem with heavy branching or iteration usually does not get one.

- **Author a per-assignment component** in `src/components/<Name>Form.svelte`, mirroring the existing forms (`InjectionBurnForm`, `LinearEquationsForm`, `FinancialPlannerForm`): number inputs bound to the same quantities the functions take, reactive results using the same equations, and an error branch for the degenerate case (a zero denominator). The root element must carry both `not-content` and `print:hidden`: `print:hidden` so it never prints, and `not-content` so Starlight's content styles do not bleed in (without it, Starlight's adjacent-sibling margins land on the form's grid cells, inflate the row tracks, and misalign the inputs). Also give every input label a reserved uniform height (`min-h-[2.5rem]`, two lines) so a label that wraps to two lines does not make its grid row taller than its neighbor and push one input out of alignment. Verify the alignment by actually rendering the page, not by reading the markup; both failure modes are invisible in the source.
- **Place it** in a `## Simulators` section just before the rubric, rendered `client:load`, followed by the print-only note that tells a reader of the PDF the simulator lives online. It must be **non-printable**: the simulator itself is `print:hidden`, and the note is `print:block hidden`.
- The simulator never replaces the given equations or the work; it is a check, not a crutch, and it carries no part of the grade.

## Draft Policy

Set `draft: true` while an assignment is in development. **Never flip it to `false` yourself and never remove the key**; only the instructor publishes. A draft assignment is not a source of truth, and `schedule.mdx` must not link to it (a link to a draft page 404s). The schedule may name the assignment in prose while it is a draft; converting that name to a link happens only when the instructor publishes.

## Style

**Specification voice.** Direct and imperative. Use "must" for hard requirements (the contract) and "you may use any defensible approach" where the implementation is the student's choice, which is nearly everywhere. State what the program must produce, not how to produce it.

**No tutorial contamination and no pre-decomposition.** Requirements state the contract, not the steps and not the pieces. "First compute X, then Y," a named helper function, and "use this operator" all belong nowhere in an assignment: at most they are an optional hint the student may ignore.

**Plain, readable openings.** The situation paragraph is the part students most often find muddy, so write it for a first-year reader who may not speak English as a first language: short sentences, common words, the goal stated plainly. The story flavor never obscures the ask, and the ask is a clear goal, not a wall of numbered micro-requirements.

**No emdashes.** Never use the emdash character or a double hyphen as a dash. Use a colon, a semicolon, a comma, or a period.

## The Accuracy Pass

An assignment is not done until every piece of code and every claimed value on the page has actually been run and confirmed, in both languages.

- Write at least one full intended solution in Python and in Rust, run every acceptance case through it, and confirm each printed output or returned value matches the page.
- Run **HAB's claim** to ground: build the wrong approach the claim describes, confirm it really produces a wrong result on a case in the visible tests, and confirm a correct approach passes. This verification lives in your work log, never on the page.
- For any printed run shown to illustrate the harness, confirm each language's exact display, including the `f64` versus Python-float difference.
- Confirm the problem is genuinely solvable within the taught subset, and that the week's target construct really is the natural tool for it.
- Fix any mismatch before shipping. No claimed value ships unverified.

## Validation

After writing or editing an assignment, run:

```bash
npm run build
```

This runs `astro check` and the link validator. Fix every error before considering the assignment done.

## What Assignments Must NOT Contain

- No pre-decomposition: never name a helper function, never sketch the solution's internal steps, never tell the student which construct or operator to use. Fix the contract only. (The rubric's implementation criterion is the one sanctioned exception: it may name the week's construct as a graded expectation, because its job is to block hardcoding, not to design the solution.)
- No numbered problem *set*: one problem per assignment (rarely with a second entry point when the family is genuinely two independent outputs).
- No "modeling step / analysis step" labels on the page; that distinction is elicited in the reflection.
- No tutorial walkthrough; requirements state what the program must produce, not how.
- No assignment that *requires* a construct the course has not introduced by its due-date week; it must be solvable with the taught subset.
- No entry-point signature, function skeleton, or `todo!()` in an assignment due before the functions lecture; those fix the printed output instead.
- No acceptance test written as printed output when the grader checks a returned value (functions era), and none written as a return value when the grader checks printed output (pre-functions).
- No HAB code draft: HAB states a wrong claim in prose, never a function to fix. No giveaway on it: never state that it is wrong, name the flaw, hand a failing case, explain the fix, or attach a `<Reveal>`.
- No requiring the student to derive or solve mathematics: when the math is complex (a system of equations, an involved formula), give the solved equations in LaTeX and let the coding be the challenge; leave only simple arithmetic for the student to work out.
- No Rubber Duck Robotics or bare-exam framing: assignments are Mission Ares.
- No paper practice whose worked solution is withheld from the page; it goes in a closed `<Reveal>` for self-checking.
- No `Assignment N:` prefix in the page title; the number lives in `sidebar.order`.
- No reduced exam-adjacent assignment and no exam-rehearsal material in an assignment.
- No point values stated in the prose anywhere on the page; the summit is labeled `(extra credit)` and the prose says nothing about how many points anything is worth. Point values live only in the rubric TSV.
- No summit placed after the paper practice or reflection; it comes directly after the code step.
- No full acceptance-test table in the problem; give exactly one worked example, and the student writes the rest in Step 1.
- No skipping the test-first order: the tests and pseudocode come before the code, each in its own step.
- No missing rubric: every assignment ends with a `RubricTable` loading `canvas/assignments/<slug>-rubrics.tsv`.
- No printable simulator: when a simulator is present it is `print:hidden`, with the print-only note beside it.
- No lecture-only or other-course component imports (`MemoryStepper`, the visualizers, `AssignmentRequirements`, `AssignmentAICritique`); the assignment tier uses `RubricTable` and its own Svelte simulator only.
- No emdashes, and no pairing language; author for one individual crew member.
