---
name: engr103-activities
description: Use when creating or editing activity files (MDX in src/content/docs/activities/). Enforces the guided, dual-language, reveal-based structure of the ENGR 103 in-class activities. Always load this skill before writing or editing any activity file.
---

# Activity Style Guide

This skill governs how activities are written and revised for the ENGR 103 course website (Astro/Starlight, MDX format). Activities are always paired with exactly one lecture. They are the guided first altitude of a problem family: the easiest sibling of an assignment problem, which is the easier sibling of a recitation problem, which is the ancestor of an exam variation.

## What an Activity Is

An activity is the guided, hands-on companion to one lecture, built to fill a 50-minute in-class session shaped like this:

- **5 minutes: a misconception opener**, drawn live from the pre-lecture check question students missed most. This happens in the room and is not authored on the activity page.
- **Three rotating blocks of about 13 minutes each.** Each block poses a problem at guided difficulty, the student attempts it, then it is resolved. These three blocks are the core of what the activity page authors.
- **A short write-it-yourself problem.** The student authors a small solution from a blank page, the session's one rep at producing code rather than reading it. The activity page authors this as a required section.
- **2 minutes: a bridge** naming the exact recitation problem family this session fed. The activity page authors this too, as a short final section.

The authored page opens with a brief **warm-up** before the three blocks: a quick refresher of the lecture's simplest building blocks, so a student who read the notes days ago, or is catching up alone after missing class, has the pieces fresh before the guided thinking starts.

An activity carries no story. Assignments dress their problems in the Rubber Duck Robotics story line and recitations and exams in Mission Ares, but an activity states the bare concept as fast as it can be read: no costume, no narrative thread, no plot to track. Like a lecture, an activity is dual-language: every example that can show both Python and Rust does so.

Every activity is paired with exactly one lecture, named in its `ai-summary` block as `paired_lecture`. The activity does not re-teach that lecture's concepts; it gives the student a first guided rep at applying them.

## Works in Class and Alone

The same page serves two audiences without changing a word.

In class, the instructor poses each block's problem, the student attempts it, and the instructor resolves it live while the page's `<Reveal>` for that block stays closed. A student who missed class opens the `<Reveal>` after attempting the problem and self-checks against the same resolution the instructor would have given live. One artifact, two audiences: nothing on the page is written differently for the two settings.

State this plainly, because it is easy to get wrong by habit carried over from other courses: **no pairing is ever mentioned on an activity page.** Whether a student works solo or with a partner is the instructor's in-the-room call, not a fact about the content. Every prompt is authored for one individual: "you predict...", "you run...", "you trace...". Never write "with a partner," "in pairs," or "discuss with your neighbor" anywhere on the page.

## The `<Reveal>` Component

`<Reveal>` is a small Astro component wrapping a native `<details>`/`<summary>` element: zero-JS, closed by default, keyboard-operable, and forced open under `print:` so a printed activity still shows its resolutions.

```mdx
import Reveal from '/src/components/Reveal.astro';
```

It takes one optional prop, `label` (string), the summary text shown before the reveal is opened. Set a pedagogically apt label for every block rather than leaving the generic default; the label should name what is behind the panel, not just say "answer."

````mdx
<Reveal label="Reveal the real output">
```python title="Evaluating the expression"
print(10 / 4 + 3 * 2)
```

```text frame="terminal" title="Output"
8.5
```

Multiplication and the `/` operator run before the addition, and Python's `/` always produces a float, so the sum lands on `8.5`, not an integer.
</Reveal>
````

Every block on the page follows the same shape: prompt, then a `<Reveal>` holding the resolution. A reveal's contents are the actual answer, never a hint or a partial nudge: the real output of a predict-then-run, the fix and the one-line reason for a debugging block, the correct choice and why each distractor is wrong for peer instruction, the completed trace for paper tracing, and a sample correct solution for a write-it-yourself block. Because `<Reveal>` renders arbitrary MDX children, a resolution can itself be dual-language: a `<Tabs syncKey="lang">` pair and a `<WhatDiffers>` can live inside it exactly as they would in a lecture.

## Reused Components

Activities import only what they use, and only from this list. Do not import a lecture-only or assignment-only component; none of those exist for the activity tier.

```mdx
import { Aside, Steps, Tabs, TabItem } from '@astrojs/starlight/components';
import Reveal from '/src/components/Reveal.astro';
import LanguageVersions from '/src/components/LanguageVersions.astro';
import WhatDiffers from '/src/components/WhatDiffers.astro';
import Latex from '/src/components/Latex.astro';
```

