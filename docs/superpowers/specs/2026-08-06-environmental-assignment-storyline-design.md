# Design: the environmental assignment story line (replacing Mission Ares)

**Status:** approved design, not yet implemented
**Date:** 2026-08-06
**Scope:** the assignment tier only (A1 to A10) and the artifacts that describe it

---

## 1. What this changes, and what it does not

VISION.md section 8 defines the course's core distinction: a **problem family is a
concept skeleton**, and a **story line is a costume** worn at one assessment tier.
This design swaps one costume. It does not touch the skeleton.

**Changes:** the assignment tier's story line moves from Mission Ares (a crewed
Mars habitat) to a regional environmental agency, and the tier's confidently-wrong
AI character moves from HAB to AURA.

**Does not change:**

- The ten assignment slots, their ladder constraints, their target constructs, or
  their paired recitations. Every slot keeps the contract shape recorded in its
  current `ai-summary` block.
- The six problem families and their skeletons.
- Recitations, which stay Rubber Duck Robotics. The tier-costume contrast is
  load-bearing: VISION's argument is that a skeleton practiced in one costume must
  be recognized in another, so a memorized or generated solution transfers only if
  the understanding did. Bleeding the environmental theme into recitations would
  destroy that.
- Activities and exams, which stay story-free.
- Assessment weights, the test-first order, the dual submission (Canvas PDF plus
  Gradescope), the paper practice, the reflection line, the summit problem, or the
  2.5 to 3.5 hour sizing budget.
- The red herring ladder's structure: one mild spec distractor plus one confident
  wrong claim per assignment, with a correct solution never depending on noticing
  either.

### Why change at all

Three gains, in order of size:

1. **Catalog outcome 3 gets a real referent.** The A10 accessibility and equity
   critique currently asks students to critique a computational tool in the
   abstract. On this theme it has a documented, verifiable subject: air quality
   monitor siting. Regulatory monitors are sparse in low-income and minority
   neighborhoods, so a dashboard reports confidently about the places it happens to
   watch and says nothing about the places it does not. The student has just built
   that dashboard.
2. **The wrong claims become claims students will actually meet.** Environmental
   arithmetic is a live site of real confusion, so a wrong claim about it trains a
   reflex that transfers off the page.
3. **The numbers are checkable.** Students can look up a grid carbon intensity and
   argue with it. They cannot look up a delta-v budget for a fictional mission.

### What the change costs

Mission Ares was chosen partly because a Mars habitat reaches every engineering
major in the room (VISION section 8 names chemical, civil, construction,
mechanical, electrical, energy, bio, nuclear, industrial). An environmental theme
is narrower on that axis unless it is deliberately built wide. Section 5 below is
the answer to that, and it is a requirement of this design rather than a nice-to-have.

---

## 2. The setting

The student is a junior engineer on the software team of the **Willamette Resource
Office (WRO)**, a fictional regional public sustainability agency in the Pacific
Northwest.

A public agency rather than a private consultancy, for three reasons:

- It legitimately touches every domain the majors need: buildings, transport,
  water, grid, waste, air, fire, ecology.
- Its outputs are public-facing, which makes the A10 equity critique an honest
  consequence of the student's own work rather than a bolted-on essay.
- Regional Pacific Northwest specifics (wildfire smoke, hydropower, stormwater,
  urban heat) are things OSU students have lived through, which is the whole point
  of "down to earth".

The regional name is a working name and is trivially swappable if a placeless
setting is preferred later. Nothing in the design depends on the geography beyond
flavor.

### The fil rouge is technical, not narrative

VISION section 8 requires each story line to carry a thread, and states that the
thread is "technical as much as narrative: within a tier, data formats, function
names, and provided helpers recur across the term." Because this design uses
independent weekly topics rather than a plot, the thread is **entirely** technical:

- **One reading-line format**, introduced in A6 and parsed for real in A10. Same
  field order, same units, same sentinel for a missing value.
