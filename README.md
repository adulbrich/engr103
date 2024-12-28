# ENGR 103: Engineering Computation and Algorithmic Thinking

To generate the documentation in PDF format, use:

```bash
npx starlight-to-pdf https://engr103.alexulbrich.com/
```

## Parking

- [ ] Figure out the CSS for the Latex component. Right now, the containing `<span>` has some padding on the right which looks odd when there's punctuation following the expression.

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