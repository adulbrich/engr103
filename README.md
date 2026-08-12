# ENGR 103: Engineering Computation and Algorithmic Thinking

This repository contains the source code for the ENGR103 website.

## Needs Your Attention: Environmental Story Line Redesign

The assignment tier is being re-skinned from Mission Ares to the Willamette
Resource Office. Design specs are in `docs/superpowers/specs/`, the plan is in
`docs/superpowers/plans/2026-08-08-environmental-storyline-and-console-input.md`,
and per-task progress is in
`.superpowers/sdd/2026-08-08-environmental-storyline-and-console-input/progress.md`.

These items are the ones a person has to settle. Everything else is being handled
in the plan.

### Verify the real-world constants before publication

Every assignment cites real figures inline. Only A3's were fetched from primary
sources during authoring; the rest were reused or cited from the authoring pass and
need a human spot-check.

- [ ] **A2 household-energy-panel:** grid intensity 275 gCO2e/kWh (EPA eGRID2022, NWPP subregion)
- [ ] **A3 ev-carbon-payback:** gasoline 8,887 g CO2/gal (EPA); eGRID2022 NWPP 275 g/kWh; eGRID2022 US average 823.1 lb/MWh; battery 61 to 106 kgCO2e/kWh (IVL 2019). These four were fetched and matched during authoring, so this is a confirmation rather than a first check.
- [ ] **A4 low-carbon-concrete:** Portland cement 919 kg CO2e/tonne (PCA industry-average EPD, ASTM-certified, 2021); slag 147 kg CO2e/tonne (Slag Cement Association EPD-011, 2015)
- [ ] **A4 fly ash, 40 kg CO2e/tonne:** deliberately presented on the page as a WRO office figure, **not** as a citation, because no clean industry-average EPD was findable. Decide whether to source it properly or leave it as an authored figure.
- [ ] **A6 sensor-drift-check:** EPA PM2.5 NAAQS figures (cited, agency/dataset/year present). The PMS5003 sensor range (0 to 500 µg/m3, manual v2.3, 2016) was cross-confirmed across several retailer mirrors because the sandbox blocks the primary Plantower PDF, so it is honest but not primary-sourced.
- [ ] **A5 burn-window-clearance:** all three windows are authored WRO figures by design, explicitly labeled, not published. Decide whether to source a real state forestry prescription instead.
- [ ] **A8:** constants not yet authored; check when that lands

### Decide on one cross-tier mismatch

VISION requires an assignment and its recitation sibling to differ by at most one
structural twist. Recitations are a stated non-goal of this project, so nothing in
them has been touched, but rewriting A5 introduced a gap.

- [ ] **A5 now takes three readings** (wind speed, relative humidity, fuel moisture) while its recitation sibling R5 (`src/content/docs/recitations/hot-tub-safety-checks.mdx`) still takes two. Either add a third reading to R5, or drop A5 to two. Watch for the same thing as A6 through A10 land.

### Look at the two new simulators

Both were verified by `getBoundingClientRect()` measurement rather than visually,
because the screenshot tool returned black during authoring. The pixel geometry is
exact, but nobody has actually looked at them.

- [ ] `src/components/EvPaybackForm.svelte` (on the A3 page)
- [ ] `src/components/ConcreteMixForm.svelte` (on the A4 page)

### Unblock three tasks

A second agent has been holding files this work also needs.

- [ ] **Commit or release `VISION.md`.** Tasks 3 (VISION rewrite) and 4 (the three public story pages, which must match VISION word for word) cannot run until it is free.
- [ ] **Say which activity files the other agent is editing.** Task 8 touches `src/content/docs/activities/variables-and-state.mdx` and `errors-input-and-validation.mdx`. If those two are clear, it can run immediately.

### Accepted costs, recorded so they are not a surprise later

No action needed unless you disagree with the call.