- **One naming convention**: `*_status` for classifier functions returning an
  integer code, `*_gco2` for anything measured in grams of CO2e, `*_ugm3` for
  particulate concentrations, `*_kwh` for energy.
- **One units discipline**, stated once and honored everywhere: grams CO2e (never
  kg inside a function signature), kWh, µg/m³, km. Unit conversion stays the
  student's work, per the assignment challenge model.

The invariant from VISION holds unchanged: every week is independently solvable and
no problem ever requires last week's solution. A student who missed a week is
behind in nothing.

---

## 3. AURA, the character replacing HAB

**AURA** is the office's AI analytics assistant, procured from a vendor. It is
helpful, tireless, and confidently wrong at narratively convenient moments. Office
rule: no AURA analysis is published unverified.

Keeping the character an **AI** is deliberate and preserves VISION's own argument
verbatim: assignments are the one tier where students may use AI freely, so a
confidently wrong AI whose reasoning they must judge is exactly the right training
partner there. Replacing it with a human colleague or a marketing brochure would
lose that.

AURA inherits every rule the `engr103-assignments` skill already states for HAB:

- AURA **states a claim in prose**, never hands over a code draft. A draft
  pre-decomposes the solution.
- The claim is confident, plausible, and subtly wrong about the approach or about a
  fact the problem turns on.
- A correct solution never depends on noticing the claim. A student who ignores
  AURA and reasons from the requirements gets the right answer.
- The page never gives away the verdict: it does not say the claim is wrong, name
  the flaw, hand a case that exposes it, explain the fix, or attach a `<Reveal>`.
- The requirements must make the claim refutable, but no requirement is ever phrased
  as a direct rebuttal of it.

### The one new rule, and it is the important one

**AURA is always wrong about the computation or the method, and never about
contested policy.**

This constraint exists because the theme makes it easy to violate. A claim like
"electric cars are not actually cleaner" asks an 18-year-old to adjudicate a
political fight for a grade, which is not what outcome O8 tests and not something a
required assignment should do. A claim like "you can average the two grid
intensities to get the blended figure" is arithmetically wrong whenever the shares
differ, and the student's own test case settles it.

The operational test, applied when authoring every AURA claim: **can a test case
the student writes falsify this claim?** If not, it is not an AURA claim.

Two corollaries:

- Real greenwashing and marketing claims may be quoted through AURA, but only where
  the flaw is a method flaw (a wrong baseline, a double count, an averaged rate that
  should be weighted, a per-unit figure multiplied by the wrong unit).
- No assignment takes a position on policy. The problems compute; the student
  interprets. This is the modeling-versus-analysis distinction (O6) doing its
  normal job.

---

## 4. Data policy

**Real cited headline constants, authored scenario values.**

- **Headline physical and reference constants are real and cited on the page**, with
  the source named inline (agency, dataset, year) so a student can look it up. These
  are the numbers that make the problem a real question: grid carbon intensity in
  gCO2/kWh, gasoline's CO2 per litre, battery manufacturing emissions per kWh of
  pack, Portland cement's embodied CO2 per kg, median solar panel degradation per
  year, a sensor's stated measurement range.
- **Scenario values inside test cases are authored** so they land cleanly on the
  ladder: whole-number break-evens for A3, exact float boundaries for A6, a
  degradation count that terminates in a sensible number of passes for A8. The page
  presents these as this year's figures for a particular site, which is what they
  are.

This split is deliberate. Fully real datasets would force the ladder to bend around
the data (real values rarely land on the integer boundaries A3 needs), go stale
annually, and drag contested numbers into a first-year course. Fully authored
numbers would forfeit the payoff that motivated the change.

### Constants to verify at authoring time

The following are the candidate constants each slot leans on. Every one must be
checked against a current primary source and cited on the page before that
assignment ships. They are recorded here as targets, not as verified values.

