# ENGR 103: Engineering Computation and Algorithmic Thinking

Source for the ENGR 103 course website at Oregon State: an Astro + Starlight
site holding every lecture note, activity, assignment, recitation, and reference
page, plus the Canvas rubrics and the PDF and slide tooling that ship alongside
them.

The course is taught in **Python and Rust simultaneously**. Every concept
appears in both languages, and students may answer in either.

```bash
npm install
npm run dev      # localhost:4321
npm run build    # astro check + astro build + link validation
```

`npm run build` is the gate for any content change. It runs `astro check`, builds
the site, and fails on any broken internal link.

## How the course is put together

Five kinds of page live in `src/content/docs/`, and they are deliberately
different genres. Mixing them is the most common authoring mistake, which is why
each has its own authoring skill in `.claude/skills/`.

| Directory | What it is | Genre | Story line |
|---|---|---|---|
| `lectures/` | Read before class, flipped | Explanation, prose | none |
| `activities/` | Run live in lecture | Guided problem sets with closed reveals | none |
| `assignments/` | Take-home, AI allowed | Requirements with one open problem | Willamette Resource Office |
| `recitations/` | Proctored, on paper, **kept as drafts** | Bare problem sheets | Rubber Duck Robotics |
| `reference/`, `practicalities/` | Language ladder, glossary, archetypes, how-tos | Terse reference and numbered how-to | none |

Two invariants worth knowing before you edit anything:

- **The language ladder** (`reference/language-ladder.mdx`) is binding. Every
  graded item must be solvable with the constructs listed for its week, and
  nothing above that row may appear.
- **`draft: true` on the recitations keeps them off the built site and nothing
  more.** This repository is public, so the handout sources are readable on
  GitHub by anyone who looks. VISION describes recitations as private handouts;
  that is currently not true in practice. See the open item below.

`VISION.md` holds the design rationale behind all of it, and `ARCHITECTURE.md`
covers the proposed recitation-lab hardware.

## Authoring conventions

Load the matching skill in `.claude/skills/` before editing a content file:
`engr103-lecture-notes`, `engr103-activities`, `engr103-assignments`, or
`engr103-recitations`. They encode the genre rules, the dual-language pattern,
and the plain-language rule the whole site is written to (first year, never
programmed, may not be a first-language English speaker).

## Needs Your Attention: Environmental Story Line Redesign

The assignment tier has been re-skinned from Mission Ares to the Willamette
Resource Office, and console input moved from lecture 9 to lecture 4. All 21
planned tasks are done and reviewed. Design specs are in
`docs/superpowers/specs/`, the plan is in
`docs/superpowers/plans/2026-08-08-environmental-storyline-and-console-input.md`,
and per-task detail including every deferred minor is in
`.superpowers/sdd/2026-08-08-environmental-storyline-and-console-input/progress.md`.

These items are the ones a person has to settle.

### Verify the real-world constants before publication

Every assignment cites real figures inline. Only A3's were fetched from primary
sources during authoring; the rest were reused or cited from the authoring pass and
need a human spot-check.

- [ ] **A2 household-energy-panel:** grid intensity 275 gCO2e/kWh (EPA eGRID2022, NWPP subregion)
- [ ] **A3 ev-carbon-payback:** gasoline 8,887 g CO2/gal (EPA); eGRID2022 NWPP 275 g/kWh; eGRID2022 US average 823.1 lb/MWh; battery 61 to 106 kgCO2e/kWh (IVL 2019). These four were fetched and matched during authoring, so this is a confirmation rather than a first check.
- [ ] **A4 low-carbon-concrete:** Portland cement 919 kg CO2e/tonne (PCA industry-average EPD, ASTM-certified, 2021); slag 147 kg CO2e/tonne (Slag Cement Association EPD-011, 2015)
- [ ] **A4 fly ash, 40 kg CO2e/tonne:** deliberately presented on the page as a WRO office figure, **not** as a citation, because no clean industry-average EPD was findable. Decide whether to source it properly or leave it as an authored figure.
- [ ] **A5 burn-window-clearance:** all three windows are authored WRO figures by design, explicitly labeled, not published. Decide whether to source a real state forestry prescription instead.
- [ ] **A6 sensor-drift-check:** EPA PM2.5 NAAQS figures are cited with agency, dataset, and year. The PMS5003 sensor range (0 to 500 µg/m3, manual v2.3, 2016) was cross-confirmed across several retailer mirrors because the sandbox blocks the primary Plantower PDF, so it is honest but not primary-sourced.
- [ ] **A8 solar-array-lifetime:** module degradation rate. Its summit (spent-fuel decay) deliberately carries **no** citation: radioactive decay is described generically because no isotope half-life could be verified in-session. Decide whether to source a real one.
- [ ] **A10 emissions-inventory:** the monitor-siting equity source was fetched and its comparison group corrected during authoring (the paper compares against the overall U.S. population). Worth one confirmation read.

### Classroom logistics that the pages now depend on

