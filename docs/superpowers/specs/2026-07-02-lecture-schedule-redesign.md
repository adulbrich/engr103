# Lecture Schedule Redesign (v2): grounding concepts in the original notes

A revision of VISION §10 (the 19-lecture schedule) driven by a careful read of
the existing C++ lecture notes. The original notes introduce concepts in an
order the VISION §10 schedule flattened and, in places, broke. This proposes a
concept ordering that reintroduces that nuance, adapted for the dual-language
(Python + Rust) course.

Status: **proposal for instructor approval.** On approval it propagates to
VISION §4 and §10, the lecture-notes spec, the `engr103-lecture-notes` skill,
and the L2 pilot (which is re-scoped and re-authored).

## 1. The diagnosis: the original weaves memory; VISION §10 dumped it late

The original notes do not have a single "memory" lecture. They **weave memory
and machine-number concepts at increasing depth, exactly where each is first
needed**:

- **Expressions (order 5, early):** binary, bits, bytes, RAM vs disk, memory as
  addressable boxes, primitive types and their **sizes** (int = 4 bytes, etc.).
  This light foundation grounds everything after it.
- **Operators (order 6):** integer division and truncation, grounded in the
  integer type just learned.
- **Variables (order 7):** a variable is a named memory location; uninitialized
  reads are **undefined behavior**.
- **Functions (order 11):** arguments and parameters are **separate memory**;
  the value is copied. This foreshadows references.
- **Scope (order 12):** memory is reclaimed when a variable falls out of scope;
  global variables as an antipattern.
- **Booleans (order 13):** the **floating-point equality trap** and epsilon
  comparison, taught alongside comparisons (not deferred).
- **Arrays (order 19):** contiguous memory, base address, direct access,
  **buffer overflow** as undefined behavior.
- **References (order 21):** indirection, addresses, aliasing, **dangling
  references**, and an explicit contrast of C/C++ (manual, unsafe) vs Python
  (garbage collection) vs Rust (compile-time ownership).

VISION §10 collapsed all of this into two late standalone lectures, "The memory
model" (L12) and "Numbers in machines" (L15), and put machine numbers on the
final only. The pilot L2 then had no bits/bytes foundation to stand on, which is
exactly why Rust's `i32`/`f64` (whose names encode bit-widths) could not be
explained without reaching forward. **The fix is not to defer the type names; it
is to restore the early, light memory foundation the original always had.**

Nuance the VISION §10 schedule also dropped, now restored below: type coercion
versus explicit casting as its own topic, undefined behavior / uninitialized
reads, `do-while` (a genuine Python/Rust "what differs"), the global-variable
antipattern, arguments-versus-parameters copy semantics, and buffer
overflow/dangling references as the motivation for memory safety.

## 2. The principle

1. **Introduce a concept where it is first needed, at the depth needed there.**
   No standalone "numbers in machines" or "memory model" dump.
2. **Weave memory light to deep.** A light foundation (binary, bits, bytes,
   storage, type sizes) lands in week 1 and grounds types, integer division, and
   Rust's `i32`/`f64`. The deep model (references, aliasing, ownership, the
   C++ danger) lands mid-course once collections make sharing matter.
3. **Distribute machine-number concepts** to their home: sizes and integer vs
   float to values/types; truncation to operators; the float-equality trap to
   comparisons; overflow as a Rust/C++ (not Python) contrast where integers are
   introduced.
4. **Use the three languages' differences as the lesson.** Python's
   arbitrary-precision `int` (no overflow) vs Rust's fixed `i32` (overflow) vs
   C++; Python aliasing vs Rust ownership vs C++ manual memory; `do-while`
   existing only in C++; bounds errors (Python `IndexError`, Rust panic, C++
   buffer overflow).

## 3. The revised 19-lecture schedule

Two lectures per week, recitation between, midterm in week 6, across ten weeks.