- **Two Rust examples do not run as pasted.** Lecture 4 shows a call with no body; lecture 9 shows a body with no `main`. Both say so explicitly on the page. This is a deliberate break with the house rule that examples be self-contained, and it follows from teaching the console-input call in week 2 while the body waits for week 5.
- **A3 prints no prompt.** Forced, not chosen: a printed prompt makes Python and Rust stdout differ and breaks A3's output-diff grading. Applies to A3 alone; from A4 on the provided `main` prompts freely because it is never graded.
- **The language ladder's generics rule was narrowed** rather than removing `parse::<T>` from the course. Students never write generic code; they call one generic method with a taught spelling.
- **A10 is a regular assignment week, not a capstone.** One entry-point function, ordinary size; the independent-copy function moved to its summit problem.
- **Bioengineering and nuclear are reached through summit problems** (extra credit) rather than required problems, which is narrower than Mission Ares managed.

## Instructor Checklist

This coming term:

- [ ] Add the Canvas assignments + rubrics to GitHub (OK for assignments 2, 3, 4, 5)

Every term:

- [x] Update professional development assignments, resources, and end-of-term survey
- [x] Update **Due Dates** in Canvas
- [x] Update **Due Times** for quizzes and exams
- [x] Update **Course Schedule** in Canvas
- [x] Update **Syllabus** in Canvas
- [x] Update the version of this repository if making changes (`package.json`)
- [x] Add TAs to Canvas
- [x] Create team **Bookings** page for assignment demo and set up slots
- [x] Update sections in Canvas
- [x] Update **TA Names** in Canvas
- [x] Update **Office Hours** in Canvas
- [x] Update **Booking Links** in Canvas
- [x] Create **GitHub Classroom** class
- [x] Add TAs to GitHub Organization and send Classroom invite link
- [x] Add assignments to GitHub Classroom
- [x] Update GitHub Classroom **links** in Canvas (assignments)
- [ ] Test all studios and assignments on the ENGR servers
- [ ] Add DAS accommodations for midterm and final exams

## Canvas

The `canvas/` directory contains assets for the Canvas LMS.

