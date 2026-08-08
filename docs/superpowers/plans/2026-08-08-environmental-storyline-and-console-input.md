# Environmental Story Line and Console Input Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Swap the assignment tier's story line from Mission Ares to the Willamette Resource Office, replace HAB with AURA, move console input from lecture 9 to lecture 4 so A3 can author it, and rebalance the A9/A10 concept load.

**Architecture:** Three phases in strict order. Phase 1 fixes the authority layer (the authoring skill, VISION, the public story bible) and resolves the one technical unknown (A9's encoding) before any student-facing assignment page is touched. Phase 2 lands the console input change across the lecture tier. Phase 3 rewrites the ten assignment pages and their rubrics, one assignment per task, each with its own build gate. The authoring skill is first because it currently hardcodes "Mission Ares" and "HAB" as requirements, so leaving it stale would make every later task revert the theme.

**Tech Stack:** Astro 7 + Starlight, MDX content, `starlight-links-validator`, Python 3.14, Rust 1.88 (`rustc` / `cargo`).

## Global Constraints

Every task's requirements implicitly include this section.

- **Specs are the authority.** `docs/superpowers/specs/2026-08-08-environmental-assignment-storyline-design.md` and `docs/superpowers/specs/2026-08-08-console-input-and-program-wholeness-design.md`. Where this plan and a spec disagree, the spec wins; stop and flag it.
- **Load the matching skill before editing any content file.** `engr103-assignments` for `src/content/docs/assignments/`, `engr103-lecture-notes` for `src/content/docs/lectures/`, `engr103-activities` for `src/content/docs/activities/`. This is non-optional and is how house style is enforced.
- **Verification command for every content task:** `npm run build`. This runs `astro check && astro build`, and `starlight-links-validator` fails the build on any broken internal link. A task is not done until this passes.
- **Dual-language rule.** Every code example must be shown in Python and Rust, and every example must actually run as pasted. Verify by running it, never by reading it.
- **The language ladder is binding.** No example or assignment may use a construct above its week's row in `src/content/docs/reference/language-ladder.mdx`. Generics (including `.parse::<T>()` turbofish) are never used in this course.
- **Plain-language rule.** First-year reader, never programmed, may not be a first-language English speaker. Short sentences, common words, no idioms, every technical term defined at first use, one name per concept forever.
- **AURA's claim must be falsifiable by a test case the student writes.** AURA is wrong about computation and method, never about contested policy. No assignment takes a position on policy.
- **Real cited constants, authored scenario values.** Every real-world figure needs a named primary source (agency, dataset, year) cited inline on the page. Scenario values in test cases are authored to land on the ladder.
- **A10 is a regular assignment week, not a capstone.** One entry-point function, ordinary size, ordinary structure.
- **No I/O-diff grading component.** The grader calls the student's function directly and reads only what it returns. `main` is never graded, in any assignment. If a task tempts you to test printed output from A4 on, the answer is no; see section 7 of the console input spec.
- **Summit problems carry the majors the main problems miss.** The spec accepts that bioengineering and nuclear are reached less directly than under Mission Ares, and recovers them in extra credit rather than distorting a required problem. At least one summit problem must be bioengineering-flavored (stream dissolved oxygen, exposure dose) and at least one nuclear-flavored (spent-fuel decay inventory). Task 17's summit is the natural home for the decay problem, since it is the same loop over a compounding factor; Task 15's is the natural home for the water-quality one.
- **Never reveal the verdict on AURA's claim.** Do not say it is wrong, name the flaw, hand a failing case, explain the fix, or attach a `<Reveal>` to it.

### Slug renames used throughout

Both the `.mdx` file and its `canvas/assignments/<slug>-rubrics.tsv` rename together.

| Slot | Old slug | New slug |
|---|---|---|
| A1 | `systems-checkout` | `field-office-setup` |
| A2 | `pre-flight-check` | `household-energy-panel` |
| A3 | `launch-window` | `ev-carbon-payback` |
| A4 | `injection-burn-math` | `low-carbon-concrete` |
| A5 | `transit-checks` | `burn-window-clearance` |
| A6 | `airlock-and-readings` | `sensor-drift-check` |
| A7 | `flight-qualification` | `dashboard-publication-tests` |
| A8 | `first-drive` | `solar-array-lifetime` |
| A9 | `comms-window` | `field-logger-codes` |
| A10 | `sol-100` | `emissions-inventory` |

Renaming breaks inbound links. That is intentional and safe: the links validator fails the build, which turns every stale cross-reference into a compiler error rather than a silent 404.

---

# Phase 1: the authority layer

## Task 1: Rewrite the assignments authoring skill

This gates every later task. Nothing in `src/content/docs/assignments/` may be touched before it lands.

**Files:**
- Modify: `.claude/skills/engr103-assignments/SKILL.md`

**Interfaces:**
- Consumes: nothing.
- Produces: the authoring contract every Phase 3 task follows. After this task, "the story" means the Willamette Resource Office and "the wrong claim" means AURA's.

- [ ] **Step 1: Read the whole file**

Read `.claude/skills/engr103-assignments/SKILL.md` end to end before editing. Known Mission Ares and HAB references are at lines 3, 52, 54, 56, 115, 117, 121, 122, 123, 125, 127, 157, 159, 200, 216, 240, 243, 256, 295, 319, and 321, but re-grep rather than trusting that list:

```bash
grep -n "Mission Ares\|HAB\|Ares\|mission" .claude/skills/engr103-assignments/SKILL.md
```

- [ ] **Step 2: Rewrite the frontmatter description**

Replace the `description:` line with:

```
description: Use when creating or editing assignment files (MDX in src/content/docs/assignments/). Enforces the take-home, dual-language, Willamette Resource Office specification structure of the ENGR 103 assignments: one open problem whose contract is fixed and whose decomposition is the student's, AURA's wrong claim to judge, and alignment to the recitation the assignment prepares. Always load this skill before writing or editing any assignment file.
```

- [ ] **Step 3: Rewrite the story section**

Replace the "The Story: Mission Ares" section with a "The Story: the Willamette Resource Office" section carrying, in prose:

- The student is a junior engineer on the software team of the WRO, a regional public sustainability agency in the Pacific Northwest.
- There is **no plot**. Each assignment is an independent piece of the office's work. Do not write "last week you..." or "next week the office will..." into an opening narrative.
- The thread is technical: the reading-line format introduced in A6 and parsed in A10, the naming conventions `*_status`, `*_gco2`, `*_ugm3`, `*_kwh`, and the units discipline (grams CO2e, kWh, µg/m³, km).
- Problem contexts rotate across domains so every engineering major sees its own field: buildings, transport, water, grid, waste, air, fire, ecology.

- [ ] **Step 4: Rewrite "HAB's Wrong Claim" as "AURA's Wrong Claim"**

Keep every existing rule verbatim (claim in prose not code; correct solution never depends on noticing it; never give away the verdict; never phrase a requirement as a direct rebuttal; one mild spec distractor alongside, never defused). Change the character to AURA, the office's vendor-supplied AI analytics assistant, and the flight rule to the office rule: no AURA analysis is published unverified.

Then **add** this new rule as its own paragraph, because the environmental theme makes it easy to violate:

> AURA is always wrong about the computation or the method, and never about contested policy. The operational test, applied to every claim before it ships: can a test case the student writes falsify this claim? If not, it is not an AURA claim. "You can average the two grid intensities to get the blended figure" is a good claim, because it is arithmetically wrong whenever the shares differ. "Electric cars are not actually cleaner" is not, because it asks a first-year student to adjudicate a political fight for a grade. Real greenwashing and marketing claims may be quoted through AURA, but only where the flaw is a method flaw: a wrong baseline, a double count, an averaged rate that should be weighted, a per-unit figure multiplied by the wrong unit.

- [ ] **Step 5: Update the remaining references**

- The `story_beat:` field description in the `ai-summary` template: "the WRO work situation, one clause".
- The opening-narrative rule: "the WRO work beat" instead of "the Mission Ares mission beat"; "their role on the office's software team" instead of "their crew role".
- Every `Aside` and `Reveal` component rule that names HAB now names AURA.
- The accuracy-pass rule: "Run **AURA's claim** to ground".
- The anti-pattern list: "No Rubber Duck Robotics or bare-exam framing: assignments are Willamette Resource Office."
- The "mine the retired assignments" guidance: keep it, and add that the **Mission Ares assignments are now also a retired source** to mine for situations, in exactly the same way (reuse the situation and the coding decisions it forces, never the framing).

- [ ] **Step 6: Add the data policy rule**

Add a new short section, "Real constants, authored scenarios":

> Headline physical and reference constants are real and cited inline on the page, with the source named (agency, dataset, year) so a student can look it up. Scenario values inside test cases are authored so they land cleanly on the ladder. Never ship an uncited real-world figure, and never present an authored scenario value as a published statistic.

- [ ] **Step 7: Add the A9 two-function exception**

Add to the contract rules:

> One named entry-point function, and a second only when the family is genuinely two independent outputs **or when the ladder forces a codec to be split**. A9 is the second case: its window is week 8, which is before references, so no `String`, `&str`, or `Vec` may be a parameter. Its encoder returns an owned `String` and its decoder primitive takes a `char`, so the round trip does not close inside A9. A10 closes it. Do not author a round-trip acceptance test in A9.

- [ ] **Step 8: Verify no stale references remain**

```bash
grep -n "Mission Ares\|HAB\|Ares\|habitat\|crew\|Mars" .claude/skills/engr103-assignments/SKILL.md
```

Expected: no matches, except where the text deliberately names Mission Ares as a retired source to mine.

- [ ] **Step 9: Commit**

```bash
git add .claude/skills/engr103-assignments/SKILL.md
git commit -m "Rewrite the assignments skill for the Willamette Resource Office and AURA"
```

---

## Task 2: Prototype and verify the A9 encoding in both languages

The spec names A9 as the slot most likely to need rework. Resolve it now, before ten pages depend on it. This task writes no course content; it produces a verified reference implementation and a decision.

**Files:**
- Create: `/private/tmp/claude-502/-Users-ulbrical-GitHub-engr103/7d8df168-df57-48e1-bc98-92910ed93cfd/scratchpad/a9_proto.py`
- Create: `/private/tmp/claude-502/-Users-ulbrical-GitHub-engr103/7d8df168-df57-48e1-bc98-92910ed93cfd/scratchpad/a9_proto.rs`
- Modify: `docs/superpowers/specs/2026-08-08-environmental-assignment-storyline-design.md` (record the verified form in the A9 entry)

**Interfaces:**
- Consumes: nothing.
- Produces: the exact verified signatures Task 18 (A9) implements. `code_for(reading_ugm3: i32) -> String` and `digit_value(c: char) -> i32`.

- [ ] **Step 1: Read the existing A9 to reuse its character-arithmetic idiom**

Read `src/content/docs/assignments/comms-window.mdx`. It already establishes how this course writes character-code arithmetic in Rust at the week-8 ladder. The prototype must use the same idiom, not invent a second one. One name per concept applies to idioms too.

- [ ] **Step 2: Write the Python prototype**

```python
# a9_proto.py
def digit_char(value):
    if value < 10:
        return chr(ord("0") + value)
    return chr(ord("a") + value - 10)


def code_for(reading_ugm3):
    high = reading_ugm3 // 1296
    mid = (reading_ugm3 // 36) % 36
    low = reading_ugm3 % 36
    return digit_char(high) + digit_char(mid) + digit_char(low)


def digit_value(c):
    if c >= "0" and c <= "9":
        return ord(c) - ord("0")
    if c >= "a" and c <= "z":
        return ord(c) - ord("a") + 10
    return -1


cases = [(0, "000"), (35, "00z"), (36, "010"), (1296, "100"), (46655, "zzz")]
for value, expected in cases:
    got = code_for(value)
    print(value, got, got == expected)
for c, expected in [("0", 0), ("9", 9), ("a", 10), ("z", 35), ("!", -1)]:
    got = digit_value(c)
    print(c, got, got == expected)
```

- [ ] **Step 3: Run the Python prototype**

Run: `python3 "$SCRATCH/a9_proto.py"` (with `SCRATCH` set to the scratchpad directory)
Expected: every line ends in `True`.

- [ ] **Step 4: Write the Rust prototype**

```rust
// a9_proto.rs
fn digit_char(value: i32) -> char {
    if value < 10 {
        (b'0' + value as u8) as char
    } else {
        (b'a' + (value - 10) as u8) as char
    }
}

fn code_for(reading_ugm3: i32) -> String {
    let high = reading_ugm3 / 1296;
    let mid = (reading_ugm3 / 36) % 36;
    let low = reading_ugm3 % 36;
    let mut code = String::new();
    code.push(digit_char(high));
    code.push(digit_char(mid));
    code.push(digit_char(low));
    code
}

fn digit_value(c: char) -> i32 {
    if c >= '0' && c <= '9' {
        return c as i32 - '0' as i32;
    }
    if c >= 'a' && c <= 'z' {
        return c as i32 - 'a' as i32 + 10;
    }
    -1
}

fn main() {
    let cases = [(0, "000"), (35, "00z"), (36, "010"), (1296, "100"), (46655, "zzz")];
    for (value, expected) in cases {
        let got = code_for(value);
        println!("{} {} {}", value, got, got == expected);
    }
    let chars = [('0', 0), ('9', 9), ('a', 10), ('z', 35), ('!', -1)];
    for (c, expected) in chars {
        let got = digit_value(c);
        println!("{} {} {}", c, got, got == expected);
    }
}
```

- [ ] **Step 5: Run the Rust prototype**

Run: `rustc -o "$SCRATCH/a9_proto" "$SCRATCH/a9_proto.rs" && "$SCRATCH/a9_proto"`
Expected: every line ends in `true`, and the codes match the Python run exactly.

- [ ] **Step 6: Check the two hard constraints**

Confirm and write down the answer to each:

1. **No negative operand reaches `%`.** With the precondition `reading_ugm3 >= 0`, every operand of `/` and `%` above is non-negative. Confirm by inspection and record the precondition.
2. **Nothing above the week-8 ladder is used.** Check each construct against `src/content/docs/reference/language-ladder.mdx` row 8. `String::push` and `char` are listed. `b'0'` byte literals and `as` casts are not explicitly listed: **if `comms-window.mdx` does not already use them, stop and flag it**, because introducing a new idiom here is a ladder change, not an authoring choice.

- [ ] **Step 7: Record the verified form in the spec**

Update the A9 entry in `docs/superpowers/specs/2026-08-08-environmental-assignment-storyline-design.md`: replace "has not been run in either language" with the verified signatures, the fixed width of 3, the range `0` to `46655`, the `reading_ugm3 >= 0` precondition, and the five worked encode cases above.

- [ ] **Step 8: Commit**

```bash
git add docs/superpowers/specs/2026-08-08-environmental-assignment-storyline-design.md
git commit -m "Verify the A9 base-36 encoding in Python and Rust and record it in the spec"
```

---

## Task 3: Update VISION.md

**Files:**
- Modify: `VISION.md`

**Interfaces:**
- Consumes: Task 1's AURA rules.
- Produces: the authority text Task 4's three public pages must match.

- [ ] **Step 1: Section 6, the story line paragraph**

Replace the paragraph beginning "Assignments wear the Mission Ares story line" with the WRO equivalent: assignments arrive as a piece of the office's work, each carries a confident but subtly wrong claim from AURA, and the upcoming recitation assesses the same skeletons in Rubber Duck Robotics costume. State that there is no plot and the thread is technical.

- [ ] **Step 2: Section 6, the contract sentence**

Find "one named entry-point function (a second only when the family is genuinely two independent outputs)". Add the ladder-forced-codec clause from Task 1 Step 7, so A9's two functions do not read as a violation.

- [ ] **Step 3: Section 6, the console input clause**

Add to the structure description: A3 is the one assignment where the student authors console input; from A4 on, `main` and its console input and output are provided code the student reads. Add the wholeness convention: every assignment from A3 on ships a `main` that prompts, reads, calls the student's function, and prints, and that `main` is never graded.

- [ ] **Step 4: Remove the capstone framing in four places**

```bash
grep -n "capstone" VISION.md
```

Expected hits at lines 639, 705, 727, 745, and 992. In each, A10 becomes "the final assignment" or "the week-10 assignment", never "the capstone". Section 6's sizing paragraph already requires week 10 to be sized like any other week; make the surrounding text agree with it instead of pulling against it.

- [ ] **Step 5: Section 8, the family table**

Rename the "Mission Ares face (assignments)" column to "Willamette Resource Office face (assignments)" and replace all six cells with the mapping in section 7 of the story line spec. Leave the "Rubber Duck face" column untouched.

- [ ] **Step 6: Section 8, the story line subsection**

Replace the whole "Mission Ares (assignments)" subsection with a "Willamette Resource Office (assignments)" subsection: the setting, the absence of a plot, the technical thread, the majors-coverage rotation (copy the table from section 5 of the story line spec), and AURA with the falsifiability rule.

- [ ] **Step 7: Section 8, the red herring ladder**

The "Assignments" bullet: one mild spec distractor plus **AURA's claim**. Keep the invariant sentence that a correct solution never depends on noticing the herring.

- [ ] **Step 8: Section 11, the carry-over notes**

Update the assignments bullet: the Mission Ares framings join the calculator, financial planner, dictionary, and linear-equation framings as retired, with their mechanics surviving inside the WRO families.

- [ ] **Step 9: Section 10, lectures 4 and 9**

Lecture 4's notes-content cell gains "console input: reading a line and turning it into a number". Lecture 9's cell loses console input and keeps errors, parsing, exceptions versus `Result`, and validation.

- [ ] **Step 10: Verify**

```bash
grep -n "Mission Ares\|HAB\|capstone\|Sol 100" VISION.md
```

Expected: no matches except where Mission Ares is named as a retired framing in section 11.

```bash
npm run build
```

Expected: PASS.

- [ ] **Step 11: Commit**

```bash
git add VISION.md
git commit -m "Update VISION for the WRO story line, AURA, console input, and A10 as a regular week"
```

---

## Task 4: The three public pages that must agree, atomically

These three state the story lines as course canon. A partial landing leaves the published site contradicting itself, so they are one task and one commit.

**Files:**
- Modify: `src/content/docs/reference/story-bibles.mdx`
- Modify: `src/content/docs/recitations/index.mdx`
- Modify: `src/content/docs/assignments/introduction.mdx`
- Modify: `src/content/docs/practicalities/finding-docs-and-helpers.mdx`

**Interfaces:**
- Consumes: Task 3's VISION section 8 text. These pages are the student-facing rendering of it and must not diverge.
- Produces: nothing later tasks depend on.

- [ ] **Step 1: Rewrite the story bible's assignment half**

In `story-bibles.mdx`, replace the "Mission Ares (assignments)" section with "The Willamette Resource Office (assignments)". Keep the existing page's voice (second person, addressed to the student). Cover: the office, the student's role, no plot, and AURA as the vendor AI assistant that is confidently wrong at convenient moments, with the office rule that no AURA analysis is published unverified.

Note the existing page says HAB "drafts a piece of code and hands it to you to sign off or reject". That is the **old** format and contradicts the skill, which requires a prose claim rather than a code draft. Fix it while you are here: AURA states a claim, never hands over a draft.

- [ ] **Step 2: Rewrite the story bible's family table**

Replace the "Mission Ares face (assignments)" column with the WRO mapping from Task 3 Step 5. The two tables (here and VISION section 8) must be word-for-word identical in the assignment column.

- [ ] **Step 3: Leave the Rubber Duck Robotics section untouched**

Verify by diff that nothing in the Gary or RDR content changed. The tier-costume contrast is the reason the swap stops at the assignment tier.

- [ ] **Step 4: Update the recitations index**

Read `src/content/docs/recitations/index.mdx` and update only the sentences that reference the assignment tier's costume when explaining the contrast. The RDR content itself does not change.

- [ ] **Step 5: Update the assignments introduction**

Rewrite `src/content/docs/assignments/introduction.mdx` for the WRO framing and AURA. Update any list of assignment titles to the new slugs from the Global Constraints table. Since Phase 3 has not run yet, links to renamed pages will not resolve; leave the old links in place in this task and fix them in Task 20, or the build fails here.

- [ ] **Step 6: Update finding-docs-and-helpers**

Read `src/content/docs/practicalities/finding-docs-and-helpers.mdx` and replace its Mission Ares and HAB references. It is a how-to guide, so keep numbered steps and do not let story flavor grow.

- [ ] **Step 7: Verify**

```bash
grep -rn "Mission Ares\|\bHAB\b" src/content/docs/reference/ src/content/docs/recitations/ src/content/docs/practicalities/
```

Expected: no matches.

```bash
npm run build
```

Expected: PASS.

- [ ] **Step 8: Commit**

```bash
git add src/content/docs/reference/story-bibles.mdx src/content/docs/recitations/index.mdx src/content/docs/assignments/introduction.mdx src/content/docs/practicalities/finding-docs-and-helpers.mdx
git commit -m "Re-skin the public story pages for the WRO and AURA"
```

---

# Phase 2: console input across the lecture tier

## Task 5: Teach console input in lecture 4

**Files:**
- Modify: `src/content/docs/lectures/variables-and-state.mdx`

**Interfaces:**
- Consumes: nothing.
- Produces: the `read_i32()` / `read_f64()` helper names that Task 6, Task 9, and Task 12 (A3) all reference. The helper names are fixed here and must not be renamed later.

- [ ] **Step 1: Load the lecture notes skill and read both lectures**

Load `engr103-lecture-notes`. Read `src/content/docs/lectures/variables-and-state.mdx` in full, and read the "Console Input" section of `src/content/docs/lectures/errors-input-and-validation.mdx` (lines 28 to 93), which is the prose you are relocating and adapting.

- [ ] **Step 2: Write the new concept section**

Add a "Console Input" section **after** the existing assignment-and-rebinding material, because input is a source of a value to bind, so binding must already be understood. It must contain:

- The definition: console input is text a program reads from the person running it while the program executes, rather than text already written into the source code.
- Python's `input(prompt)`: prints the prompt, pauses, returns what was typed as a string with the trailing newline already removed.
- The conversion to a number with `int(...)` or `float(...)`.
- Rust's provided helper, shown in full so the reader can see what it does, with the four deferred pieces named and each pointed at the lecture that pays it off (`use std::io;` standard-library plumbing; `String::new()` lecture 13; `&mut` lecture 15; `.expect()` lecture 9).
- One sentence planting lecture 9: if the person types something that is not a number, the program stops with an error message, and week 5 is about what to do instead.

- [ ] **Step 3: Write the tabbed example**

```mdx
<Tabs syncKey="lang">
<TabItem label="Python">
```python title="Reading a number someone types"
countdown_seconds = int(input("Countdown (s): "))
print(countdown_seconds)
```
</TabItem>
<TabItem label="Rust">
```rust title="Reading a number someone types"
use std::io;

fn read_i32() -> i32 {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("failed to read line");
    text.trim().parse().expect("not a whole number")
}

fn main() {
    println!("Countdown (s): ");
    let countdown_seconds = read_i32();
    println!("{}", countdown_seconds);
}
```
</TabItem>
</Tabs>
```

Note the parse uses the **return type annotation** on `read_i32`, not a turbofish. Generics are never used in this course, so `.parse::<i32>()` is forbidden.

- [ ] **Step 4: Write the WhatDiffers call-out**

Fill the `differs` slot with: Python folds the whole job into one call that hands back the typed text with the newline already removed. Rust spells the same job out across several steps, so this course hands you `read_i32` and `read_f64` written for you; the text Rust reads still has the newline attached, which is why the helper calls `.trim()` before turning it into a number.

- [ ] **Step 5: Verify both examples run**

```bash
echo "500" | python3 "$SCRATCH/l4.py"
rustc -o "$SCRATCH/l4" "$SCRATCH/l4.rs" && echo "500" | "$SCRATCH/l4"
```

Expected: both print `500`. Paste the tab contents into those files exactly as they appear on the page; if a paste does not run, the example is not self-contained and must be fixed.

- [ ] **Step 6: Update the ai-summary block**

Add to `covers:` "console input with input() and the provided read_i32/read_f64 helpers". Add to `glossary_terms:` "console input, parsing".

- [ ] **Step 7: Verify**

```bash
npm run build
```

Expected: PASS.

- [ ] **Step 8: Commit**

```bash
git add src/content/docs/lectures/variables-and-state.mdx
git commit -m "Teach console input in lecture 4 with a provided Rust helper"
```

---

## Task 6: Unbundle lecture 9

**Files:**
- Modify: `src/content/docs/lectures/errors-input-and-validation.mdx`

**Interfaces:**
- Consumes: Task 5's `read_i32` / `read_f64` names and the four deferrals list.
- Produces: nothing later tasks depend on.

- [ ] **Step 1: Remove the Console Input section**

Delete the "Console Input" section (currently lines 28 to 93, including its tabbed example, its run commands, its `:::note` on piped input and EOF, and its `WhatDiffers`). Keep the `:::note` content about EOF only if it still reads naturally in the validation context; if it does not, move it to lecture 4 in a follow-up rather than orphaning it.

- [ ] **Step 2: Rewrite the opening**

The current opening (lines 24 to 26) introduces input as a new idea. Rewrite it so input is **recalled**, not introduced: the reader already knows how to read a number someone types, from lecture 4. What they do not know is what happens when the text is not a number at all, or is a number that makes no sense. That is this lecture.

- [ ] **Step 3: Pay off the deferred `.expect()`**

Task 5 promised lecture 9 would explain `.expect()`. Add a short passage in the `Result` material that closes that loop explicitly, naming `read_i32`'s `.expect()` calls as the thing being explained.

- [ ] **Step 4: Update the ai-summary block**

Remove "console input with input() and read_line" from `covers:`. Remove "console input" from `glossary_terms:` (it now belongs to lecture 4). Keep parsing, exception, Result, and input validation.

- [ ] **Step 5: Verify no input teaching remains**

```bash
grep -n "read_line\|input(" src/content/docs/lectures/errors-input-and-validation.mdx
```

Expected: matches only where input is recalled or where `.expect()` is being explained, never where it is being taught for the first time.

- [ ] **Step 6: Verify**

```bash
npm run build
```

Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add src/content/docs/lectures/errors-input-and-validation.mdx
git commit -m "Unbundle lecture 9: input moves to lecture 4, validation stays"
```

---

## Task 7: Move input on the language ladder and fill the glossary gap

**Files:**
- Modify: `src/content/docs/reference/language-ladder.mdx`
- Modify: `src/content/docs/reference/glossary.mdx`

**Interfaces:**
- Consumes: Task 5 and Task 6.
- Produces: the binding ladder rows every Phase 3 task checks against.

- [ ] **Step 1: Edit the week 2 ladder row**

Line 19. Add `input`; `int()`/`float()` parsing to the Python column, and `read_i32`/`read_f64` (provided) to the Rust column.

- [ ] **Step 2: Edit the week 5 ladder row**

Line 22. Remove `input` from the Python column and `read_line` from the Rust column. Keep `.parse()`, `Result`, `match`, `try`/`except`, `elif` chains, and float tolerance.

- [ ] **Step 3: Add the glossary entries**

`glossary.mdx` currently has no `console input` entry despite lecture 9 listing it as a glossary term, so this closes a pre-existing gap. Add, in the page's established format:

- **console input**: text a program reads from the person running it while the program executes, rather than text already written into the source code.
- **parsing**: turning text into a value of another type, such as turning the text `"25"` into the whole number `25`.

- [ ] **Step 4: Verify**

```bash
npm run build
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/content/docs/reference/language-ladder.mdx src/content/docs/reference/glossary.mdx
git commit -m "Move console input to ladder week 2 and add the missing glossary entries"
```

---

## Task 8: Update the two affected activities

**Files:**
- Modify: `src/content/docs/activities/variables-and-state.mdx`
- Modify: `src/content/docs/activities/errors-input-and-validation.mdx`

**Interfaces:**
- Consumes: Task 5's helper names.
- Produces: nothing later tasks depend on.

- [ ] **Step 1: Load the activities skill**

Load `engr103-activities`. Activities are thin prompt sheets, not tutorials: a listing, a question, room to predict.

- [ ] **Step 2: Add an input block to the variables activity**

Add one activity block. It must include a **run-it-yourself step**, not only a predict-the-output step. This is deliberate: lecture 4 is week 2's Wednesday lecture and A3 is due the following Monday, so the activity is the only supervised chance a student gets to run an interactive program before being graded on writing one.

The block should have students run a two-line program that reads a number and prints something computed from it, then run it again typing a word instead of a number, and write down what happened. That second run is the planted setup for lecture 9.

- [ ] **Step 3: Trim the errors activity**

Remove the part of `activities/errors-input-and-validation.mdx` that introduces input for the first time. Keep everything about validation and about handling a parse that fails.

- [ ] **Step 4: Verify**

```bash
npm run build
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/content/docs/activities/variables-and-state.mdx src/content/docs/activities/errors-input-and-validation.mdx
git commit -m "Move the input activity block from week 5 to week 2"
```

---

## Task 9: Document the starter-repo wholeness convention

**Files:**
- Modify: `src/content/docs/practicalities/starter-repo-and-check.mdx`

**Interfaces:**
- Consumes: Task 5's helper names.
- Produces: the convention every Phase 3 assignment page's "How This Assignment Works" paragraph asserts.

- [ ] **Step 1: Load the skill and read the page**

This is a how-to guide: numbered steps, one goal per page, no story flavor.

- [ ] **Step 2: Document the interactive main convention**

State that every assignment from A3 on ships a `main` that prompts, reads, calls the student's function, and prints, so the program can be run at a terminal like any other program. From A4 on that `main` is provided and is never graded; the grader calls the student's function directly.

- [ ] **Step 3: Document the main-versus-check constraint**

State plainly, because this is the failure mode that would waste the most student time:

> Running `check` never asks you to type anything. The local tests call your function directly and do not go through `main`. In Python the input lives behind the `if __name__ == "__main__":` guard; in Rust the tests call your function without running `main`. If `check` ever sits waiting for you to type, that is a bug in the starter, not something you did.

- [ ] **Step 4: Document the Rust helpers**

Name `read_i32()` and `read_f64()`, say they are provided in the starter, point at lecture 4 for what they do, and say Python needs no equivalent because `input()` already is one.

- [ ] **Step 5: Verify**

```bash
npm run build
```

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/content/docs/practicalities/starter-repo-and-check.mdx
git commit -m "Document the interactive main convention and the check-never-blocks rule"
```

---

# Phase 3: the ten assignments

Tasks 10 through 19 share one shape. **Every one of them runs these steps**, and the per-task sections below give only what differs.

**The shared step list, run for each assignment:**

1. Load the `engr103-assignments` skill. Read it, do not recall it.
2. Read the spec's slot entry in section 6 of `docs/superpowers/specs/2026-08-08-environmental-assignment-storyline-design.md`, and its row in section 6a.
3. Read the assignment being replaced, for its structure and its ladder discipline, never for its framing.
4. Check the ladder row for the assignment's week in `src/content/docs/reference/language-ladder.mdx`. Nothing above that row may appear.
5. `git mv` the old `.mdx` to the new slug and `git mv` the old rubric TSV to `canvas/assignments/<new-slug>-rubrics.tsv`.
6. Write the page: `ai-summary` block, opening narrative (2 to 4 sentences, no plot references), How This Assignment Works, What to Submit, The Problem, AURA's claim as its own `<Aside>`, one mild spec distractor planted and never defused, Step 1 acceptance tests, Step 2 pseudocode, Step 3 code, Summit Problem, Paper Practice with its `<Reveal>`, Reflection, Rubric.
7. **Verify every code example runs**, in both languages, by pasting it into a scratch file and running it. Reading is not verification.
8. **Run AURA's claim to ground**: implement the wrong approach the claim describes, confirm it produces a wrong result on a case in the visible tests, and confirm a correct approach passes. This verification goes in the work log, never on the page.
9. **Cite every real constant** inline with agency, dataset, and year. If a figure cannot be sourced, either source it or replace it with an authored scenario value that is not presented as published.
10. Rewrite the rubric TSV, keeping the `RubricTable src=` path in the page in sync with the new filename, and keeping an implementation criterion that names the week's construct.
11. Run `npm run build`. Expected: PASS.
12. Commit page and rubric together.

## Task 10: A1, field-office-setup

> **Run the twelve shared steps listed under "Phase 3: the ten assignments" above.
> They are the task.** Everything below is only what differs for this assignment,
> and is not a substitute for the shared list. Do not start without reading it.

**Files:**
- Rename: `src/content/docs/assignments/systems-checkout.mdx` to `field-office-setup.mdx`
- Rename: `canvas/assignments/systems-checkout-rubrics.tsv` to `field-office-setup-rubrics.tsv`

Onboarding only. No AURA claim, no spec distractor, no summit problem, no paper practice. Toolchain, starter repository, the `check` harness, the `__main__` idiom, Gradescope submission. The re-skin is a text swap: the new crew member checking out a flight-software workstation becomes the new hire checking out the office workstation.

## Task 11: A2, household-energy-panel

> **Run the twelve shared steps listed under "Phase 3: the ten assignments" above.
> They are the task.** Everything below is only what differs for this assignment,
> and is not a substitute for the shared list. Do not start without reading it.

**Files:**
- Rename: `pre-flight-check.mdx` to `household-energy-panel.mdx`, and its rubric

**Ladder:** literals and precedence, **no variables, no functions**, graded by comparing printed output. Every division uses float operands so both languages agree. No mixing of whole numbers and floats in one expression. Must exercise whole numbers, floats, booleans, and a binary literal. **No console input** (there are no variables to bind it to).

**Problem:** a household energy audit panel printing annual kWh from an appliance's rated watts and daily hours, the resulting kg CO2e at the local grid intensity, a boolean for whether the household sits above the state median, and the smart meter's status bitfield as a binary literal.

**Constants to cite:** grid carbon intensity in gCO2/kWh (EPA eGRID, named subregion and year).

## Task 12: A3, ev-carbon-payback

> **Run the twelve shared steps listed under "Phase 3: the ten assignments" above.
> They are the task.** Everything below is only what differs for this assignment,
> and is not a substitute for the shared list. Do not start without reading it.

**Files:**
- Rename: `launch-window.mdx` to `ev-carbon-payback.mdx`, and its rubric

This task carries **both** changes and is the densest slot in the plan. Read the console input spec as well as the story line spec.

**Ladder:** variables, integer division and the remainder operator, **no functions**, graded by comparing printed output.

**Problem:** an electric vehicle carries higher manufacturing emissions (the battery) and lower per-km operating emissions than its combustion equivalent. Compute the payback distance, then express it as whole years of typical driving plus the leftover kilometres in the final year:

- `payback_km = extra_manufacturing_g // saving_per_km_g`
- `whole_years = payback_km // annual_km`
- `leftover_km = payback_km % annual_km`

**Console input:** the scenario values arrive from standard input, not from starter-set variables. The page's problem statement changes from "the starter program sets for you" to a description of reading them. Requirements, worked example, assumptions, and edge cases are otherwise unchanged in shape. Add one rubric row for reading the values correctly.

**Modeling note to preserve:** the EV's per-km figure depends on the grid it charges from, and the Northwest figure and the US average give visibly different paybacks. That is the point, and the reflection line's "which intermediate values did you name and why" question is where it surfaces. Do not resolve it on the page.

**Constants to cite:** gasoline CO2 per litre (EPA), grid intensity (EPA eGRID), battery manufacturing kgCO2e per kWh of pack (IVL or Argonne GREET).

**AURA claim:** a claim that treats a rate as a total, or that averages two grid intensities that should be weighted by share.

## Task 13: A4, low-carbon-concrete

> **Run the twelve shared steps listed under "Phase 3: the ten assignments" above.
> They are the task.** Everything below is only what differs for this assignment,
> and is not a substitute for the shared list. Do not start without reading it.

**Files:**
- Rename: `injection-burn-math.mdx` to `low-carbon-concrete.mdx`, and its rubric

**Ladder:** functions. Two entry points. Solved equations given in LaTeX; the student never derives math.

**Problem:** choose the masses of two supplementary cementitious materials to hit both a total binder mass target and a total embodied-CO2 target. With `x` and `y` the two masses, `M` the binder target, `C` the CO2 target, and `a` and `b` the intensities:

`x + y = M` and `a*x + b*y = C`, solved as `x = (C - b*M) / (a - b)` and `y = M - x`.

Give both solved forms in `<Latex>` on the page.

**Units:** the target arrives in tonnes and kg CO2 per tonne while the intensities are per kg, so a conversion sits inside the problem. Leave it for the student; simple arithmetic is never pre-decomposed.

**Constants to cite:** Portland cement embodied CO2 per kg, fly ash and slag under byproduct allocation (industry EPD averages, named).

## Task 14: A5, burn-window-clearance

> **Run the twelve shared steps listed under "Phase 3: the ten assignments" above.
> They are the task.** Everything below is only what differs for this assignment,
> and is not a substitute for the shared list. Do not start without reading it.

**Files:**
- Rename: `transit-checks.mdx` to `burn-window-clearance.mdx`, and its rubric

**Ladder:** comparisons joined by `and`, `or`, `not`. Four outcomes with a priority override. **No `elif` or `else if` chains.** No loops or collections. Scalar parameters only, never a string.

**Problem:** wind speed, relative humidity, and fuel moisture must all sit inside their windows for a prescribed burn to proceed, and a county-declared burn ban overrides everything regardless of conditions.

**AURA claim:** about the override's precedence, or about joining three conditions with the wrong connective.

## Task 15: A6, sensor-drift-check

> **Run the twelve shared steps listed under "Phase 3: the ten assignments" above.
> They are the task.** Everything below is only what differs for this assignment,
> and is not a substitute for the shared list. Do not start without reading it.

**Files:**
- Rename: `airlock-and-readings.mdx` to `sensor-drift-check.mdx`, and its rubric

**Ladder:** an `elif` / `else if` chain or Rust `match`, checked in a fixed priority order, over a physical range plus a float tolerance against a trusted fallback. The harness parses; the student's function receives a number. Scalar parameters only.

**Problem:** `reading_status(reading_ugm3, reference_ugm3, tolerance_ugm3)` returning `1` below zero (shorted line), `2` above the sensor's stated ceiling (railed high), `3` in range but further than the tolerance from the co-located reference monitor (drift), `0` otherwise (trusted).

**This task also fixes the reading-line format** that A10 parses and A9 feeds. Define it here, once, and record it in the spec so Tasks 18 and 19 use the identical format.

**Constants to cite:** low-cost sensor measurement range (PMS5003 datasheet), EPA PM2.5 standards.

**AURA claim:** the tolerance boundary claim (whether a gap exactly equal to the tolerance counts as agreement).

## Task 16: A7, dashboard-publication-tests

> **Run the twelve shared steps listed under "Phase 3: the ten assignments" above.
> They are the task.** Everything below is only what differs for this assignment,
> and is not a substitute for the shared list. Do not start without reading it.

**Files:**
- Rename: `flight-qualification.mdx` to `dashboard-publication-tests.mdx`, and its rubric

**Ladder:** an assert-based test suite against a **provided** implementation carrying two rules: an inclusive band, and a freshness rule with an override. The grader runs the student's test function and reads which assertions raise. The suite must accept the correct implementation and reject every broken one.

**Problem:** certify the check that decides whether a reading may go to the public dashboard: the value must sit inside the plausible band and the timestamp must be fresh within the stated window, with a sensor clock-fault flag overriding the freshness rule.

**AURA claim:** that two passing cases covering both outcomes are enough to qualify the check.

**Note:** you must author the broken implementations the suite has to reject, and verify that a correct suite rejects all of them and accepts the correct one. This is the task's real verification and it goes in the work log.

## Task 17: A8, solar-array-lifetime

> **Run the twelve shared steps listed under "Phase 3: the ten assignments" above.
> They are the task.** Everything below is only what differs for this assignment,
> and is not a substitute for the shared list. Do not start without reading it.

**Files:**
- Rename: `first-drive.mdx` to `solar-array-lifetime.mdx`, and its rubric

**Ladder:** loops. **No collections. No power operator.** A running value carried across a data-dependent number of passes.

**Problem:** a solar module loses a fixed fraction of its output each year. Accumulate the total kWh generated across the array's life, looping year by year until annual output falls below the warranty floor. The count depends on the degradation rate and the floor, not on a constant.

**Rubric implementation criterion (required):** the total must be computed by iteration. A pasted geometric-series closed form does not earn the credit in a loops week. Name this in the rubric explicitly.

**Constants to cite:** median module degradation per year (NREL).

**AURA claim:** that the total is the first-year output times the number of years, or that the degradation applies once.

## Task 18: A9, field-logger-codes

> **Run the twelve shared steps listed under "Phase 3: the ten assignments" above.
> They are the task.** Everything below is only what differs for this assignment,
> and is not a substitute for the shared list. Do not start without reading it.

**Files:**
- Rename: `comms-window.mdx` to `field-logger-codes.mdx`, and its rubric

**Ladder:** window is week 8, which is **before references**, so **no `String`, `&str`, or `Vec` may be a parameter**. Returning an owned `String` is legal.

**Contract, verified in Task 2:**
- `code_for(reading_ugm3: i32) -> String`, a fixed-width 3-character base-36 code, valid for `0` to `46655`, with the precondition `reading_ugm3 >= 0`.
- `digit_value(c: char) -> i32`, returning `0` to `35` for a valid base-36 digit character and `-1` otherwise.

**Worked cases** (use these, they are verified): `code_for(0)` is `"000"`, `code_for(35)` is `"00z"`, `code_for(36)` is `"010"`, `code_for(1296)` is `"100"`, `code_for(46655)` is `"zzz"`. `digit_value('0')` is `0`, `digit_value('a')` is `10`, `digit_value('z')` is `35`, `digit_value('!')` is `-1`.

**Two rules specific to this task:**

1. **The two functions are not inverses of each other.** `code_for` builds a multi-character code; `digit_value` decodes one character. **Do not author a round-trip acceptance test**, and do not let the page imply one exists. A10 closes the round trip.
2. **No negative operand may reach `%`.** State the `reading_ugm3 >= 0` precondition in the assumptions.

**Problem framing:** remote stations transmit over a satellite or LoRa link where every byte costs battery, so each reading is packed into a compact fixed-width base-36 code.

**AURA claim:** about digit order (that the code can be built least-significant-digit first and read the same way), or that a value too large for the fixed width can be clamped rather than rejected.

## Task 19: A10, emissions-inventory

> **Run the twelve shared steps listed under "Phase 3: the ten assignments" above.
> They are the task.** Everything below is only what differs for this assignment,
> and is not a substitute for the shared list. Do not start without reading it.

**Files:**
- Rename: `sol-100.mdx` to `emissions-inventory.mdx`, and its rubric

**A10 is a regular assignment week.** No capstone framing, no "everything you have learned", one problem at ordinary size.

**Ladder:** **one** entry-point function. It parses a raw text line character by character, validates it, conditionally mutates the caller's list **in place**, and returns a formatted report line. The grader inspects the caller's list after the call. Strings and lists only, no dict or HashMap.

**Problem:** raw readings arrive from the field network as text lines in the format Task 15 fixed, carrying the base-36 codes Task 18 produced. Validate the line, fold it into the running station manifest in place when it qualifies, and return the formatted report line. This is where the A9 round trip closes: A10 reads back what A9 wrote.

**The summit problem** is the independent-copy function: return a freshly built list independent of the one it was given, proven by the grader mutating the original afterward. Extra credit, so it never enters the required weekly budget.

**The equity critique** (one page, completion-graded, in the Canvas PDF): air quality monitor siting. Regulatory monitors are distributed unevenly, so the inventory the student just built reports confidently about the neighborhoods it watches and is silent about the ones it does not, and the silence reads as absence of a problem. The student critiques the tool they built: what it assumes, where it fails, and whom it fails. Cite a real source on monitor distribution.

**AURA claim:** that a function cannot change the caller's manifest unless it hands it back, so the return value is the only thing that matters.

---

## Task 20: Final sweep

**Files:**
- Modify: `src/content/docs/assignments/introduction.mdx` (fix the links deferred in Task 4)
- Modify: any file the greps below turn up
- Create: `/Users/ulbrical/.claude/projects/-Users-ulbrical-GitHub-engr103/memory/` updates

- [ ] **Step 1: Fix the deferred assignment links**

Update every link in `assignments/introduction.mdx` to the new slugs.

- [ ] **Step 2: Sweep for stale story references**

```bash
grep -rin "mission ares\|\bHAB\b\|sol 100\|airlock\|rover\|martian\|habitat" src canvas VISION.md .claude/skills
```

Expected: no matches, except where section 11 of VISION names Mission Ares as a retired framing.

- [ ] **Step 3: Sweep for stale slugs**

```bash
grep -rn "systems-checkout\|pre-flight-check\|launch-window\|injection-burn-math\|transit-checks\|airlock-and-readings\|flight-qualification\|first-drive\|comms-window\|sol-100" src canvas
```

Expected: no matches.

- [ ] **Step 4: Sweep for uncited constants**

Read each of the ten assignment pages and confirm every real-world figure carries an inline source. This cannot be automated; it is a read-through.

- [ ] **Step 5: Full build**

```bash
npm run build
```

Expected: PASS, with the links validator reporting no broken links.

- [ ] **Step 6: Update memory**

Update `engr103-project.md`, `engr103-assignment-tier-status.md`, and `engr103-assignment-challenge-model.md` in the memory directory: the story line is now the Willamette Resource Office, the wrong-claim character is AURA with the falsifiability rule, A10 is a regular week rather than a capstone, and console input is authored only in A3. Update `MEMORY.md` pointers if any hook text changed.

- [ ] **Step 7: Commit**

```bash
git add -A
git commit -m "Final sweep: fix deferred links, verify no stale story or slug references"
```

---

## Notes on scope

This plan covers both specs in full. If you would rather run it in two sittings, the natural cut is **after Task 9**: Phases 1 and 2 leave the repository in a consistent, buildable state where every authority document and the whole lecture tier reflect both specs, and only the ten assignment pages still wear the old costume. That is an ugly-but-coherent intermediate state, not a broken one. Do not cut inside Phase 1: Tasks 3 and 4 must land together or the published site contradicts itself.
