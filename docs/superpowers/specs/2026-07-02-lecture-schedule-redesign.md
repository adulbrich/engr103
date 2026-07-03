# Lecture Schedule Redesign (v4): calendar-accurate, foundation-forward, interactive

Supersedes v3. Two corrections drove this version: the real Winter-term calendar
(one lecture slot is lost to the MLK holiday, and the two week-10 special topics
are dropped), and the instructor's emphasis that the hard foundations (binary,
bits and bytes, memory, the underlying machinery of programming) must be covered
**well and with interactive/visual components**, not just enriched prose.

Status: **proposal for instructor approval** before propagating to VISION and
the specs/skill/pilot.

## 1. The calendar reality (this is the slot budget)

Two lectures per week (Monday and Wednesday) across ten weeks = 20 slots. But:

- **Week 3 Monday is the MLK holiday: no class.** That removes one slot.
- **The midterm consumes one slot** (week 6).
- **The two week-10 special topics (web/graphics, AI/ML) are dropped.**

So there are **18 content-lecture slots**, not 19 or 20. The redesign must fit 18
lectures. The dropped special-topic slots become week-10 synthesis, not new
material.

## 2. Foundations are first-class, and taught interactively

The originals lean hard on memory and machine representation (binary/bits/bytes
in Expressions, memory boxes and sizes, contiguous memory and addresses in
Arrays, aliasing and dangling in References). The instructor wants these covered
**well and visually/interactively**. Concretely:

- **A dedicated early foundation lecture (L2, "Data representation and memory")**
  teaches binary and decimal, bits and bytes, ASCII, memory as addressable
  boxes, and type sizes, before any type name (`i32`, `f64`) is used to mean a
  bit-width. This replaces the "enrich L2 in place" compromise: the foundation
  earns its own lecture now that dropping the special topics frees room.
- **Interactive components (un-deferring the memory stepper):**
  - a **binary/bits/bytes visualizer** (decimal to binary, show the byte, show a
    value's bit pattern) for L2;
  - a **memory-box / address diagram** (values in addressable boxes, a variable
    as a named box) for L2 and L4;
  - the **dual-language memory stepper** (Python aliasing, Rust
    ownership/moves/borrows), ported from `CppMemoryStepper`, for L13.
  These are built as part of the lecture track (a small component sub-track),
  reversing the earlier "defer the stepper" decision.
- Memory is still **threaded** at rising depth after L2 (L4 uninitialized/
  binding, L5 copy foreshadow, L10 scope lifetime, L13 the deep model, L16
  numeric robustness), but now each hard step has a visual or interactive aid.

## 3. The 18-lecture sequence

| # | Lecture | Foundational / interactive note |
|---|---|---|
| 1 | How programs run | compute, interpret vs compile, running, errors as messages, the debugging method, two-language philosophy and ladder |
| 2 | **Data representation and memory** | binary and decimal, bits, bytes, ASCII, memory as addressable boxes, type sizes. **Interactive: binary/bits visualizer + memory-box diagram.** Grounds every later type name |
| 3 | Values, types, and expressions | values and types; static vs dynamic typing; Python arbitrary-precision `int` vs Rust `i32`/`f64` (now grounded by L2); operators, precedence, integer division and truncation; implicit coercion; literals |
| 4 | Variables and state | names vs values; assignment; `let`/`let mut`; initialization and uninitialized reads (Rust forbids, C++ undefined behavior). **Interactive: memory-box diagram of a variable** |
| 5 | Functions I | define/call, parameters, arguments, return; arguments and parameters are separate memory, value copied (foreshadows the memory model) |
| 6 | Booleans and conditionals | comparisons, logical operators, short-circuit; `if`/`else`; the floating-point equality trap and epsilon, grounded in L2/L3 |
| 7 | Decision structures | chains, `match`, decision tables, guard clauses, condition bugs |
| 8 | Loops | `while`, counted `for`, termination; `do-while` as a C++-only "what differs" |
| 9 | Loop patterns | accumulate, count, search, sentinel, nested |
| 10 | Functions II: scope and the call stack | local/nested scope, shadowing, lifetime, the global-variable antipattern, the call stack by hand |
| 11 | Strings and characters | strings as sequences; encoding and ASCII (grounded in L2); slicing, searching, building |
| 12 | Collections I: lists and vectors | indexing, iteration, growing/mutating; out-of-bounds as `IndexError` vs Rust panic vs the C++ buffer-overflow danger |
| 13 | The memory model | names as labels; Python aliasing vs Rust ownership/moves/borrows; the C++ danger as why safety matters. **Interactive: dual-language memory stepper** |
| 14 | Sharing and mutation | passing collections; when the caller sees changes; defensive copying; Rust `&`/`&mut` |
| 15 | Errors, input, and validation | syntax/runtime/logic; console input and parsing; exceptions vs `Result`; validating |
| 16 | Numeric robustness | integer overflow (Rust wrap/panic vs Python arbitrary precision); floating-point accumulation and tolerance in depth; units and magnitudes |
| 17 | Collections II: dictionaries and maps | key-value thinking; choosing list vs map; frequency and lookup |
| 18 | Program design and testing (synthesis) | decomposition top-down, from word problem to program; test cases from specs, boundary/error cases, assertions (consolidating the spec-to-tests skill threaded since week 3). Synthesis only, no new archetype |

Dropped from the v3/VISION list to fit 18 slots: the standalone "Computing with
judgment" lecture (outcome O8 lives in the assignment critique thread per VISION
§6, so no lecture is required), and the separate "Type conversions" lecture
(folded into L3). Testing is consolidated into the week-10 synthesis (L18)
because the spec-to-tests skill is threaded from week 3, so L18 introduces no new
examinable archetype.

## 4. What still needs verifying (and one open risk)

- **Recitation/family lockstep and the MLK week.** With week 3 losing its Monday
  lecture, the "Lecture A, recitation, Lecture B" rhythm for week 3 needs the
  instructor's actual recitation day to finalize. The lecture *content* order
  above preserves the family prerequisites (variables before the Launch Window
  recitation, loops before Rover, strings+lists before Comms, memory model
  before the Comms memory recitation, dicts before the Manifest capstone), but
  the exact week each recitation lands must be re-checked against the real
  calendar once the foundation lecture (L2) shifts everything one slot.
- **Exam invariant.** Testing is consolidated in the week-10 synthesis; because
  it is threaded from week 3 it introduces no new examinable atom, so the
  "nothing new after week 9" rule still holds. Confirm this reading.

## 5. Decisions needed

1. Dedicated **L2 "Data representation and memory"** lecture (recommended, now
   that special topics are dropped), versus keeping it enriched-in-place.
2. **Interactive component scope:** build all three (binary/bits visualizer,
   memory-box diagram, dual-language memory stepper), or a subset?
3. **18-lecture fit:** drop the standalone judgment lecture and fold type
   conversions into L3 and testing into the week-10 synthesis (recommended), or
   keep one of them and drop something else?

## 6. Ripple (on approval)

VISION §10 becomes 18 lectures (this sequence), the midterm and MLK gap noted,
special topics removed; VISION §4 coverage updated. The lecture-notes spec gains
the interactive-component sub-track. The skill's legibility gate and the L2
pilot (already grounded) stay; the pilot may split into L2 (data representation,
with the visualizer) and L3 (values/types/expressions) to match the new L2/L3.
