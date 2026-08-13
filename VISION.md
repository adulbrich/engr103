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
| 2. Assignment | Take-home, independent | Low (completion + auto-graded correctness) | Anything, including GenAI | Willamette Resource Office |
| 3. Recitation | In person, on paper | High (graded like exams) | None (printed handout only) | Rubber Duck Robotics |
| 4. Exam | In person, on paper, multiple choice, no notes | High | None | None, bare concept |

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
O2, and O4, and give one guided rep at O3 in the write-it-yourself block every
session carries.

---

## 3. Assessment architecture

| Component | Weight | Venue | Integrity model |
|---|---|---|---|
| Final exam | 25% | In person, paper, multiple choice, no notes, finals week | Proctored |
| Midterm exam | 20% | In person, paper, multiple choice, no notes, 45 min in a week 6 lecture slot | Proctored |
| Recitations (9 graded, lowest dropped) | 40% | In person, on paper, 110 min | Proctored + oral check |
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

Both exams are **multiple choice**: every item is a stem with one correct
option and distractors mined from real misconceptions (the wrong answers
students actually produce in activities, assignments, and recitations). The
format is chosen deliberately: it grades identically for every student, scales
to any section size with zero grading load, and pairs naturally with the
archetype inventory below, whose items all pose cleanly as selections. The
production skills the exams cannot observe directly (writing a whole program)
are trained on assignments and certified in the recitations; the exams certify
that the underlying concepts transferred.

- Midterm: 45 minutes maximum, on paper, no notes, no devices, held during the
  second lecture slot of week 6 (the 50-minute slot leaves room for seating and
  collection). It covers **everything taught so far** (lectures 1 to 10), and
  that is the whole coverage rule: no special emphasis, no rehearsal session,
  no curated subset. The time box shapes the paper: roughly twenty to
  twenty-five items drawn across the archetypes.
- Final: 110 minutes, on paper, in the university final slot. It covers
  **everything taught in the term**, with roughly 60% of points on weeks 6 to
  10 material because that material is fresher and builds on the rest.
- Both exams test **introductory programming concepts only**: no git, no shell,
  no editor trivia, no professional-development content, no library trivia
  beyond the published subset. Exam problems wear no story skin: like
  activities, they are stated as bare archetype items.

### Language policy on paper

Code in exam items appears in Python, in Rust, or in both (same program twice,
side by side, when the point is the concept). Students never write code on an
exam: every answer is a selection. Where an archetype is about producing code,
the item shows candidate versions and asks which one is correct, so the
judgment being tested is the same and the grading is mechanical. Each exam's
cover page restates this rule.

### The public archetype inventory

Exams are built exclusively from ten published question archetypes, each posed
in multiple-choice form. Students see the complete list in week 1, practice
every archetype in activities, assignments, and recitations, and face fresh
variations (new values, new contexts, same skeleton) on exam day. Few problem
families with variations, fully disclosed.

1. **Evaluate.** Which value (and type) does this expression produce
   (precedence, integer vs float division, boolean logic, string operations)?
2. **Trace.** Which variable-value table correctly traces this loop or call;
   or what is the value of `x` after the marked line?
3. **Predict output.** Short program; which of these is exactly what it
   prints?
4. **Find the bug.** A program with one logic error; which line is wrong, or
   which fix makes it correct?
5. **Choose the implementation.** From a short spec, which of these candidate
   functions meets it? (The multiple-choice face of write-a-function; the
   production form lives in assignments and recitations.)
6. **Pick the memory diagram.** Which diagram of names, values, and references
   matches the state after the marked line; or which call stack matches the
   marked moment?
7. **Explain.** Which one-sentence description states what this function does,
   at the level of purpose?
8. **Choose tests.** Given a spec, which test case can tell a correct
   implementation from a given broken one; which case is the boundary case?
9. **Complete the code.** Which line correctly fills the labeled blank in a
   mostly-written program?
10. **Concept question.** For example: compile-time vs run-time error, why
    `0.1 + 0.2 != 0.3`, when a copy happens vs when a name is shared.

### Coverage

Each exam covers everything taught up to it, and nothing else.

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

110 minutes, weekly, between the two lectures, in person. Each recitation is a
**printed handout** the instructor hands out in the room; students work the
problems by hand and are graded on their written answers, so the problems are
stated language-neutrally and are first and foremost answerable on paper. The
handouts are **kept private**: a recitation is never published on the course
site, because a published one could be found and pre-solved, which would defeat
a proctored assessment. They live in the course repository as permanent drafts,
authored there for the instructor to print. Each handout carries only the
problems (a warm-up, a couple of multiple-choice quick checks, two core
problems, and one extra-credit problem), with no submission mechanics, no
autograder, and no rubric on the page. The paper format also makes the session
robust: it needs no lab machine, no network, and no toolchain, and it doubles as
the DAS and make-up format.

