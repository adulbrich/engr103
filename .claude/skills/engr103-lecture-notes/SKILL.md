---
name: engr103-lecture-notes
description: Use when creating or editing lecture notes (MDX files in src/content/docs/lectures/). Enforces the concept-first, dual-language, plain-language style of the ENGR 103 course. Always load this skill before writing or editing any lecture file.
---

# Lecture Notes Style Guide

This skill governs how lecture notes are written and revised for the ENGR 103 course website (Astro/Starlight, MDX format). ENGR 103 teaches every concept twice, once in Python and once in Rust, to a reader who has never programmed before. The lecture notes are the flipped material students read before class; they teach the concept and nothing else.

## What Lecture Notes Are For

Lecture notes are explanations in the Diataxis sense: understanding oriented material meant to deepen and connect what the student already knows. They are not tutorials (step by step tasks), not how-to guides (task oriented procedures), and not reference material (dry enumeration of facts).

A student reads a lecture note before a flipped class session, so the note has to stand entirely on its own. It motivates a concept, defines it in plain words, shows it working in both languages, and explains why it matters. It does not walk the student through building anything, and it does not summarize what class will cover; class assumes the reading already happened.

**Deepen understanding.** Explain why a concept exists and what problem it solves, not just what it is. A definition without a reason behind it is easy to forget.

**Make connections.** Tie a new concept to concepts from earlier lectures ("A variable's type behaves like the type of an expression from the previous lecture: it is fixed once chosen") and, where natural, to something the reader already knows from everyday life.

**Keep explanation bounded.** If a concept needs an example to land, give a short, self-contained example, not a walkthrough of a larger project. Resist the pull toward step-by-step instructions; that is a different genre and belongs elsewhere.

## Audience and the Plain-language Rule

Every reader is in their first year of college, has never written a program before, and may not speak English as a first language. Write for that reader on every page, every time.

- Use short sentences and common words. Avoid idioms ("under the hood," "boils down to") because they do not translate and do not teach.
- Define every technical term in plain language the first time it appears, then use that exact same name for that concept for the rest of the lecture, and for every later lecture. A function is always a function: never a "method," a "routine," or a "procedure." One name per concept, forever.
- Prefer the concrete word to the abstract one. Say "the loop repeats three times" before you say "the loop iterates."
- A sentence a first-year reader could misread is rewritten, not footnoted. If you catch yourself wanting a parenthetical to patch an ambiguous sentence, rewrite the sentence instead.

**Example of a first-use definition done well:**

> A **variable** is a name that points to a value stored in memory. Once you give a value a name, you can use that name anywhere later in your code instead of writing the value out again.

## The Dual-language Rule

Every ENGR 103 lecture teaches its concepts in Python and in Rust side by side, so students see the same idea expressed in a language with a fast, interactive interpreter and in a language with a compiler that checks more for you before the program ever runs.

Every concept section that introduces a new idea ends with two things, in this order:

1. A `<Tabs syncKey="lang">` component with one `<TabItem>` for Python and one for Rust, in that order. Each tab's example is self-contained and paste-runnable: a reader can copy just that block into a file and run it, with no earlier scaffolding required.
2. A `<WhatDiffers>` callout **only when something substantive differs beyond the standard boilerplate**. Fill the `differs` slot with what genuinely changed between the two renderings: a keyword, a spelling (`True` versus `true`), an operator's behavior, whether the code even compiles. It names **only** those differences; the shared concept is already in the prose above, so do not restate it. The standard boilerplate, Rust's `fn main() { ... }` wrapper and its `println!` with a `{}` placeholder versus Python's bare `print(...)`, is **not** a substantive difference: explain it once, when the two-language convention is first introduced to students, and never restate it in later examples or lectures. If that boilerplate is the *only* thing that differs between the two tabs, omit the `<WhatDiffers>` entirely and let the section end with the tabbed example alone.

`syncKey="lang"` keeps every `Tabs` component on a page (and across pages) switching together, so a reader who picks Rust once sees Rust everywhere.

**Example**, for a variable-assignment section:

````mdx
<Tabs syncKey="lang">
<TabItem label="Python">
```python title="Naming a value"
num_students = 10
print(num_students)
```
</TabItem>
<TabItem label="Rust">
```rust title="Naming a value"
fn main() {
    let num_students = 10;
    println!("{}", num_students);
}
```
</TabItem>
</Tabs>

