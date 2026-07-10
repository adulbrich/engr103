// @ts-check
import { defineConfig } from "astro/config";
import sitemap from "@astrojs/sitemap";
import starlight from "@astrojs/starlight";
import tailwindcss from "@tailwindcss/vite";
import mermaid from 'astro-mermaid';
import svelte from "@astrojs/svelte";
import starlightPageActions from 'starlight-page-actions';
import starlightLinksValidator from 'starlight-links-validator';

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
        starlightLinksValidator(),
        starlightPageActions({
          baseUrl: "https://engr103.alexulbrich.com",
          actions: {
            markdown: false,
            // custom: {
            //   grok: {
            //     label: "Open in Grok",
            //     href: "https://grok.com/?q=",
            //   },
            // },
          },
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
          items: [{ autogenerate: { "directory": "overview" } }],
        },
        {
          label: "Practicalities",
          items: [{ autogenerate: { "directory": "practicalities" } }],
        },
        {
          label: "Lecture Notes",
          items: [{ autogenerate: { "directory": "lectures" } }],
        },
        {
          label: "Activities",
          items: [{ autogenerate: { "directory": "activities" } }],
        },
        {
          label: "Assignments",
          items: [{ autogenerate: { "directory": "assignments" } }],
        },
        {
          label: "Recitations",
          items: [{ autogenerate: { "directory": "recitations" } }],
        },
        {
          label: "Reference",
          items: [{ autogenerate: { "directory": "reference" } }],
        },
        {
          label: "About",
          items: [{ autogenerate: { "directory": "about" } }],
        },
      ],
      customCss: ["./src/styles/global.css"],
    }),
    svelte(),
  ],
});
