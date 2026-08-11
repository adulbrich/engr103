# Course Review: ENGR 103 against "Computing Education When Writing Code Is No Longer the Challenge"

**Date:** 2026-08-11
**Branch:** `website-redesign`
**Source:** Albluwi et al., *Communications of the ACM*, Opinion, posted 2026-08-03. DOI 10.1145/3808702. Derived from Dagstuhl Seminar 25311.
**Scope:** Design review only. No content files were edited.

---

## 1. The article's claim, in its own terms

Code production is no longer the bottleneck. The challenges have moved to both
sides of it: **before**, the capacity to express desired outcomes with
sufficient precision; **after**, comprehending code, evaluating code quality,
and validating correctness and trustworthiness. The authors name a competency
triad that should replace code-writing at the center of CS1: **design**
(specify and prompt), **orchestrate** (problem solving and decomposition), and
**evaluate** (verification and validation).

Their recommendations come as a three-rung ladder of ambition:

| Rung | Learning outcomes | Assessment tasks | Teaching activities |
|---|---|---|---|
| **Quickfix** | State GenAI use explicitly; de-emphasise code writing early | Explicit per-course GenAI policy; secured summative, unsecured formative; unsecured assessments collect the *full artifact range* (design and specification documents, prompt logs, code quality, test cases) | Teach students their own role in an AI-assisted learning process; emphasise metacognition |
| **Renovation** | Realign course goals with GenAI-age design practice | Consistent policy across course groups; open-ended authentic projects | Course-calibrated AI tutor (CodeHelp, CodeAid, UIUC Chat); active learning and flipped classrooms |
| **Visionary** | Rebuild curriculum around effective, critical, ethical GenAI use | Department- or school-level policy (the Sydney model: GenAI permitted in all take-home work) | Department-wide consistent AI tooling |

The postscript is the newest and sharpest part. GenAI has moved from reactive
code generators to **agentic systems** that plan multi-step solutions, invoke
tools autonomously, maintain state across sessions, and refine their own
output. The authors' conclusion: students must learn to "supervise, constrain,
and critically and ethically audit semi-autonomous computational agents."

One local note: **Chris Hundhausen**, a co-author, is a professor and associate
head in OSU's School of EECS. The article's department-level recommendation is
actionable here rather than abstract.

---

## 2. Where this course sits on that ladder

This is not a course that needs to be argued into the article's position. On
the article's central assessment recommendation it is already **past
Visionary**, and it got there first. On tooling and process artifacts it sits
at **Quickfix**. On agentic AI it is **absent**.

### Ahead of the article: the assessment architecture

The article's headline assessment advice is "summative assessments should often
occur in secured environments, formative in unsecured environments." VISION §3
states the same rule more strictly and then actually pays for it:

- 85% of the grade is earned in a proctored room (exams 45%, recitations 40%).
- Nothing unsupervised can move a grade by more than 15%.
- The integrity policy *is* the architecture. No detection, no accusations, no
  regrade wars.

The training-set / test-set framing (VISION §6) is a cleaner articulation of
the article's point than the article manages: the assignment is the training
set, the recitation is the test set drawn from the same family, and the story
line changes between tiers so that a memorized or generated solution transfers
only if the understanding did. The course tells students this in week 1.

The article's "comprehension, verification, validation" emphasis is likewise
already the spine here. Every one of the ten exam archetypes is a
comprehension or evaluation act, not a production act. Recitations forbid
extensive code writing on principle and mix reading items with short
production items. This is Mills' 1988 "reading code as a managerial activity"
argument, which the article cites, implemented rather than quoted.

Flipped classroom, active learning, metacognitive prompts (the reflection
line), and an explicit per-course GenAI policy: all present. Quickfix and most
of Renovation's teaching-activity column are done.

### The one structural gap: O8 is trained but never certified

This is the finding that matters most, and it is visible only by reading three
VISION sections together.

- **§2 outcome mapping:** "exams certify O1, O2, O3, O5, O6 … Recitations
  certify O1, O3, O4, O5, O7 … Assignments **train** all outcomes and carry the
  O8 thread."