| Wk | # | Lecture | Core concepts (dual-language) |
|---|---|---|---|
| 1 | 1 | How programs run | computation, interpret vs compile, running a program, errors as messages, the systematic debugging method, the two-language philosophy and the ladder |
| 1 | 2 | Values, types, and how computers store them | values and types; binary, bits, bytes, storage; integer vs floating-point types and their sizes; Python arbitrary-precision `int` vs Rust fixed `i32`/`f64`; static vs dynamic typing; booleans, characters, strings as values; literals |
| 2 | 3 | Expressions and operators | arithmetic, precedence; integer division and truncation (grounded in L2 sizes); modulo; mixed-type/precision; evaluating by hand |
| 2 | 4 | Variables and state | names vs values; assignment and rebinding; `let`/`let mut` and mutability; initialization and uninitialized reads (Rust forbids, C++ undefined behavior); constants; tracing with a variable table |
| 3 | 5 | Type conversions | implicit coercion vs explicit casts; truncation; Rust requires explicit `as`; safe vs lossy conversions |
| 3 | 6 | Functions I | define and call; parameters, arguments, return; signatures; arguments and parameters are separate memory, value copied (foreshadows references); how the autograder calls your functions |
| 4 | 7 | Booleans and conditionals | comparisons; logical operators and short-circuit; `if`/`else`; building conditions; the floating-point equality trap and epsilon, taught here with comparisons |
| 4 | 8 | Decision structures | `elif`/`else if` chains and `match`; mutual exclusivity; nesting; guard clauses; decision tables; condition bugs (chained inequalities, `=` vs `==`) |
| 5 | 9 | Loops | `while`, counted `for`, loop variables, termination; `do-while` as a Python/Rust "what differs"; tracing loops |
| 5 | 10 | Loop patterns | accumulate, count, search, sentinel, validate-until-correct; nested loops; choosing the pattern from the problem |
| 6 | 11 | Functions II: scope and the call stack | local and nested scope, shadowing, lifetime; the global-variable antipattern; the call stack by hand; decomposition into functions |
| 6 | - | Midterm | on paper, L1 to L11 |
| 7 | 12 | Strings and characters | strings as sequences of characters; character encoding and ASCII (grounded in L2 bits/bytes); indexing, slicing, searching, building; Python `str` vs Rust `String`/`&str` |
| 7 | 13 | Collections I: lists and vectors | lists/vectors; indexing; iteration; growing and mutating; out-of-bounds as Python `IndexError` vs Rust panic vs the C++ buffer-overflow danger |
| 8 | 14 | The memory model | names are labels for values; copies vs shared references; Python aliasing (two names, one list); Rust ownership, moves, borrows-lite; the C++ danger (dangling, buffer overflow) as why memory safety matters; drawing memory diagrams |
| 8 | 15 | Sharing and mutation | passing collections to functions; when the caller sees changes; defensive copying; Python aliasing gotchas; Rust `&`/`&mut`; why Rust makes you declare intent |
| 9 | 16 | Errors, input, and validation | syntax vs runtime vs logic errors; console input and parsing; exceptions vs `Result`; validating input; failing loudly; the input-buffer nuances |
| 9 | 17 | Collections II: dictionaries and maps | dictionaries and hashmaps; key-value thinking; choosing list vs map; frequency counting and lookup |
| 10 | 18 | Testing and program design | test cases from specs, boundary and error cases, assertions; systematic debugging as hypothesis testing; decomposition top-down; from word problem to program |
| 10 | 19 | Computing with judgment | evaluating code you did not write; limitations and failure modes; who is excluded by our tools; course synthesis (non-exam) |

## 4. What changed versus VISION §10

- **Added an early "how computers store values" lecture (L2)** carrying the
  light memory foundation (binary/bits/bytes/sizes). This is the central fix.
- **Distributed "Numbers in machines" (old L15)** into L2 (sizes, overflow), L3
  (integer division), and L7 (float equality). No standalone late lecture.
- **Added "Type conversions" (L5)** as its own lecture (coercion vs casting),
  which the original had and VISION dropped.
- **Kept the deep memory model (L14) and sharing (L15)** but placed them right
  after collections (L12, L13), where sharing first matters, and folded in the
  C++ danger as motivation.
- **Merged testing and program design (L18)** so week 10 introduces no new
  examinable atom (the week-10 invariant from the earlier §4 decision holds).
- Restored `do-while`, undefined behavior, argument-copy semantics, the
  global-variable antipattern, and buffer overflow/dangling as dual-language
  contrasts.

## 5. Ripple (on approval)

- **VISION §4 (exams):** machine-number concepts are no longer a single late
  topic. The midterm (L1 to L11) may now test type sizes, integer division, and
  the float-equality concept. The final still weights collections, the memory
  model, errors, and testing. Update the coverage split accordingly.
- **VISION §10:** replace the schedule table with section 3 above.
- **Lecture-notes spec + skill:** the ladder and the "legibility" rule already
  agreed still hold; the skill's legibility gate (no forward references, anchored
  to the ladder) is what would have caught the pilot. Add it, and annotate the
  ladder so early type names are grounded by L2 rather than deferred.
- **L2 pilot:** re-scope from "Values, types, expressions" to "Values, types,
  and how computers store them" (values/types + the light memory foundation);
  expressions/operators move to L3. Re-author the pilot against the new L2.

## 6. Open questions for the instructor

1. Does the early light-memory placement (L2) match how you want to teach it, or
   would you rather a dedicated "how computers store data" lecture between L1 and
   L2 (making 20 lectures)?
2. Overflow: introduce it in L2 as the Python-vs-Rust integer contrast, or defer
   the deeper treatment (wrap-around, checked arithmetic) to the memory week?
3. Is merging testing and program design into one L18 acceptable, or should they
   be two lectures (dropping the non-exam L19 judgment lecture, or moving it)?