| Constant | Rough figure | Candidate source |
|---|---|---|
| Gasoline combustion CO2 | ~2.3 kg CO2 per litre | EPA GHG emission factors |
| US average grid intensity | ~370 gCO2/kWh | EPA eGRID |
| Northwest (NWPP) grid intensity | materially lower than US average | EPA eGRID subregion |
| EV battery manufacturing | ~60 to 100 kg CO2e per kWh of pack | IVL / Argonne GREET |
| Portland cement embodied CO2 | ~0.9 kg CO2 per kg cement | industry EPD averages |
| Fly ash and slag embodied CO2 | near zero under byproduct allocation | industry EPD averages |
| Solar module degradation | ~0.5% per year, median | NREL degradation studies |
| Low-cost PM2.5 sensor range | 0 to 1000 µg/m³ | PMS5003 datasheet |
| PM2.5 health breakpoints | EPA revised 2024 annual standard | EPA NAAQS |

A note on verification: the sandbox in this session reaches only
`catalog.oregonstate.edu`, so none of the above was checked online while writing
this spec. The implementation plan must make citation a per-assignment gate.

---

## 5. Majors coverage

This is a requirement, not an observation. Mission Ares covered every major by
rotating subsystems; this design covers them by rotating domains, and the rotation
below is what makes the swap defensible.

| Major | Reached by |
|---|---|
| Civil, construction, architectural | A4 concrete mix, A10 monitor siting and the built environment |
| Chemical, environmental | A4 blending, A6 air quality, A7 data quality |
| Mechanical, transportation | A3 vehicle lifecycle, A2 appliance energy |
| Electrical, energy systems | A8 solar array, A9 field logger, A2 metering |
| Industrial, manufacturing | A7 quality certification, A10 inventory |
| Forest, ecological | A5 prescribed burn, A9 field network |
| Bioengineering | summit problems (stream dissolved oxygen, exposure dose) |
| Nuclear | summit problems (spent-fuel decay inventory), A3 as a generation source |

Bioengineering and nuclear are the two majors Mars reached more directly than this
theme does. The design deliberately recovers them in **summit problems** rather
than distorting a main problem, because summit problems are extra credit and
therefore never enter the required weekly budget. This is an accepted, stated
trade-off: those two majors see their field in an optional problem rather than a
required one. If that proves unacceptable in practice, the cheapest correction is
swapping A8's solar degradation for a spent-fuel decay inventory, which is the same
loop over a compounding factor.

---

## 6. The ten slots

Ladder constraints are copied verbatim from each assignment's current `ai-summary`
block and are non-negotiable. Only the topic column changes.

### A1: onboarding

- **Ladder:** no concepts. Toolchain, starter repo, `check` harness, `__main__`
  idiom, Gradescope submission.
- **Topic:** first day at WRO, checking out the workstation.
- **Notes:** no AURA claim, no spec distractor. This slot is pure setup and the
  re-skin is a text swap.

### A2: literals, arithmetic, and precedence

- **Ladder:** literals and operator precedence, **no variables and no functions**,
  graded by comparing printed output. Every division uses float operands so both
  languages agree; no mixing of whole numbers and floats in one expression. Must
  exercise whole numbers, floats, booleans, and a binary literal.
- **Topic:** a household energy audit panel. Prints annual kWh from an appliance's
  rated watts and daily hours, the resulting kg CO2e at the local grid intensity, a
  boolean for whether the household sits above the state median, and the smart
  meter's status bitfield as a binary literal.
- **Why the binary literal fits:** utility meters really do report status as a
  bitfield, so the week-one literal type arrives with a reason rather than as an
  exercise.
- **AURA claim sketch:** something about combining a daily figure into an annual one
  by the wrong factor, or about precedence in the kWh expression.

### A3: variables, integer division, and remainder

- **Ladder:** variables, integer division and the remainder operator, **no
  functions**, graded by comparing printed output. The answer must be a whole
  number reachable with `//` and `%`.
- **Topic:** **EV carbon payback.** An electric vehicle carries higher manufacturing
  emissions (the battery) and lower per-km operating emissions than its combustion
  equivalent. Compute the payback distance, then express it as whole years of
  typical driving plus the leftover kilometres in the final year.