- **§5 recitation scope:** "Recitations carry no distractors and no trick
  helpers … the judging-provided-code skill lives on the assignment tier
  instead."
- **§8 red herring ladder:** "**Recitations:** none."

O8 (Judge: evaluate a computational solution critically, including whether code
from a peer, a library, or an AI tool is correct and appropriate) is the only
operational outcome with **no supervised certification anywhere**. It is
deliberately excluded from the 85%, by two independent design rules, and lives
entirely in the 10% tier where students may use any tool they like.

That is precisely the competency the article puts at the center of
post-production computing education. The course has correctly identified it as
an outcome, written it into O8 and catalog outcome 3, threaded it weekly
through HAB/AURA's wrong claim, and then certified everything except it.

Catalog outcome 3 (equity and accessibility limitations) has the same problem
for the same reason: VISION §6 places the critique thread wholly in the
assignment track, completion-graded.

**The cheapest fix in this review.** The machinery already exists and is
already on the certified list. Exam archetype 4 (find the bug) and archetype 8
(choose tests) are both comprehension-of-code-you-did-not-write in
multiple-choice form. Two changes, structural cost zero:

1. Add O8 to the exams' certified list in §2. Archetypes 4, 8, and 10 already
   sample it; the mapping line simply does not claim it.
2. Frame one recitation core reading item per term as *"here is the code AURA
   produced for Gary's ticket. Find what is wrong with it."* This is
   find-the-bug wearing a label. It does not violate the no-trick-helpers rule,
   because the code is honestly broken rather than adversarial, and §5 already
   requires one code-reading core item per session. Only the framing changes.

The no-distractors rule should stay exactly as written. Distractors in a
proctored room measure composure; judging honestly-broken provided code
measures O8. They are different things and the rule only forbids the first.

### Substituted, not missing: specification

VISION §9 contains the review's most quotable collision with the article:

> "Students never author prose requirements themselves: reading specifications
> is the course skill, and writing them is a later course's job."

The article says the pre-code challenge is "our capacity to express our desired
outcomes with sufficient precision," which reads as a direct contradiction.

It is not, quite. The same section carries **"Tests as requirements"**, and
every assignment is worked test-first: the student designs their own acceptance
tests, including boundary and regime-stressing cases, before writing any code,
and those tests are graded on coverage and clarity. The course does teach
precision of expression. It teaches it through **executable specifications
rather than prose ones**, which is defensible and arguably correct for a
first-year prerequisite that serves every engineering major.

The problem is that this is nowhere stated. The design substitutes one form of
specification for another and documents only the refusal. **Revise the
sentence, not the design.** One paragraph in §9 saying "specification precision
is trained as executable tests rather than prose requirements, because
executable specs are checkable and prose specs are not gradeable at this scale"
turns an apparent gap into a defended position.

### Genuinely thin: orchestration

The article expects that removing the production bottleneck lets introductory
courses reach "complex, meaningful projects." This course goes the other way,
for reasons that are stated and sound:

- Three credits, a ~9 hour weekly budget, ~5.5 hours outside scheduled time.
- Assignments capped at a 2.5 to 3.5 hour median, one problem, never more.
- No full programs on recitations or exams, by the paper format.
- The Sol 100 capstone framing was removed in the in-flight story line swap.

The result: the largest artifact a student produces all term is a single
entry-point function reachable in about three hours. Decomposition (O4) is real
but small: the student chooses their own helpers inside one fixed contract, and
writes Polya plans for stretch problems. Nothing in the course exercises
orchestration at a scale where decomposition is the hard part.

**This gap should be acknowledged, not closed.** Closing it breaks the load
budget, and the load budget is already flagged in VISION §12 as running near
the ceiling. The only headroom that exists is the extra-credit tier: the summit
problems and A10. If the course wants one orchestration rep, that is where it
goes, as optional work, with the constraint stated openly.

### Absent: process artifacts, quality evaluation, agentic AI

