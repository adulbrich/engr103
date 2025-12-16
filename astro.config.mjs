// @ts-check
import { defineConfig } from "astro/config";
import sitemap from "@astrojs/sitemap";
import starlight from "@astrojs/starlight";
import tailwindcss from "@tailwindcss/vite";
import mermaid from 'astro-mermaid';
import svelte from "@astrojs/svelte";
import starlightPageActions from 'starlight-page-actions'

// https://astro.build/config
export default defineConfig({
  vite: {
    plugins: [tailwindcss()],
  },
  site: "https://engr103.alexulbrich.com",
  integrations: [
    mermaid({
      theme: 'dark',
      autoTheme: true
    }),
    sitemap(),
    starlight({
      plugins: [
        starlightPageActions({
          baseUrl: "https://engr103.alexulbrich.com",
        })
      ],
      title: "ENGR 103 Engineering Computation and Algorithmic Thinking",
      social: [
        { icon: "github", label: "GitHub", href: "https://github.com/adulbrich/engr103" },
      ],
      lastUpdated: true,
      head: [
        {
          tag: "script",
          attrs: {
            src: "/knowledge/js/script.outbound-links.js",
            "data-api": "/knowledge/api/event",
            "data-domain": "engr103.alexulbrich.com",
            defer: true,
          },
        },
      ],
      sidebar: [
        {
          label: "Overview",
          autogenerate: { directory: "overview" },
        },
        {
          label: "Practicalities",
          autogenerate: { directory: "practicalities" },
        },
        {
          label: "Lecture Notes",
          autogenerate: { directory: "lectures" },
        },
        {
          label: "Studios",
          autogenerate: { directory: "studios" },
        },
        {
          label: "Assignments",
          autogenerate: { directory: "assignments" },
        },
        {
          label: "About",
          autogenerate: { directory: "about" },
        },
      ],
      customCss: ["./src/styles/global.css"],
    }),
    svelte(),
  ],
});