- [ ] **Week 2's activity needs `read_i32` and `read_f64` in the room.** The new input block has students run a Rust program that calls them, but the starter repositories do not begin until A3. Either release the A3 starter before week 2's Wednesday lecture, or hand out the two helpers in that session. The page now states this dependency, but stating it does not solve it.

### Look at the two new simulators

Both were verified by `getBoundingClientRect()` measurement rather than visually,
because the screenshot tool returned black during authoring. The pixel geometry is
exact, but nobody has actually looked at them.

- [ ] `src/components/EvPaybackForm.svelte` (on the A3 page)
- [ ] `src/components/ConcreteMixForm.svelte` (on the A4 page)

### Cross-tier alignment, deliberately out of this project's scope

VISION requires an assignment and its recitation sibling to differ by at most one
structural twist. Recitations were a stated non-goal, so nothing in them was
touched, but rewriting the assignments exposed drift.

- [ ] **A5 takes three readings** (wind speed, relative humidity, fuel moisture) while its sibling R5 (`recitations/hot-tub-safety-checks.mdx`) still takes two. Either add a third to R5 or drop A5 to two.
- [ ] **Re-derive the whole recitation slate against the new assignments.** The ten recitations still wear Rubber Duck Robotics, which is correct and deliberate, but their problems were written against the Mars assignments. Each one should be checked as a same-level sibling of its new assignment.

### Recitation handouts are publicly readable

The nine graded recitation handouts are `draft: true`, which excludes them from
the built site but not from this public repository. They have been readable at
`github.com/adulbrich/engr103` since they were authored and pushed, and they are
readable right now. A proctored assessment whose questions are published is not
proctored in any useful sense.

Treat the current handouts as compromised regardless of what happens next.
Content that has been pushed to a public repository cannot be un-published:
clones, forks, and code-search indexes may retain it, so making the repository
private later stops future exposure without undoing past exposure.

- [ ] **Decide where recitations live** (private repository, private submodule, or outside git entirely) and stop the ongoing exposure.
- [ ] **Rewrite the nine graded handouts** as fresh variations, wherever they end up.
- [ ] **Correct VISION section 5**, which claims recitations are "kept private" and "never published on the course site." The second half is true; the first is not, and the section reads as though the draft flag delivers the privacy.

### Known gaps nobody owns yet

- [ ] **`engr103-assignments/SKILL.md` line 278 points at `linear-equations-rubrics.tsv`**, deleted before this project began. Pre-existing, out of scope for the sweep, still broken.
- [ ] **The `total = total * total` double-reference case is gone.** It lived in the week-2 activity block that the input block replaced. The general evaluate-then-bind idea survives in two other examples, but that specific harder case has no home now.

### Accepted costs, recorded so they are not a surprise later

No action needed unless you disagree with the call.

- **Two Rust examples do not run as pasted.** Lecture 4 shows a call with no body; lecture 9 shows a body with no `main`. Both say so explicitly on the page. This is a deliberate break with the house rule that examples be self-contained, and it follows from teaching the console-input call in week 2 while the body waits for week 5.
- **A3 prints no prompt.** Forced, not chosen: a printed prompt makes Python and Rust stdout differ and breaks A3's output-diff grading. Applies to A3 alone; from A4 on the provided `main` prompts freely because it is never graded.
- **The language ladder's generics rule was narrowed** rather than removing `parse::<T>` from the course. Students never write generic code; they call one generic method with a taught spelling.
- **A10 is a regular assignment week, not a capstone.** One entry-point function, ordinary size; the independent-copy function moved to its summit problem.
- **Bioengineering and nuclear are reached through summit problems** (extra credit) rather than required problems, which is narrower than Mission Ares managed.

## Instructor Checklist

Every term:

- [ ] Update **Due Dates**, **Due Times**, **Course Schedule**, and **Syllabus** in Canvas
- [ ] Update sections, **TA Names**, **Office Hours** in Canvas
- [ ] Add TAs to Canvas
- [ ] Update professional development assignments, resources, and end-of-term survey
- [ ] Bump the repository version in `package.json` if content changed
- [ ] Set up the Gradescope course, upload the autograder image, and link it from Canvas
- [ ] Publish the starter repositories on OSU GitLab and link them from Canvas
- [ ] Print the recitation handouts and arrange secure storage between sections
- [ ] Add DAS accommodations for the midterm, the final, and the recitations
- [ ] Dry-run every assignment end to end on a clean machine

## Canvas

The `canvas/` directory contains assets for the Canvas LMS.

