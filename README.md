# amitrahman.me

My personal website built with Astro. Hosts my blog, resume, and contact info.

## Development

```bash
npm install
npm run dev     # localhost:1234
npm run build
npm run preview
```

## Tech Stack

- **Framework**: Astro 4.11
- **Styling**: Tailwind CSS + Inconsolata font
- **Content**: Markdown with MDX support
- **Search**: Pagefind
- **Hosting**: GitHub Pages

## Structure

```
src/
├── pages/           # Routes (index, blog, contact, etc.)
├── content/blog/    # Blog posts in markdown
├── components/      # Astro components
└── styles/          # Global CSS

public/              # Static assets
└── CNAME           # Custom domain config
```

## Deployment

Pushes to `main` trigger GitHub Actions deployment to `amitrahman.me`.

## Writing

New blog posts go in `src/content/blog/` with frontmatter:

```markdown
---
title: "Post Title"
description: "Brief description"
date: "2024-01-01"
tags: ["rust", "webdev"]
---

Content here...
```

Tags are auto-generated into filterable pages.