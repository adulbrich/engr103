# ENGR 103 Redesign: Course Vision

**Engineering Computation and Algorithmic Thinking, dual-language edition (Python + Rust)**

This document is a blank-sheet redesign of ENGR 103. It starts from the learning
outcomes, derives the assessments from them (exams first, then recitations, then
assignments, then lecture activities), and ends with the lecture schedule. It is
written for a 10-week quarter with a finals week, two 50-minute lectures per week,
and one 110-minute recitation per week scheduled between the two lectures.

The audience constrains everything: ENGR 103 is a prerequisite for every
engineering major in the college and the first real programming course students
encounter. It assumes zero prior programming, and it must serve the chemical,
civil, and construction engineer as well as the electrical and mechanical one.

---

## 1. Mission and design philosophy

### The problem

Every fundamental skill this course teaches (writing, tracing, explaining, and
fixing small programs) can now be performed instantly by generative AI tools that
every student carries in their pocket. Any unsupervised artifact is therefore
unreliable as evidence of learning. Pretending otherwise burns TA time on grading
theater and teaches students that the course rewards prompt-typing.

### The answer

The purpose of an engineering education is to produce people who can **reliably
judge under uncertainty in safety-critical, economically consequential systems**.
For an introductory programming course, that translates to a concrete claim we
must be able to certify for every passing student:

> Given a small program, this student can predict what it does, explain why,
> spot when it is wrong, and build a correct one, with no tools but their own
> understanding.

The redesign follows one rule: **assessment weight lives where supervision
lives**. Everything unsupervised is low-stakes practice; everything high-stakes
happens in a room we control.

### One problem family, four altitudes

The course is organized around six recurring **problem families** (mapped in section 8): arithmetic with units and time, decision tables and interlocks, loops over state and grids, encoding and decoding sequences, validating and summarizing a noisy stream, and key-value lookup and counting. A family is a concept skeleton, not a story. Each family appears
four times, at escalating stakes (difficulty rises gently from activity to
assignment, then holds), wearing a different story line at each tier:

| Altitude | Venue | Stakes | Support | Story line |
|---|---|---|---|---|
| 1. Activity | Lecture, guided, in pairs | Completion (submit .py/.rs on Canvas) | Instructor, peers | None, bare concept |
| 2. Assignment | Take-home, independent | Low (completion + auto-graded correctness) | Anything, including GenAI | Mission Ares |
| 3. Recitation | In person, locked-down computer | High (graded like exams) | Official docs only | Rubber Duck Robotics |
| 4. Exam | In person, on paper, no notes | High | None | None, bare concept |

This makes the GenAI question moot by construction. Students may use any tool on
assignments, and we say so openly: the assignment is the training set, and the
recitation is the test set drawn from the same family. A student who outsources
the training set walks into the supervised event unprepared, and they know it in
week 2, not week 11. The story-line change between tiers sharpens the same
point: a skeleton practiced in one costume must be recognized in another, so a
memorized (or generated) solution transfers only if the understanding did.

### Two languages

The course is taught in **Python and Rust** simultaneously. Students choose
either language for every recitation problem and assignment (they may even mix
within a week). Exams accept answers in either language or in a defined
pseudocode.

Why two languages at all:

- It forces the separation of **concept from syntax**. A student who has seen a
  `while` loop in two notations has learned "loop", not "the `while` keyword".
  This is exactly the transferable knowledge the paper exams certify.
- It makes the memory model teachable. Python's aliasing behavior and Rust's
  ownership rules are two windows onto the same underlying truth (names are not
  values, and shared mutable state is dangerous). Each language makes visible
  what the other hides.
- It reflects engineering reality: computational engineers read and write more
  than one language across their careers, and outcome-level transfer is the
  skill that survives language churn.

Why Rust rather than C++ as the second language:

- **The compiler is a teaching assistant.** Rust rejects the same beginner
  mistakes C++ accepts silently (uninitialized reads, out-of-bounds access,
  dangling references), and it rejects them with the best error messages in the
  industry. C++ punishes novices with undefined behavior; Rust punishes them
  with an explanation.
- **Explicit types and mutability** (`let` vs `let mut`, `i32` vs `f64`) make
  state and types visible, which is precisely what week 1 to 4 students need to
  internalize and what Python hides.
- No header files, no preprocessor, no manual build invocations: `cargo run` is
  as simple as `python3 main.py`.
- Every concept on the exam inventory (section 4) is expressible in a small,
  stable subset of Rust with no lifetimes and no generics.

C++ is not abandoned: a short **"C++ bridge" appendix** in the lecture notes maps
the course subset of Rust onto C++ syntax for students continuing into
C++-based courses. Teaching all three languages as co-equal tracks is explicitly
out of scope; two is the pedagogical sweet spot, three is a logistics tax on
every lab machine, autograder, rubric, and TA.

Why not MATLAB, the tool many majors use downstream: MATLAB is a superb
computational instrument but a poor vehicle for the concepts this course
certifies. Its matrix-first, 1-indexed, weakly typed model is the exception
students would later have to unlearn, and it hides exactly what the exams test
(types, memory, function contracts). The bet is the opposite: certify
language-independent concepts here so that MATLAB, which most majors meet in
their second year, becomes a week of syntax rather than a course. Outcome O7
names this transfer explicitly, and a short **MATLAB bridge** appendix joins
the C++ bridge, mapping the course subset onto MATLAB for students heading
into MATLAB-based major courses. Licensing seals it: Python and Rust run free
on the lab image and in the Gradescope container; MATLAB does not.

The same logic scales down for the longer tail of languages students will
meet. Full bridge appendices exist only where the audience is large and the
need immediate: C++ and MATLAB. The rest live in a single **language field
guide**, a reference page of one-page concept maps (Fortran for the nuclear
and HPC-bound, R for the statistics-bound in bio, environmental, ecological,
and industrial engineering), each written in the same fixed format: the course
subset mapped construct by construct, plus a "what will surprise you" list
(1-indexing in both R and Fortran, for a start). One page per language keeps
maintenance honest, and one page is enough: the point of the concepts-first
design is that students arrive at any imperative language already knowing what
to look for. The format extends cheaply when a new destination language earns
its page.

**Restricted subsets.** Both languages are fenced to a course subset that grows
week by week (published as a "language ladder" page). Early weeks: no
comprehensions, no f-string format specs beyond basics, no lambdas in Python; no
borrowing, no traits, no lifetimes, everything in `main` plus free functions in
Rust. The fences keep GenAI-generated cleverness detectable in take-home work
and keep everyone tracing the same constructs.

---

## 2. Learning outcomes

### Catalog outcomes (unchanged, what the university certifies)

1. Demonstrate the ability to create a computer program to solve a problem using
   universal design.
2. Demonstrate the use of software to perform engineering problem solving.
3. Use critical thinking to identify computational solutions and articulate
   limitations related to social structural inequities such as racial, cultural,
   gender, socioeconomic and accessibility.
