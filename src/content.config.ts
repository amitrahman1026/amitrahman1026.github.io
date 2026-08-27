import { defineCollection, z } from "astro:content";
import { glob } from "astro/loaders";

const blog = defineCollection({
  loader: glob({ pattern: "**/*.md", base: "./src/content/blog" }),
  schema: z.object({
    title: z.string(),
    description: z.string(),
    date: z.coerce.date(),
    draft: z.boolean().optional(),

    tags: z.array(z.string()).optional(),
  }),
});

const publications = defineCollection({
  loader: glob({ pattern: "**/*.md", base: "./src/content/publications" }),
  schema: z.object({
    title: z.string(),
    description: z.string(),
    date: z.coerce.date(),
    paperURL: z.string().optional(),
    authors: z.string().optional(),
    codeURL: z.string().optional(),
    webURL: z.string().optional(),
    dataURL: z.string().optional(),
    img: z.string().optional(),
    imgAlt: z.string().optional(),
    pub: z.string().optional(),
  }),
});

export const collections = { blog, publications };