- **`Reveal`**: the resolution panel for every block. See above.
- **`Tabs` / `TabItem`**: the dual-language pair, always `syncKey="lang"`, Python first and Rust second, exactly as in lecture notes. Use it whenever a block's prompt or its reveal shows the same idea in both languages.
- **`LanguageVersions`**: place at the very top of the body, before the opening paragraph, exactly as a lecture does. It states the Python and Rust versions the activity's code was written and verified against, sourced from `src/data/languageVersions.ts`.
- **`WhatDiffers`**: use inside a `<Reveal>` when a same-program-two-languages block's resolution needs to name a real difference between the tabs, beyond the standard `fn main() { ... }` and `println!` boilerplate. Follow the same rule as lecture notes: omit it if boilerplate is the only difference.
- **`Aside`**: a brief tip, note, or caution about the immediate block, always with a `title`. Use sparingly; it is not a place to smuggle in concept explanation that belongs in the lecture.
- **`Steps`**: use inside a block's prompt only when the student's attempt is genuinely a short numbered sequence (for example, the steps of a paper trace). Most prompts are a short code block or a `<Tabs>` pair and need no `<Steps>` at all.
- **`Latex`**: for mathematical notation, when a prompt or a reveal genuinely needs it.

## The Five Rotating Block Types

An activity picks three of the following five for its rotating blocks (a separate, always-present write-it-yourself block is described just below). Each is authored as *prompt, then `<Reveal>`*.

**Predict-then-run.** Show a short program and ask the student to commit to its output before running it, on paper or in a comment. The `<Reveal>` shows the real output and reconciles the gap between what was predicted and what happened. This type fits best where the true result surprises: operator precedence, integer versus float division, floating-point equality.

**Same program, two languages.** Present the same concept in Python and Rust in a `<Tabs syncKey="lang">` pair and ask the student to name what changed between the two renderings and what could not change. The `<Reveal>` gives the answer, usually with a `<WhatDiffers>` naming the real difference.

**Live debugging.** Show a program with one planted bug and ask the student to diagnose it using the systematic method: read the error, form a hypothesis, test it. The `<Reveal>` shows the diagnosis and the one-line fix, plus the exact error message the broken program produces.

**Peer instruction.** Pose a concept question with distractor answers mined from likely misconceptions, ideally ones a prior recitation actually produced. The `<Reveal>` gives the correct answer and, for each distractor, the one-line reason it is wrong.

**Paper tracing.** Ask the student to trace an exam archetype by hand: a variable table, a call stack, an expression reduction. The `<Reveal>` shows the completed trace, step by step, matching exactly what a careful hand-trace would produce.

## The Write-It-Yourself Block

In addition to the three rotating blocks, every activity carries one **write-it-yourself** block: a small task the student authors from a blank page. It is the session's single rep at *producing* code, rather than predicting, comparing, choosing, tracing, or fixing code that is given. It is the "now you make one" that the other blocks set up, so it always comes last among the problem sections, right before the bridge.

Keep it **small and genuine**. Small: one short task, a few lines, sized to the last minutes of the session. Genuine: the student must build something new with the week's tools, never retype a lecture example. Ask for a computation the lecture's constructs can express but did not already spell out, so producing it is real work and not recall.

It is **ladder-adaptive**. Before the Functions lecture, "write the solution" means a short program that computes and prints its result; from the Functions lecture on, it means a small function the student defines against a stated signature. It never reaches ahead of the paired lecture's week, exactly like every other example on the page.

The prompt is one or two sentences naming the task, then a `<Reveal label="Reveal a sample solution">` holding **one correct solution in both languages** (a `<Tabs syncKey="lang">` pair, Python then Rust) and its real output. Because more than one solution can be correct, the reveal shows *a sample* solution, not "the answer": say so, so a student whose own working program differs from the sample does not conclude theirs is wrong.

This block is a **required structural section**, like the warm-up and the bridge, not one of the three rotating blocks. It is not listed in the `block_types` field.

## Difficulty and Progression

An activity is a first guided rep, but a rep worth doing. The warm-up reproduces the lecture's simplest building blocks so the pieces are fresh. The three sections must then **go beyond reproducing a lecture example**. A section that just re-runs code the student already read in the notes teaches nothing new and wastes the session: if a student who read the lecture could answer it without thinking because they saw this exact example, it is too easy.

