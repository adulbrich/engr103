---
name: engr103-recitation-coverage
description: Use when creating or editing a recitation page in src/content/docs/recitations/ of the public ENGR 103 repo. These are coverage pages for R2 to R10, the graded sessions whose problems live in the private engr103-assessments repo. A coverage page carries only the session's premise in the Rubber Duck Robotics story and the concepts and skills it evaluates, never a problem, an example value, or an answer. Recitation 1 is exempt and is governed by nothing here. Always load this skill before writing or editing a recitation page.
---

# Recitation Coverage Page Style Guide

A **coverage page** is the public face of a private assessment. Recitations R2 to R10 are graded, proctored, and worked on paper, so their problems live in the private `engr103-assessments` repository. What lives here is one page per session telling a student what the session is about, what it will ask them to do, and how to get ready.

The page exists because hiding an assessment badly is worse than publishing what it covers. A student who knows the topic can prepare; a student who has read the problems has not been assessed. So this page names everything except the problems.

## The One Rule That Overrides Everything

**Nothing on this page may help a student answer a question they have not seen.** Concretely, a coverage page never contains:

- a problem, in any form, including a paraphrase
- an example value, an input, or an output taken from the handout
- a code listing
- the handout's twist, the bug that is planted, or which archetype a specific core problem uses
- an answer to anything

The test to apply before shipping: if a student read this page and then sat the session, would any item be easier because of something on the page beyond honest preparation? If yes, cut that thing. A skill named in general terms ("trace a loop by hand") is preparation. The same skill named specifically ("trace the loop that counts duck laps and watch the boundary") is a leak.

When you change a handout in the private repo, update its coverage page in the same sitting. A page that describes the wrong topic sends students to prepare for the wrong thing, which is the one failure mode this tier cannot absorb.

## The Two Things a Coverage Page Carries

### 1. The premise

Two or three sentences of the session's **Rubber Duck Robotics** setting: the company is a mediocre but lovable novelty-gadget maker (self-stirring mugs, motivational bathtub ducks, duck-race timers, the occasional smart hot tub), the student is the new firmware intern, and Gary is the senior engineer handing out the week's work. Name the gadget this session is about and what is wrong with it in general terms.

The premise is flavor and orientation, not the ticket. Write the situation ("the duck-race timer is giving finish times nobody believes"), never the task ("compute the elapsed seconds between two readings"). A student should recognize the world when they open the handout and learn nothing about the work.

### 2. The concepts and skills evaluated

A short bulleted list of what the session tests, written as capabilities rather than tasks. Each bullet names a concept from the lectures it assesses or a skill from the assignment it follows.

Good bullets are general and honest:

- Reading a short program and predicting exactly what it prints
- Writing a short function from a plain-language description
- Deciding which branch of a decision table applies to a value
- Tracing how a variable's value changes as a program runs

Bad bullets leak or mislead:

- "Finding the off-by-one error in the volume chain" (names the planted bug)
- "The `duck_laps` function" (names an actual problem)
- "Everything from week 4" (tells the student nothing they can act on)

Two to five bullets is the right size. If the list runs longer, the session is either doing too much or the bullets are too specific.

## Page Structure

In this order, and nothing else:

1. **Opening paragraph.** What the session covers, in bold; which week's lectures it assesses; and which assignment it follows, as a link. Then the premise (above), which may be its own short paragraph.
2. **`## What this session covers`.** The bulleted concepts and skills.
3. **`## How the session runs`.** Length, that it is in person and on paper, that a printed handout is given out in the room, that either language earns full marks, and that the problems are not published, with the one-sentence reason. This section is nearly identical across pages by design; do not make it interesting.
4. **`## How to prepare`.** Do the paired assignment honestly and alone. Name what support is removed in the room (no hints, no worked sub-steps, no tools) and state plainly that a student who did the practice should expect to succeed.

No rubric, no point values, no submission instructions, no worked anything.

## Frontmatter and the `ai-summary` Block

```yaml
---
title: "The Handout's Name"
description: "What recitation N covers and how the session runs. The handout itself is given out in the room."
sidebar:
  order: <the recitation number>
---
```

The `title` is the handout's name with **no `Recitation N:` prefix**; the number lives in `sidebar.order`. These pages are published, so they carry no `draft` flag.

Immediately after the frontmatter:

````mdx
{/* ai-summary
type: recitation-coverage
slug: <filename without .mdx>
order: <sidebar.order>
family: <the problem family, by description>
paired_assignment: <the assignment slug this session certifies>
assesses_lectures: <comma-separated lecture slugs from week N-1>
graded: true
handout_location: the private engr103-assessments repo. This session is graded, proctored, and worked on paper, so its problems are never published here; this page is a coverage page only.
*/}
````

## Recitation 1 Is Not a Coverage Page

`intern-orientation.mdx` is ungraded onboarding and is published here **in full**, because it has nothing to pre-solve. It is written as a **tutorial**: the student opens a terminal, gets Python and Rust answering, writes and runs the same program in both languages, breaks one on purpose to read each language's error message, and then continues straight into Assignment 1. It explains every tool and term at first use, because almost none of that material is taught in a lecture.

Nothing in this skill applies to it. It has no premise-plus-bullets shape, it uses `Steps`, real code blocks, and full explanations, and it runs **before** its assignment rather than after. Leave it alone unless you are deliberately editing R1, and do not reshape it to match its neighbours.

## Style

**Plain language.** Write for a first-year reader who may not speak English as a first language: short sentences, common words, no idioms.

**Second person, present tense.** "You get a printed handout when you arrive."

**No emdashes.** Use a colon, a semicolon, a comma, or a period.

**Calm.** These pages are read by a nervous student the night before a graded session. State what happens and what to do about it. Never dramatize the stakes.

## Validation

After writing or editing a coverage page, run `npm run build`, which runs `astro check` and the link validator, and fix every error. These pages are published, so a link from one to a `draft: true` page fails the build.

Then read the page once more against the one rule at the top: does anything here make a question easier?