Two rules govern what a recitation problem may ask, both consequences of the
paper format:

- **Every item is quickly and fairly gradeable from an answer key.** The
  answer forms are: a selected option, a numeric value, a filled trace table,
  a short written answer of a sentence or two, or a short code fragment. A TA
  grading a stack of handouts should be able to mark most items against the
  key without judgment calls; the judgment-call grading budget is spent on the
  short fragments and the stretch problem.
- **No extensive code writing.** Writing whole programs is assignment work,
  done at a keyboard with a toolchain. On paper a student may be asked for a
  short function or fragment (a handful of lines), never a full program. The
  recitation certifies that the practiced understanding transferred, and
  reading, tracing, predicting, and fixing code demonstrate that as well as
  writing it does. Each handout therefore mixes item kinds across the
  archetypes: at least one item has the student reading real code (a trace, a
  predict-output, or a find-the-bug over a short listing shown in both
  languages), alongside the short production items.

### Scope and difficulty

Recitations wear the Rubber Duck Robotics story line and its weekly ticket
(section 8); each session is one ticket from Gary, the senior engineer, handing
the intern the week's gadget work. Recitations carry **no distractors and no
trick helpers**: they are honest, straightforward problem sheets, and the
judging-provided-code skill lives on the assignment tier instead. Recitation in
week N assesses the **two lectures of week N-1**, which students have had a full
weekend and one assignment to practice; the current week's Monday lecture is
assessed the following week.

Recitations are deliberately **not much harder than the assignments** that
prepare them. The variable a recitation adds is the conditions, not the
difficulty: on paper, alone, from memory, time-boxed. High-stakes events are
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
- **Quick checks (a few minutes).** One or two multiple-choice questions on the
  week's ideas, alongside the warm-up.
- **Core problems (75% of points).** Two problems at assignment
  difficulty: the same skeletons students practiced, in fresh Rubber Duck costume with
  new values and at most one small twist, answerable in either language by hand.
  The two cores mix archetypes rather than repeating one form: a typical pair is
  one code-reading problem (a trace table, a predict-output, or a find-the-bug
  over a short listing printed in both languages, the student reading either)
  and one short production problem (a short function or fragment, never a full
  program). Warm-up, quick checks, and core are the full 100%: a prepared
  student can earn a perfect session without touching the stretch.
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
  editor, terminal, git, "hello" in both languages). Every graded week,
  including week 6 and week 10, is a regular session over the previous week's
  two lectures; no recitation is a cumulative synthesis session, because the
  exams simply cover everything taught before them and need no dedicated
  rehearsal event.
- The instructor collects the written handouts at the end of the session and
  grades them by hand: the warm-up, the quick checks, and the two core problems
  make up the 100%, and the extra-credit problem adds up to +10 on top. There is
  no autograder and no online submission at the recitation tier; the written
  answers are the audit trail. Because the work is on paper, a WiFi or machine
  failure cannot cost a student their session.
- Students may answer in either language, by hand. The problems are stated
  language-neutrally, so a correct Python answer and a correct Rust answer both
  earn full marks.

### Staffing and scale (25 students, two undergraduate TAs)

The session works at a 25:2 ratio only because recitations are exams, not help
labs. TAs proctor and check; they do not tutor during the session. The place
for help is office hours and the assignment week, so no help queue competes
with the oral checks. Per session, that leaves passive proctoring (both TAs,
continuous), about six oral checks per TA at two to three minutes each, and
collecting the handouts at the end. Grading afterward is by hand against an
answer key, and the item rules above are what make that sustainable: selected
options, numeric values, and trace tables mark mechanically; the short written
answers and code fragments are the only judgment calls, and they are short by
construction. The stretch problem's Polya plans are the largest single grading
item. That is one to two hours per TA per week, which two undergraduates can
actually sustain.

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

Assignments wear the Willamette Resource Office (WRO) story line (section 8):
each assignment arrives as an independent piece of the office's work, not an
episode in a plot, and carries a confident but subtly wrong claim from AURA for
the student to judge, the take-home rung of the red herring ladder. There is no
plot; the thread that carries from week to week is technical (one reading-line
format, shared naming conventions, one units discipline). The upcoming
recitation assesses the same skeletons in Rubber Duck Robotics costume.