- **Why it fits the slot:** `payback_km = extra_manufacturing_g // saving_per_km_g`,
  then `whole_years = payback_km // annual_km` and
  `leftover_km = payback_km % annual_km`. Integer division and remainder both arise
  from the problem rather than being bolted onto it, which is the hard part of
  authoring this slot.
- **Modeling note:** the per-km operating figure for the EV depends on the grid it
  charges from, which is the modeling decision the reflection line asks about. The
  Northwest grid figure and the US average figure give visibly different paybacks,
  and that is the point rather than a flaw.
- **AURA claim sketch:** a claim about the payback that treats a rate as a total, or
  averages two intensities that should be weighted by share.

### A4: functions from given equations

- **Ladder:** functions (week 3). A small linear system whose **solved** equations
  are given in LaTeX; the student writes the code, never derives the math. Two
  entry-point functions. Grader reads return values only.
- **Topic:** **low-carbon concrete mix design.** Choose the masses of two
  supplementary cementitious materials to hit both a total binder mass target and a
  total embodied-CO2 target.
- **The system:** with `x` and `y` the two masses, `M` the binder target, `C` the
  CO2 target, and `a` and `b` the two materials' CO2 intensities:
  `x + y = M` and `a*x + b*y = C`, solved as `x = (C - b*M) / (a - b)` and
  `y = M - x`. Given in LaTeX on the page, exactly as the retired linear-equation
  situation and the current `injection-burn-math` do it.
- **Units:** the target arrives in tonnes and kg CO2 per tonne while the intensities
  are per kg, so a conversion the student must notice sits inside the problem. Per
  the assignment challenge model, simple arithmetic like that is left to the student.
- **AURA claim sketch:** a claim about the denominator's sign, or that the two
  materials' intensities can be averaged.

### A5: booleans with a priority override

- **Ladder:** comparisons joined by `and`, `or`, `not`; a four-outcome status with a
  priority override; **no elif or else-if chains**; no loops or collections; scalar
  parameters only. Grader reads return values only.
- **Topic:** **prescribed burn clearance.** Wind speed, relative humidity, and fuel
  moisture must all sit inside their windows for a burn to proceed, and a
  county-declared burn ban overrides everything regardless of conditions.
- **Why it fits:** a real interlock with a genuine commanded override, which is the
  exact shape `transit-checks` has today. The consequence is visible and the domain
  is one Oregon students recognize.
- **AURA claim sketch:** a claim about the override's precedence, or about combining
  three conditions with the wrong connective.

### A6: tiered classifier with a float tolerance

- **Ladder:** an `elif` / `else if` chain or Rust `match`, checked in a fixed
  priority order, over a physical range plus a float tolerance against a trusted
  fallback. Parsing happens in the provided harness; the student's function receives
  a number. Scalar parameters only.
- **Topic:** **low-cost PM2.5 sensor validation.** A reading below zero means a
  shorted sensor line. A reading above the sensor's stated ceiling means it has
  railed high. A reading in range but further than the tolerance from the
  co-located reference monitor means drift. Otherwise the reading is trusted.
- **Why it fits:** this is a one-to-one map onto `airlock-and-readings` and it is
  what air quality networks genuinely do. Low-cost sensors are co-located against
  reference monitors precisely to catch drift.
- **Thread note:** this slot introduces the **reading-line format** that A10 parses.
- **AURA claim sketch:** the tolerance boundary claim that `airlock-and-readings`
  already uses, restated for this domain (whether a gap exactly equal to the
  tolerance counts as agreement).

### A7: writing a test suite against provided code

- **Ladder:** an assert-based test suite against a **provided** implementation
  carrying two rules: an inclusive band, and a freshness rule with an override. The
  grader runs the student's test function and reads which assertions raise. The
  suite must accept the correct implementation and reject every broken one.
- **Topic:** **certify the publish-to-dashboard check.** A reading may go to the
  public dashboard only if its value sits inside the plausible band and its timestamp
  is fresh within the stated window, with a sensor clock-fault flag overriding the
  freshness rule.