<WhatDiffers>
<Fragment slot="differs">
Python needs no keyword to create a variable; writing a name and a value is enough. Rust requires `let` before the name, and wraps runnable code in a `fn main() { ... }` function; Python does not.
</Fragment>
</WhatDiffers>
````

Never end a concept section with only one language. Include the `<WhatDiffers>` whenever a real difference exists, because seeing where two notations diverge is what separates the concept from the syntax; skip it when the only difference is the boilerplate just described, so the callout never degrades into restating `fn main` and `println!` on every example.

One exception: some foundational concepts are genuinely language-agnostic, with no meaningful Python-versus-Rust difference to show (what a bit is, what a byte is, what a memory address is). Do not manufacture an artificial example just to satisfy the pattern. Teach such a concept in prose and let it share the tabbed example of the nearest section that does have a real dual-language difference (for example, group "bits" with "binary", whose literal syntax genuinely differs), rather than pinning a hollow `<Tabs>` onto a concept that reads identically in both languages.

## Show What Wrong Code Looks Like

Correct code shows what to do; wrong code shows why. Wherever a mistake teaches, include a short example of code that is wrong, and **mark the offending line with an inline comment** saying what is wrong and, when it is a compile error, that it will not compile.

This matters most in Rust, where the compiler is the teacher: showing the exact line the compiler rejects turns "Rust checks more for you" from a claim into something the reader can see. For example:

```rust title="Reassigning an immutable value (will not compile)"
let x = 5;
x = 6; // error: cannot assign twice to immutable variable `x` (Rust will not compile this)
```

Or a subtle logic mistake in either language:

```python title="Division: / versus //"
print(1 / 2)   # 0.5, not 0: Python's / always gives a float
print(1 // 2)  # 0: // is the whole-number division you probably meant
```

Keep bad examples short and always commented, so a reader skimming the code never mistakes the wrong line for the right way. Never show wrong code without the comment that flags it. Run the accuracy pass on bad examples too: confirm the compile error or the wrong output is really what the comment claims.

When the whole point of a dual-language contrast is that the *same* code one language accepts, the other rejects, put the identical code in both `<Tabs>` tabs and show the failing side failing. Do not substitute different, compiling code into the rejecting language's tab to keep it runnable: writing Rust's `3.0 + 2.5` where the lesson is that Rust rejects `3 + 2.5` hides the exact difference you are teaching, and gives that tab a title that no longer matches its code. This is the one case where a `<Tabs>` tab holds code that does not run, because its not running is the lesson. Title that tab with the same concept plus "(will not compile)", comment the offending line, and let the shared code make the contrast obvious.

## Code Block Titles

Every fenced code block carries a `title` attribute (Expressive Code syntax). This is required for accessibility: screen readers announce the title, and it labels the example for anyone scanning the page. A code block without a title is incomplete.

```python title="Area of a circle"
print(3.14159 * 2 * 2)
```

Titles are short and say what the snippet shows. A Python and a Rust block inside the same `<Tabs>` pair may share one title. For a bad-code example, name it as such, for example `title="Reassigning an immutable value (will not compile)"`.

## Terminal Commands and Output

When a lecture shows a command run in the terminal, or the output a program prints, use the terminal frame, and split the command a reader types from the output it produces into two separate blocks.

