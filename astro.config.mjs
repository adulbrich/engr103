// @ts-check
import { defineConfig } from "astro/config";
import sitemap from "@astrojs/sitemap";
import starlight from "@astrojs/starlight";
import tailwind from "@astrojs/tailwind";

import svelte from "@astrojs/svelte";

// https://astro.build/config
export default defineConfig({
  site: "https://engr103.alexulbrich.com",
  integrations: [
    sitemap(),
    starlight({
      title: "ENGR 103 Engineering Computation and Algorithmic Thinking",
      social: {
        github: "https://github.com/adulbrich/engr103",
      },
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
      customCss: ["./src/tailwind.css"],
    }),
    tailwind({ applyBaseStyles: false }),
    svelte(),
  ],
});
