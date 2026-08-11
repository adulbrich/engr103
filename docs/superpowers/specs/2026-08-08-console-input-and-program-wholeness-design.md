# Design: console input in week 2, and the program-wholeness convention

**Status:** approved design, not yet implemented
**Date:** 2026-08-08
**Scope:** lecture 4, lecture 9, the language ladder, assignment A3, and the
starter-repository convention for every assignment

---

## 1. The problem

No assignment currently has the student write code that reads a value typed at
the terminal. Console input is introduced in lecture 9 (week 5), by which point
the assignment tier has moved to the function contract, where the grader calls a
named function directly and never touches standard input. The result is that
console input is taught but never authored by a student in graded work.

Two things are wanted:

1. Students should write console input at least once, so that a program of theirs
   behaves like a program a person can use.
2. Every assignment's program should be **whole**: runnable on its own at a
   terminal, prompting and printing, not just a function fragment.

These turn out to be two different problems with two different answers.

---

## 2. Decisions

1. **Console input moves from lecture 9 to lecture 4** (variables and state,
   week 2), so it is in scope for A3.
2. **A3 is the only assignment where the student authors the input.** A3 is
   already graded by comparing printed output, so its six values arrive from
   standard input and the whole program is the student's.
3. **Rust gets a provided `read_i32()` / `read_f64()` helper.** Lecture 4 teaches
   only the call; the helper's body lives in the starter, is documented on the
   starter-repo practicalities page, and is explained in lecture 9 where `Result`
   and `.expect()` are taught. Lecture 4 therefore defers nothing.
4. **Every assignment from A4 on ships an interactive `main`** in the starter:
   it prompts, reads, calls the student's function, and prints. This is a
   convention of the starter repositories, not a graded surface, which is exactly
   why it may prompt. **A3 is the exception and prints no prompt**, because it is
   the one assignment that is both output-diffed and student-authored; see
   section 5.
5. **No I/O-diff grading component is added** to A4 through A10. See section 7.

---

## 3. Why lecture 4

Input produces a value that gets bound to a name, which is exactly lecture 4's
subject. A3's window is L3 and L4 (`prereq_lectures: expressions-and-operators,
variables-and-state`), so a concept taught in L4 is available to A3 with no
schedule change whatsoever.

### The split this creates, and why it is an improvement

Lecture 9 currently bundles two distinct things: *getting a value in*, and
*distrusting the value you got*. That bundle is the reason input arrives in week
5. Unbundling gives:

- **L4:** read a line, turn it into a number, use it. If the person types
  something that is not a number, the program stops with an error message, and
  week 5 is about what to do instead. One sentence, and it plants L9.
- **L9:** why that assumption fails, `try`/`except` versus `Result`, `match` on
  `Ok`/`Err`, validating a value against what is physically sensible, and, as the
  natural payoff, what the Rust `read_i32` helper's body has been doing all along.

L9 gets a sharper subject and gains content rather than only losing it: it is now
the lecture that explains the helper, at the point where `Result` and `.expect()`
are its own material.

### Timing note

L4 is week 2's Wednesday lecture and A3 is due the following Monday. That is the
normal lag every week-2 concept gets, but input is the concept where a student
most needs to have actually run something before the assignment. **The L4
activity carries more weight than usual** and must include a run-it-yourself
block, not only a predict-the-output block.

---

## 4. Rust at week 2

Python is a non-issue: `countdown = int(input("Countdown (s): "))` is one line,
and calling a built-in function is already in the week-1 vocabulary via `print`.

Rust is the whole decision. Its input block pulls in material from later weeks.
Lecture 9 already handles this by declaring the block a fixed recipe whose pieces
"become clear as the course reaches it"; the question is only how large the
recipe is when it arrives three weeks earlier.

### Avoid the turbofish

Write the parse with a type annotation on the binding, not a turbofish on the
call:

```rust
let countdown_seconds: i32 = text.trim().parse().expect("not a whole number");
```

The reason is pedagogical, not a rule. An earlier version of this spec argued that
the ladder's "generics are not used in this course" line made `.parse::<i32>()`
off-limits outright. That was wrong: the course teaches the turbofish by name in
lecture 9 and uses it in lecture 12, the errors activity, and an assignment, and
lecture 12 explicitly frames it as part of a fixed input recipe. The ladder line is
what is imprecise, and it is being narrowed to say that students never *write*
generic code, while `parse::<T>` stands as one taught spelling.

What survives is the real argument for week 2: a turbofish is alien syntax to a
reader two weeks into their first programming course, and it would need explaining
on a page whose subject is variables. Week-2 students have just met `let` with
explicit types (`i32` and `f64` are on the ladder from week 1), so the annotated
form is aligned with what they just learned. Lecture 9 remains where the turbofish
is introduced and explained.

### The helper, and why lecture 4 does not show its body

**Lecture 4 teaches the call and nothing else.** The student writes:

```rust
let countdown_seconds = read_i32();
```

against Python's `countdown_seconds = int(input())`. Symmetric one-liners.

The helper's body is **not shown in lecture 4**. It lives in the starter
repository, is documented on the starter-repo practicalities page, and is
explained properly in lecture 9, where `Result` and `.expect()` are taught
anyway.

This is a correction to an earlier version of this spec, and the reason matters.
The earlier version showed the body in lecture 4 and argued the move was safe
because it deferred four things while lecture 9's inline recipe deferred five.
That argument was wrong on its own terms. Showing the body requires
`fn read_i32() -> i32 { ... }`, and the language ladder puts function definitions
in **week 3**, listing functions under "not yet introduced" for week 2. Counting
that, lecture 4 would defer five, not four. Lecture 9 defers only four, because
functions have been taught since week 3. Five versus five is not an argument for
moving anything.

Teaching only the call makes the real argument much stronger:

| Where | What the week-2 reader must accept on faith |
|---|---|
| Python | nothing; `input()` and `int()` are calls, and calling is week-1 material |
| Rust | nothing; `read_i32()` is a call, the same shape as `println!` |

**Zero deferrals at week 2**, and no ladder violation at all. Calling something
the course hands you is already what every student has done since `print` in
lecture 1.

The body is not lost, it is relocated to where it can be explained rather than
excused:

- **The starter repository** carries it, so a curious student can open it.
- **The starter-repo practicalities page** documents it as part of the harness.
- **Lecture 9** explains it, at the point where `Result` and `.expect()` are the
  lecture's actual subject. This turns lecture 9's change from a pure deletion
  into a genuine payoff: it stops being the lecture that lost console input and
  becomes the lecture that finally explains how console input works in Rust.

For reference, the body the starter ships:

```rust
use std::io;