**Prompt logs and process artifacts.** The article explicitly names "design and
specification documents, prompt logs, code quality, and test cases" as what
unsecured assessment should collect. This course collects tests, pseudocode, a
paper trace, and a two-sentence reflection. The reflection is a *disclosure*
("did you use AI, and for what"), ungraded on content, completion-graded. Rubric
audit across all ten TSVs confirms it: `Reflection: attached, with AI
disclosure and the modeling sentence` appears 7 times as a completion item. No
criterion anywhere asks what the AI got wrong or how the student caught it.

Upgrading the reflection from disclosure to a short graded critique of one AI
interaction is a small change with high alignment. It also feeds the
instructor's weekly misconception review with better data than a yes/no.

**Quality evaluation is assessed nowhere.** Cutting column 1 of all ten rubric
TSVs: criteria cover correctness, test-case design, pseudocode, paper practice,
reflection, and an implementation criterion naming the week's construct. No
criterion asks a student to *evaluate* the quality of code, their own or
anyone else's. This is not a complaint about grading student style. It is that
the article's second post-production challenge, judging whether code is good
rather than merely correct, has no surface in the course. `practicalities/style`
exists but touches no rubric and no assessment.

**Agentic AI is absent from every artifact.** Grep across all content outside
the legacy studios returns two hits for "copilot" and zero for "agentic" or
"autonomous", all in the stale gen-ai page. The course's operating model of AI
is HAB/AURA: a chatbot that answers a question and is sometimes confidently
wrong. That is the 2023 model. The article's postscript describes tools that
plan multi-step solutions, invoke tools, maintain state across sessions, and
refine their own output, and argues the required student skill is supervising
and auditing them.

Lecture 18 (Computing with Judgment) is the obvious home and currently covers
reviewing code you did not write, failure modes of generated and library code,
who our tools leave out, and responsible engineering. It is a week-10 advanced
extra, explicitly not required for the outcomes. Given the §4 invariant that
nothing examinable is first taught in week 10, adding agentic content there is
cheap and consistent: it is judgment content, not examinable content. One
section on what changes when the assistant runs multi-step work on its own, and
what a first-year engineer's supervision obligation is, would close the gap
without touching the exam surface.

**AI tutor (Renovation tier).** The article recommends a course-calibrated AI
tutor. Note it as optional and do not headline it. At 25 students and two
undergraduate TAs with no stated budget, integrating and calibrating CodeHelp
or equivalent is a heavy lift against a staffing model that is already tight.
The existing lever, office hours plus the assignment week, is not obviously
worse.

---

## 3. Fix-first: `practicalities/gen-ai.mdx` contradicts course policy

This is not staleness. It is a live contradiction on a student-facing policy
page, and it should be fixed before anything else in this document.

The page currently tells students:

- "**Don't outsource the assignment**: asking for complete solutions (or
  copying them) is likely an integrity violation."

The assignments introduction, on the same site, tells them:

- "The assignments are your take-home practice: the place to work through a
  problem with **any tool you like, including AI**."

A student reading both pages cannot tell what the policy is. The whole
integrity design rests on the second statement being unambiguous, and the first
statement is the old course's rule surviving into the new one.

Three further defects on the same page:

1. **It is entirely C++.** Every prompt template, every example, and every
   weakness listed ("subtle C++ mistakes", `std::vector::append`, cppreference)
   belongs to the retired language. The course is Python and Rust.
2. **It cites a rubric item that does not exist.** "ENGR 103 assignments have
   an AI Critique rubric item." No such criterion appears in any of the ten
   rubric TSVs.
3. **It never states the training-set / test-set framing**, which VISION §11
   explicitly requires: "the gen-ai page is rewritten around the
   training-set/test-set framing."

The rewrite is already specified by VISION and is the single highest-value edit
available on this branch. While rewriting, it is the natural place to put the
agentic material: what an agent is, what it does that a chat model does not,
and why "I ran the agent and the tests passed" is not the same claim as "I know
this is correct."