### Structure (weekly, always due before the recitation it prepares)

Timing is governed by one invariant: **the assignment practicing a family is
due before the recitation that assesses that family**, which in turn precedes
the exam that distills it. Concretely, each assignment is released right after
a recitation and due the Monday evening before the next one, covering the two
lectures of the previous week that the next recitation will assess. Students get
a full week with the earlier of those two lectures and a full weekend with the
later one before being graded on it.

Each assignment is worked **test first** (design the acceptance tests, then
pseudocode, then code) and submitted in two places: everything that is not code
goes in a single **PDF on Canvas** (the acceptance tests, the pseudocode, the
paper trace, and the reflection), and the code goes to **Gradescope**. Its parts:

1. **The problem (auto-graded).** One substantial problem in the same family
   as the upcoming recitation and at the same difficulty: the recitation sibling
   differs in costume, values, and at most one small twist, not in level. The
   assignment hands the student a work situation and a goal and fixes only the
   **observable contract**; how to reach it, which intermediate values to
   compute, whether to add helper functions, and which constructs to reach for is
   the student's decision. That freedom is the point: the student must work out
   how to use what they have learned instead of transcribing a decomposition the
   spec already made. The challenge is always implementation, never deriving
   mathematics: simple arithmetic (a unit conversion) is left for the student to
   work out, but when a problem rests on math a first-year could not be expected to
   derive, such as a system of equations, the spec gives the solved equations (in
   LaTeX) and the student's work is turning them into clean, correct code. Before
   the functions lecture, the contract is the program's
   printed output in an exact stated format, and grading compares that output.
   This covers A3, the one assignment where the student authors the program's
   own console input: the scenario values arrive from standard input instead of
   being set by the starter, and the program prints no prompt, because a
   printed prompt would make the two languages' output differ and break the
   exact-format diff A3 is graded by.
   From the functions lecture (week 3) on, the contract is one named entry-point
   function (a second only when the family is genuinely two independent outputs,
   or when the ladder itself forces a split: two halves of one codec, one
   building a multi-character code and the other decoding a single character,
   kept apart because a string cannot legally be a parameter on one side of the
   ladder boundary yet) with an exact signature and return. A split forced by
   the ladder like that is not a second independent output: the two functions
   are never tested as inverses of each other, and the round trip they form
   does not close until the next assignment reads the code back. This is never
   license for an arbitrary second entry point. The grader calls
   the entry point directly with values it
   chooses and reads only what it returns, so every helper written inside is the
   student's to choose. From A4 on, every assignment still ships a whole,
   runnable program: the starter's `main` prompts, reads the values, calls the
   student's function, and prints the result, but that `main` is provided code
   and is never graded. Console input is therefore authored once, by the
   student, in A3; from A4 on it exists only as provided code the student
   reads. Starter code is distributed as self-contained git
   repositories on OSU GitLab, each carrying the spec (as its README), the twin
   scaffolds, the visible tests, and the `check`/`pack` tools; submission and
   autograding run on Gradescope, whose Docker-based autograder is language
   agnostic, so a single custom container image carries both the Python test
   runner and the Rust toolchain. Passing the visible tests earns the credit. Twin
   scaffolds in both languages; through week 3 students must submit in each
   language at least once (to force an informed choice of their main language),
   free choice afterwards. Because the recitations and exams ask for no
   extensive code, the assignment is the one tier where all substantial
   programs get written, and it is sized accordingly: a problem here may be a
   genuinely consequential piece of work, not a warm-up. An assignment may
   also ship **provided code** beyond the harness: a module the student's
   function must plug into, a parser that feeds it, or a provided function the
   student must write tests against or find the fault in. Provided code widens
   what a week can practice (reading, testing, judging, and extending code the
   student did not write, the O5 and O8 skills) and never decomposes the
   student's own solution; the student's part remains one open contract. The
   work is ordered test first: the student is given
   one worked example, then writes their own acceptance tests including boundary
   and regime-stressing cases (a plain input-and-expected table before the testing
   lecture, assertions after), then writes pseudocode, then the code. The tests
   and pseudocode go in the Canvas PDF and are graded on coverage and clarity; the
   code goes to Gradescope. This keeps "did you understand the spec" gradable and
   makes the planning visible. Every assignment closes with a Canvas rubric
   (rendered on the page from its TSV) and, when the computation is a
   straightforward calculation, a small non-printable simulator the student can
   check answers against. The rubric does one more job: it carries an
   **implementation criterion** that makes the intended practice non-optional.
   Hidden tests vary sizes and values so a hardcoded answer fails outright, and
   where the week's construct is the point, the rubric names it and grades it
   (a loops-week charge must be computed by iteration, not a pasted closed
   form; a conditionals-week status must branch, not table-lookup the visible
   cases). The spec stays open about how to solve the problem; the rubric
   closes the door on not solving it.