So each section shows the lecture pattern and then **raises the ceiling on it**: combine two ideas the lecture introduced separately, choose inputs where the right answer is genuinely not obvious, add a realistic wrinkle, or push the pattern one step further than the notes did. A predict-then-run should land on a result that actually surprises, not one lifted from a lecture example. A debugging block should plant a bug the student has to reason through, in fresh code, not a snippet copied from the notes. A paper trace should trace a case with enough moving parts to be worth doing by hand. A two-language block should contrast a case where the difference bites, not a trivial one.

Stay inside the paired lecture's ladder: raise the difficulty by combining and extending what has been taught, never by reaching for a construct from a later week. The goal is a real stretch within the concepts the student now has, so that finishing the activity means they can do more than repeat the notes.

## Frontmatter and the `ai-summary` Block

### Frontmatter

```yaml
---
title: "The paired lecture's name"
description: "One sentence describing what the student will do and end up able to do."
sidebar:
  order: <number matching the paired lecture's order>
draft: true
---
```

The activity's `title` is simply the **name of its paired lecture** (for example `Functions`, `Loops`, `Booleans and Conditionals`). Do not invent an action-oriented title per activity: naming every activity after the block types it happens to use ("Predicting and Fixing...", "Tracing and Fixing...") only repeats a handful of verbs across the sidebar and tells the reader nothing the lecture name does not. One activity pairs with one lecture, so the lecture's name identifies it cleanly.

### The `ai-summary` block

Immediately after the imports, every activity carries an `ai-summary` MDX comment, hidden from students:

````mdx
{/* ai-summary
type: activity
slug: <filename without .mdx>
order: <sidebar.order, matching the paired lecture>
paired_lecture: <one real lecture slug>
practices: <concepts from the paired lecture, phrased as what the student does; semicolon-separated; one line>
prereq_activities: <comma-separated slugs, or empty>
block_types: <the three chosen block types, comma-separated>
output: <what the student ends up having predicted, fixed, or traced, one clause>
*/}
````

Rules:

- `paired_lecture` is required and must be the real slug of exactly one lecture.
- `practices` overlaps with the paired lecture's `covers` field but is phrased as student action, not lecture exposition: "predicts the result of mixed integer and float division," not "integer versus float division."
- `block_types` must name exactly the three **rotating** block types the page actually uses, one of: `predict-then-run`, `same-program-two-languages`, `live-debugging`, `peer-instruction`, `paper-tracing`. The always-present write-it-yourself block is structural and is not listed here.
- Leave a field blank rather than guessing a value.

**Example** (illustrative, for an expressions-and-operators activity):

````mdx
{/* ai-summary
type: activity
slug: expressions-and-operators
order: 3
paired_lecture: expressions-and-operators
practices: predicts the value of a mixed-operator expression before running it; names what changes between Python's // and Rust's integer / across languages; diagnoses a silent precedence bug from a wrong result
prereq_activities:
block_types: predict-then-run, same-program-two-languages, live-debugging
output: a corrected expression and a reconciled prediction for each of the three blocks
*/}
````

## Page Structure

1. **`<LanguageVersions />`** at the very top of the body, before any other prose.
2. **Opening paragraph** (no heading, 2 to 3 sentences): names the paired lecture with a link, says what the student will do across the session, and states what they will be able to do by the end. It does not explain any concept; that is the lecture's job.
3. **Warm-up** (a short `##` section): a quick refresher of the lecture's simplest building blocks, before the guided thinking starts. Show a handful of the smallest examples the lecture rests on, in both languages, with their output, and stop there: no `<Reveal>` (a warm-up is a reminder, not a challenge) and no new concept or exposition. For an expressions activity, for instance, this is a few bare literals of each kind (a whole number like `5`, a negative like `-1`, a decimal like `0.345`, a string like `"duck"`, a boolean like `True`/`true`) and a couple of simple operators (`2 + 3`, `4 - 1`). Keep it to a `<Tabs syncKey="lang">` pair and its output.
4. **Three `##` sections**, separated by `---`, one per chosen block type. Give each a short **descriptive heading that says what the student does**, for example `## Predict the Value`, `## The Same Division, Two Languages`, or `## Find the Silent Bug`. Never title a section `Block 1`, `Block 2`, or any other generic label: the heading names the task, not its slot in the session. Each section opens with 1 to 2 sentences of framing (what problem it poses, and why it matters, practically, not conceptually), then the prompt, then the `<Reveal>` holding the resolution.
5. **Write It Yourself** (a required `##` section, after the three rotating blocks and before the bridge): the write-it-yourself block described above. Give it a short descriptive heading that names the task (for example `## Write the Sol Clock` or `## Write a Tolerance Check`, never a generic label). One or two sentences of spec, then a `<Reveal label="Reveal a sample solution">` holding a sample solution in both languages and its real output. Ladder-adaptive: a short program before the Functions lecture, a small function from the Functions lecture on.
6. **Bridge** (final short `##` section): one or two sentences naming which recitation problem family this session feeds, **by its concept description, not a code name**. The families do not carry names; refer to each by what it does, for example the arithmetic-with-units-and-time family or the decision-tables-and-interlocks family.

