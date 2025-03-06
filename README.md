# ENGR 103: Engineering Computation and Algorithmic Thinking

## PDFs

To generate the documentation in PDF format, use [starlight-to-pdf](https://github.com/Linkerin/starlight-to-pdf).

Here's the command to generate one large PDF.

```bash
npx starlight-to-pdf https://engr103.alexulbrich.com/ --footer ./footer.html --header ./header.html --filename engr103 --no-contents --margins '2cm 1cm 2cm 1cm'
```

To generate lecture notes in separate files (one per page), use the corresponding bash script:

```bash
chmod +x ./pdf/generate-lecture-notes.sh
./pdf/generate-lecture-notes.sh
```

To generate slides from `marp`, use:

```bash
cd slides
npx @marp-team/marp-cli@latest web-graphics.md --pdf --allow-local-files
```

## Parking

- [ ] Use the `print:` pseudo-class in Tailwind to see if we can enable/disable things in the PDF (such as the flowcharts or simulators).
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

```
.
├── public/
├── src/
│   ├── assets/
│   ├── content/
│   │   ├── docs/
│   │   └── config.ts
│   └── env.d.ts
├── astro.config.mjs
├── package.json
├── tailwind.config.mjs
└── tsconfig.json
```

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
- [How to create an Astro LaTeX component](https://danidiaztech.com/create-astro-latex-component/) -- LaTeX component
- [KaTeX: The fastest math typesetting library for the web.](https://katex.org/) -- LaTeX component
- [Adding Mermaid diagrams to Astro MDX](https://xkonti.tech/blog/astro-mermaid-mdx/) -- Mermaid component
- [Mermaid User Guide](https://mermaid.js.org/intro/getting-started.html) -- Mermaid component