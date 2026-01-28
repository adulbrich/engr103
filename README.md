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

- [ ] Itemize rubrics for "subjective" demo grading
- [ ] Consider starting with functions earlier.
- [ ] Create a Question? component that uses details/summary for better UX (ie in studio to challenge students)
- [ ] Update Svelte components to Svelte 5.
- [ ] Add learning objectives to each lecture, optionally studios and assignments as well
- [ ] Improve the "secondary" learning outcomes for the class (to be more programming specific)
- [ ] Illustrate how variables are stored in memory (when discussing variables), i.e. show that the variable name is just a label for a memory address
- [ ] Illustrate how arguments / parameters are passed on the call stack (when discussing functions) and stored in memory, also discuss the return value/address
- [ ] Illustrate base address and references (when discussing arrays and references)
- [ ] Make the dev env lecture notes much simpler and add information about using other IDEs/editors
  - make additional concepts of the shell clearly optional
- [ ] Mention using `getline()` in dictionary assignment instead of `cin`
- [ ] Provide larger code examples/programs the students can play with

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
