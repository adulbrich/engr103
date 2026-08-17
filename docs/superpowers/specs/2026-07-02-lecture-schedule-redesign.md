# Lecture Schedule Redesign (v7, locked): correctness-first, loops after the midterm

Supersedes v6. Instructor restructuring: errors and testing are split into two
lectures and introduced before the midterm; loops move to after the midterm;
Collections II (dictionaries) moves to week 10 as an advanced extra.

## 1. Slot budget (calendar-accurate)

Two lectures per week (Monday and Wednesday), ten weeks = 20 slots. Minus the
**MLK holiday (week 3 Monday, no class)** and the **midterm (a week 6 slot)** =
**18 teaching slots**: 16 outcome-bearing lectures in weeks 1 to 9, plus 2
advanced-extra lectures in week 10.

## 2. Shape of the term

- **First half (through the midterm): correctness foundations without
  iteration.** Data representation and memory, values/types, variables,
  functions, scope, conditionals, error handling, and testing. Students learn to
  write, validate, and test straight-line and branching code, and to reason
  about memory, before loops add their complexity.
- **Second half (after the midterm): iteration and data.** Loops and loop
  patterns, strings, lists, the memory model (aliasing/ownership), and sharing.
- **Week 10: advanced extras** (dictionaries, and one more topic of the
  instructor's choice), not required for the outcomes.

## 3. Foundations first-class; interactive tools used throughout

- **A dedicated early lecture, L2 "Data representation and memory"** (values and
  types; binary, bits, bytes, memory-boxes, type sizes), before any type name
  means a bit-width. ASCII is deferred to L13, where characters are taught.
- **Reusable interactive tools, embedded wherever the concept is live:** a
  **binary/bits visualizer** (L2); a **memory-box / address diagram** (L2, L4);
  and the **dual-language memory stepper** (Python aliasing vs Rust
  ownership/moves/borrows) used at **variable assignment (L4), function calls
  (L5), scope (L6), and aliasing/sharing (L15, L16)**. There is no "stepper"
  lecture; it is a tool like `WhatDiffers`.

## 4. The 16 outcome lectures (weeks 1 to 9), calendar-mapped

| Wk | Slot | # | Lecture |
|---|---|---|---|
| 1 | Mon | 1 | How programs run |
| 1 | Wed | 2 | **Data representation and memory** (values and types; static vs dynamic typing; binary, bits, bytes, memory-boxes, sizes; Python `int` vs Rust `i32`/`f64`). Tools: binary visualizer + memory-box diagram. ASCII deferred to L13 |
| 2 | Mon | 3 | Expressions and operators (arithmetic, precedence; integer vs float division and truncation, grounded in L2's types/sizes; coercion; evaluating by hand) |
| 2 | Wed | 4 | Variables and state (`let`/`let mut`; initialization and uninitialized reads). Tool: memory stepper |
| 3 | Mon | - | **NO CLASS (MLK holiday)** |
| 3 | Wed | 5 | Functions I (define/call, parameters, arguments, return; arguments and parameters are separate memory, value copied). Tool: memory stepper |
| 4 | Mon | 6 | Scope and the call stack (local/nested scope, shadowing, lifetime, the global-variable antipattern, the call stack by hand). Tool: memory stepper |
| 4 | Wed | 7 | Booleans and conditionals (comparisons, logical operators, short-circuit; the floating-point equality trap, grounded in L2/L3) |
| 5 | Mon | 8 | Decision structures (chains, `match`, decision tables, condition bugs) |
| 5 | Wed | 9 | **Errors, input, and validation** (syntax/runtime/logic errors; console input and parsing; exceptions vs `Result`; validating a value and failing loudly; the loop-based validate-until-correct pattern is deferred to L12) |
| 6 | Mon | 10 | **Testing** (test cases from a specification; boundary and error cases; assertions and test functions; consolidates the spec-to-tests thread from week 3) |
| 6 | Wed | - | **Midterm** (paper, lectures 1 to 10: foundations, functions, scope, conditionals, errors, testing; no loops) |
| 7 | Mon | 11 | **Loops** (`while`, counted `for`, termination; `do-while` as a C++-only "what differs") |
| 7 | Wed | 12 | Loop patterns (accumulate, count, search, sentinel, validate-until-correct, nested) |
| 8 | Mon | 13 | Strings and characters (characters and ASCII introduced here, grounded in L2's bits/bytes; slicing, searching, building) |
| 8 | Wed | 14 | Collections I: lists and vectors (indexing, iteration, mutating; out-of-bounds as `IndexError` vs Rust panic vs the C++ buffer-overflow danger) |
| 9 | Mon | 15 | The memory model: aliasing and ownership (names as labels; Python aliasing vs Rust ownership/moves/borrows; the C++ danger as why safety matters). Tool: memory stepper. *Late because aliasing needs lists (L14) to exist first* |
| 9 | Wed | 16 | Sharing and mutation (passing collections; when the caller sees changes; Rust `&`/`&mut`). Tool: memory stepper |
| 10 | Mon | 17 | **Collections II: dictionaries and maps** (advanced extra; the capstone is list-based) |
| 10 | Wed | 18 | **Computing with judgment** (advanced extra): evaluating code you did not write (a peer's, a library's, an AI's); limitations and failure modes; who is excluded by our tools; course synthesis. Recommended; instructor may swap |

Woven rather than given a standalone lecture: **numeric robustness** (integer
overflow in L2/L3, floating point in L7, accumulation and tolerance in L9), and
**program design / decomposition** (Functions I, Scope, the Polya thread, the
capstone recitation). **Type conversions** fold into L3. The standalone
"Computing with judgment" lecture is dropped (O8 lives in the assignment
critique thread, VISION §6).

## 5. Outcomes and invariants

- **All learning outcomes are met by the end of week 9.** O3's "basic
  collections" is satisfied by lists (L14); dictionaries (L17) are a week-10
  extra. Exam archetype 6 "Draw memory" is served by L15; archetypes 4 "Find the
  bug" and 8 "Choose tests" by L9 and L10; O5 test-and-debug by L9/L10; O7
  transfer by the dual-language memory model (L15).
- **Scope before control flow** (L6 before L7 onward), matching the concept
  dependency.
- **Errors and testing before the midterm** (L9, L10), reinforcing the
  test-and-debug discipline threaded from week 1.
- **Recitation families** map onto this lecture order and are finalized by the
  instructor against the real recitation day: the Rover (loops) and Comms
  (strings/lists/memory) families now sit in the second half; the Telemetry
  (validation) family can move earlier since errors/validation is pre-midterm;
  the Manifest capstone is list-based (dictionaries are a week-10 extra). The
  prerequisite order is preserved: each family's recitation follows the lecture
  that teaches its concept.

## 6. Ripple (on approval to propagate)

- **VISION §10** becomes this schedule; **VISION §4** midterm covers lectures 1
  to 10 (no loops), the final weights loops, strings, collections (lists), the
  memory model, sharing, errors, and testing, and week 10 is non-outcome.
- **Lecture-notes spec:** the interactive-component sub-track (binary visualizer,
  memory-box diagram, dual-language memory stepper as a reusable tool); mapping
  updated to 16 lectures.
- **Skill:** the legibility gate stays; note the stepper as a reusable component.
- **Pilot:** split into L2 (data representation, with the visualizer/stepper) and
  L3 (values, types, expressions); the grounded storage content moves to L2.