2. **Paper practice (self-checked).** One tracing or memory-diagram exercise in
   exam archetype format, done by hand and photographed into the Canvas PDF. The
   worked solution sits in a closed reveal on the assignment page: the student
   attempts the trace, then opens it to self-check. Graded on completion, not
   correctness. This is the only regular rehearsal of the paper modality outside
   lectures, so it is non-negotiable in every assignment.
3. **Reflection line (two sentences plus the modeling line).** The student must
   explicitly disclose whether they used AI and, if so, for what (and state
   plainly when they did not), then name any other help used (peers, docs, the
   notes) and one thing they still cannot do without it, and then name which
   intermediate variables they chose to represent the situation and why. Ungraded
   content, graded completion; it goes in the Canvas PDF. The explicit disclosure
   normalizes honesty about tool use and feeds the instructor's weekly
   misconception review.
4. **Summit problem (extra credit).** One problem at stretch altitude, worth
   extra credit and the take-home training ground for the recitation's stretch
   problem. It follows the same rules as the main problem. The assignment page
   labels it as extra credit and states no point values.

### Sizing to the weekly budget

The course is a three-credit class, budgeted at roughly **nine hours of student
work per week**: about 3.5 scheduled hours (the two lectures and the
recitation) leave about 5.5 hours for everything else. Assignments are authored
to fit inside that budget, not to fill it:

- **The problem:** a **median completion time of about 2.5 to 3.5 hours** for a
  student who did the reading, the test cases they submit included. One problem,
  sized to that target and never past it, at assignment (not stretch) difficulty.
  The upper half of the band is not a stretch: since the assignment is the only
  tier where substantial code gets written, a problem that lands at 3.5 hours of
  honest implementation work is doing its job. If it lands well short, the
  problem is too thin to sit above the activity; make it demand more decisions,
  not more sub-parts.
- **Paper practice:** one exercise, 30 minutes or less.
- **Reflection line:** a few minutes.
- **Summit problem:** optional extra credit, so it never enters the required
  weekly budget.
- **Pre-lecture reading:** each lecture's notes are sized to about 45 minutes,
  read primarily in the student's chosen language while skimming the other;
  the "what differs" call-out is the part read in full.

Exam-adjacent weeks (week 6 around the midterm, week 10 around the final
assignment) are the pinch points, so keep those assignments sized to the same
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
story (bare concept, fastest possible read), assignments live at the
Willamette Resource Office, and recitations wear Rubber Duck Robotics. Every
problem must be solvable while ignoring the story entirely; the story is a
spine, not a cage.

Each story line is its tier's common thread (fil rouge). For recitations the
thread is technical as much as narrative: Gary's tickets all follow the same
RDR firmware conventions, and later weeks should feel like returning to a
familiar codebase rather than starting from zero. Assignments carry no plot at
all, so their thread is entirely technical: one reading-line format
(`station_id,code,reference_ugm3`), introduced in the week 6 assignment and
parsed for real in the week 10 assignment; one naming convention (`*_status`
for a classifier's return code, `*_gco2`, `*_ugm3`, and `*_kwh` for a value
measured in that unit); and one units discipline, grams CO2e, kWh, µg/m³, and
km, held constant everywhere. One limit keeps grading fair either way: every
week remains independently solvable, and no problem ever requires last week's
solution. A student who missed a week is behind in nothing.

The six skeletons and their two faces:

| Family (skeleton) | Rubber Duck face (recitations) | Willamette Resource Office face (assignments) | Core topics | Weeks |
|---|---|---|---|---|
| **Arithmetic with units and time** | Duck-race countdown timers, novelty-mug fill volumes | Vehicle lifecycle payback, appliance energy, concrete mix targets | Expressions, types, variables, functions | 1 to 3 |
| **Decision tables and interlocks** | Hot-tub controller: temperature, occupancy, and timer cutoff rules | Burn-window clearance, discharge permits, dashboard publication gates | Booleans, conditionals, decision tables | 3 to 4 |
| **Loops over state and grids** | Warehouse duckbot fetching parts, ASCII duck-parade rendering | Solar array degradation, multi-year inventory accumulation | Loops, loop patterns, nested loops, functions | 4 to 6 |
| **Encode, decode, clean a sequence** | Garbled customer order codes, novelty-pager messages | Compact field-logger telemetry codes, station identifiers | Strings, lists/vectors, memory model, sharing | 6 to 8 |
| **Validate and summarize a noisy stream** | The smart bathtub's water-level sensor returns nonsense | Low-cost sensor drift against a reference monitor, tolerance alarms | Errors, input validation, floating point, RNG noise | 8 to 9 |
| **Key-value lookup and counting** | Duck SKU inventory after a warehouse mix-up | Station manifests, waste stream categories (week-10 extra) | Dictionaries/hashmaps, choosing structures | 9 to 10 |

The **Weeks** column above reflects the pre-reorder plan. Section 10's schedule
now teaches errors and testing before the midterm and loops after it, so the
family week-ranges are re-derived to follow that lecture order when the
recitations are authored: the loops-over-state-and-grids family moves to the second half, the key-value-lookup-and-counting family becomes a week-10 extra, and the final
assignment is list-based. The prerequisite rule is unchanged: each family's
recitation follows the lecture that teaches its concept.

Why these instead of the current course's calculator, financial planner,
dictionary, and linear-equation solver: same concepts, but every problem has a
visible consequence. A concrete mix comes in under its CO2 budget, a
burn-window check refuses to greenlight an unsafe day, a garbled field code
decodes into a real reading. Fun in an intro course is mostly fast, visible
feedback plus a reason to care, and these families are chosen so that plain
terminal output is the payoff, no graphics stack required.

### Willamette Resource Office (assignments)

The student is a junior engineer on the software team of the **Willamette
Resource Office (WRO)**, a fictional regional public sustainability agency in
the Pacific Northwest. There is no plot: each assignment arrives as an
independent piece of the office's work rather than an episode in a story, so a
student who misses a week is behind in nothing, not even a subplot. A public
agency reaches every domain the majors need (buildings, transport, water,
grid, waste, air, fire, ecology), and its outputs are public-facing, which
makes the final assignment's equity critique (section 6) a consequence of the
student's own work rather than a bolted-on essay.

Assignment problem contexts deliberately rotate across domains so every major
sees its own field more than once a term:

| Major | Reached by |
|---|---|
| Civil, construction, architectural | Concrete mix design; monitor siting and the built environment |
| Chemical, environmental | Concrete mix blending; air quality validation; data quality |
| Mechanical, transportation | Vehicle lifecycle payback; appliance energy |
| Electrical, energy systems | Solar array degradation; field logger telemetry; metering |
| Industrial, manufacturing | Quality certification; inventory |
| Forest, ecological | Prescribed burn clearance; field network |
| Bioengineering | Summit problems only (stream dissolved oxygen, exposure dose) |
| Nuclear | Summit problems (spent-fuel decay inventory); vehicle lifecycle as a generation source |

Bioengineering and nuclear are the two majors this theme reaches less directly
than the retired Mars-habitat setting did. They are recovered in summit
problems rather than by distorting a main problem, because summit problems are
extra credit and never enter the required weekly budget; this is an accepted
trade-off, not an oversight.

