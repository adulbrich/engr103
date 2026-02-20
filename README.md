# ENGR 103: Engineering Computation and Algorithmic Thinking

This repository contains the source code for the ENGR103 website.

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
bun x @marp-team/marp-cli@latest web-graphics.md --pdf --allow-local-files
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

- [ ] Consider starting with functions earlier? It's already pretty early because we need to discuss variables/expressions first.
- [ ] Illustrate how variables are stored in memory (when discussing variables), i.e. show that the variable name is just a label for a memory address
- [ ] Illustrate how arguments / parameters are passed on the call stack (when discussing functions) and stored in memory, also discuss the return value/address
- [ ] Illustrate base address and references (when discussing arrays and references)
- [ ] Make the dev env lecture notes much simpler and add information about using other IDEs/editors
  - make additional concepts of the shell clearly optional (or have a practicalities/guide page for advanced shell usage)
- [ ] Mention using `getline()` in dictionary assignment instead of `cin`
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

This project uses `bun` instead of `npm`. Feel free to use `npm`, `pnpm`, or `yarn` if you prefer.

| Command                   | Action                                           |
| :------------------------ | :----------------------------------------------- |
| `bun install`             | Installs dependencies                            |
| `bun run dev`             | Starts local dev server at `localhost:4321`      |
| `bun run build`           | Build your production site to `./dist/`          |
| `bun run preview`         | Preview your build locally, before deploying     |
| `bun run astro ...`       | Run CLI commands like `astro add`, `astro check` |
| `bun run astro -- --help` | Get help using the Astro CLI                     |

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
