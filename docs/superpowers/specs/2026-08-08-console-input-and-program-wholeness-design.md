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
3. **Rust gets a provided `read_i32()` / `read_f64()` helper**, visible in the
   starter and explained in lecture 4, rather than the full recipe inline.
4. **Every assignment from A3 on ships an interactive `main`** in the starter:
   it prompts, reads, calls the student's function, and prints. This is a
   convention of the starter repositories, not a graded surface.
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
  `Ok`/`Err`, and validating a value against what is physically sensible.

L9 gets shorter and gains a sharper subject. It loses nothing it actually taught,
because plumbing was never its point.

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

This is not a workaround. Generics are listed under "Not used in this course" on
the language ladder, so `.parse::<i32>()` is off-limits by the course's own
rules. Week-2 students have just met `let` with explicit types (`i32` and `f64`
are on the ladder from week 1), so the annotated form is **better** aligned with
what they just learned than lecture 9's current form is.

### The helper, and the deferral debt

The starter provides, in a file the student can open and read:

```rust
use std::io;

fn read_i32() -> i32 {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("failed to read line");
    text.trim().parse().expect("not a whole number")
}
```

and `read_f64()` alongside it. The student writes `let countdown_seconds =
read_i32();` against Python's `countdown_seconds = int(input())`. Symmetric
one-liners, and the messy block exists once rather than six times.

Lecture 4 shows the helper's body and names exactly four things the reader is
asked to accept on faith for now, each with the lecture that pays it off:

| Deferred piece | Paid off in |
|---|---|
| `use std::io;` (modules) | never formally; it is standard-library plumbing |
| `String::new()` | lecture 13, strings |
| `&mut` in `read_line(&mut text)` | lecture 15, the memory model |
| `.expect()` on a `Result` | lecture 9, errors and validation |

That is **four** deferrals. Lecture 9's current inline recipe has the same four
plus the turbofish, so moving input earlier makes the deferral debt smaller, not
larger. This is the central argument that the move is safe.

Python deliberately gets **no** helper. Hiding `input()` behind a wrapper would
hide the one thing worth teaching on the Python side. The `WhatDiffers` call-out
states the asymmetry plainly: Python folds the work into one call, Rust spells it
out, and the course hands Rust students the spelling until they can read it.

---

## 5. A3: the one student-authored input

A3 (`launch-window` today) already takes six values that the starter "sets for
you," and its page already says "the grader may replace the six input values."
That is source substitution. It becomes standard input:

- The student's program prompts for and reads the six values, then prints the
  four required lines.
- The grader pipes the six values and compares the printed lines.
- **The requirements, the worked example, the assumptions, and the rubric are
  unchanged.** Only the sentence describing where the values come from changes,
  plus one rubric row if reading the values is to be graded separately.

A2 is unaffected: it has no variables at all, so input has nowhere to go.

---

## 6. The wholeness convention

**Every assignment from A3 on ships a `main` that prompts, reads, calls the
student's function, and prints.** From A4 on this `main` is provided code.

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
| `lectures/variables-and-state.mdx` | One new concept section in house style: what console input is, the Python call, the Rust helper and its four deferrals, a tabbed example, a `WhatDiffers`. Update `ai-summary` `covers:` and `glossary_terms:`. Largest single item. |
| `lectures/errors-input-and-validation.mdx` | Remove the console-input section; rewrite the opening so input is recalled, not introduced; keep parsing and validation. Update `ai-summary`. Mostly deletion. |
| `reference/language-ladder.mdx` | Move `input` / `read_line` from the week 5 row to week 2; adjust the "not yet introduced" column. Two rows. |
| `reference/glossary.mdx` | Add `console input` and `parsing`. Neither exists today despite L9 listing them as glossary terms, so this closes a latent gap either way. |
| `activities/variables-and-state.mdx` | Add an input block, including a run-it-yourself step (see section 3's timing note). |
| `activities/errors-input-and-validation.mdx` | Drop the input introduction; keep validation. |
| A3's assignment page | Reword "the starter program sets for you" to describe reading from the console. Requirements untouched. |
| A3's rubric TSV | Optional single row for reading the six values. |
| Starter scaffolds and Gradescope config | Write the stdin path and the `read_i32`/`read_f64` helpers. **Does not exist yet, so this is authoring, not rework.** |
| `practicalities/starter-repo-and-check` | Document the interactive-`main` convention and the `main`-vs-`check` constraint. |
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
