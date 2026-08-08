# Design: the environmental assignment story line (replacing Mission Ares)

**Status:** approved design, not yet implemented
**Date:** 2026-08-08
**Scope:** the assignment tier only (A1 to A10) and the artifacts that describe it

---

## 1. What this changes, and what it does not

VISION.md section 8 defines the course's core distinction: a **problem family is a
concept skeleton**, and a **story line is a costume** worn at one assessment tier.
This design swaps one costume. It does not touch the skeleton.

**Changes:** the assignment tier's story line moves from Mission Ares (a crewed
Mars habitat) to a regional environmental agency, and the tier's confidently-wrong
AI character moves from HAB to AURA.

Two adjustments ride along with the swap, both recorded here and both narrow:

- **A9 and A10 are rebalanced** so A9 uses both of its lectures instead of one. See
  section 6a.
- **A3 gains student-authored console input**, specified separately in
  [the console input design](./2026-08-08-console-input-and-program-wholeness-design.md).

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
  number reachable with `//` and `%`. Per
  [the console input design](./2026-08-08-console-input-and-program-wholeness-design.md),
  A3 is also **the one assignment where the student authors console input**: the
  scenario values arrive from standard input rather than being set by the starter,
  and the grader pipes them.
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

### A9: character-code arithmetic and string building

- **Ladder:** the hardest slot to re-skin. Its window is week 8 (L13 strings, L14
  collections), and week 8 is **before** references, so **no `String`, `&str`, or
  `Vec` may be a parameter**. Only scalars go in. Returning an owned `String` or
  `Vec` is legal, because building one needs no reference.
- **Topic:** **a bandwidth-starved field logger.** Remote stations transmit over a
  satellite or LoRa link where every byte costs battery, so each reading is packed
  into a compact fixed-width **base-36 code**. The student writes `code_for(reading_ugm3) -> String`,
  which builds the code one character at a time, and `digit_value(c: char) -> i32`,
  the per-character inverse primitive.
- **Why this shape:** see section 6a. A9's window covers both strings and
  collections, but a chars-only design leaves half of it unused and pushes the
  remainder onto A10. String **building** is the part of week 8 that is
  reachable without references, so it belongs here.
- **Why this and not a cipher:** a cipher motivates the wraparound narratively but
  not technically. Compact base-36 encoding on a power-budgeted logger is a real
  low-bandwidth telemetry technique, and the fixed width exists for a real reason.
- **The two functions are not inverses of each other, by construction.**
  `code_for` builds a multi-character code; `digit_value` decodes a single
  character. **The round trip is therefore not closed inside A9**, because closing
  it would need a function that consumes a string, which is week 9. Do not author a
  round-trip acceptance test here and do not let the page imply one. The two
  functions are two halves of one codec split along the ladder boundary, not two
  independent outputs.
- **The escalation into A10:** A9 **writes** the code, A10 **reads it back**, and
  that is where the round trip closes. The A6 reading-line format threads straight
  through both.
- **Hard constraint on the contract: no negative operand may reach `%`.** Python and
  Rust disagree on the sign of the remainder for negative operands (`-5 % 36` is
  `31` in Python and `-5` in Rust). A9's contract is dual-language against **one
  shared test suite**, so a divergence here does not make an interesting trap, it
  breaks the slot. A fixed-width encoding of a non-negative reading satisfies this
  naturally, since every operand is non-negative by construction. State the
  precondition that the reading is zero or greater, and verify at draft time that no
  reasonable implementation reaches `%` with a negative value.
- **Risk:** this is still the slot most likely to need rework once drafted. It has
  not been run in either language. Draft it second, right after the skill file.
- **AURA claim sketch:** a claim about the digit order (that the code can be built
  least-significant-digit first and read the same way), or that a value too large for
  the fixed width can be clamped rather than rejected.

### A10: memory model and sharing

**A10 is a regular assignment week, not a capstone.** It is the last assignment, and
it carries the equity critique because it is last, but it is one problem at the
ordinary size with the ordinary structure, and nothing about it caps or synthesizes
the term. This also brings the slot into line with VISION section 6's own rule that
week 10 must be "sized to the same weekly budget as any other week, never heavier."

- **Ladder:** **one** entry-point function. It parses a raw text line character by
  character, validates it, conditionally mutates the caller's list **in place**, and
  returns a formatted report line. The grader inspects the caller's list after the
  call. Strings and lists only, no dict or HashMap. Plus the one-page accessibility
  and equity critique.
- **The independent-copy function moves to the summit problem**, where extra load
  belongs. Aliasing is fully exercised by the in-place mutation on its own: the
  caller seeing your change *is* the concept. The defensive-copy contrast becomes the
  stretch, so it is extra credit and never enters the required weekly budget.
- **Topic:** **the annual emissions inventory.** Raw readings arrive from the field
  network as text lines in the format A6 introduced, carrying the base-36 codes A9
  produced. Validate the line, fold it into the running station manifest in place
  when it qualifies, and return the formatted report line.
- **Concept load:** three new things, all from A10's own window (consuming a string
  parameter, passing a collection to a function, and in-place mutation the caller
  sees). See section 6a.
- **The equity critique:** air quality monitor siting. Regulatory monitors are
  distributed unevenly, so the inventory the student just built reports confidently
  about the neighborhoods it watches and is silent about the ones it does not, and
  the silence reads as absence of a problem. The student critiques the tool they
  built: what it assumes, where it fails, and whom it fails. This is catalog outcome
  3 with a real referent.
