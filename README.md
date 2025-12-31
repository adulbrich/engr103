# ENGR 103: Engineering Computation and Algorithmic Thinking

This repository contains the source code for the ENGR103 website.

## Instructor Checklist

This coming term:

- [ ] Update the professional development assignments (text is wrong and weights are wrong)
- [ ] Create slide deck for intro lecture
- [ ] Add the Canvas assignments + rubrics to GitHub

Every term:

- [ ] Update professional development assignments, resources, and end-of-term survey
- [x] Update **Due Dates** in Canvas
- [x] Update **Due Times** for quizzes and exams
- [x] Update **Course Schedule** in Canvas
- [x] Update **Syllabus** in Canvas
- [x] Update the version of this repository if making changes (`package.json`)
- [ ] Add TAs to Canvas - missing one FERPA compliant student
- [ ] Create team **Bookings** page for assignment demo and set up slots (**not working**, investigate alternatives?)
- [x] Update sections in Canvas
- [x] Update **TA Names** in Canvas
- [ ] Update **Office Hours** in Canvas
- [ ] Update **Booking Links** in Canvas
- [ ] Create **GitHub Classroom** class, add TAs, add assignments
- [ ] Update GitHub Classroom **links** in Canvas (assignments)
- [ ] Test all studios and assignments on the ENGR servers

## PDFs

The `scripts/generate-pdfs.sh` helper will retrieve and generate individual PDFs for all lectures, studios, assignments, and practicalities. It will then generate a combined version of all files. To run it, use:

```bash
cd scripts
chmod +x ./generate-pdfs.sh
./generate-pdfs.sh
```

To generate slides from `marp`, use:

```bash
cd slides
npx @marp-team/marp-cli@latest web-graphics.md --pdf --allow-local-files
```

## To Dos

- [ ] Fix "FIX" comments
- [ ] Fix "IMPROVE" comments
- [ ] Create a Question? component that uses details/summary for better UX (ie in studio to challenge students)
- [ ] Add AI guidelines to syllabus
- [ ] Update rubrics for assignments to account for the potential design changes and for the AI critique component
- [ ] Add learning objectives to each lecture, optionally studios and assignments as well
- [ ] Improve the "secondary" learning outcomes for the class (to be more programming specific)
- [ ] Add visual explanations of memory allocation and references
- [ ] Make the dev env lecture notes much simpler and add information about using other IDEs/editors
  - make additional concepts of the shell clearly optional
- [ ] Add visual explanations for the C++ basics
  - what constitutes a computer (incl. hardware components relevant to programming), ties in with memory later
  - diagram to show how tranlsators work (compiler, linker, loader)
  - diagram to show build pipeline for C++
- [ ] Add visual explanations for git
- [ ] Mention using `getline()` in dictionary assignment instead of `cin`
- [ ] Provide larger code examples/programs the students can play with

### If making multiple versions (C++, Python, etc)

- [ ] Use [Internationalization](https://starlight.astro.build/guides/i18n/) to have multiple version of the course for different terms or different languages.
- [ ] Change the icon for the language dropdown (code instead of translation)
- [ ] Figure out the CSS for the Latex component. Right now, the containing `<span>` has some padding on the right which looks odd when there's punctuation following the expression.

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

## 🚀 Project Structure

Inside of your Astro + Starlight project, you'll see the following folders and files:

```text
.
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

- `scripts/`: helper scripts for generating PDFs, etc.
- `slides/`: slide decks for lectures in Marp Markdown format
- `src/content/docs/`: all lecture notes, studios, assignments, practicalities, and other documentation, in `mdx` format
- `src/components/`: reusable components for assignments and lectures, inlcuding a Latex component, and reactive Svelte components for assignments
- `astro.config.mjs`: Astro configuration file, update sidebar entries here

## 🧞 Commands

All commands are run from the root of the project, from a terminal.

This project uses `bun` instead of `npm`.

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

## Acknowledgments

The original content for the first iteration of this textbook (C++) was provided by Alexander Guyer under CC-BY. This includes lectures notes, studios, assignments, practicalities, and other content. The ENGR 103 course was originally developed by Jennifer Parham-Mocello and Natasha Mallette. They developed the contents in their capacity of faculty at Oregon State University.
