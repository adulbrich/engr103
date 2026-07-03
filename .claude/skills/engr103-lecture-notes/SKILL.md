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
2. A `<WhatDiffers>` callout, filling the `differs` slot with what changed between the two renderings (syntax, keywords, what the compiler checks) and the `cannot` slot with what stayed the same no matter which language you read (the concept itself, the order things happen in, the underlying idea).

`syncKey="lang"` keeps every `Tabs` component on a page (and across pages) switching together, so a reader who picks Rust once sees Rust everywhere.

**Example**, for a variable-assignment section:

````mdx
<Tabs syncKey="lang">
<TabItem label="Python">
```python
num_students = 10
print(num_students)
```
</TabItem>
<TabItem label="Rust">
```rust
fn main() {
    let num_students = 10;
    println!("{}", num_students);
}
```
</TabItem>
</Tabs>

<WhatDiffers>
<Fragment slot="differs">
Python needs no keyword to create a variable; writing a name and a value is enough. Rust requires `let` before the name. Rust also wraps runnable code in a `fn main() { ... }` function; Python does not.
</Fragment>
<Fragment slot="cannot">
In both languages, `num_students` is a name pointing at the value `10`, and printing the name reads the value back. Once assigned, both languages remember the value under that name until it is reassigned or goes out of scope.
</Fragment>
</WhatDiffers>
````

Never end a concept section with only one language, and never skip the `<WhatDiffers>` callout: the contrast is where the deeper understanding lives, not the code alone.

## The Language-ladder Fence

Every ENGR 103 example draws only on constructs the language ladder has already introduced for that lecture's week. The ladder lives at `/reference/language-ladder` and grows one week at a time; a construct allowed in week 3 stays allowed in every later week, but nothing appears in an example before its week arrives.

Before writing an example, check the ladder for the lecture's week. If a natural example would need a construct not yet on the ladder (a dictionary in a week 4 lecture, say), simplify the example instead of reaching ahead. Link `/reference/language-ladder` in prose the first time a lecture depends on knowing what is and is not available yet; do not restate the ladder's table.

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

**Example of a strong opening:**

> Every program you write eventually needs to remember something: a count, a name, a running total. Without a way to store and reuse a value, you would have to retype it everywhere it is needed, and update every copy by hand whenever it changes. Variables solve this problem by giving a value a name you can use and update from anywhere in your code.

**Weak openings to avoid:**

- "In this lecture we will cover variables and assignment."
- "Variables are an important topic in programming."

### Section Structure

Every `##` and `###` section opens with prose before any code block, table, or component. That opening prose explains what the section covers, why it matters, and, where relevant, how it connects to the section before it. Never start a section with a code block, a table, or a `<Tabs>` component; always lead with a sentence or two of explanation.

### Concept Depth

Every technical term is explained in full when it first appears:

1. Define it in plain language.
2. Explain how it works, at the depth appropriate for a first-year reader.
3. Give a concrete example.
4. Explain why it matters: what it makes possible, or what goes wrong without it.

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

A lecture runs roughly **1,800 to 3,500 words**, including its code examples. That range is much shorter than a full lecture in an adjacent course, because ENGR 103 notes are flipped material a first-year student reads in about 45 minutes before class.

Length is a diagnostic, not a target. A lecture under 1,800 words usually means a concept was flattened or skipped; one over 3,500 words usually means tutorial-style or story material has crept in and needs to move out or be cut. Run `wc -w src/content/docs/lectures/<file>.mdx` after a substantial edit to sanity-check the count.

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
- **No construct ahead of the language ladder.** Every example stays inside the constructs the ladder has introduced by that lecture's week.