fn read_i32() -> i32 {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("failed to read line");
    text.trim().parse().expect("not a whole number")
}
```

and `read_f64()` alongside it. Note the parse takes its target type from the
function's return type, so no turbofish appears.

Python deliberately gets **no** helper. Hiding `input()` behind a wrapper would
hide the one thing worth teaching on the Python side. The `WhatDiffers` call-out
states the asymmetry plainly: Python folds the work into one call, Rust spells it
out, and the course hands Rust students the spelling until they can read it.

---

## 5. A3: the one student-authored input

A3 (`launch-window` today) already takes six values that the starter "sets for
you," and its page already says "the grader may replace the six input values."
That is source substitution. It becomes standard input:

- The student's program reads the scenario values from standard input, then prints
  the required result lines.
- **A3 prints no prompt**, and this is forced rather than chosen. The `read_i32()`
  helper takes no prompt parameter, so a Rust prompt would need a bare `print!` plus
  an explicit `.flush()`; Rust's stdout is block-buffered when piped, so without the
  flush the prompt would not even appear in the right place. `print!` and `Write`
  are both well above the week-2 ladder. Python's `input(prompt)` meanwhile writes
  without a trailing newline while Rust's `println!` adds one, so any printed prompt
  makes the two languages' stdout differ and breaks the byte-for-byte diff this
  assignment is graded by. Symmetric silence is the only option that keeps both
  languages identical without pulling week-3 material forward.
- **This constraint applies to A3 and only A3.** It is the one assignment that is
  both output-diffed and student-authored. From A4 on, `main` is provided and never
  graded (section 7), so it prompts freely and the programs stay whole at a
  terminal. The page must state the no-prompt rule as an explicit requirement and
  the rubric must grade it, or a student will add a prompt and fail the grader for a
  reason they cannot see.
- The grader pipes the six values and compares the printed lines.
- **The requirements, the worked example, the assumptions, and the rubric are
  unchanged.** Only the sentence describing where the values come from changes,
  plus one rubric row if reading the values is to be graded separately.

A2 is unaffected: it has no variables at all, so input has nowhere to go.

---

## 6. The wholeness convention

**Every assignment from A4 on ships a `main` that prompts, reads, calls the
student's function, and prints**, and that `main` is provided code. A3 reads and
prints but does not prompt, for the reason given in section 5.

This is the answer to "students should be able to run their program
independently," and it is already the stated design in five assignment pages,
which carry the sentence "The program's `main`, its console input and output, and
the local test runner are already written." The remaining pages are silent on it.
Since the starter repositories do not exist yet, making the convention universal
costs nothing.

### The constraint the starter must honor

**`main` is interactive; `check` is not.** Running the local visible tests must
never require typing, or a student runs `check` and watches the terminal hang.

- **Python:** the input path sits behind the `if __name__ == "__main__":` guard,
  which is already in A1's scope. Tests import the module and call the function.
- **Rust:** the test path calls the function directly and never goes through
  `main`.

This belongs in the starter-repo design so it is built correctly the first time
rather than discovered by a stuck student.

### The bonus worth naming

The provided `main` becomes a **weekly worked example of the input code lecture 4
taught**, sitting in a file the student opens every week for the rest of the
term. Input is authored once and then re-read nine times, which is a better
retention story than authoring it once and never seeing it again.

---

## 7. Explicitly not done: I/O-diff grading

An I/O-diff component was considered for A4 through A10 and rejected, on grounds
stronger than effort: **it would not deliver the thing that motivated it.**

The programs are already whole. The starter's `main` already prompts and prints,
so a student can already run the program at a terminal and watch their own code
respond. Adding an I/O diff would certify `main`, which is code the student did
not write. It buys grading of provided code at the cost of doubling the graded
surface every week.

It also cuts against the function contract's reason for existing. VISION section
6 fixes the contract as a named entry point the grader calls with values it
chooses, precisely so the grader is free to choose those values. An I/O test
constrains that freedom for no outcome the exams or recitations ever assess.

**Consequence to accept knowingly:** student-authored console input appears in A3
and nowhere else. From A4 on, input exists only as provided code students read.
This is not the same as `print`, which stays in every rubric from A4 on through
the printed report lines; input genuinely stops being authored.

---

## 8. Effort by file

| File | Work |
|---|---|
| `lectures/variables-and-state.mdx` | One new concept section in house style: what console input is, the Python call, the Rust helper **call only** (never its body), a tabbed example, a `WhatDiffers`. Update `ai-summary` `covers:`. Largest single item. |
| `lectures/errors-input-and-validation.mdx` | Remove the console-input section; rewrite the opening so input is recalled, not introduced; keep parsing and validation. Update `ai-summary`. Mostly deletion. |
| `reference/language-ladder.mdx` | Move `input` to the week 2 Python row and add the provided `read_i32` / `read_f64` helpers to the week 2 Rust row; remove console input from the week 5 row. `read_line` itself does **not** move: it stays inside the helper body, which lecture 4 never shows and lecture 9 explains. Also narrow the "not used in this course" generics bullet so it forbids writing generic code rather than `parse::<T>`. |
| `reference/glossary.mdx` | Add `console input` and `parsing`. Neither exists today despite L9 listing them as glossary terms, so this closes a latent gap either way. |
| `activities/variables-and-state.mdx` | Add an input block, including a run-it-yourself step (see section 3's timing note). |
| `activities/errors-input-and-validation.mdx` | Drop the input introduction; keep validation. |
| A3's assignment page | Reword "the starter program sets for you" to describe reading from the console. Requirements untouched. |
| A3's rubric TSV | Optional single row for reading the six values. |
| Starter scaffolds and Gradescope config | Write the stdin path and the `read_i32`/`read_f64` helpers. **Does not exist yet, so this is authoring, not rework.** |
| `practicalities/starter-repo-and-check` | Document the interactive-`main` convention, the `main`-vs-`check` constraint, and **the `read_i32` / `read_f64` helper bodies** as part of the harness. |
| `VISION.md` | Section 10's L4 and L9 entries gain a clause each. Nothing structural. |

No assignment other than A3 changes, and A2 does not change at all.

---

## 9. Risks

- **A3 gains a concept.** It already carries variables plus integer division plus
  the remainder operator; input makes four. They are still two lectures' worth,
  which is the norm, but A3 becomes the densest of the early slots. Watch it in
  the pilot term.
- **Week-2 students will hit a crash.** Typing a non-number at an L4 program
  raises `ValueError` in Python and panics in Rust. This is intended (it plants
  lecture 9), but the L4 notes and the activity must say so in advance, or it
  reads as the student's own bug.
- **The helper is Rust-only, which is a visible asymmetry** between the two
  language tracks. It is the right call, but it is the first place in the course
  where one track gets a crutch the other does not, and the notes should say why
  rather than let students notice it unexplained.
- **`check` hanging on input** is the failure mode that would waste the most
  student time. It is prevented by section 6's constraint and should be an
  explicit test of the starter tooling.