Rubrics are stored in a `.tsv` format, which can be imported directly into Canvas using the [Canvas LMS Mods (Basic) Extension](https://chromewebstore.google.com/detail/bnpdolbpbjiniodlbahddbnkollgojon?utm_source=item-share-cb). Go to your Canvas course Rubrics tab, click "Add Rubric", then click "Import Details". A template of the expected format can be found in `canvas/_template/template_rubric_details_name.tsv`.

Assignments are stored in HTML format. When editing a Canvas assignment, click the `</>` icon in the rich text editor to switch to HTML mode, then copy and paste the contents of the corresponding HTML file from the `canvas/` directory. The HTML assignments are not full-fledged HTML pages but rather snippets that work for Canvas assignments. CSS styles are included inline in the HTML snippets.

## PDFs

The `scripts/generate-pdfs.sh` helper will retrieve and generate individual PDFs for all lectures, activities, assignments, and practicalities from the live deployment. Recitations are excluded because they are never published. It will then generate a combined version of all files. To run it, use:

```bash
cd scripts
chmod +x ./generate-pdfs.sh
./generate-pdfs.sh
```

You can change the `BASE` variable in the script to point to a different deployment if needed.

To generate slides from `marp`, use:

```bash
cd slides
npx @marp-team/marp-cli@latest web-graphics.md --pdf --allow-local-files
```

Change `web-graphics.md` to the desired slide deck.

## Backlog

### Course design

- [ ] Add learning objectives to each lecture, and optionally to activities and assignments
- [ ] Author the pre-lecture checks: three archetype-drawn multiple-choice questions per lecture, with distractors mined from real student mistakes
- [ ] Build the exam bank against the archetype inventory
- [ ] Write the recitation handouts as same-level siblings of the rewritten assignments

### Contents

- [ ] Provide larger code examples and programs students can play with
- [ ] Illustrate base addresses for arrays (done for references already)
- [ ] Fill out the C++ and MATLAB bridge appendices and the language field guide

### Housekeeping

- [ ] Create a Question? component using details/summary for better UX
- [ ] Update the remaining Svelte components to Svelte 5 (`EvPaybackForm` and `ConcreteMixForm` already are)

## 🚀 Project Structure

Inside of your Astro + Starlight project, you'll see the following folders and files:

```text
.
├── canvas/
├── public/
├── scripts/
├── slides/
├── src/
│   ├── assets/
│   ├── components/
│   ├── content/
│   │   ├── docs/
│   │   └── config.ts
│   └── env.d.ts
├── astro.config.mjs
├── package.json
├── tailwind.config.mjs
└── tsconfig.json
```

Important directories and files include:

- `canvas/`: assets for Canvas LMS (TSV rubrics, HTML assignments)
- `scripts/`: helper scripts for generating PDFs, etc.
- `slides/`: slide decks for lectures in Marp Markdown format
- `src/content/docs/`: all lecture notes, activities, assignments, recitations, references, and practicalities, in `mdx` format
- `src/components/`: reusable components for assignments and lectures, inlcuding a Latex component, and reactive Svelte components for assignments
- `astro.config.mjs`: Astro configuration file, update sidebar entries here

## 🧞 Commands

All commands are run from the root of the project, from a terminal.

This project uses `npm`. Feel free to use `pnpm`, `yarn`, or `bun` if you prefer.

| Command                   | Action                                           |
| :------------------------ | :----------------------------------------------- |
| `npm install`             | Installs dependencies                            |
| `npm run dev`             | Starts local dev server at `localhost:4321`      |
| `npm run build`           | Build your production site to `./dist/`          |
| `npm run preview`         | Preview your build locally, before deploying     |
| `npm run astro ...`       | Run CLI commands like `astro add`, `astro check` |
| `npm run astro -- --help` | Get help using the Astro CLI                     |

## Resources

- [Starlight Getting Started](https://starlight.astro.build/getting-started/) -- Astro template
- [TailwindCSS](https://tailwindcss.com/) -- for styling
- [Svelte](https://svelte.dev/docs/svelte/overview) -- for reactive components
- [Starlight Page Actions](https://github.com/dlcastillop/starlight-page-actions), or alternatively [Starlight Contextual Menu](https://github.com/corsfix/starlight-contextual-menu)
- [Starlight Links Validator](https://github.com/HiDeoo/starlight-links-validator)
- [How to create an Astro LaTeX component](https://danidiaztech.com/create-astro-latex-component/) -- LaTeX component
- [KaTeX: The fastest math typesetting library for the web.](https://katex.org/) -- LaTeX component
- [Mermaid User Guide](https://mermaid.js.org/intro/getting-started.html) -- Mermaid component

## Notes

### Markdown Slides to PDF

I tried Quarto + Reveal.js to generate slides instead of Marp but it was difficult to automate the PDF generation. Marp seems to work well enough for now. Reveal.js is very feature-rich, tough, and would be a good option if we wanted to make more complex slides in the future.

A better option seem to be [Slidev](https://sli.dev/), which is also Markdown-based, is feature-rich, flexible, AND can export to PDF easily. To be tested in the future, maybe in a separate repository.

### VS Code

In the future, have vim and the ENGR servers as optional dev environments, and focus more on VS Code since it's more common nowadays. Find a stack (C++ or Rust or Python) that is easy to install and set up on different OSes and VS Code. Don't rely on OS-specific dependencies.

## Acknowledgments

The original content for the first iteration of this textbook (C++) was provided by Alexander Guyer under CC-BY. This includes lectures notes, studios, assignments, practicalities, and other content. The ENGR 103 course was originally developed by Jennifer Parham-Mocello and Natasha Mallette. They developed the contents in their capacity of faculty at Oregon State University.
