---
title: "Building a Personal Blog Site in 2024: A Rusty Adventure"
description: "How I built my blog site using Rust and WebAssembly instead of the usual React/Next.js route"
date: "2024-01-31"
tags: ["rust", "webdev", "wasm", "blog"]
draft: false
---

Hey there! Let me tell you about how I built my blog site. It's 2024, and while
I could've gone the usual route (React, Next.js, you know the drill), I decided
to shake things up a bit. Enter Rust and WebAssembly.

### What Was I Looking For?

#### Writing:

Markdown seemed like the ideal choice. It's like the Swiss Army knife of text
formatting. Pasting images is a breeze. I don't really need anything else (for
now).

#### Deploying:

I wanted my blog to be like those static site generators (Jekyll, Hugo...).
Super simple HTML, no unnecessary frills, no excess JavaScript. Just the
essentials.

### Why Rust?

Well, honestly? I was curious. The idea of compiling to WebAssembly and having
near-native performance for my blog sounded pretty cool. Plus, Rust's type
system is fantastic - it catches so many potential issues at compile time.

### The Stack

Here's what I ended up using:
- **Rust** for the core logic
- **Yew** as the frontend framework 
- **pulldown-cmark** for markdown parsing
- **reqwest** for fetching content
- **Trunk** for building and bundling

### Building It

The architecture is pretty straightforward. I have markdown files in an assets
folder, and the Rust app fetches and renders them dynamically. The routing
handles different pages (about, blog posts, etc.), and everything compiles down
to WebAssembly.

One thing I really love about this setup is how maintainable it is. The type
safety means I catch issues early, and the performance is excellent.

### Deployment

GitHub Pages makes deployment super simple. I set up a GitHub Action that:
1. Builds the Rust project
2. Generates the static files
3. Deploys to GitHub Pages

### What's Next?

I'm thinking about adding:
- A proper theme system
- Better SEO optimization
- Maybe some interactive components

This has been a fun project, and I'm really happy with how it turned out. Rust
might not be the conventional choice for web development, but it's definitely
capable and enjoyable to work with.