- **Why it fits:** the two-rule shape with an override is exactly
  `flight-qualification`'s, and "should this go on a public dashboard" is a real
  data-quality gate with a real consequence.
- **AURA claim sketch:** the qualification claim that two passing cases covering
  both outcomes are enough to certify the check.

### A8: a loop that compounds over a data-dependent count

- **Ladder:** loops (week 7). **No collections. No power operator.** A running value
  carried across a data-dependent number of passes.
- **Topic:** **solar array degradation.** A module loses a fixed fraction of its
  output each year. Accumulate the total kWh generated across the array's life,
  looping year by year until annual output falls below the warranty floor.
- **Why it fits:** the count is data-dependent (it depends on the degradation rate
  and the floor, not on a constant), the accumulation is a running product and a
  running sum in the same loop, and the closed form is genuinely awkward, which is
  what makes the rubric's implementation criterion enforceable.
- **Rubric note:** per VISION section 6, the rubric must name iteration explicitly.
  A pasted geometric-series closed form does not earn the credit in a loops week.
- **AURA claim sketch:** a claim that the total can be had by multiplying the
  first-year output by the number of years, or by applying the degradation once.

### A9: per-character encode and decode

- **Ladder:** the hardest slot to re-skin. Only `char` and `i32` may cross a
  function boundary, never a `String`, `&str`, or `Vec`. A forward function and its
  inverse, using character-code arithmetic with a wraparound.
- **Topic:** **a bandwidth-starved field logger.** Remote stations transmit over a
  satellite or LoRa link where every byte costs battery, so each reading is sent as
  a **base-36 delta** from the previous reading, wrapped modulo 36 so it always fits
  in exactly one character. The student writes the encoder (`i32` delta plus a base
  `char` to a `char`) and the decoder (`char` plus a base `char` back to an `i32`).
- **Why this and not a cipher:** a cipher motivates the wraparound narratively but
  not technically. Delta encoding with a modular wrap is a real low-bandwidth
  telemetry technique, the wraparound exists for a real reason (the value must stay
  one character), and both functions still take and return only `char` and `i32`.
- **Risk:** this is the slot most likely to need rework once drafted. If base-36
  proves awkward, the fallback is a rotating station-code check character over the
  same alphabet, which preserves the constraint and the wrap.
- **AURA claim sketch:** a claim about the wrap arithmetic, or about a negative
  delta's remainder (the two languages differ on the sign of `%` for negative
  operands, which is exactly the kind of trap that belongs here).

### A10: the capstone

- **Ladder:** parse a raw text line character by character; validate it; conditionally
  mutate the caller's list **in place**; return a formatted report line. A second
  function returns a freshly built, **independent** copy. The grader inspects the
  caller's list after the in-place call and proves the archive's independence by
  mutating the original afterward. Strings and lists only, no dict or HashMap. Plus
  the one-page accessibility and equity critique.
- **Topic:** **the annual emissions inventory.** Raw readings arrive from the field
  network as text lines in the format A6 introduced. Validate each one, fold the
  qualifying ones into the running station manifest in place, emit the formatted
  report line, and snapshot an independent archive copy.
- **The equity critique:** air quality monitor siting. Regulatory monitors are
  distributed unevenly, so the inventory the student just built reports confidently
  about the neighborhoods it watches and is silent about the ones it does not, and
  the silence reads as absence of a problem. The student critiques the tool they
  built: what it assumes, where it fails, and whom it fails. This is catalog outcome
  3 with a real referent.
- **AURA claim sketch:** a claim about the archive copy being independent when it
  is not, which is the aliasing trap the week teaches.

---

## 7. The family table re-skin

VISION section 8's family table and the public `reference/story-bibles.mdx` page
both carry a "Mission Ares face" column. It is re-skinned as follows. The Rubber
Duck face column is unchanged.