Rubrics are stored in a `.tsv` format, which can be imported directly into Canvas using the [Canvas LMS Mods (Basic) Extension](https://chromewebstore.google.com/detail/bnpdolbpbjiniodlbahddbnkollgojon?utm_source=item-share-cb). Go to your Canvas course Rubrics tab, click "Add Rubric", then click "Import Details". A template of the expected format can be found in `canvas/_template/template_rubric_details_name.tsv`.

Assignments are stored in HTML format. When editing a Canvas assignment, click the `</>` icon in the rich text editor to switch to HTML mode, then copy and paste the contents of the corresponding HTML file from the `canvas/` directory. The HTML assignments are not full-fledged HTML pages but rather snippets that work for Canvas assignments. CSS styles are included inline in the HTML snippets.

## PDFs

The `scripts/generate-pdfs.sh` helper will retrieve and generate individual PDFs for all lectures, studios, assignments, and practicalities from the live deployment. It will then generate a combined version of all files. To run it, use:

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

## To Dos

### Course Design

- [ ] Add more content on proper design (including testing and coming up with good test cases)
- [ ] Completely re-evalute assessment in this class, assuming that everyone has access to strong GenAI tools (i.e., tools that can reason, write code, and explain code -- basically tackle any of the fundamental concepts they should be learning). Here are some ideas:
  - How do we define the goal underlying engineering studies? Here's a possible answer: reliably judge under uncertainty in safety-critical, economically consequential systems.
  - Could we do more in-person live oral (vivas) assessments in addition to the written exams? We already do live demos but many students still use GenAI to write their code.
  - Could we have vivas where students have to think through a problem on a whiteboard? They would not know the problem in advance, and this would be similar to "live" coding interviews that are common in industry. Instead of 70+ unique problems, we could have afew with variations in values. we don't have to grade everyone every week or every assignment, but at least grade students twice every term in a viva format. This would require coming up with good rubric items to make it easy for TAs to conduct these sessions and give consistent feedback.
  - Studios/recitations have very simple problems to help students practice fundamentals. If they use GenAI or other resources to "give" them the answers, they are not learning those fundamentals. Is that a problem? Will it be a problem? How can we make those studios useful? SHould we re-design the problems or the format/assessment?
  - Add constraints to entry-level assignments, forbidding the use of advanced features of the language (no references, pointers, arrays, strings, or new libraries in early assignments, for example).
  - Separate an "auto-graded" outcome part from the "process" and "understanding" part of the grade. Provide a function that accepts required inputs and outputs for the auto-graded part, for each assignment.
  - Force process through mutli-commits in their repository history. Require a certain number of commits, and require that they include messages that indicate their process (e.g., "initial commit", "added function to calculate factorial", "added test cases for edge cases", etc.). This would make it easier for TAs to grade the process and understanding part of the grade, even if the code itself is generated by GenAI.
  - Ask students to submit a hand-drawn explanation of the memory before/after lines are executed, or the call stack for a function, or the flow of data through their program.
  - The design part should have a hand-drawn workflow diagram, or a hand-drawn sketch of the program structure, or a hand-drawn sketch of the memory layout, etc.
  - Midterm/final => 50% of the grade
  - Viva component => 25% of the grade
  - Studios/assignments/participation => 25% of the grade
  - Randomize in-person instructor interview on larger assignments -- could this reduce incentive to use GenAI for the entire assignment, since they won't know which part will be discussed in the interview?
  - Reward understanding over output (output could be lower grade, auto-graded, and understanding would count for much more). Need to have TAs trained to handle this grading, including adequate rubrics and training sessions to ensure consistency across TAs.
  - Rubrics should include submitting all AI prompts and responses as part of the assignment, and grading the quality of the prompts and the understanding of the responses. This would encourage students to use GenAI as a learning tool rather than a shortcut to get answers.
  - Don't ban AI tools, but focus on process and verification?
- [ ] Itemize rubrics for "subjective" demo grading
- [ ] Add learning objectives to each lecture, optionally studios and assignments as well
- [ ] Improve the "secondary" learning outcomes for the class (to be more programming specific)

### Contents

- [ ] Consider additional explanations about static vs dynamic array allocations, the STL (resizable and dynamic) and automatic vs dynamic storage duration (bonus)
- [ ] Consider additional object-oriented programming contents, including examples of class declaration and usage, and the design of classes (bonus)
- [ ] Illustrate base address and references (when discussing arrays and references) - done for references, not for arrays yet
- [ ] Provide larger code examples/programs the students can play with

### Housekeeping

- [ ] Create a Question? component that uses details/summary for better UX (ie in studio to challenge students)
- [ ] Update Svelte components to Svelte 5.

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
- `src/content/docs/`: all lecture notes, studios, assignments, practicalities, and other documentation, in `mdx` format
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

### Multi-Code Version (C++, Python, etc.)

One option is to use [Internationalization](https://starlight.astro.build/guides/i18n/) to have multiple version of the course for different terms or different languages. We would need to change the icon for the language dropdown (code instead of translation) so that it is clear what the purpose is.

Here's an example of the change to be made to `astro.config.mjs` to enable different versions:

```js
defaultLocale: 'root',
locales: {
  root: {
    label: 'C++',
    lang: 'cpp', 
  },
  'py': {
    label: 'Python',
    lang: 'py',
  },
},
```

I don't think this will be necessary. When I change the programming language to Rust or Python, I might:

- Fork or cretae a new repository for the new language, or
- Replace the content in this repository entirely, or
- Keep the C++ contents in the repository but in another directory.

### VS Code

In the future, have vim and the ENGR servers as optional dev environments, and focus more on VS Code since it's more common nowadays. Find a stack (C++ or Rust or Python) that is easy to install and set up on different OSes and VS Code. Don't rely on OS-specific dependencies.

## Acknowledgments

The original content for the first iteration of this textbook (C++) was provided by Alexander Guyer under CC-BY. This includes lectures notes, studios, assignments, practicalities, and other content. The ENGR 103 course was originally developed by Jennifer Parham-Mocello and Natasha Mallette. They developed the contents in their capacity of faculty at Oregon State University.
