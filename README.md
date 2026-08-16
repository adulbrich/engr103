# ENGR 103: Engineering Computation and Algorithmic Thinking

This repository contains the source for the ENGR 103 course website at Oregon
State: an Astro + Starlight site holding every lecture note, activity,
assignment, recitation coverage page, and reference page, plus the Canvas
rubrics, the assignment starters, and the PDF and slide tooling that ship
alongside them.

The course is taught in **Python and Rust simultaneously**. Every concept
appears in both languages, and students may answer in either.

## Contributing

Course-design and content work is tracked in
[issues](https://github.com/adulbrich/engr103/issues), organised by area
(`area:lectures`, `area:assignments`, ...) and by kind of work (`type:content`,
`type:decision`, ...). The **Next Iteration** milestone holds what is committed
for the next offering; **Backlog** holds what is wanted but not yet scheduled.

Found a typo, a broken link, or an instruction that does not work? Please open
an issue.

The per-term instructor checklist lives in
[`.github/ISSUE_TEMPLATE/term-setup.md`](.github/ISSUE_TEMPLATE/term-setup.md);
open a fresh copy of it at the start of each term.

Authoring conventions for the different kinds of page live in
[`AGENTS.md`](AGENTS.md) and in the skills under `.claude/skills/`.
`VISION.md` holds the design rationale behind the whole course, and
`ARCHITECTURE.md` covers the proposed recitation-lab hardware.

## Assessments live in a separate, private repository

Graded, on-paper, offline material lives in the private
`github.com/adulbrich/engr103-assessments`: the R2
through R10 recitation handouts and their answer keys, the midterm and final
banks with their practice papers, the pre-lecture check QTI packages, the
autograders, and the paper build scripts. This repository is public, so a
problem authored here is a published problem.

What that leaves here: `src/content/docs/recitations/` holds coverage pages that
state each session's premise and the skills it evaluates, and carry no problems.
Recitation 1 (`intern-orientation.mdx`) is ungraded and is authored here in
full.

## Assignment starters

Each assignment ships as a downloadable starter rather than a repository to
clone:

```bash
./scripts/build-starters.sh              # every starter
./scripts/build-starters.sh <slug> ...   # only the named ones
```

Starters are assembled into `zip/<slug>/` and zipped alongside as
`zip/<slug>.zip` for upload to Canvas. `zip/` is build output and is not
tracked. Per-assignment files live in `starters/<slug>/`; the `check` and `pack`
tools live once in `starters/_lib/` and are copied in at build time, so there is
one copy of the harness to maintain.

## Canvas

`canvas/assignments/` holds one rubric per assignment as a `.tsv`. The
assignment pages import these at build time through `RubricTable.astro`, so they
are not optional: renaming or deleting one breaks the build.

The same files import directly into Canvas using the [Canvas LMS Mods (Basic)
Extension](https://chromewebstore.google.com/detail/bnpdolbpbjiniodlbahddbnkollgojon?utm_source=item-share-cb).
Go to your Canvas course Rubrics tab, click "Add Rubric", then click "Import
Details". A template of the expected format is in
`canvas/assignments/_template/template_rubric_details_name.tsv`.

## PDFs

The `scripts/generate-pdfs.sh` helper will retrieve and generate individual PDFs
for all lectures, activities, assignments, and practicalities from the live
deployment, then generate a combined version of all files. Output lands in
`pdf/`, which is not tracked. To run it, use:

```bash
cd scripts
chmod +x ./generate-pdfs.sh
./generate-pdfs.sh
```

You can change the `BASE` variable in the script to point to a different
deployment if needed. Recitation and exam PDFs are built in
`engr103-assessments` instead, by its own script.

To generate slides from `marp`, use:

```bash
cd slides
npx @marp-team/marp-cli@latest web-graphics.md --pdf --allow-local-files
```

Change `web-graphics.md` to the desired slide deck.

## 🚀 Project Structure

Inside of this Astro + Starlight project, you'll see the following folders and
files:

```text
.
├── .claude/
│   └── skills/
├── .github/
│   └── ISSUE_TEMPLATE/
├── canvas/
├── public/
├── scripts/
├── slides/
├── starters/
├── src/
│   ├── assets/
│   ├── components/
│   ├── content/
│   │   └── docs/
│   ├── content.config.ts
│   └── env.d.ts
├── astro.config.mjs
├── package.json
└── tsconfig.json
```

Important directories and files include:

- `.claude/skills/`: authoring skills that define the house style for lecture
  notes, activities, assignments, and recitation coverage pages
- `canvas/`: TSV rubrics for Canvas LMS, imported by the assignment pages at
  build time
- `scripts/`: helper scripts for generating PDFs and building the starters
- `slides/`: slide decks for lectures in Marp Markdown format
- `starters/`: per-assignment starter code, plus the shared `check` and `pack`
  harness in `starters/_lib/`
- `src/content/docs/`: all lecture notes, activities, assignments, recitation
  coverage pages, references, and practicalities, in `mdx` format
- `src/components/`: reusable components for assignments and lectures, including
  a LaTeX component and reactive Svelte components
- `astro.config.mjs`: Astro configuration file, update sidebar entries here

## 🧞 Commands

All commands are run from the root of the project, from a terminal.

This project uses `npm`. Feel free to use `pnpm`, `yarn`, or `bun` if you
prefer.

| Command                   | Action                                           |
| :------------------------ | :----------------------------------------------- |
| `npm install`             | Installs dependencies                            |
| `npm run dev`             | Starts local dev server at `localhost:4321`      |
| `npm run build`           | Build your production site to `./dist/`          |
| `npm run preview`         | Preview your build locally, before deploying     |
| `npm run astro ...`       | Run CLI commands like `astro add`, `astro check` |
| `npm run astro -- --help` | Get help using the Astro CLI                     |

`npm run build` is the gate for any content change: it runs `astro check`, and
the `starlight-links-validator` integration fails the build on a broken internal
link.

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

I tried Quarto + Reveal.js to generate slides instead of Marp but it was
difficult to automate the PDF generation. Marp seems to work well enough for
now. Reveal.js is very feature-rich, tough, and would be a good option if we
wanted to make more complex slides in the future.

A better option seem to be [Slidev](https://sli.dev/), which is also
Markdown-based, is feature-rich, flexible, AND can export to PDF easily. To be
tested in the future, maybe in a separate repository.

## License

Licensed under [Creative Commons Attribution-ShareAlike 4.0
International](https://creativecommons.org/licenses/by-sa/4.0/) (CC BY-SA 4.0).
See [`LICENSE`](LICENSE).

You may share and adapt this material, including commercially, provided you give
appropriate credit and license your adaptations under the same terms.

## Acknowledgments

The original content for the first iteration of this textbook (C++) was provided
by Alexander Guyer under CC-BY. This includes lectures notes, studios,
assignments, practicalities, and other content. The ENGR 103 course was
originally developed by Jennifer Parham-Mocello and Natasha Mallette. They
developed the contents in their capacity of faculty at Oregon State University.