| Family (skeleton) | WRO face (assignments) |
|---|---|
| Arithmetic with units and time | Vehicle lifecycle payback, appliance energy, concrete mix targets |
| Decision tables and interlocks | Burn-window clearance, discharge permits, dashboard publication gates |
| Loops over state and grids | Solar array degradation, multi-year inventory accumulation |
| Encode, decode, clean a sequence | Compact field-logger telemetry codes, station identifiers |
| Validate and summarize a noisy stream | Low-cost sensor drift against a reference monitor, tolerance alarms |
| Key-value lookup and counting | Station manifests, waste stream categories (week-10 extra) |

---

## 8. Files to change

From `grep -ril "mission ares\|\bHAB\b"` across the repo, plus the derived artifacts.

**Authority document**

- `VISION.md`: section 6 (the Mission Ares paragraph and the HAB reference),
  section 8 (the family table's "Mission Ares face" column, the whole "Mission Ares
  (assignments)" subsection, and the red herring ladder's HAB bullet), section 11
  (the carry-over notes).

**The authoring skill (do this early)**

- `.claude/skills/engr103-assignments/SKILL.md`: the frontmatter `description`, the
  "The Story: Mission Ares" section, the "HAB's Wrong Claim" section, the
  `story_beat` field description, the opening-narrative rule, the component rules
  that name HAB, the accuracy-pass rule, and the anti-pattern list.
  This file must change **before** any assignment page is rewritten. It encodes
  "Mission Ares" and "HAB" as requirements, so leaving it stale means every future
  authoring pass quietly reverts the theme.

**Student-facing pages**

- The ten assignment MDX files in `src/content/docs/assignments/`. Note that slugs
  and titles change too, so redirects or link fixes may be needed.
- `src/content/docs/assignments/introduction.mdx`
- `src/content/docs/reference/story-bibles.mdx`
- `src/content/docs/practicalities/finding-docs-and-helpers.mdx`
- `src/content/docs/recitations/index.mdx` (references the assignment tier's costume
  when explaining the contrast; the RDR content itself is unchanged)

**Rubrics**

- The ten TSVs in `canvas/assignments/`. Row wording references the specific
  functions and cases, so each is rewritten alongside its assignment, and each keeps
  its implementation criterion.

**Memory**

- `engr103-project.md`, `engr103-assignment-tier-status.md`, and
  `engr103-assignment-challenge-model.md` all reference Mission Ares or the assignment
  slate and need updating once the change lands.

---

## 9. Non-goals

- Changing recitations, activities, exams, lecture notes, or the schedule.
- Changing any assessment weight, deadline structure, or submission mechanism.
- Adding a new problem family or retiring one.
- Sourcing or shipping real datasets as files. All real numbers arrive as cited
  scalar constants in the spec text, because the ladder forbids file I/O and
  collections for most of the term.
- Taking any policy position. The assignments compute; the student interprets.

---

## 10. Risks and open questions

- **A9 is the fragile slot.** Base-36 delta encoding with a modular wrap satisfies
  every constraint on paper, but it has not been drafted or run in both languages.
  Rust and Python disagree on the sign of `%` for negative operands, which is a
  genuine teaching opportunity and a genuine authoring hazard. Draft and verify A9
  early rather than last.
- **Constants are unverified.** Section 4's table is a set of targets. Every figure
  needs a checked primary source and an inline citation before its assignment ships,
  and the implementation plan must gate on that per assignment.
- **Bio and nuclear coverage is optional-only.** Accepted trade-off, stated in
  section 5, with a named correction if it proves unacceptable.
- **Slug changes ripple.** Assignment slugs appear in cross-references from
  recitations, lectures, and the schedule. A link check is required after the pages
  are rewritten.
- **AURA claim discipline is the main authoring risk.** The theme makes
  policy-flavored claims easy to write by accident. The falsifiable-by-a-test-case
  rule in section 3 is the gate, and it belongs in the skill, not just in this spec.
- **Tone.** Mission Ares was heroic, which VISION says gave the take-home tier a
  reason to pull students through practice they might otherwise skip. A regional
  agency is less heroic by construction. The compensating pull is that the questions
  are real and the answers are arguable. Watch the first pilot term for whether
  that lands, because it is the one thing this design trades away and cannot easily
  get back.
