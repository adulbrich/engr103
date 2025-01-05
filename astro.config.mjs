// @ts-check
import { defineConfig } from "astro/config";
import sitemap from "@astrojs/sitemap";
import starlight from "@astrojs/starlight";
import tailwind from "@astrojs/tailwind";

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
  ],
});