The recurring character is **AURA**, the office's AI analytics assistant,
procured from a vendor: helpful, tireless, and confidently wrong at
narratively convenient moments. Office rule: no AURA analysis is published
unverified. AURA is how judge-this-claim problems arrive in story ("AURA says
you can average the two grid intensities to get the blended figure; decide
whether to trust it"), which makes outcome O8 a weekly reflex instead of a
week 10 lecture topic. AURA is always wrong about the computation or the
method, and never about contested policy: the assignments compute, the
student interprets, and the test applied when authoring every claim is
whether a test case the student writes can falsify it. If it cannot, it is not
an AURA claim. Placing AURA in the assignment tier is deliberate: it is the
one tier where students may use AI freely, so a confidently wrong AI whose
reasoning they must judge is exactly the right training partner there. The
setting is deliberately less heroic than the retired Mars habitat; what pulls
the student through the practice instead is that the questions are real and
the numbers are checkable.

### Rubber Duck Robotics (recitations)

The graded recitations are set at **Rubber Duck Robotics**, a mediocre but
lovable novelty-gadget company (self-stirring mugs, motivational bathtub ducks,
the occasional smart hot tub). In the room the student is the new firmware
intern; each recitation arrives as a ticket from **Gary**, the senior engineer,
who is enthusiastic and overcommitted and hands the intern the week's gadget
work. The tone is comic, a deliberate contrast to the grounded, real-stakes
tone of the take-home assignments: the comedy lightens the test anxiety a
high-stakes proctored room otherwise breeds (a novelty-mug problem worth
points is still worth points), and the company name is a planted joke that
pays off when rubber-duck debugging is introduced. Stakes come from
proctoring and grade weight, not from tone.

### The red herring ladder

Real specifications contain noise, so problem statements plant calibrated
distractors that scale with the tier:

- **Activities:** none; bare concept.
- **Assignments:** one mild spec distractor (a quantity that is never needed),
  plus **AURA's claim**: a confident assertion about how to approach the problem,
  or a fact about it, that is subtly wrong. Following it uncritically leads to a
  wrong answer that a visible test catches.
- **Recitations:** none. The recitation is a straight test of practiced work;
  its problems carry no distractors and no trick helper.
- **Exams:** the distilled paper version, the find-the-bug archetype.

The rule that keeps this honest: a correct solution never depends on noticing
the herring. Distractors add noise, never ambiguity, and punish only uncritical
copying. The ladder trains O4 (extract the computational core from a noisy
spec) and O8 (judge code you did not write).

### Rules for family authors

- Every family must work identically in both languages with one shared test
  suite.
- Every family must have at least one paper-friendly sibling per exam archetype
  it touches (a manifest trace works as well on paper as in a terminal).
- Same skeleton, different surface: an assignment problem and its recitation
  sibling must differ in story, values, and at most one structural twist (an
  extra edge case, one more parameter), never in underlying concept and never
  in level. The recitation sibling must feel like a fresh variation of
  practiced work, not a harder problem. Difficulty headroom belongs to the
  stretch problem alone.
- Activities stay story-free, but may borrow any context that serves the
  concept (a Wordle-style guessing round is a fine strings activity).
- Every family problem has a modeling step (building the representation of the
  engineering situation) and an analysis step (running it and interpreting the
  result), kept distinct so the modeling-versus-analysis distinction (O6, catalog
  outcome 4) is rehearsed continuously rather than taught once. On assignments the
  distinction is drawn out of the student rather than labeled for them: the spec no
  longer names which step is which, and the reflection asks the student which
  variables they chose to represent the situation and why.

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
| Activities | Guided problem sets | A warm-up, then blocks of listing, question, and room to predict; every resolution ships on the page in a closed reveal |
| Assignments | Requirements, scaffolded | A piece of the office's work: one open problem, the contract fixed and the decomposition left to the student, plus optional hints |
| Recitations | Problem sheets, bare | Gary's ticket: mixed archetype items on paper, scaffolding stripped |
| Exams | Archetype items | Multiple-choice items, minimal wording, no story skin |

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

### Activities: guided problem sets, not tutorials

In a course where students may skip early lectures, activities would have to
read like tutorials, because they would be standing in for the lecture. Here
the flipped notes carry the exposition and the instructor runs the activity
live, so a tutorial-style document would compete with the session it supports.
What an activity is instead is a guided problem set: a warm-up that re-shows
the lecture's simplest pieces, then three blocks, each posing a listing and a
question the student commits to an answer for, and a closing block where the
student writes a small solution from a blank page.

The explanation is not absent from the page; it sits after the attempt rather
than before it. Every block's resolution ships with the page from the day it is
published, closed inside a reveal panel, and forced open when the page is
printed. In class the panels stay shut while the instructor resolves the
problem live; a student catching up alone attempts the problem first, then
opens the panel and self-checks against the same resolution the room got. One
artifact serves both audiences, and there is no second, after-class publication
step.

That placement is the whole design. The recitation and the exam ask the student
to predict, trace, fix, choose, and produce with no support, so the activity
rehearses exactly that modality at low stakes: attempt first, then read why. A
tutorial would train following steps instead, which is a real skill but not one
any assessment in this course samples. The only true tutorials in the course
are week 1's onboarding (R1, A1, and the setup how-tos they lean on).

The warm-up is the one place the page gives before it asks, and it stays a bare
refresher: the lecture's simplest pieces shown in both languages with their
output, and nothing worked through. The support a beginner needs before their
first attempt is supposed to arrive from the pre-lecture reading and the
misconception opener that starts the session, not from a worked example on the
activity page, which would duplicate the notes and pull the artifact into the
explanation genre.

### Assignments and recitations: shared skeletons, two formats

Assignments read as requirements under one template: **Context** (the story
beat, skippable), **Requirements** (numbered, testable statements),
**Interface** (provided signatures and types), **Acceptance tests** (the
visible cases), **Assumptions and edge cases**, plus scaffolding inside it:
optional hints, story commentary, a suggested order of attack. The Interface
names only the entry point the grader calls; the student owns every decision
inside it, which is the training the take-home tier exists for.

Recitation production items use the same requirements shape, stripped of all
scaffolding, so the format is never the surprise. Recitation reading items
(a trace, a predict-output, a find-the-bug) instead show a short listing in
both languages and ask the archetype's question, which is exactly the shape
the exam's multiple-choice items take; the recitation is where that modality
is rehearsed with real stakes before exam day. Support moves between the
tiers; difficulty holds. Students never author prose requirements themselves:
reading specifications is the course skill, and writing them is a later
course's job.

### Tests as requirements (what replaces pre-assignments)

The pre-assignment ritual (students submit their own explanation, pseudocode,
and test plan before coding) is deliberately absent: unsupervised, it produces
GenAI theater and TA grading load with little signal. The skills it targeted
survive in machine-checkable or supervised forms:

- **Requirement comprehension** is trained by the red herring ladder (section
  8), probed live by the oral check, and tested on paper by the explain
  archetype.
- **Test writing** is the first step of every assignment, worked test first:
  from week 3 on the student is given one worked example, then designs their own
  acceptance tests (including boundary and regime-stressing cases) before writing
  any code. The tests go in the Canvas PDF and are graded on coverage against the
  rubric, which makes "did you understand the spec" gradable and puts the planning
  before the coding. Early assignments use a plain data format (input, expected
  output); assertion syntax arrives with the testing lecture. The same skill
  appears on paper as archetype 8, and it graduates to the supervised tier as
  choose-tests recitation items (the testing-week recitation especially).
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
| 1 | 1 | **How programs run.** Computation and algorithms; source code, interpreters (Python) and compilers (Rust); running a program; printing results; errors as messages, not verdicts; the systematic debugging method and the modeling-versus-analysis distinction, both introduced here and threaded all term; the two-language philosophy and the language ladder. (This lecture, and perhaps lecture 2, get a full in-class walkthrough; the rest are read before class.) | R1 (ungraded): environment, terminal, editor, the `check` harness, hello in both languages. | A1: setup, toolchains, hello in both languages. |
| 1 | 2 | **Data representation and memory.** Values (integers, floats, booleans, strings) and their types; static vs dynamic typing; binary and decimal; bits and bytes; memory as addressable boxes; type sizes; Python's arbitrary-precision `int` vs Rust's fixed `i32`/`f64`, and what the numbers in `i32`/`f64` mean. Taught with the interactive binary/bits visualizer and a memory-box diagram, before any type name is used to mean a bit-width. Character encoding and ASCII are deferred to lecture 13, where characters are taught. | | |
| 2 | 3 | **Expressions and operators.** Arithmetic operators and precedence; integer vs float division and truncation (grounded in lecture 2's types and sizes); implicit coercion; mixed-type expressions; evaluating expressions by hand. | R2: how programs run and data representation. | A2: Types-and-representation families + paper trace. |
| 2 | 4 | **Variables and state.** Names vs values; assignment and rebinding; `let` and `let mut`, shadowing; initialization and uninitialized reads (Rust forbids, C++ undefined behavior); constants; tracing with a variable table; console input: reading a line and turning it into a number. Uses the memory stepper. | | |
| 3 | 5 | **Functions I.** Defining and calling; parameters, arguments, return values; signatures and types; arguments and parameters are separate memory and the value is copied; how the autograder calls your functions. Uses the memory stepper. (Week 3 Monday is the MLK holiday, so this is the only week-3 lecture.) | R3: expressions and variables. | A3: Arithmetic-with-units families + paper trace. |
| 4 | 6 | **Scope and the call stack.** Local and nested scope; shadowing; lifetimes of names; the global-variable antipattern; the call stack drawn by hand; decomposing a program into functions. Uses the memory stepper. Taught before control flow because scope governs the blocks inside conditionals and loops. | R4: functions. | A4: Function families + paper trace. |
| 4 | 7 | **Booleans and conditionals.** Comparisons, logical operators, short-circuit; `if`/`else`; building conditions from specifications; the floating-point equality trap and comparison with tolerance, grounded in lecture 2. | | |
| 5 | 8 | **Decision structures.** `elif`/`else if` chains and `match`; nesting vs chaining; decision tables; guard clauses; common boundary bugs. | R5: scope and conditionals. | A5: Scope-and-conditionals families + paper trace. |
| 5 | 9 | **Errors, input, and validation.** Kinds of errors (syntax, run-time, logic); parsing and why it fails; exceptions vs `Result`; validating a value and failing loudly. (The loop-based validate-until-correct pattern is deferred to lecture 12.) | | |
| 6 | 10 | **Testing.** Test cases from a specification; boundary and error cases; assertions and test functions in both languages; consolidates the spec-to-tests skill practiced since week 3. | R6: decision tables and errors (from week 5). | A6: decision-table and validation families + paper trace. |
| 6 | | **Midterm**, multiple choice on paper during the second lecture slot, covering everything taught so far (lectures 1 to 10; no loops). | | |
| 7 | 11 | **Loops.** `while` for unknown counts, counted loops; loop variables; termination; `do-while` as a C++-only "what differs"; tracing loops. | R7: testing. | A7: Testing families + paper trace. |
| 7 | 12 | **Loop patterns.** Accumulate, count, search, sentinel, validate-until-correct; nested loops (ASCII rendering); choosing the pattern from the problem statement. | | |
| 8 | 13 | **Strings and characters.** Strings as sequences; characters and character encoding (ASCII, introduced here and grounded in lecture 2's bits and bytes); slicing, searching, building strings. | R8: loops and loop patterns (duckbot run-planning loops). | A8: Loop families + paper trace. |
| 8 | 14 | **Collections I: lists and vectors.** Indexing, length, iteration; growing and mutating; out-of-bounds as Python's `IndexError` vs Rust's panic (and the C++ buffer-overflow danger that motivates both). | | |
| 9 | 15 | **The memory model: aliasing and ownership.** Names point at values; aliasing in Python (two names, one list) and ownership/moves/borrows-lite in Rust; the C++ dangling/buffer-overflow danger as why memory safety matters; drawing memory diagrams. Uses the memory stepper. (Late because aliasing needs lists to exist first; the foundation and the stepper are early.) | R9: strings and lists (order-code tallies and letter-by-letter decoding). | A9: Encode/decode families + paper trace. |
| 9 | 16 | **Sharing and mutation.** Passing collections to functions; when the caller sees your changes; defensive copying; Rust `&`/`&mut` and why Rust makes you declare intent. Uses the memory stepper. | | |
| 10 | 17 | **Collections II: dictionaries and maps** (advanced extra, not required for the outcomes). Dictionaries and hashmaps; key-value thinking; choosing between list and map; frequency counting and lookup. | R10: memory model and sharing (from week 9). | A10: the final assignment, list-based, parsing a reading line into the station manifest with tests; accessibility/equity critique; paper trace. |
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
- **Assignments** (calculator, financial planner, dictionary, linear
  equations) are retired as framings, and the Mission Ares framing that
  briefly replaced them is retired too, but their mechanics survive inside the
  Willamette Resource Office families: the financial planner's accumulation
  loops become loops-over-state-and-grids problems, the dictionary becomes the
  key-value lookup and counting family, and the linear-equation and calculator
  arithmetic becomes arithmetic-with-units-and-time computations.
- **Practicalities** (Polya, debugging, style, gen-ai) survive intact; the
  gen-ai page is rewritten around the training-set/test-set framing.
- **One-on-one assignment demos are retired** (section 5): supervised
  production replaces after-the-fact interrogation, and the demos page goes
  with them.
- **Canvas rubric and PDF tooling** carries over. Assignment distribution moves
  to Canvas downloads and autograding moves from GitHub Classroom to Gradescope
  (its Docker-based autograder is language agnostic, and community Rust/Cargo
  autograder examples exist). The local `check`/`pack` tooling is new
  infrastructure for the assignment tier (`ARCHITECTURE.md`); recitations,
  being paper handouts, need none.

## 12. Risks and open questions

- **Paper-based recitations (decided).** Recitations run entirely on paper,
  the way the exams do. This removes the former long pole (machine imaging,
  on-device allowlists, seat capacity per machine) outright; proctoring is as
  simple as an exam, and the remaining logistics are printing, secure storage
  of unused handouts, and the DAS timing arrangements. The accepted cost is
  that a paper recitation does not certify driving the real toolchain under
  test conditions; that skill is trained on assignments, where all substantial
  code is written, and observed live through the oral checks.
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