4. Describe the separate roles of modeling and analysis in engineering practice.
5. Acquire and apply new knowledge from external sources in engineering
   computation.
6. Refine goals for academic, personal, and professional achievement by
   generating a resume, digital portfolio, and/or code repository.

Catalog outcome 6 is met through a professional development track that is
administered separately from the course content in this document; it does not
appear in the assessments below.

### Operational outcomes (what we actually measure)

Every assessment item in the course maps to at least one of these. The verbs are
chosen to be observable in a proctored setting.

- **O1 Trace.** Given a short program in Python or Rust, predict its output and
  the values of its variables at any point, including through conditionals,
  loops, and function calls, by hand.
- **O2 Explain.** State in plain English what a given program or function does
  and why, at the level of purpose, not line-by-line paraphrase.
- **O3 Write.** Design and implement a program from a specification using
  variables, expressions, conditionals, loops, functions, and basic collections,
  in at least one of the two course languages.
- **O4 Decompose.** Break an engineering problem into inputs, outputs,
  assumptions, and an ordered sequence of steps before writing any code.
- **O5 Test and debug.** Devise test cases (including boundary and error cases)
  for a specification, and locate and fix defects in a program systematically
  rather than by trial and error.
- **O6 Model.** Translate a simple engineering model into a program, respecting
  units and the limits of machine arithmetic, and distinguish the modeling step
  from the analysis step.
- **O7 Transfer.** Map a concept between the two course languages, carry it
  onto the tools of the student's own major (MATLAB for many), and use
  official documentation to learn an unfamiliar library function without a
  tutorial.
- **O8 Judge.** Evaluate a computational solution critically: identify what it
  assumes, where it fails, whom it fails (accessibility and equity limitations),
  and whether code from an external source (a peer, a library, an AI tool) is
  correct and appropriate.

Mapping: exams certify O1, O2, O3, O5, O6 (concept tier). For O6 specifically,
the exam surface is the *concept* of modeling versus analysis (tested as a
concept short answer, archetype 10) plus units and the limits of machine
arithmetic (lecture 15); the modeling-versus-analysis distinction is introduced
in week 1 and threaded all term, and is never hands-on-coded on paper.
Recitations certify O1, O3, O4, O5, O7 (performance tier). Assignments train all
outcomes and carry the O8 thread (reflection, critique); catalog outcome 6 lives
in the separate professional development track. Lecture activities rehearse O1,
O2, O4.

---

## 3. Assessment architecture

| Component | Weight | Venue | Integrity model |
|---|---|---|---|
| Final exam | 25% | In person, paper, no notes, finals week | Proctored |
| Midterm exam | 20% | In person, paper, no notes, 45 min in a week 6 lecture slot | Proctored |
| Recitations (9 graded, lowest dropped) | 40% | In person, locked-down computer, 110 min | Proctored + oral check |
| Assignments (10) | 10% | Take-home | Anything goes; completion plus auto-graded correctness |
| Lecture preparation and activities | 5% | Pre-lecture checks + activities (completion, .py/.rs on Canvas) | Low stakes by design |

Properties worth stating explicitly:

- **85% of the grade is earned in a proctored room.** This is the entire
  integrity policy. There is no cat-and-mouse detection, no accusations, no
  regrade wars over AI suspicion.
- Recitations (40%) weigh as much as the two exams combined (45%), per the
  design requirement that recitations be graded as much as exams. They are the
  performance half of the certificate; the exams are the concept half.
- Nothing unsupervised can move a grade by more than 15%, so nothing
  unsupervised needs policing.
- Drops (one recitation, one assignment) absorb illness and life without an
  excuse bureaucracy; DAS accommodations apply to both exams and recitations.

### Headroom for students who arrive experienced

Some students walk in already knowing half the course. The design gives them
altitude without splitting the class into tracks:

- **The stretch problem** in every recitation is extra credit (up to +10 on a
  100-point session, component capped at its 40% course weight), openly labeled
  as beyond practiced territory. It is the sanctioned place to be challenged,
  and its take-home sibling (the optional summit problem in each assignment)
  is the sanctioned place to train for it.
- **The second language is the real advanced track.** A student fluent in
  Python has never met the borrow checker; the standing challenge for
  experienced students is to solve recitation problems in their weaker
  language. Two-language fluency is worth more than any bonus points, and the
  course says so.
- **Nothing above the ladder is required, everything below it is fair game:**
  experienced students may use any construct already introduced, and the
  archetype inventory gives them a clear picture of what mastery means here
  (tracing and explaining, not just producing).

---

## 4. The exams (design these first; everything else feeds them)

### Format

- Midterm: 45 minutes maximum, on paper, no notes, no devices, held during the
  second lecture slot of week 6 (the 50-minute slot leaves room for seating and
  collection). The time box shapes the paper: five to six short items drawn
  from the quicker archetypes (evaluate, trace, predict output, find the bug,
  complete the code, plus one short write-a-function). Breadth is the final's
  job; the midterm is an early, reliable signal on core mechanics. The week 6
  recitation, held two days earlier, is a cumulative synthesis session that
  doubles as the midterm's performance-tier rehearsal.
- Final: 110 minutes, on paper, in the university final slot. Cumulative, with
  roughly 60% of points on weeks 6 to 10 material.
- Both exams test **introductory programming concepts only**: no git, no shell,
  no editor trivia, no professional-development content, no library trivia
  beyond the published subset. Exam problems wear no story skin: like
  activities, they are stated as bare archetype items.

### Language policy on paper

Code-reading questions appear in Python, in Rust, or in both (same program twice,
side by side, when the point is the concept). Code-writing questions may be
answered in Python, Rust, or the course pseudocode, and the rubric is
**concept-strict, syntax-lenient**: a missing semicolon or `mut` costs nothing;
an off-by-one loop bound or a wrong condition costs points. Each exam's cover
page restates this rule.

### The public archetype inventory

Exams are built exclusively from ten published question archetypes. Students see
the complete list in week 1, practice every archetype in activities and
assignments, and face fresh variations (new values, new contexts, same skeleton)
on exam day. Few problem families with variations, fully disclosed.

1. **Evaluate.** Compute the value and type of expressions (precedence, integer
   vs float division, boolean logic, string operations).
2. **Trace.** Fill a variable-value table for a loop or a call, line by line.
3. **Predict output.** Short program, write exactly what it prints.
4. **Find the bug.** A program with one or two logic errors; identify and fix.
5. **Write a function.** From a short spec, on paper (8 to 15 lines).
6. **Draw memory.** Diagram names, values, and references before and after a
   marked line; or draw the call stack at a marked moment.
7. **Explain.** One or two sentences: what does this function do, at the level
   of purpose.
8. **Choose tests.** Given a spec, propose test cases and identify the boundary
   and error cases (and, for the final, the floating-point traps).
9. **Complete the code.** Fill labeled blanks in a mostly-written program.
10. **Concept short answer.** For example: compile-time vs run-time error, why
    `0.1 + 0.2 != 0.3`, when a copy happens vs when a name is shared.

