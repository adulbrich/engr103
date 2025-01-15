# ENGR 103: Engineering Computation and Algorithmic Thinking

## PDFs

To generate the documentation in PDF format, use [starlight-to-pdf](https://github.com/Linkerin/starlight-to-pdf):

```bash
npx starlight-to-pdf https://engr103.alexulbrich.com/ --footer ./footer.html --header ./header.html --filename engr103 --no-contents --margins '2cm 1cm 2cm 1cm'
```

Here's an example to output the PDF of one lecture and stopping at the next one and excluding it (because I cannot find how to print to PDF only one webpage):

```bash
npx starlight-to-pdf https://engr103.alexulbrich.com/lectures/expressions --last /lectures/operators --exclude /lectures/operators --footer ./footer.html --header ./header.html --filename lecture-notes-expressions --no-contents --margins '2cm 1cm 2cm 1cm'
```

## Parking

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

All commands are run from the root of the project, from a terminal:

| Command                   | Action                                           |
| :------------------------ | :----------------------------------------------- |
| `npm install`             | Installs dependencies                            |
| `npm run dev`             | Starts local dev server at `localhost:4321`      |
| `npm run build`           | Build your production site to `./dist/`          |
| `npm run preview`         | Preview your build locally, before deploying     |
| `npm run astro ...`       | Run CLI commands like `astro add`, `astro check` |
| `npm run astro -- --help` | Get help using the Astro CLI                     |

## Resources

Displaying Latex:

- https://danidiaztech.com/create-astro-latex-component/
- https://katex.org/