- **AURA claim sketch:** a claim that a function cannot change the caller's manifest
  unless it hands it back, so the return value is the only thing that matters. That
  is the aliasing trap the week teaches, and a test that inspects the caller's list
  after the call settles it.

---

## 6a. Concept load across the ten slots

The escalation rule is that each assignment adds roughly **one** new advanced
concept over the one before it. Auditing the current slate against that rule found
one real imbalance, and it is worth recording why, because the cause is structural
rather than an authoring slip.

| Slot | Window | New concepts the student must actually use | Count |
|---|---|---|---|
| A1 | L1 | toolchain only | 0 |
| A2 | L1 to L3 | literals, types, precedence; the printed-output contract | 1 |
| A3 | L3 to L4 | variables; integer division and remainder; **console input** | 2 |
| A4 | L5 | functions: define, call, return | 1 |
| A5 | L6 to L7 | scope; booleans and `if`/`else` | 1 |
| A6 | L8 to L9 | `elif` / `match` chain; float tolerance | 1 |
| A7 | L10 | assert-based tests; judging provided code | 1 |
| A8 | L11 to L12 | loops; accumulation over a data-dependent count | 1 |
| A9 | L13 to L14 | `char` and character-code arithmetic; **string building** | 2 |
| A10 | L15 to L16 | consuming a string parameter; passing a collection; in-place mutation the caller sees | 3 |

### The constraint that shapes the back half

**Strings and lists cannot be function parameters until week 9.** The language
ladder introduces strings, `char`, `Vec`, and their methods in week 8, and
introduces `&` / `&mut` references and "passing lists to functions" in week 9. This
is confirmed in the source: `lectures/strings.mdx` contains no function signatures
at all and never mentions references. Every week-8 string example lives inside
`main`.

So A9, whose window is week 8, can only take scalars into its entry points. That is
why its functions are typed over `char` and `i32`, and it was forced rather than
chosen.

### The imbalance, and the fix applied

Before this audit, A9 used only L13 (characters) and left L14 (collections) unused,
so A10 absorbed both string handling and list handling on top of its own two
lectures, while also carrying two entry-point functions and a capstone framing. The
escalation ran flat, flat, flat, then a cliff of four to five new concepts in the
final week.

Two changes fix it:

1. **A9 gains string building.** The available lever is that **returning** an owned
   `String` or `Vec` needs no reference, even though **taking** one does. So A9 adds
   `code_for(reading_ugm3) -> String`, which uses the second half of its window, and
   A10 keeps only string consuming, which genuinely needs week 9.
2. **A10 becomes a regular week.** The capstone framing is dropped, it goes to one
   entry-point function, and the independent-copy function moves to the summit
   problem. VISION section 6 already required week 10 to be sized like any other
   week; the capstone label was pulling against that rule.

### The limit, stated honestly

**A10 is still the conceptually densest slot, because week 9 stacks references and
sharing and A10 is the only assignment in that window.** Collections-as-parameters
is inherently week-9 material and there is nowhere earlier for it to go. What the
two changes remove is the load that did not belong there: borrowed week-8 content,
a second entry point, and a framing that invited the problem to grow. One problem,
three concepts, ordinary size.

If A10 still runs long in a pilot term, the lever is trimming the inventory's field
count, never moving a concept, since there is nowhere earlier for a week-9 concept
to go.

**A3 carries two** once console input lands there, making it the densest of the
early slots. Both concepts come from its own two lectures, so this is within the
rule, but it is the slot to watch first in a pilot term.

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
- `VISION.md`'s **capstone framing goes away**, in four places: section 6's sizing
  paragraph ("the Sol 100 capstone"), section 8's fil rouge paragraph ("the Sol 100
  capstone assignment parses"), section 8's family-week paragraph ("the capstone is
  list-based"), and section 10's week-10 row ("the list-based Sol 100 mission-status
  capstone program"). A10 is the last assignment, not a capstone, and section 6
  already requires week 10 to be sized like any other week.
- `VISION.md` section 6's **contract sentence** needs one added clause. It currently
  allows "a second [entry point] only when the family is genuinely two independent
  outputs." A9's two functions are **not** two independent outputs; they are two
  halves of one codec, split because the ladder forbids a string parameter before
  week 9. Without the clause, the first person authoring A9 against VISION will read
  two functions as a violation. Section 6 must also gain the console-input clause
  described in the companion spec.

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
  every constraint on paper, but neither it nor its fallback has been drafted or run
  in both languages, and the negative-remainder divergence described in the slot
  entry can break the one-shared-test-suite invariant if the contract is stated
  carelessly. Draft and verify A9 **second**, right after the skill file, not last.
- **Three files must agree, atomically.** VISION section 8's family table,
  `reference/story-bibles.mdx`, and `recitations/index.mdx` all state the two story
  lines as course canon. They are one edit, not three, and a partial landing leaves
  the site contradicting itself in public.
- **Rubric TSVs are a gate, not a cleanup.** The `RubricTable` component reads its
  TSV by path, so a renamed assignment with a stale TSV path is a broken page rather
  than a stale row. Each assignment's rubric rewrite belongs in the same step as its
  page, never in a trailing pass.
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
