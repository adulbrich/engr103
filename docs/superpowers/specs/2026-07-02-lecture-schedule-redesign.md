# Lecture Schedule Redesign (v5, locked): 16 outcome lectures + 2 week-10 extras

Supersedes v4. Locked with instructor decisions: a calendar-accurate slot
budget, all learning outcomes met by the end of week 9, a dedicated
foundation-forward early lecture taught with interactive components, and week 10
reserved for advanced extra topics that are not required for the outcomes.

## 1. Slot budget (calendar-accurate)

Two lectures per week (Monday and Wednesday), ten weeks = 20 slots. Minus the
**MLK holiday (week 3 Monday, no class)** and the **midterm (a week 6 slot)** =
**18 teaching slots**. Learning outcomes must be met by the end of week 9, so:

- **Weeks 1 to 9 hold the 16 outcome-bearing lectures** (18 slots minus MLK
  minus midterm = 16).
- **Week 10 holds 2 lectures on advanced extra topics** that may introduce new
  material but are not required for the course learning outcomes (the old
  web/graphics and AI/ML special topics are the model; the exact topics are the
  instructor's choice).

## 2. Foundations are first-class and interactive

- **A dedicated early lecture, L2 "Data representation and memory"** (binary and
  decimal, bits, bytes, ASCII, memory as addressable boxes, type sizes), taught
  before any type name is used to mean a bit-width.
- **Three interactive/visual components are built** (a small component sub-track
  reversing the earlier "defer the stepper" decision):
  1. a **binary/bits/bytes visualizer** (decimal to binary, show the byte and a
     value's bit pattern) for L2;
  2. a **memory-box / address diagram** (values in addressable boxes, a variable
     as a named box) for L2 and L4;
  3. the **dual-language memory stepper** (Python aliasing vs Rust
     ownership/moves/borrows), ported from `CppMemoryStepper`, for L13.
- Memory is threaded at rising depth (L2 foundation, L4 binding, L5 copy, L10
  scope lifetime, L13 the deep model), each hard step with a visual aid.

## 3. The 16 outcome lectures (weeks 1 to 9), calendar-mapped

| Wk | Slot | # | Lecture |
|---|---|---|---|
| 1 | Mon | 1 | How programs run (compute, interpret vs compile, errors as messages, the debugging method, two-language philosophy and ladder) |
| 1 | Wed | 2 | **Data representation and memory** (binary, bits, bytes, ASCII, memory-boxes, type sizes). Interactive: binary/bits visualizer + memory-box diagram |
| 2 | Mon | 3 | Values, types, expressions (grounded by L2; operators, precedence, integer division and truncation; coercion; static vs dynamic typing; Python `int` vs Rust `i32`/`f64`) |
| 2 | Wed | 4 | Variables and state (names vs values; `let`/`let mut`; initialization and uninitialized reads). Interactive: memory-box diagram of a variable |
| 3 | Mon | - | **NO CLASS (MLK holiday)** |
| 3 | Wed | 5 | Functions I (define/call, parameters, arguments, return; arguments and parameters are separate memory, value copied) |
| 4 | Mon | 6 | Booleans and conditionals (comparisons, logical operators, short-circuit; the floating-point equality trap and epsilon, grounded in L2/L3) |
| 4 | Wed | 7 | Decision structures (chains, `match`, decision tables, condition bugs) |
| 5 | Mon | 8 | Loops (`while`, counted `for`, termination; `do-while` as a C++-only "what differs") |
| 5 | Wed | 9 | Loop patterns (accumulate, count, search, sentinel, nested) |
| 6 | Mon | 10 | Functions II: scope and the call stack (nested scope, shadowing, lifetime, the global-variable antipattern, the call stack by hand) |
| 6 | Wed | - | **Midterm** (paper, lectures 1 to 10) |
| 7 | Mon | 11 | Strings and characters (encoding and ASCII grounded in L2; slicing, searching, building) |
| 7 | Wed | 12 | Collections I: lists and vectors (indexing, iteration, mutating; out-of-bounds as `IndexError` vs Rust panic vs the C++ buffer-overflow danger) |
| 8 | Mon | 13 | The memory model (names as labels; Python aliasing vs Rust ownership/moves/borrows; the C++ danger as why safety matters). Interactive: dual-language memory stepper |
| 8 | Wed | 14 | Sharing and mutation (passing collections; when the caller sees changes; Rust `&`/`&mut`) |
| 9 | Mon | 15 | Errors, input, validation, and testing (syntax/runtime/logic; console input and parsing; exceptions vs `Result`; validating; test cases from specs, boundary and error cases, assertions, consolidating the spec-to-tests thread) |
| 9 | Wed | 16 | Collections II: dictionaries and maps (key-value thinking; choosing list vs map; frequency and lookup) |
| 10 | Mon/Wed | 17, 18 | **Advanced extra topics** (new but not required for the outcomes; instructor's choice, taught in-depth) |

Woven rather than given a standalone lecture, so all outcomes still land by week
9: **numeric robustness** (integer overflow in L2, floating point in L6 and L15,
tolerance in L15), and **program design / decomposition** (through Functions I
and II, the Polya thread, and the capstone recitation). **Testing** is folded
into L15. The standalone "Computing with judgment" lecture is dropped; outcome
O8 lives in the assignment critique thread (VISION §6). **Type conversions**
fold into L3.

## 4. Invariants re-verified

- **All learning outcomes are met by the end of week 9.** Week 10 introduces
  only non-outcome extras. The midterm covers lectures 1 to 10.
- **Recitation families preserved.** Launch Window (weeks 1 to 3), Airlock
  (4), Rover (5), Comms (7 to 8), Telemetry (9), Manifest capstone (10). Each
  recitation's needed lecture precedes it: R2/R3 after values/types/variables/
  functions; R4 after conditionals; R5 after loops; R7 after strings and lists;
  R8 after the memory model; R9 after errors/validation/testing; R10 after
  dictionaries. Because L2 is inserted, the exact split of content between R2
  and R3 within the Launch Window family shifts slightly (R2 expressions and
  types, R3 functions and variables); the instructor finalizes the split with
  the real recitation day.
- **MLK gap handled:** week 3 has a single lecture (Functions I, Wednesday);
  the Launch Window assignment and recitation still precede and follow it.
- **One major topic per lecture** holds; L2 is "types and their representation"
  as one topic, and the deep model stays at L13.

## 5. Ripple (on approval to propagate)

- **VISION §10:** replace with this 18-slot schedule (16 outcome + 2 extras),
  noting the MLK gap and midterm; drop the special topics as required content.
- **VISION §4:** the midterm covers lectures 1 to 10 (now including data
  representation, integer division, the float-equality concept); the final
  weights collections, the memory model, strings, errors, testing, and numeric
  robustness; week 10 is non-outcome.
- **Lecture-notes spec:** add the interactive-component sub-track (the three
  components), and update the mapping to 16 lectures.
- **Skill:** the legibility gate stays.
- **Pilot:** split the existing L2 pilot into L2 (data representation, with the
  visualizer) and L3 (values, types, expressions); the already-grounded storage
  content moves to L2.