### Coverage split

- **Midterm (lectures 1 to 10):** values and types, how values are stored (bits,
  bytes, sizes; integer vs floating-point representation), expressions and
  integer division, variables and state, functions, scope and the call stack,
  conditionals (including the floating-point equality trap as a concept), errors
  and validation, and testing (choosing test cases, boundary and error cases).
  Loops and collections are taught after the midterm and first appear on the
  final.
- **Final (cumulative, weighted to the second half):** loops and loop patterns,
  strings and characters, collections (lists and vectors), the memory model
  (aliasing, references, ownership-lite), sharing and mutation, plus (from the
  first half) errors and validation and testing; numeric robustness
  (floating-point tolerance, integer overflow) is woven and tested as concepts;
  dictionaries are a week-10 extra and not required on the exams;
  debugging, and program design (the last
  as synthesis of skills practiced all term, introducing no new archetype;
  modeling versus analysis is tested only as a concept and is introduced in
  week 1).

Every archetype-by-topic cell that can appear on an exam has appeared at least
once in an activity, an assignment, or a recitation first. The exams contain no
ambushes; they contain variations. A second invariant guards the runway: **no
new examinable archetype-by-topic cell is introduced after week 9.** Week 10 is
synthesis (lecture 18) and non-exam judgment (lecture 19) only, so every exam
topic has a full assignment-and-recitation practice cycle before the final.
Concepts that culminate late (the systematic debugging method, the
modeling-versus-analysis distinction) are introduced in week 1 and threaded, then
consolidated, never first-taught in week 10.

---

## 5. Recitations (the performance certificate)

### Format

110 minutes, weekly, between the two lectures, in person, on **locked-down lab
machines** provided by the course. Each machine is imaged with the
toolchains and editors only (Python 3, Rust, VS Code, vim); the week's
starter repository, cloned at session start, brings the problem specs,
scaffolds, tests, and the local `check` command. Each machine
enforces its own allowlist (baked into a read-only image, no student root):
the official Python and Rust documentation, the course starter repositories
on OSU GitLab, and Gradescope for submission, plus the login chain those
require, over ordinary campus WiFi. No
search engines, no AI, no mail or chat, no personal devices (Canvas is
deliberately excluded during sessions: its messaging is a live channel).
`ARCHITECTURE.md` specifies the full setup. Fallback if machines are
unavailable: locked-down laptops or a paper-based contingency version of the
problem set (each recitation is authored with a paper variant for DAS and
make-up cases anyway). Whether that paper variant should instead become the
primary format, replacing the locked-down machines entirely, is an open
question (section 12).

### Scope and difficulty

Recitations wear the Rubber Duck Robotics story line and its weekly ticket
(section 8); each session is one ticket from Gary, and the find-the-bug portions
arrive as Gary's subtly wrong helper awaiting sign-off. Recitation in week N
assesses the **two lectures of week N-1**, which students have had a full
weekend and one assignment to practice; the current week's Monday lecture is
assessed the following week.

Recitations are deliberately **not much harder than the assignments** that
prepare them. The variable a recitation adds is the conditions, not the
difficulty: locked down, alone, docs only, time-boxed. High-stakes events are
stressful enough on their own, so testing practiced content under supervision
measures understanding, while testing novel difficulty under supervision
measures stress tolerance. The course does test composure, because working
engineers face unpracticed problems under pressure, but only in one disclosed
place (the stretch problem) and only as extra credit; the graded core never
ambushes. A student who did the week's assignment honestly
should walk in expecting to succeed, and that expectation is the point: it is
what makes outsourcing the assignment self-defeating, and it keeps test
anxiety from swamping the signal. Recitation problems are fresh variations of
practiced skeletons (new costume, new values, at most one small twist), never
new territory; the scaffolding is thinner (no hints, no worked sub-steps), and that is the
only support removed. The recitation is where students demonstrate they can do
the practiced work alone; the exams then confirm the underlying concepts
transferred off the keyboard.

### Structure of a session

- **Warm-up (15 min, 25% of points).** One direct application of the current
  problem family, at assignment difficulty. Everyone should finish it.
- **Core problems (70 min, 75% of points).** Two problems at assignment
  difficulty: the same skeletons students practiced, in fresh Rubber Duck costume with
  new values and at most one small twist, each solvable in either language.
  Provided test suites give partial credit objectively; hidden cases run at
  grading time. Warm-up plus core is the full 100%: a prepared student can earn
  a perfect session without touching the stretch.
