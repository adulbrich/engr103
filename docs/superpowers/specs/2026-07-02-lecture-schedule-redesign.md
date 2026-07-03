# Lecture Schedule Redesign (v6, locked): 16 outcome lectures + 2 week-10 extras

Supersedes v5. Fixes two ordering/tooling errors: scope belongs right after
Functions I (it governs the blocks in conditionals and loops), and the memory
stepper is a reusable tool embedded wherever a concept is live, not a lecture.

## 1. Slot budget (calendar-accurate)

Two lectures per week (Monday and Wednesday), ten weeks = 20 slots. Minus the
**MLK holiday (week 3 Monday, no class)** and the **midterm (a week 6 slot)** =
**18 teaching slots**. Learning outcomes are met by the end of week 9:

- **Weeks 1 to 9 hold the 16 outcome-bearing lectures.**
- **Week 10 holds 2 lectures on advanced extra topics** (new but not required
  for the outcomes; instructor's choice of topic).

## 2. Foundations first-class; interactive tools used throughout

- **A dedicated early lecture, L2 "Data representation and memory"** (binary and
  decimal, bits, bytes, ASCII, memory as addressable boxes, type sizes), taught
  before any type name means a bit-width.
- **Interactive/visual components are reusable tools, embedded wherever the
  concept is live, not tied to one lecture:**
  1. a **binary/bits/bytes visualizer** (decimal to binary, show the byte and a
     value's bit pattern), used mainly in L2;
  2. a **memory-box / address diagram** (values in addressable boxes, a variable
     as a named box), used in L2, L4, and later where addresses matter;
  3. the **dual-language memory stepper** (Python aliasing vs Rust
     ownership/moves/borrows), ported from `CppMemoryStepper`, used **wherever
     the memory picture helps: variable assignment (L4), function calls (L5),
     scope and shadowing (L6), aliasing and ownership (L13), sharing (L14)**.
     There is no "memory stepper" lecture; it is a tool like `WhatDiffers`.
- Memory is threaded at rising depth (L2 foundation, L4 binding, L5 copy, L6
  scope lifetime, L13 the deep model), each hard step with the stepper or a
  diagram.

## 3. The 16 outcome lectures (weeks 1 to 9), calendar-mapped

| Wk | Slot | # | Lecture |
|---|---|---|---|
| 1 | Mon | 1 | How programs run |
| 1 | Wed | 2 | **Data representation and memory** (binary, bits, bytes, ASCII, memory-boxes, sizes). Tools: binary visualizer + memory-box diagram |
| 2 | Mon | 3 | Values, types, expressions (grounded by L2; operators, precedence, integer division; coercion; static vs dynamic typing; Python `int` vs Rust `i32`/`f64`) |
| 2 | Wed | 4 | Variables and state (names vs values; `let`/`let mut`; initialization and uninitialized reads). Tool: memory stepper (assignment) |
| 3 | Mon | - | **NO CLASS (MLK holiday)** |
| 3 | Wed | 5 | Functions I (define/call, parameters, arguments, return; arguments and parameters are separate memory, value copied). Tool: memory stepper (call) |
| 4 | Mon | 6 | **Scope and the call stack** (local/nested scope, shadowing, lifetime, the global-variable antipattern, the call stack by hand). Tool: memory stepper. *Moved here, right after Functions I, because scope governs the blocks used in conditionals and loops* |
| 4 | Wed | 7 | Booleans and conditionals (comparisons, logical operators, short-circuit; the floating-point equality trap and epsilon, grounded in L2/L3) |
| 5 | Mon | 8 | Decision structures (chains, `match`, decision tables, condition bugs) |
| 5 | Wed | 9 | Loops (`while`, counted `for`, termination; `do-while` as a C++-only "what differs") |
| 6 | Mon | 10 | Loop patterns (accumulate, count, search, sentinel, nested) |
| 6 | Wed | - | **Midterm** (paper, lectures 1 to 10) |
| 7 | Mon | 11 | Strings and characters (encoding and ASCII grounded in L2; slicing, searching, building) |
| 7 | Wed | 12 | Collections I: lists and vectors (indexing, iteration, mutating; out-of-bounds as `IndexError` vs Rust panic vs the C++ buffer-overflow danger) |
| 8 | Mon | 13 | The memory model: aliasing and ownership (names as labels; Python aliasing vs Rust ownership/moves/borrows; the C++ danger as why safety matters). Tool: memory stepper |
| 8 | Wed | 14 | Sharing and mutation (passing collections; when the caller sees changes; Rust `&`/`&mut`). Tool: memory stepper |
| 9 | Mon | 15 | Errors, input, validation, and testing (syntax/runtime/logic; console input and parsing; exceptions vs `Result`; validating; test cases from specs, boundary and error cases, assertions) |
| 9 | Wed | 16 | Collections II: dictionaries and maps (key-value thinking; choosing list vs map; frequency and lookup) |
| 10 | Mon/Wed | 17, 18 | **Advanced extra topics** (new but not required for the outcomes; taught in-depth) |

Woven rather than given a standalone lecture: **numeric robustness** (integer
overflow in L2/L3, floating point in L7, accumulation and tolerance in L15), and
**program design / decomposition** (Functions I and Scope, the Polya thread, the
capstone recitation). **Testing** is folded into L15. **Type conversions** fold
into L3. The standalone "Computing with judgment" lecture is dropped (O8 lives in
the assignment critique thread, VISION §6).

## 4. Invariants and open logistics

- **All learning outcomes met by end of week 9.** Midterm covers lectures 1 to
  10. Week 10 is non-outcome.
- **Scope before control flow** (L6 before L7 to L10), matching the original
  course and the concept dependency.
- **Recitation families** (Launch Window, Airlock, Rover, Comms, Telemetry,
  Manifest) map onto this lecture order; because L2 is inserted and scope moved,
  the exact week each recitation lands and the content split within a family are
  **derived from the lecture order and finalized by the instructor with the real
  recitation day** (especially around the MLK week, where week 3 has one
  lecture). The prerequisite order is preserved: each family's recitation
  follows the lecture that teaches its concept.
- **One major topic per lecture** holds; L2 is "types and their representation";
  the deep memory model is L13.

## 5. Ripple (on approval to propagate)

- **VISION §10** becomes this schedule (16 outcome + 2 extras, MLK gap, midterm
  week 6, special topics dropped as required content).
- **VISION §4:** midterm covers lectures 1 to 10; week 10 is non-outcome.
- **Lecture-notes spec:** add the interactive-component sub-track (binary
  visualizer, memory-box diagram, dual-language memory stepper as a reusable
  tool); update the mapping to 16 lectures.
- **Skill:** the legibility gate stays; add a note that the memory stepper is a
  reusable component to embed wherever the memory picture helps.
- **Pilot:** split into L2 (data representation, with the visualizer and stepper)
  and L3 (values, types, expressions); the grounded storage content moves to L2.
