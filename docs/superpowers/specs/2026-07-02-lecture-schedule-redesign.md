# Lecture Schedule Redesign (v3): weave memory early, keep the family lockstep

A revision of VISION §10 driven by a careful read of the original C++ lecture
notes. The originals weave memory and machine-number concepts at increasing
depth exactly where each is first needed; VISION §10 flattened that into two
late standalone lectures (L12 "memory model", L15 "numbers in machines"), which
left week-1 content (and Rust's `i32`/`f64`) ungrounded.

**Key finding from re-checking the interlock:** VISION §10's lecture *order* is
already correctly aligned to the recitations and their problem families. The
defect is **content placement**, not sequence. So this redesign is surgical: it
enriches early lectures and reframes the two late ones, and it does **not**
reorder the sequence, which keeps every recitation aligned to the lecture it
needs. Status: **proposal for instructor approval** before propagating to
VISION §4/§10, the lecture-notes spec, the skill, and the L2 pilot.

## 1. Diagnosis: the originals weave memory; VISION §10 dumped it late

The original notes have no single "memory" lecture. They introduce memory and
machine-number ideas at rising depth, each where it is first needed:

- **Expressions (order 5):** binary, bits, bytes, storage, primitive types and
  their **sizes**. The light foundation that grounds everything after.
- **Operators (order 6):** integer division and truncation, grounded in the
  integer type's size.
- **Variables (order 7):** a variable is a named memory location; uninitialized
  reads are undefined behavior.
- **Functions (order 11):** arguments and parameters are separate memory; the
  value is copied. Foreshadows references.
- **Scope (order 12):** memory reclaimed on scope exit; the global-variable
  antipattern.
- **Booleans (order 13):** the floating-point equality trap and epsilon, taught
  with comparisons, not deferred.
- **Arrays (order 19):** contiguous memory, base address, buffer overflow.
- **References (order 21):** indirection, aliasing, dangling references, and an
  explicit C/C++ (manual) vs Python (garbage collection) vs Rust (compile-time
  ownership) contrast.

VISION §10 moved the light foundation and the machine-number concepts to L12 and
L15 and put machine numbers on the final only. That is why the L2 pilot could not
explain `i32`/`f64` without reaching forward. The fix is to **restore the early
light foundation**, not to defer the type names.

## 2. The surgical fix (no reorder)

1. **Enrich L2** so "what is a type" includes its machine representation: bits,
   bytes, sizes; integer vs floating-point representation; Python's
   arbitrary-precision `int` vs Rust's fixed `i32`/`f64`; static vs dynamic
   typing. This grounds `i32`/`f64` and integer division at the point of use.
   This is one topic (types and their representation), not two.
2. **Foreshadow the deep model in L4 (Functions I):** arguments and parameters
   are separate memory, the value is copied (as the originals do).
3. **Teach the float-equality trap with comparisons (L5/L6)**, grounded in L2's
   representation, exactly as the original booleans lecture did.
4. **Reframe L12 (memory model) and L15** as *deepenings of an early
   foundation*, not first introductions. L12 builds the aliasing/ownership model
   on L2 and L4; L15 becomes "numeric robustness" (accumulation error, tolerance,
   integer overflow in depth) feeding the week-9 Telemetry recitation, building
   on the float and size ideas seeded in L2/L6.
5. **Fold type coercion vs explicit casting into L2/L3** (Rust's stricter `as`),
   which the originals had as a topic and VISION dropped.
6. **Restore dual-language contrasts** as lessons: Python arbitrary-precision
   `int` (no overflow) vs Rust fixed `i32`; `do-while` exists only in C++ (a
   Python/Rust "what differs" in loops); bounds errors as Python `IndexError`
   vs Rust panic vs the C++ buffer-overflow danger.

## 3. The full verified grid

Order is unchanged from VISION §10; the **bold** text marks enriched or reframed
content. Each row lists the recitation that week (its Mission Ares family) and
the assignment due before it, so the lecture -> recitation -> family lockstep is
visible. Recitation in week N assesses through week N's first lecture.

| Wk | # | Lecture (enriched) | Recitation (family) | Assignment due |
|---|---|---|---|---|
| 1 | 1 | How programs run **+ a first look at how data is represented (bits and bytes)** | R1 onboarding (ungraded) | A0 setup |
| 1 | 2 | Values, types, expressions **and how they are stored: bits/bytes/sizes, integer vs float representation, Python arbitrary-precision `int` vs Rust `i32`/`f64`, static vs dynamic typing, implicit coercion** | | |
| 2 | 3 | Variables and state **(uninitialized reads: Rust forbids, C++ undefined behavior; `let`/`let mut`; explicit casts, Rust's `as`)** | R2 Launch Window (expressions, types, variables) | A1 Launch Window arithmetic |
| 2 | 4 | Functions I **(arguments and parameters are separate memory, value copied: foreshadows references)** | | |
| 3 | 5 | Booleans and conditionals **(the floating-point equality trap and epsilon, taught here with comparisons)** | R3 Launch Window (writing and testing functions) | A2 Launch Window functions |
| 3 | 6 | Decision structures (chains, `match`, decision tables, condition bugs) | | |
| 4 | 7 | Loops (`while`, counted `for`; **`do-while` as a C++-only "what differs"**) | R4 Airlock (conditionals, decision tables) | A3 Airlock rules |
| 4 | 8 | Loop patterns (accumulate, count, search, sentinel, nested) | | |
| 5 | 9 | Functions II: scope and the call stack **(nested scope, shadowing, the global-variable antipattern, memory reclaimed on scope exit)** | R5 Rover (loops + functions) | A4 Rover drive |
| 5 | 10 | Collections I: lists and vectors **(out-of-bounds: Python `IndexError` vs Rust panic vs the C++ buffer-overflow danger)** | | |
| 6 | 11 | Strings and characters **(character encoding and ASCII, grounded in L2 bits/bytes)** | R6 Mission Readiness Review (cumulative, midterm rehearsal) | A5 cumulative |
| 6 | - | **Midterm** (paper, L1 to L11; may now test type sizes, integer division, float-equality concept) | | |
| 7 | 12 | The memory model **(deepens L2/L4: names as labels, copies vs shared references, Python aliasing, Rust ownership/moves/borrows-lite, the C++ dangling/buffer-overflow danger as why memory safety matters)** | R7 Comms (lists + strings; ciphers, checksums) | A6 Comms encode/decode |
| 7 | 13 | Sharing and mutation (passing collections; Python aliasing gotchas; Rust `&`/`&mut`) | | |
| 8 | 14 | Errors, input, and validation (syntax/runtime/logic; console input and parsing; exceptions vs `Result`) | R8 Comms (memory model; packet buffers) | A7 memory and sharing |
| 8 | 15 | **Numeric robustness** (deepens L2/L6: floating-point accumulation and tolerance, integer overflow: Rust wrap/panic vs Python arbitrary precision; units and magnitudes) | | |
| 9 | 16 | Collections II: dictionaries and maps | R9 Telemetry (validation + floats) | A8 Telemetry watchdog |
| 9 | 17 | Testing (test cases from specs, boundary/error cases, assertions; consolidates the spec-to-tests skill; the debugging *method* is woven into every lecture and threaded via the weekly live-debugging activity, not introduced here) | | |
| 10 | 18 | Program design and modeling (decomposition, from word problem to program; **synthesis only, no new examinable atom**) | R10 capstone Sol 100 (Manifest) | A9 Manifest + critique |
| 10 | 19 | Computing with judgment (evaluate code you did not write; non-exam) | | |

## 4. Invariants re-verified

- **Recitation lockstep:** R2 needs variables (L3, week 2 lecture A, before R2)
  and values/types/expressions (L2). R3 needs functions (L4, before R3). R4
  needs booleans and decision tables (L5/L6, before R4). R5 needs loops (L7/L8,
  before R5). R7 needs strings (L11) and lists (L10, before R7). R8 needs the
  memory model (L12, week 7, before R8 in week 8). R9 needs errors/validation
  (L14) and numeric robustness (L15, before R9). R10 needs dictionaries (L16,
  before R10). All hold, because the order is unchanged.
- **Assignment before recitation:** unchanged from VISION §10, still holds.
- **No new examinable atom after week 9:** testing is L17 (week 9); week 10 is
  program design (synthesis) and non-exam judgment. Holds.
- **No lecture carries two major topics:** L2 is "types and their
  representation" (one topic; the bits/bytes content is the substrate of "what a
  type is", not a separate memory lecture). The deep model stays L12.

## 5. Ripple (on approval)

- **VISION §4 (exams):** machine numbers are no longer a single late topic. The
  midterm (L1 to L11) may test type sizes, integer division, and the
  float-equality concept. The final still weights collections, the memory model,
  errors, testing, and numeric robustness. Update the coverage split.
- **VISION §10:** keep the order; replace the lecture descriptions with the
  enriched ones above; rename L15 to "Numeric robustness".
- **Lecture-notes skill:** add the legibility gate already agreed (no forward
  references, anchored to the ladder) and annotate the ladder so `i32`/`f64` are
  grounded by L2. The gate is what would have caught the pilot.
- **L2 pilot:** re-scope to include the light memory foundation (bits/bytes/
  sizes and the Python-vs-Rust integer models) so `i32`/`f64` are grounded, and
  re-author against the enriched L2 (plus the reviewer's two accuracy fixes).

## 6. Open questions for the instructor

1. Enriching L2 in place keeps 19 lectures and the family lockstep. The
   alternative is a dedicated week-1 "how computers store data" lecture (20
   lectures), which desyncs the recitations by a week and needs the family map
   shifted. The in-place enrichment is recommended; confirm.
2. L15 reframed from "numbers in machines" to "numeric robustness" (a deepening
   that feeds the Telemetry recitation), with the first exposure moved early to
   L2/L6. Does that satisfy "not too late", or do you want overflow/float given
   a fuller early lecture at the cost of the lockstep?
3. Testing and debugging stays a single week-9 lecture (L17). Acceptable, or
   split (which would push program design or judgment out of week 10)?