There is no penultimate artifact section and no closing "further reading" section on an activity page: those are cs312 tutorial conventions built around a build-something walkthrough, and an ENGR 103 activity is a bare-concept, participation-only session, not a build-an-artifact exercise.

**Example opening paragraph:**

```text
This activity puts into practice the [Expressions and Operators](/lectures/expressions-and-operators/) lecture.
You will predict, run, and fix short expressions in Python and Rust.
By the end, you will be able to evaluate a mixed-operator expression by hand and explain why integer and float division give different results.
```

## Draft Policy

Set `draft: true` while an activity is being developed. Only the instructor removes it; never remove it yourself, and never treat a draft activity as a source of truth for another page to link against.

Activities are not listed individually in `schedule.mdx`. An activity inherits its week from its paired lecture's `sidebar.order`; do not add a per-activity row to the schedule.

## Style

**Ruthless minimal explanation.** The lecture owns exposition. An activity visits a concept hands-on with at most a one-sentence context clue; it never repeats a concept the lecture already explained. If a sentence of framing starts to read like a paragraph of the lecture, cut it.

**Tutorial voice.** Second person, present tense, short active sentences: "You predict the output," "You run the program," "You trace the call stack." One idea per sentence.

**No emdashes.** Never use the emdash character or a double hyphen as a dash. Use a colon, a semicolon, a comma, or a period instead.

**Solvable with what was just taught.** An activity should stay within the constructs the paired lecture's week has introduced, so it is a fair first rep the student can actually do with what they have just learned. Do not reach ahead for a construct a later lecture teaches, even if it would make a prompt shorter. The [language ladder](/reference/language-ladder/) records what has been introduced by each week; it is a recommendation for students, but the activity itself must be doable within the taught subset.

## Word Band

An activity page runs roughly **1,800 to 4,000 words**. Depth comes first: the three sections have to genuinely stretch the student (see Difficulty and Progression), and a more involved example, with its prompt, its reveal, and the reasoning a solo student needs, takes room. Do not clip a good, harder example to hit a lower number, and do not pad a thin one to look substantial. Landing well under this band usually means the sections only reproduced lecture patterns instead of extending them, which is the exact failure to avoid; landing well over usually means lecture-style exposition crept in that belongs in the paired lecture. Run `wc -w src/content/docs/activities/<file>.mdx` after a substantial edit and use it to catch those two drifts, not to pad or trim toward a number.

## The Accuracy Pass

An activity is not done until every code block and every claimed output on the page, including a planted bug's exact error message, has actually been run and confirmed.

- Run every Python example with `python3` (version 3.14).
- Run every Rust example, as a single file, with `rustc` (version 1.88).
- Compare the real output, value, or error text to whatever the reveal claims. A reveal's job is to show the student the truth; it cannot do that with an invented or approximated output.
- Fix any mismatch between the page and what actually ran before shipping.

No claimed output ships unverified. A `<Reveal>` that states an output the accuracy pass has not confirmed is not finished.

## Validation

After writing or editing an activity, run:

```bash
npm run build
```

This runs `astro check` and the link validator. Fix every error before considering the activity done.

## What Activities Must NOT Contain

- No story or narrative costume of any kind; bare concept only.
- No mention of pairing, partners, or discussing with a neighbor.
- No conceptual deep dives; those belong in the paired lecture.
- No penultimate artifact-to-show section and no closing further-reading section.
- No reference to grading mechanics, submission, or assignment deliverables (naming the recitation problem family in the Bridge is expected and is not this).
- No prompt that *requires* a construct the paired lecture's week has not yet introduced; the activity must be doable with the taught subset (a student may still use more if they know it).
- No lecture-only or assignment-only component imports.
- No generic section headings (`Block 1`, `Block 2`); every section is titled by what the student does.
- No section that only reproduces a lecture example without extending it (see Difficulty and Progression).
- No activity missing its required write-it-yourself section, and no write-it task that only retypes a lecture example instead of asking the student to produce something new.