- **A command goes in its own block using a shell language** (```` ```bash ````), which Expressive Code renders as a terminal and gives the reader a copy button that copies exactly the command. **Never put a `$` or `#` prompt character in front of a command.** The prompt is not part of the command: including it means the copy button hands the reader a string that will not run, and a screen reader announces a meaningless symbol before every command.
- **The program's output goes in a separate block** marked ```` ```text frame="terminal" ````, so it reads as a terminal result and is never mixed in with the command that produced it. The reader can then copy the command on its own, without dragging the output along.

Wrong, a prompt character with the command and its output mixed into one block:

````text
```text
$ python3 area.py
78.5
```
````

Right, the command copyable on its own and the output separate and terminal-framed:

````text
```bash title="Run the program"
python3 area.py
```

```text frame="terminal" title="Output"
78.5
```
````

This applies to every command (`python3 ...`, `rustc ...`, `./program`) and every captured run of a program, error output included. Source-code examples that are not terminal sessions keep their normal `python`/`rust` code blocks inside `<Tabs>`.

## Staying Within What Has Been Taught

Every ENGR 103 example should draw only on constructs the course has already introduced by that lecture's week, because a reader following the course in order cannot understand a construct they have not met yet. The language ladder at `/reference/language-ladder` records what has been introduced by each week and grows one week at a time; a construct introduced in week 3 stays available in every later week. This is a teaching-quality rule, not a restriction on what students may write: the ladder is a recommendation for them, but a lecture that leans on an untaught construct simply fails to teach.

Before writing an example, check the ladder for the lecture's week. If a natural example would need a construct not yet introduced (a dictionary in a week 4 lecture, say), simplify the example instead of reaching ahead: an example the reader cannot yet follow does not teach. Link `/reference/language-ladder` in prose the first time a lecture depends on knowing what is and is not available yet; do not restate the ladder's table.

## Legibility: Rest Only on What Has Been Taught

Staying within taught constructs governs syntax; this rule governs concepts. Every sentence of a lecture must be understandable by a student who has read only the lectures up to this one. An explanation may rest only on concepts already introduced, whether on the ladder or in an earlier lecture; it must never reach forward into a later lecture to justify something here.

This is the trap that "it is factually true" hides. A statement can be perfectly correct and still illegible if it depends on a concept the student has not met. For example, in an early lecture, calling Rust's `i32` "a 32-bit integer" is true but illegible if bits and bytes have not been taught yet: the reader cannot decode the "32". When a construct's name carries a later concept and that concept has not been taught, either the concept belongs in this lecture (teach it, where the schedule places the foundation) or you name the construct as a plain label and defer its meaning with one short forward pointer ("we will see what the 32 means when we reach how numbers are stored"). Prefer teaching it where the schedule already grounds it.

Two checks before a lecture is done:

- Read it as a student who has seen only the earlier lectures, and flag any sentence that assumes a not-yet-taught concept.
- For every technical term, and every digit or size baked into a type name, confirm the reader has the background to understand it here.

The accuracy pass verifies the code is correct; this legibility pass verifies the prose is understandable given only what has been taught. Both are required.

## Glossary Discipline

The lecture is the canonical home for a term's definition. The first time a lecture defines a term, give it a stable `##` or `###` heading (or make it the clear subject of one) so the definition has a durable anchor, for example `## Variables` or `### Assignment`.

The site's glossary page links to that anchor and gives only a one-sentence plain-language gloss; it does not restate the lecture's full explanation. When writing a lecture, do not duplicate a definition that another lecture already owns: reference the earlier lecture by name in prose ("A function's parameters work the same way as the variables introduced in the Variables lecture") instead of redefining the term.

## Concepts and Dual-language Only: Fully Story-agnostic

ENGR 103's lectures are completely story-agnostic. A lecture teaches its concept in Python and in Rust and says nothing about where or how that concept is later exercised.

This means a lecture never mentions:

- Git, the shell, an editor, or any other tooling.
- Any story, problem family, assignment, activity, or recitation, by name or by implication.
- A single running scenario threaded through the whole lecture. Use independent, self-contained examples per section instead; a scenario that persists across sections quietly turns the lecture into a tutorial.

There is no "name the problem family where this will be exercised" clause in this course's lecture notes, even though that pattern exists in adjacent course styles. That omission is deliberate and overrides any older guidance that suggested otherwise: the narrative and assessment structure belongs entirely to the activity, assignment, and recitation tiers. The lecture's job is to teach the concept, full stop.

## Structure

### Language Versions on the Page

Every lecture body opens with a `<LanguageVersions />` component, placed at the very top of the body, before the opening problem statement and before any other prose or component. It tells the reader exactly which Python and which Rust version the lecture's examples were written and verified against.

The versions it displays come from a single source, `src/data/languageVersions.ts`. A lecture's prose never states a Python or Rust version directly; if a version number needs to appear at all, `<LanguageVersions />` is how it appears. This keeps every lecture on the site in agreement automatically: updating the one file updates every lecture at once, and no lecture can drift out of sync with another by hard-coding a number that then goes stale.

### Opening Paragraph (The Problem Statement)

Every lecture opens with one or two paragraphs establishing why the concept matters, using a concrete, relatable problem, not a course-mechanics problem. The opening should:

- Present a real difficulty the concept solves, in plain, concrete terms.
- Never say "in this lecture we will..." or otherwise describe the lecture instead of the problem.
- Never name a story, assignment, activity, or recitation (see above).
- Be specific to this lecture. Never reuse a neighboring lecture's opening hook, and never let two adjacent lectures begin the same way. If the lecture before this one already used a problem, open on a different door into your own subject.

**Example of a strong opening:**

> Every program you write eventually needs to remember something: a count, a name, a running total. Without a way to store and reuse a value, you would have to retype it everywhere it is needed, and update every copy by hand whenever it changes. Variables solve this problem by giving a value a name you can use and update from anywhere in your code.

**Weak openings to avoid:**

- "In this lecture we will cover variables and assignment."
- "Variables are an important topic in programming."

### Section Structure and Flow

Every `##` and `###` section leads with prose before any code block, table, or component; never open a section with a `<Tabs>`, a code block, or a table.

That lead-in must **teach or connect, never announce**. Do not begin a section by describing what the section is about. Sentences like "This section names the smallest unit a computer can store," "In this section we look at...," or "This part covers..." are banned: the heading already says what the section is, so the first sentence must do real work, either stating the section's first substantive idea or carrying the thread forward from the section before it. A reader who deleted every heading should still read one continuous, flowing explanation, not a stack of blocks that each introduce themselves.

**One heading per concept; never echo a heading inside itself.** A concept gets a single heading at the right level. Never place a `### X` immediately inside a `## X` of the same or nearly the same name: a `## Values` whose only child is `### Values` is wrong. Put the explanation directly under the one heading. Use a `##` parent only when it genuinely groups two or more *distinct* `###` concepts (for example `## Types` over `### Static typing` and `### Dynamic typing`), and even then the parent's own prose must teach or transition, not restate the heading.

Weak section openers to avoid, both of which only announce:

- "This section names the basic unit of data and the kinds it comes in."
- "This section looks at what each language does when the types do not match."

Strong section openers, which teach or connect immediately:

- "A bit is the smallest piece of information a computer can store: a single 0 or a single 1."
- "Knowing that everything is stored as bits is only half the picture; the other half is where those bits actually live."

### Depth: Teach the Mechanism, Not the Shape

The most common failure of a lecture is staying on the surface: it defines a term, shows the syntax in both languages, and moves on. That teaches a reader what a thing is called and how to spell it, not how it works or why. Every concept must go deeper than its shape. The bar: a reader should finish a section able to explain how the thing works and why it behaves as it does, not merely name it.

For each concept, the prose must do all of the following (as flowing explanation, never as a checklist on the page):

1. **Define it precisely** in plain language. Where a term has a real technical meaning (an interpreter, a stack frame, a `Result`), give the real definition, not a vague gesture at it.
2. **Explain the mechanism:** what actually happens, step by step, underneath the behavior. Not "the interpreter runs your code" but how it runs it, in what order, and what it does at each step. Where it helps, connect down to what an earlier lecture established (bits and bytes, memory, the call stack).
3. **Walk one concrete example through, step by step.** Trace a specific input to a specific result, naming what happens at each step, rather than only showing code and stating its output.
4. **Show how to actually run it, and what the tools print.** A reader must be able to reproduce it: give the real command (`python3 area.py`; `rustc area.rs` then `./area`) and the exact output, including the error text when the point of the example is an error.
5. **Name the why and the failure mode:** what the concept makes possible, and what specifically goes wrong without it or when it is misused.

The discriminating test for every section, applied honestly: **does a reader come away understanding how and why this works, or just what it is called and its syntax?** If it is the latter, the section is not done, no matter how cleanly it reads.

**Dual-language does not mean saying everything twice.** The deep material, the mechanism, the why, the step-by-step trace, the connection to memory or hardware, is almost always language-agnostic: it belongs in rich prose (and asides), written once. The `<Tabs>` pair carries only the two syntaxes for the same idea. This is how a lecture reaches real depth without the two-language format consuming all its room: go deep in shared prose, and let the tabs stay short.

### Depth Through Asides

Starlight callout asides are the tool for depth that would otherwise break the main thread: an under-the-hood detail, a common gotcha, a boundary case, a "what really happens here" that a first read does not need but a curious reader wants. Use them.

```mdx
:::note[What actually happens]
The interpreter does not read the whole file first; it ...
:::

:::caution[A common mistake]
Forgetting to ... produces ...
:::
```

Reach for `:::note`, `:::tip`, and `:::caution` to hold mechanism, gotchas, and deeper dives that would clutter the main line of explanation. A short, genuinely interesting piece of real-world or historical context (why a language works the way it does) is welcome as optional seasoning, but it is not what "depth" means and never substitutes for explaining the mechanism. Being story-agnostic (below) forbids the course's problem-family narratives, not rich technical context or real examples.

### Prose Over Lists

Favor flowing prose over bullet lists. Reach for a list only to enumerate genuinely parallel, discrete items (for example, the rules for a valid identifier) or to compare options side by side. When an idea can be explained in a paragraph, write the paragraph; do not fragment it into bullets for the sake of tidiness.

### Takeaways

The penultimate section of every lecture is titled `## Takeaways`. It is prose, not bullets, and it synthesizes the lecture's concepts into one coherent picture: how the ideas in this lecture fit together, and what the reader can now reason about that they could not before. It may name adjacent lectures in prose to orient the reader ("the next lecture builds on this to introduce functions"), but never links to them.

### Resources (optional, final section)

A lecture may end with an optional `## Resources` section: an unordered list of external links (official documentation, articles, videos) where a reader can encounter the same concept through a different explanation. Do not duplicate links already given inline in the lecture's prose. If a lecture has nothing extra worth pointing to, omit the section entirely.

## Lecture Frontmatter and the `ai-summary` Block

### Frontmatter

```yaml
---
title: "Descriptive Title"
description: "One-sentence summary of what this lecture covers and why it matters."
sidebar:
  order: <number>
---
```

`order` fixes the lecture's place in the sidebar and its sequence relative to neighboring lectures.

### The `ai-summary` block

Immediately after the imports, every lecture carries an `ai-summary` MDX comment: a terse, machine-parseable record of coverage and cross-page relationships, hidden from students.

````mdx
{/* ai-summary
type: lecture
slug: <filename without .mdx>
order: <sidebar.order>
covers: <main concepts, semicolon-separated, one line, under 200 chars>
prereq_lectures: <comma-separated slugs, or empty>
followup_lectures: <comma-separated slugs, or empty>
glossary_terms: <comma-separated terms this lecture defines canonically, or empty>
*/}
````

Rules:

- `covers` is a one-line, semicolon-separated list of the lecture's main concepts, not a table of contents.
- Every slug in `prereq_lectures` and `followup_lectures` must be the real filename (without `.mdx`) of another lecture. Every term in `glossary_terms` must be a term this lecture actually defines at a stable heading anchor (see Glossary Discipline).
- This block carries no story field, no problem-family field, and no assignment or activity field. Those concepts do not exist at the lecture layer.
- Leave a field blank rather than guessing a value.

**Example** (`variables-and-state.mdx`):

````mdx
{/* ai-summary
type: lecture
slug: variables-and-state
order: 3
covers: names vs values; assignment and rebinding; let and let mut; shadowing; constants; tracing a variable table
prereq_lectures: values-types-expressions
followup_lectures: functions-i
glossary_terms: variable, assignment, shadowing, constant
*/}
````

## Imports

Import only the components a lecture actually uses.

```mdx
import { Aside, Tabs, TabItem } from '@astrojs/starlight/components';
import LanguageVersions from '/src/components/LanguageVersions.astro';
import WhatDiffers from '/src/components/WhatDiffers.astro';
import Latex from '/src/components/Latex.astro';
import AsciiTable from '/src/components/AsciiTable.astro';
import MemoryPoolDiagram from '/src/components/MemoryPoolDiagram.astro';
import VariableLifecycleDiagram from '/src/components/VariableLifecycleDiagram.astro';
```

- **`LanguageVersions`**: every lecture imports and renders this at the top of its body, before the opening problem statement. See Language Versions on the Page, above.
- **`Tabs` / `TabItem`**: the dual-language example pair. Always `syncKey="lang"`, always Python first, Rust second.
- **`WhatDiffers`**: the callout that follows every dual-language example pair. Use the `differs` and `cannot` slots as shown above.
- **`Latex`**: for mathematical notation, when a concept genuinely needs it (floating-point representation, for example).
- **`AsciiTable`**: for illustrating memory or character layouts as fixed-width grids.
- **`MemoryPoolDiagram`** and **`VariableLifecycleDiagram`**: the static diagrams for teaching the memory model; use only in lectures that cover memory, ownership, or variable lifetime.
- **`Aside`**: brief tips, notes, or cautions, each with a `title` attribute. Do not overuse; central ideas belong in the main prose, not in a callout.

Do not import any assignment, activity, or recitation component. None exist for lecture notes, and none should ever be added here.

## Writing Style

- **No emdashes.** Never use the emdash character or the double-hyphen convention in place of one. Use a colon, a semicolon, a comma, a period, or parentheses instead.
- **Active voice.** "The loop repeats the block" rather than "the block is repeated by the loop."
- **Second person.** Address the reader directly: "you will see," "you can check," "your next step is."
- **Present tense for always-true concepts.** "A variable stores a value" rather than "a variable will store a value." Use future tense only for what the reader will be able to do: "by the end of this lecture, you will be able to trace a variable's value by hand."
- **Define acronyms at first use.** "CPU (central processing unit)," then "CPU" afterward.
- **Tone.** Gentle and encouraging, calmer and plainer than a confident-expert voice aimed at a peer. The reader is nervous and new; the writing should never sound impatient or assume prior exposure, while still always explaining the why behind a rule, not just the rule itself.

**Good:** "In Python a value carries its type; the name does not. `x = 10` makes `x` refer to an integer; a later `x = "hi"` rebinds `x` to a string. The type travels with the value, which is why Python is dynamically typed."

**Too formal:** "Variables in Python are subject to dynamic type binding, wherein the type association is determined by the bound value rather than by a static declaration."

**Too casual:** "Python variables are pretty chill about types, they just kind of go with whatever you give them."

## Length Calibration

Length is an output of teaching a concept properly, never a target to hit. Teach the mechanism, walk one example through step by step, show how to run it and what it prints, and name the failure mode (see "Depth" above); the length is then whatever that takes. Do not pad to reach a number, and never cut mechanism to stay under one.

As a rough sanity check only, a flipped ENGR 103 note tends to land around **2,500 to 4,500 words** including code. Landing well under that is the warning sign to watch for: it almost always means a concept was flattened, its mechanism skipped, or an example left un-traced, and the fix is to deepen it, not to stretch it with filler. Landing well over usually means genuine tutorial hand-holding or problem-family story material has crept in and should move out. Run `wc -w src/content/docs/lectures/<file>.mdx` after a substantial edit to catch the thin case, not to enforce a ceiling.

## The Authoritative Accuracy Pass

A lecture is not done until it has passed this check, and this check replaces any weaker "factual currency" habit from other courses: before a lecture ships, verify every factual claim it makes and execute every code example it shows.

- **Verify every factual claim against official documentation.** For Python, that means docs.python.org; for Rust, that means doc.rust-lang.org. If the lecture states how a language feature behaves, what a function returns, what a keyword does, or what version introduced something, confirm that statement against the official docs before trusting it.
- **Execute every code example in both languages.** Run each Python example with `python3` and each Rust example, as a single file, with `rustc`. Do this for every `<Tabs syncKey="lang">` pair in the lecture, not just the ones that look risky.
- **Compare real output to claimed output.** Whatever the prose says the example prints, returns, evaluates to, or raises as an error, check it against what actually happened when you ran it: the real output, the real value, the real type, the real error message.
- **Fix any mismatch.** If the text and the real behavior disagree, the text is wrong; correct the prose or the example until they agree.

No claimed output or behavior ships unverified. A lecture that has not been run through this accuracy pass is not finished, regardless of how polished its prose reads.

## Validation

After writing or editing a lecture, run:

```bash
npm run build
```

This runs `astro check` and the link validator, catching MDX syntax errors, bad imports, malformed component usage, and broken links. Fix every error before considering the lecture done. This site does not exclude drafts from the build: there is no `draft: true` flag to reach for, so a new lecture is simply added, built cleanly, and reviewed like any other page.

## What Lecture Notes Must NOT Contain

- **No tooling.** No git, shell, editor setup, or other environment instructions. That material belongs elsewhere on the site.
- **No stories, problem families, assignments, activities, or recitations,** by name or by implication. See Concepts and Dual-language Only, above.
- **No tutorial walkthroughs.** A lecture explains a concept and illustrates it with short, self-contained examples; it does not guide the reader step by step through building something.
- **No single running scenario threaded through the whole lecture.** Independent examples per section only.
- **No example the reader cannot yet follow.** Every example stays inside the constructs the course has introduced by that lecture's week, so a reader in sequence can understand it.