---

## 4. Second priority: VISION.md is stale against its own branch

VISION.md is the declared authority for the three authoring skills. Skills
resolve conflicts in its favour ("When this skill and VISION.md disagree,
VISION.md wins and this skill is the bug"). It is currently wrong in ways that
will actively mislead the next authoring session:

- §6, §8, and §10 still describe **Mission Ares**, **HAB**, and the **Sol 100
  capstone**. The branch has renamed all ten assignments to environmental slugs
  and the authoring skill has been rewritten for the Willamette Resource Office
  and AURA.
- §8's family week ranges are marked "pre-reorder" and are re-derived elsewhere.
  That is documented, but it means the table cannot be read directly.
- §11's carry-over notes still describe studios becoming assignments and the
  gen-ai rewrite as pending.

Plan task 3 covers this. It has not landed. Until it does, VISION and the
assignments skill give contradicting instructions about the same tier.

**In-flight context, not a finding:** plan tasks 11 through 21 rewrite the ten
assignment bodies from Mission Ares to Willamette. All ten pages still carry
HAB (4 to 9 occurrences each; `field-office-setup` is already clean). This is
known remaining work on
`docs/superpowers/plans/2026-08-08-environmental-storyline-and-console-input.md`,
not a defect.

**Housekeeping:** the eleven legacy C++ `studios/` pages are all `draft: true`
but a `dist/studios` directory exists in the build output. Worth one look given
this repo's known draft-link build behaviour, then delete the directory if the
content is genuinely retired.

---

## 5. Recommended actions, in order

| # | Action | Cost | Why |
|---|---|---|---|
| 1 | Rewrite `practicalities/gen-ai.mdx`: kill the C++, kill the contradicting integrity rule, adopt the training-set / test-set framing, add an agentic section | One page | A student-facing policy page currently contradicts the course's central integrity design |
| 2 | Land plan task 3 (VISION.md story line update) | Small | VISION is the skills' authority and currently disagrees with them |
| 3 | Add O8 to the exams' certified outcome list in §2 | One line | Archetypes 4, 8, and 10 already sample it; the mapping just fails to claim it |
| 4 | Frame one recitation core reading item per term as judging AURA's provided code | Framing only | Certifies O8 under supervision without touching the no-distractors rule |
| 5 | Upgrade the reflection from AI disclosure to a short graded critique of one AI interaction | One rubric row per assignment | The article's "prompt logs" recommendation, at a cost this course can pay |
| 6 | Add a §9 paragraph defending executable specs as the substitute for prose specs | One paragraph | Turns an apparent contradiction with the article into a stated position |
| 7 | Add an agentic-AI section to lecture 18 | One section | Closes the largest content gap, in the one place with no exam consequences |
| 8 | Note the orchestration gap and its cause in §12 risks | One bullet | Honest documentation of a bounded, budget-driven limitation |
| 9 | Delete the retired `studios/` pages | Housekeeping | Legacy C++ content in the tree |
| — | Course-calibrated AI tutor | Heavy | Optional. Do not attempt at 25:2 staffing without budget |

---

## 6. The honest summary

This course was designed on the same premise the article argues for, roughly a
year before the article appeared, and it implements the article's central
assessment recommendation more rigorously than the article proposes. The
distance between them is not philosophical.

The real difference is one of framing. **This course treats AI as a training
modality**: permitted in the 10% tier, tested away in the 85% tier, with
HAB/AURA as a weekly reminder that the machine lies. **The article treats AI
collaboration as a competency** that must itself be taught, practised, and
assessed.

Every remaining gap in this review descends from that one distinction. O8 is
trained but not certified. Prompt process is disclosed but not graded. Quality
evaluation has no surface. Agentic supervision does not appear at all. Each is
a case of the course teaching students to work *despite* AI where the article
asks it to also certify their ability to work *with* it.

Closing that distance does not require redesigning anything. Items 3 through 7
above are a mapping line, a framing change, one rubric row, one paragraph, and
one lecture section. The architecture already holds them.