- **Stretch problem (remaining time, extra credit up to +10).** The one place
  where difficulty genuinely rises, and sometimes it deliberately tests
  composure in unpracticed territory, because engineering practice includes
  exactly that. The course is upfront about it: every stretch problem carries
  the same label ("this is beyond what you practiced; attempting it can only
  help you"). A session can therefore reach 110%, but the recitation component
  still caps at its 40% course weight, so stretch credit can repair a weaker
  week beyond the drop, and never raises the stakes of this one. Partial
  credit for a written Polya plan (restated requirements, decomposition,
  planned tests; O4, section 9) even without working code.
- **Oral check (embedded, gate not points).** Each week a random half of the
  roster gets a visit, about six checks per TA per session, and every student
  is checked at least four times a term without knowing which weeks. The check
  is one fixed-form question about the student's own code, designed to be
  answerable with a value, a line number, or a single word ("what does this
  print if the input is 0?", "point to the line that stops the loop"), never
  with a paragraph. It is scored on understanding signals, not English
  fluency, and a student may answer by pointing, writing, or drawing. Scored
  satisfactory or not on a three-item rubric; two unsatisfactory checks in a
  term trigger an instructor conversation. This is the viva backstop that
  keeps memorized solutions from passing, at a fraction of the cost (and the
  language bias) of one-on-one demos. TAs are trained on the rubric in week 0
  with calibration examples.

### Grading and logistics

- Weeks 2 to 10 are graded (9 recitations); the lowest is dropped, so 8 count
  at exactly 5% each. Week 1 is an ungraded onboarding session (environment,
  editor, terminal, git, "hello" in both languages). The week 6 recitation is
  cumulative synthesis over the midterm scope and sits two days before the
  midterm,
  so it is both a graded performance assessment and the best possible exam
  rehearsal.
- Submission is continuous and direct: students upload to Gradescope from
  their seats as often as they like, exactly as they do for take-home
  assignments, and the last submission before the TA closes the window
  counts. Same workflow at both tiers, no minute-110 crunch, no lost work
  when a machine dies late. Gradescope's submission list doubles as the TA
  dashboard, which lets the oral-check sampling target students whose code
  exists, and its timestamps are the audit trail. If the classroom WiFi
  fails mid-session, students keep working (`check` is local), a TA phone
  hotspot serves as the emergency uplink (enforcement lives on the device,
  not the network), and a TA-carried USB stick is the deep fallback
  (`ARCHITECTURE.md`, failure ladder). Rubric: provided tests 50%,
  hidden tests 30%, style and decomposition 20%, with the style share scored
  by the course linter configurations (ruff, clippy) and TAs spot-checking
  only what linters cannot see.
- Language choice is per problem. The starter repo contains twin scaffolds
  (`problem1/py/` and `problem1/rs/`) with identical test cases.

### Staffing and scale (25 students, two undergraduate TAs)

The session works at a 25:2 ratio only because recitations are exams, not help
labs. TAs proctor and check; they do not tutor during the session. The place
for help is office hours and the assignment week, so no help queue competes
with the oral checks. Per session, that leaves passive proctoring (both TAs,
continuous), about six oral checks per TA at two to three minutes each, and
closing the Gradescope window at the end; submission happens seat-by-seat,
directly to Gradescope, throughout the session. Grading afterward is autograded tests plus
linter-scored style; the human share is stretch-problem Polya plans and
whatever the linters cannot judge, done asynchronously in Gradescope's
problem-at-a-time rubric view. That is one to two hours per TA per week, which
two undergraduates can actually sustain.

The current course's integrity tool, the one-on-one assignment demo, is
retired entirely and deliberately: demos cost three rounds of TA time per
term, verify code after the fact when the talking can be coached even if the
code was outsourced, penalize students who understand but cannot narrate
fluently in English, and overflow to the instructor. The replacement
principle: **supervised production replaces after-the-fact interrogation**.
The recitation watches the work happen instead of asking students to defend
it later.

If the ratio still strains (larger sections, a weak TA cohort), the fallback
ladder in order of preference: (1) reduce oral checks to two scheduled
five-minute vivas per student per term, sampled a few students per week;
(2) replace the live check with a per-student randomized written explain item
inside the session (archetype 7, answered on the machine), which costs zero
session time and keeps most of the deterrence; (3) deep-grade a published
subset of problems each week and completion-grade the rest. What never gives:
the proctored setting itself and the autograded correctness core.

---

## 6. Assignments (the training set)

### Role

Assignments exist to make students ready for the next recitation and,
cumulatively, for the exams. They are low-stakes (10% total; the auto-graded problem set is scored for
correctness, and the paper practice and reflection for completion), and they
stay low-stakes precisely because they are unsupervised. The course says this
out loud in week 1: "You may use any tool on assignments, including AI. The
recitation will test fresh variations of these same problems, in the lab,
without AI. If you
did the practice yourself, the recitation will hold no surprises. Skipping the
practice only shows up in the 85% of the grade you earn in the room."

Assignments wear the Mission Ares story line (section 8): each problem
set arrives as a mission episode and includes HAB's subtly wrong snippet
awaiting sign-off, the take-home rung of the red herring ladder. The upcoming
recitation assesses the same skeletons in Rubber Duck Robotics costume.

### Structure (weekly, always due before the recitation it prepares)

Timing is governed by one invariant: **the assignment practicing a family is
due before the recitation that assesses that family**, which in turn precedes
the exam that distills it. Concretely, each assignment is released right after
a recitation and due the Monday evening before the next one, covering the two
lectures of the previous week that the next recitation will assess. Students get
a full week with the earlier of those two lectures and a full weekend with the
later one before being graded on it.

Each assignment has three parts:

1. **Problem set (auto-graded).** Three to five problems in the same families
   as the upcoming recitation, at the same difficulty: the recitation siblings
   will differ in costume, values, and at most one small twist, not in level.
   Starter code is
   distributed as self-contained git repositories on OSU GitLab, each
   carrying the spec (as its README), the twin scaffolds, the visible tests,
   and the `check`/`pack` tools; submission and autograding run on
   Gradescope, whose Docker-based autograder is language agnostic, so a single
   custom container image carries both the Python test runner and the Rust
   toolchain. Passing the visible tests earns the credit. Until the
   errors-and-validation week (week 8), every problem is a set of functions
   written against a provided harness: the `main`, all console I/O, and the
   local test runner are given, so grading is direct function calls rather than
   fragile output parsing. Twin scaffolds in both languages; through week 3, students must
   submit at least one problem in each language (to force an informed choice of
   their main language), free choice afterwards. From week 3 on, one problem
   per set is the spec-to-tests problem (section 9): given requirements and a
   signature, write only the test suite.
2. **Paper practice (self-checked).** One tracing or memory-diagram exercise in
   exam archetype format, done by hand, photographed and attached. The worked
   solution sits in a closed reveal on the assignment page: the student attempts
   the trace, then opens it to self-check. Graded on completion, not
   correctness. This is the only regular rehearsal of the paper modality outside
   lectures, so it is non-negotiable in every assignment.
3. **Reflection line (two sentences).** The student must explicitly disclose
   whether they used AI and, if so, for what (and state plainly when they did
   not), then name any other help used (peers, docs, the notes) and one thing
   they still cannot do without it. Ungraded content, graded completion; the
   explicit disclosure normalizes honesty about tool use and feeds the
   instructor's weekly misconception review.
4. **Summit problem (optional, worth nothing).** One problem at stretch
   altitude, the take-home training ground for the recitation's stretch
   problem. It carries no points at all, on purpose: extra credit at the
   take-home tier would reward spare time rather than mastery. What it earns
   is preparation for the +10 that is available in the room.

### Sizing to the weekly budget

The course is a three-credit class, budgeted at roughly **nine hours of student
work per week**: about 3.5 scheduled hours (the two lectures and the
recitation) leave about 5.5 hours for everything else. Assignments are authored
to fit inside that budget, not to fill it:

- **Problem set:** a **median completion time of about 2.5 to 3 hours** for a
  student who did the reading, tests included. The "three to five problems"
  count bends to this target and never past it: five short problems or three
  slightly larger ones, whichever hits the time, all at assignment (not
  stretch) difficulty.
- **Paper practice:** one exercise, 30 minutes or less.
- **Reflection line:** a few minutes.
- **Summit problem:** optional and uncounted, so it never enters the budget.
- **Pre-lecture reading:** each lecture's notes are sized to about 45 minutes,
  read primarily in the student's chosen language while skimming the other;
  the "what differs" call-out is the part read in full.

Exam-adjacent weeks (week 6 around the midterm, week 10 around the Sol 100
capstone) are the pinch points, so keep those assignments sized to the same
weekly budget as any other week, never heavier. Exam rehearsal itself lives
outside the assignment track, in separately provided practice materials, rather
than in a bulked-up assignment.

### The critique thread

Catalog outcome 3 (equity and accessibility) lives in the assignment track, not
the exams: the final assignment includes a one-page critique of a computational
tool's accessibility and equity limitations (O8), completion-graded like
everything else in the track. Catalog outcome 6 (resume, portfolio, repository)
is not an assignment: it belongs to the separate professional development
track.

---

## 7. Lectures and activities (the guided tier)

### Flipped format

Lecture notes are read **before** class. A three-question pre-lecture check
(auto-graded, due an hour before lecture, counts toward the 5% preparation
grade) verifies the reading and, more importantly, tells the instructor which
misconception to open with. Lecture time itself is spent almost entirely on
examples, activities, and problems; the notes carry the exposition. Only lecture
1, and perhaps lecture 2, receive a full in-class walkthrough (the onboarding
week); from then on the notes are the exposition and class time is activities.

### Shape of a 50-minute session

- **5 min: misconception opener.** The most-missed pre-check question, live.
- **3 activity blocks of about 13 min each.** Each block: pose a problem (an
  exam archetype at guided difficulty), students attempt it solo or in pairs on
  paper or in the browser, then resolve it live. Activity types rotate:
  - **Predict-then-run:** commit to an output on paper, then run it and
    reconcile. The gap between prediction and reality is the lesson.
  - **Same program, two languages:** the concept in Python and Rust side by
    side; students identify what changed and what could not change.
  - **Live debugging:** a planted-bug program fixed as a class, narrating the
    systematic method (read the error, form a hypothesis, test it).
  - **Peer instruction:** a concept question with distractors mined from last
    week's recitation mistakes; vote, discuss, revote.
  - **Paper tracing:** exam archetype 2 or 6 by hand, since the exam is on
    paper and hands need reps.
- **2 min: bridge.** Exactly which recitation problem family this session fed.

Activities are the first altitude of each problem family: every activity problem
is the easiest sibling of an assignment problem, which is the easier sibling of
a recitation problem, which is the ancestor of an exam variation. Students can
literally see the escalator.

---

## 8. Problem families and story lines

A **problem family is a concept skeleton**: accumulate over a stream, apply a
decision table, walk a grid, encode and decode a sequence, validate against
tolerance, look up and count by key. A **story line is a costume** worn at one
assessment tier, and each tier dresses differently: activities and exams have no
story (bare concept, fastest possible read), assignments live at Mission Ares,
and recitations wear Rubber Duck Robotics. Every problem must be
solvable while ignoring the plot entirely; the story is a spine, not a cage.

Each story line is its tier's common thread (fil rouge), and the thread is
technical as much as narrative: within a tier, data formats, function names,
and provided helpers recur across the term. The telemetry line format
introduced in the week 8 assignment is the same one the Sol 100 capstone
assignment parses; the recitations' Rubber Duck tickets all follow the same RDR
firmware conventions. Later weeks should feel
like returning to a familiar codebase rather than starting from zero. One
limit keeps grading fair: every week remains independently solvable, and no
problem ever requires last week's solution. A student who missed a recitation
is behind in the plot, never in the prerequisites.

The six skeletons and their two faces:

| Family (skeleton) | Rubber Duck face (recitations) | Mission Ares face (assignments) | Core topics | Weeks |
|---|---|---|---|---|
| **Arithmetic with units and time** | Duck-race countdown timers, novelty-mug fill volumes | Sols vs Earth days, fuel mass fractions, delta-v budgets | Expressions, types, variables, functions | 1 to 3 |
| **Decision tables and interlocks** | Hot-tub controller: temperature, occupancy, and timer cutoff rules | Never both doors open, pressure thresholds, suit checks | Booleans, conditionals, decision tables | 3 to 4 |
| **Loops over state and grids** | Warehouse duckbot fetching parts, ASCII duck-parade rendering | Drive commands, battery budget, terrain rendering | Loops, loop patterns, nested loops, functions | 4 to 6 |
| **Encode, decode, clean a sequence** | Garbled customer order codes, novelty-pager messages | Ciphers, checksums, packet buffers | Strings, lists/vectors, memory model, sharing | 6 to 8 |
| **Validate and summarize a noisy stream** | The smart bathtub's water-level sensor returns nonsense | Dust-storm sensor streams, tolerance alarms | Errors, input validation, floating point, RNG noise | 8 to 9 |
| **Key-value lookup and counting** | Duck SKU inventory after a warehouse mix-up | Cargo inventory, resupply planning (the Rust pun is intended) | Dictionaries/hashmaps, choosing structures | 9 to 10 |

The **Weeks** column above reflects the pre-reorder plan. Section 10's schedule
now teaches errors and testing before the midterm and loops after it, so the
family week-ranges are re-derived to follow that lecture order when the
recitations are authored: the loops-over-state-and-grids family moves to the second half, the key-value-lookup-and-counting family becomes a week-10 extra, and the capstone is
list-based. The prerequisite rule is unchanged: each family's recitation follows
the lecture that teaches its concept.

Why these instead of the current course's calculator, financial planner,
dictionary, and linear-equation solver: same concepts, but every problem has a
visible consequence. A rover renders its path, an airlock refuses to kill the
crew, a garbled message decodes into words. Fun in an intro course is mostly
fast, visible feedback plus a reason to care, and these families are chosen so
that plain terminal output is the payoff, no graphics stack required.

### Mission Ares (assignments)

The student is a member of the flight software crew of a crewed Mars habitat,
and the assignments follow a small plot, one episode per assignment: launch
preparation, transit and its interlocks, landing and the first drive, then the
surface phase (a comms window with Earth, a solar conjunction that garbles
packets, a dust storm that stresses the sensors), building to the **Sol 100**
status report as the capstone assignment: parse a telemetry line, validate it,
update the manifest, decide whether to raise an alarm, format the report. The
exact episode-to-week mapping is derived when the assignments are authored,
following the reordered family weeks (section 10). The habitat setting covers
every engineering major in the room: life support and water recycling are
chemical, environmental, and ecological engineering; the rover is mechanical;
power, comms, and protection interlocks are electrical and energy systems;
habitat structures and build-out sequencing are civil, architectural, and
construction engineering; crew biosensors are bioengineering; radiation dose
monitoring is nuclear; inventory and operations are industrial. Assignment
problem contexts deliberately rotate across these subsystems so every major
sees its own field in the mission more than once a term.

The recurring character is **HAB**, the habitat's onboard AI assistant, which
is helpful, tireless, and confidently wrong at narratively convenient moments.
Flight rule: no HAB code runs unverified. HAB is how find-the-bug and
judge-this-code problems arrive in story ("HAB drafted this pressure check;
sign off or reject it"), which makes outcome O8 a weekly reflex instead of a
week 10 lecture topic. Placing HAB in the assignment tier is deliberate: it is
the one tier where students may use AI freely, so a confidently wrong AI whose
code they must judge is exactly the right training partner there, and the heroic
tone gives the take-home work a reason to pull the student through practice they
might otherwise skip or outsource.

### Rubber Duck Robotics (recitations)

The graded recitations are set at **Rubber Duck Robotics**, a mediocre but
lovable novelty-gadget company (self-stirring mugs, motivational bathtub ducks,
the occasional smart hot tub). In the room the student is the new firmware
intern; each recitation arrives as a ticket from **Gary**, the senior engineer,
who is enthusiastic, overcommitted, and reliably wrong about one thing, and
Gary's subtly wrong helper is the code to verify before use. The tone is comic
where the take-home mission is heroic, and that is deliberate: the comedy
lightens the test anxiety a high-stakes proctored room otherwise breeds (a
novelty-mug problem worth points is still worth points), and the company name is
a planted joke that pays off when rubber-duck debugging is introduced. Stakes
come from proctoring and grade weight, not from tone.

### The red herring ladder

Real specifications contain noise, so problem statements plant calibrated
distractors that scale with the tier:

- **Activities:** none; bare concept.
- **Assignments:** one mild spec distractor (a quantity that is never needed),
  plus **HAB's snippet**: a provided function that looks convenient but is
  subtly wrong or unnecessary. Copying it uncritically costs a visible test.
- **Recitations:** fuller specifications with irrelevant data fields, plus
  Gary's helper that must be verified before use.
- **Exams:** the distilled paper version, the find-the-bug archetype.

The rule that keeps this honest: a correct solution never depends on noticing
the herring. Distractors add noise, never ambiguity, and punish only uncritical
copying. The ladder trains O4 (extract the computational core from a noisy
spec) and O8 (judge code you did not write).

### Rules for family authors

- Every family must work identically in both languages with one shared test
  suite.
- Every family must have at least one paper-friendly sibling per exam archetype
  it touches (a rover trace works as well on paper as in a terminal).
- Same skeleton, different surface: an assignment problem and its recitation
  sibling must differ in story, values, and at most one structural twist (an
  extra edge case, one more parameter), never in underlying concept and never
  in level. The recitation sibling must feel like a fresh variation of
  practiced work, not a harder problem. Difficulty headroom belongs to the
  stretch problem alone.
- Activities stay story-free, but may borrow any context that serves the
  concept (a Wordle-style guessing round is a fine strings activity).
- Every family problem labels its modeling step (building the representation of
  the engineering situation) and its analysis step (running it and interpreting
  the result), so the modeling-versus-analysis distinction (O6, catalog outcome
  4) is rehearsed continuously rather than taught once.

---

## 9. House styles: how each artifact reads

Each artifact group is written in exactly one genre, chosen for the job the
artifact does. Mixing genres is the most common authoring failure mode (notes
that drift into step-by-step instructions, assignment specs that drift into
tutorials), so the table is normative:

| Artifact | Genre | Shape |
|---|---|---|
| Lecture notes | Explanation | Wordy, paragraph-based prose; concept first; small self-contained tabbed Python/Rust examples |
| Language ladder, archetype inventory, language field guide | Reference | Terse tables and lists, no prose |
| Practicalities | How-to guides | Numbered steps, one goal per page: setup, editors, submitting to Gradescope, the recitation lab, the debugging workflow |
| Activities | Prompt sheets | Minimal: a listing, a question, room to predict; worked resolution published after class |
| Assignments | Requirements, scaffolded | Mission episode: same spec template as recitations, plus hints and worked sub-steps |
| Recitations | Requirements, bare | Gary's ticket: the same template, scaffolding stripped |
| Exams | Archetype items | Minimal wording, no story skin |

### The plain-language rule (applies to every genre)

Every artifact is written for a reader who is in their first year, has never
programmed, and may not speak English as a first language. Concretely:

- Short sentences and common words; no idioms. "Wordy" in the lecture-notes
  style means generous with explanation, never ornate with vocabulary.
- Every technical term is defined in plain words at first use and then used
  consistently forever: one name per concept across all artifacts (a function
  is always a function, never a method, routine, or procedure), backed by one
  shared glossary page.
- No example uses syntax that is not yet on the language ladder.
- Story-line flavor never obscures the technical ask: requirements are
  numbered, testable, and fully understandable with the story skipped.
- A sentence that can be misread is rewritten, not footnoted. Ambiguity in a
  spec is a bug of the same severity as a bug in the autograder.

### Lecture notes: explanation

Notes answer "what" and "why" in full paragraphs, read before class. Each
concept section motivates the concept, defines it, shows it in both languages
(examples must be self-contained: runnable as pasted, no project scaffolding),
contrasts the two renderings in a "what differs" call-out,
and names the problem family where the concept will be exercised. Notes never
give step-by-step instructions; explaining is their job, instructing is the
how-to guides' job.

### Activities: prompt sheets, not tutorials

In a course where students may skip early lectures, activities must read like
tutorials because they replace the lecture. Here the flipped notes carry the
exposition and the instructor runs the activity live, so a tutorial-style
document would compete with the session it supports. The activity artifact is
deliberately thin: a code listing, a question, space to commit to a prediction.
The worked resolution, published after class, is what serves absentees. The
only true tutorials in the course are week 1's onboarding (R1, A0, and the
setup how-tos they lean on).

### Assignments and recitations: one genre, two scaffolding densities

Both read as requirements, and that is the point: the training set and the
test set share a format, so the format itself is never the surprise. One
template for both: **Context** (the story beat, skippable), **Requirements**
(numbered, testable statements), **Interface** (provided signatures and types),
**Acceptance tests** (the visible cases), **Assumptions and edge cases**.
Assignments add scaffolding inside that template: hints, worked sub-steps,
mission commentary, a suggested order of attack. Recitations strip the
scaffolding and say less. Support moves between the tiers; difficulty holds,
and the genre never changes. Students never author prose requirements themselves: reading
specifications is the course skill, and writing them is a later course's job.

### Tests as requirements (what replaces pre-assignments)

The pre-assignment ritual (students submit their own explanation, pseudocode,
and test plan before coding) is deliberately absent: unsupervised, it produces
GenAI theater and TA grading load with little signal. The skills it targeted
survive in machine-checkable or supervised forms:

- **Requirement comprehension** is trained by the red herring ladder (section
  8), probed live by the oral check, and tested on paper by the explain
  archetype.
- **Test writing** gets a dedicated problem type: from week 3 on, each
  assignment contains one **spec-to-tests problem**, where the student receives
  requirements and a function signature and writes only the test suite.
  Gradescope grades it by running the suite against a hidden reference
  implementation (every test must pass) and against hidden buggy variants
  (each must be caught by at least one test). A test suite is an executable
  statement of the requirements, which makes "did you understand the spec"
  autogradable. Early sets use a plain data format (input, expected output);
  assertion syntax arrives with the testing lecture. The same skill appears on
  paper as archetype 8, and it graduates to the supervised tier in the
  capstone recitation, which requires a test suite alongside the program.
- **Design and pseudocode** live in the Polya thread below.

### The Polya thread

Polya's method (understand, plan, execute, look back) is the course's standard
way to attack a specification, and it lives in graded surfaces rather than on
a page students read once:

- The week 1 notes introduce it as the way to read any spec in this course.
- Recitation stretch problems award partial credit for a written Polya plan
  (restated requirements, decomposition, planned tests) when code is
  incomplete, which makes the plan worth writing exactly when it matters.
- The oral check probes the understand and look-back steps ("what would this
  do on an empty stream?", "how do you know it works?").
- Lecture 18 (program design) is Polya at full scale, from word problem to
  program.

The existing Polya practicalities page survives as the canonical reference,
linked from week 1 and from every stretch problem.

---

## 10. Lecture schedule (the lecture notes contents)

Sixteen outcome-bearing lectures across weeks 1 to 9, plus two advanced-extra
lectures in week 10, with the midterm in week 6. The Winter-term calendar gives
twenty Monday/Wednesday slots, minus the MLK holiday (week 3 Monday, no class)
and the midterm slot, leaving eighteen teaching sessions. **All learning
outcomes are met by the end of week 9; week 10 introduces only advanced material
not required for the outcomes.** The term is correctness-first: the first half
(through the midterm) teaches foundations, functions, scope, conditionals,
errors, and testing without iteration; loops and data come after the midterm.

Notes are concept-first and dual-language throughout: every concept section ends
with a tabbed Python/Rust example pair and a "what differs"
call-out. **The notes are read before class (the flipped model, section 7):
class time is spent on activities, and only lecture 1 (and perhaps lecture 2)
receives a full in-class walkthrough; every other lecture's exposition lives in
its notes.** The reusable memory stepper and the binary/bits visualizer are
embedded wherever the representation or memory picture helps (variable
assignment, function calls, scope, aliasing, sharing), not confined to one
lecture.

The recitation and assignment columns below show the intended
lecture-to-family alignment. Because this schedule reorders the term (errors and
testing before the midterm, loops after it), the exact week each family
recitation lands, and the family week-ranges in section 8, are
**re-derived to follow this lecture order when the recitations are authored**.
Each recitation assesses the previous week's two lectures (the one-week lag),
and the resulting week-by-week alignment is set out on the schedule page; the
prerequisite rule holds (each family's recitation follows the lecture that
teaches its concept).

| Wk | Lecture | Notes content (read in advance) | Recitation that week | Assignment due before that recitation |
|---|---|---|---|---|
| 1 | 1 | **How programs run.** Computation and algorithms; source code, interpreters (Python) and compilers (Rust); running a program; printing results; errors as messages, not verdicts; the systematic debugging method and the modeling-versus-analysis distinction, both introduced here and threaded all term; the two-language philosophy and the language ladder. (This lecture, and perhaps lecture 2, get a full in-class walkthrough; the rest are read before class.) | R1 (ungraded): environment, terminal, editor, the `check` harness, hello in both languages. | A0: setup, toolchains, hello in both languages. |
| 1 | 2 | **Data representation and memory.** Values (integers, floats, booleans, strings) and their types; static vs dynamic typing; binary and decimal; bits and bytes; memory as addressable boxes; type sizes; Python's arbitrary-precision `int` vs Rust's fixed `i32`/`f64`, and what the numbers in `i32`/`f64` mean. Taught with the interactive binary/bits visualizer and a memory-box diagram, before any type name is used to mean a bit-width. Character encoding and ASCII are deferred to lecture 13, where characters are taught. | | |
| 2 | 3 | **Expressions and operators.** Arithmetic operators and precedence; integer vs float division and truncation (grounded in lecture 2's types and sizes); implicit coercion; mixed-type expressions; evaluating expressions by hand. | R2: expressions and types. | A1: Arithmetic families + paper trace. |
| 2 | 4 | **Variables and state.** Names vs values; assignment and rebinding; `let` and `let mut`, shadowing; initialization and uninitialized reads (Rust forbids, C++ undefined behavior); constants; tracing with a variable table. Uses the memory stepper. | | |
| 3 | 5 | **Functions I.** Defining and calling; parameters, arguments, return values; signatures and types; arguments and parameters are separate memory and the value is copied; how the autograder calls your functions. Uses the memory stepper. (Week 3 Monday is the MLK holiday, so this is the only week-3 lecture.) | R3: functions and variables. | A2: Function families + paper trace. |
| 4 | 6 | **Scope and the call stack.** Local and nested scope; shadowing; lifetimes of names; the global-variable antipattern; the call stack drawn by hand; decomposing a program into functions. Uses the memory stepper. Taught before control flow because scope governs the blocks inside conditionals and loops. | R4: functions and scope synthesis. | A3: Scope and call-stack practice. |
| 4 | 7 | **Booleans and conditionals.** Comparisons, logical operators, short-circuit; `if`/`else`; building conditions from specifications; the floating-point equality trap and comparison with tolerance, grounded in lecture 2. | | |
| 5 | 8 | **Decision structures.** `elif`/`else if` chains and `match`; nesting vs chaining; decision tables; guard clauses; common boundary bugs. | R5: conditionals and decision tables. | A4: Decision-table rule families + paper trace. |
| 5 | 9 | **Errors, input, and validation.** Kinds of errors (syntax, run-time, logic); console input and parsing; exceptions vs `Result`; validating a value and failing loudly. (The loop-based validate-until-correct pattern is deferred to lecture 12.) | | |
| 6 | 10 | **Testing.** Test cases from a specification; boundary and error cases; assertions and test functions in both languages; consolidates the spec-to-tests skill practiced since week 3. | R6: cumulative synthesis over lectures 1 to 10; doubles as midterm rehearsal. | A5: cumulative families + paper trace. |
| 6 | | **Midterm**, on paper during the second lecture slot (lectures 1 to 10: foundations, functions, scope, conditionals, errors, testing; no loops). | | |
| 7 | 11 | **Loops.** `while` for unknown counts, counted loops; loop variables; termination; `do-while` as a C++-only "what differs"; tracing loops. | R7: loops. | A6: Loop families + paper trace. |
| 7 | 12 | **Loop patterns.** Accumulate, count, search, sentinel, validate-until-correct; nested loops (ASCII rendering); choosing the pattern from the problem statement. | | |
| 8 | 13 | **Strings and characters.** Strings as sequences; characters and character encoding (ASCII, introduced here and grounded in lecture 2's bits and bytes); slicing, searching, building strings. | R8: strings and lists (ciphers and checksums). | A7: Encode/decode families + paper trace. |
| 8 | 14 | **Collections I: lists and vectors.** Indexing, length, iteration; growing and mutating; out-of-bounds as Python's `IndexError` vs Rust's panic (and the C++ buffer-overflow danger that motivates both). | | |
| 9 | 15 | **The memory model: aliasing and ownership.** Names point at values; aliasing in Python (two names, one list) and ownership/moves/borrows-lite in Rust; the C++ dangling/buffer-overflow danger as why memory safety matters; drawing memory diagrams. Uses the memory stepper. (Late because aliasing needs lists to exist first; the foundation and the stepper are early.) | R9: memory model and sharing (packet buffers). | A8: memory/sharing families + memory-diagram paper practice. |
| 9 | 16 | **Sharing and mutation.** Passing collections to functions; when the caller sees your changes; defensive copying; Rust `&`/`&mut` and why Rust makes you declare intent. Uses the memory stepper. | | |
| 10 | 17 | **Collections II: dictionaries and maps** (advanced extra, not required for the outcomes). Dictionaries and hashmaps; key-value thinking; choosing between list and map; frequency counting and lookup. | R10: capstone recitation, a cumulative Rubber Duck synthesis over the term's families. | A9: the list-based Sol 100 mission-status capstone program with tests; accessibility/equity critique; paper trace. |
| 10 | 18 | **Computing with judgment** (advanced extra, not required for the outcomes). Evaluating code you did not write (a peer's, a library's, an AI's); limitations and failure modes; who is excluded by our tools; course synthesis and final-exam concept map. | | |

Alignment checks built into the table:

- Every recitation's material has had at least one full lecture, one assignment,
  and several activities before it is assessed, and the assignment covering a
  family is always due before the recitation that assesses it.
- **All learning outcomes are met by the end of week 9.** Week 10 introduces only
  advanced material not required for the outcomes; nothing examinable is
  first-taught there.
- **Scope is taught before control flow** (lecture 6, before conditionals and
  loops), because scope governs the blocks inside `if`, `while`, and `for`.
- **Errors and testing are taught before the midterm** (lectures 9 and 10),
  reinforcing the test-and-debug discipline threaded from week 1; loops come
  after the midterm.
- **The light memory foundation is early** (lecture 2, with the interactive
  binary/bits visualizer and memory-box diagram); the deep memory model is late
  (lecture 15) only because aliasing needs lists to exist first. The reusable
  memory stepper is used wherever the memory picture helps (lectures 4, 5, 6, 15,
  16), not confined to one lecture.
- The midterm covers lectures 1 to 10 (no loops). No new examinable
  archetype-by-topic cell is introduced after week 9; concepts that culminate
  late (the systematic debugging method, the modeling-versus-analysis
  distinction) are introduced in week 1 and threaded, never first-taught in week
  10.
- Tooling (git, shell, editor, file I/O) lives in activities and assignments
  only, never on exams; git in particular is a standalone extra-credit activity.
- No lecture carries two major topics; lecture 2 is "values and their
  representation" as one topic.

---

## 11. What carries over from the current course

- **Lecture notes:** most existing pages (variables, expressions, operators,
  booleans, if-statements, loops, functions, scope, strings, arrays, references,
  error handling) map directly onto the new schedule; the work is converting
  C++ examples to tabbed Python/Rust pairs and adding the "what differs" call-outs.
  `cpp-basics.mdx` becomes the C++ bridge appendix. The `CppMemoryStepper`
  component is ported to a dual-language memory stepper (Python aliasing vs Rust
  ownership) and used as a reusable tool wherever the memory picture helps
  (lectures 4, 5, 6, 15, 16, and archetype 6 practice); a new binary/bits
  visualizer and memory-box diagram support the week-1 data-representation
  lecture.
- **Studios** become the assignment problem sets (their current difficulty is
  the take-home tier); recitation problems are authored new as same-level
  variations of the assignment problems.
- **Assignments** (calculator, financial planner, dictionary, linear equations)
  are retired as framings, but their mechanics survive inside the mission
  families: the financial planner's accumulation loops become loops-over-state-and-grids problems, the dictionary becomes the key-value lookup and counting family, and the linear-equation and calculator arithmetic becomes arithmetic-with-units-and-time computations.
- **Practicalities** (Polya, debugging, style, gen-ai) survive intact; the
  gen-ai page is rewritten around the training-set/test-set framing.
- **One-on-one assignment demos are retired** (section 5): supervised
  production replaces after-the-fact interrogation, and the demos page goes
  with them.
- **Canvas rubric and PDF tooling** carries over. Assignment distribution moves
  to Canvas downloads and autograding moves from GitHub Classroom to Gradescope
  (its Docker-based autograder is language agnostic, and community Rust/Cargo
  autograder examples exist). Recitations add the self-filtering device
  fleet with its golden image, and the local `check`/`pack` tooling, as new
  infrastructure (`ARCHITECTURE.md`).

## 12. Risks and open questions

- **Lab logistics** are the long pole: machine imaging, seat capacity for
  every section, and the on-device allowlist need a full-room dry run before
  week 1, including running the filter in log-only mode during mock sessions
  to pin the real hostname list (`ARCHITECTURE.md`).
  The paper-variant contingency per recitation is the hedge.
- **Paper-based recitations as the primary format (alternative, deferred).**
  Rather than locked-down lab machines with paper only as a fallback,
  recitations could run entirely on paper, the way the exams already do,
  promoting the existing paper variant to the main delivery mode. The appeal,
  raised in colleague discussion, is that it removes the lab-logistics long
  pole above outright: no machine imaging, no on-device allowlist, no
  seat-capacity-per-machine, and proctoring as simple as an exam. The cost is
  that a paper recitation no longer certifies that a student can drive the real
  toolchain and the `check` loop under test conditions, which is part of what
  the locked-down format exists to prove, and it narrows what a problem can ask
  (no running code, no live test feedback, tighter time budget). If adopted, the
  assessment-tier table in section 2 and the recitation format in section 5 both
  change. Revisit before finalizing recitation logistics.
- **Rust in week 1** is a bet. The mitigations are the restricted subset (no
  borrowing until week 7) and Python as the always-available softer on-ramp;
  monitor week 2 to 3 pre-checks and be ready to let struggling students go
  Python-only without penalty (the outcomes require one language, not two).
- **TA load** shifts from grading take-home code to proctoring, oral checks, and
  rubric calibration. Budget a week-0 training session and mid-term calibration.
- **Gradescope has no built-in Rust template**, so the autograder needs a
  custom Docker image with the Rust toolchain baked in. This is well-trodden
  ground (the platform is language agnostic by design), but build the image
  early and dry-run it: cold `cargo` builds are slow, so the image should
  pre-compile dependencies and cache the target directory.
- **Two-language authoring cost** roughly doubles scaffold and solution
  maintenance. The twin-scaffold layout and shared test cases (same I/O contract
  for both languages) keep it to one test suite per problem.
- **Campus parity (optional):** Energy Systems Engineering and Engineering
  Science students take the course at OSU-Cascades. Nothing in the design
  depends on that campus; if replicating the recitation lab model and
  proctoring there is practical, do it so the 85% supervised grade means the
  same thing on both campuses, and if not, scope this vision to Corvallis and
  let the Cascades sections adapt it independently.
- **Catalog constraints:** confirm the registrar's outcome text and the graded
  recitation weight comply with university assessment policy before publishing
  the syllabus.
- **Weekly load runs near the ceiling.** The design budgets to about nine hours
  per week for three credits (section 6), but exam-adjacent weeks (6 and 10)
  push against it, which is why those weeks' assignments must ship the reduced
  problem set section 6 prescribes. Two contact-hour questions ride along for
  the registrar: the 3.5 scheduled hours (two lectures plus a 110-minute
  recitation) are heavy for a pure three-credit lecture and fit only if the
  recitation counts as the studio or lab component, and the dual-language notes
  add a reading premium that the "read one language, skim the other" rule is
  meant to contain. A pilot-term timing survey should confirm the real median
  completion time before scaling to all sections.
