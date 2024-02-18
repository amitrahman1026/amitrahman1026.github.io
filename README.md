# Personal Website

Welcome to the development guide for my personal website, hosted on GitHub
Pages. This website is crafted with Rust, leveraging the Yew framework for a
dynamic front-end experience, and utilizes markdown for content management.

## Technology Stack

- **Rust:** Core programming language.
- **Yew:** Front-end framework for Rust.
- **Reqwest:** Asynchronous HTTP client for fetching markdown content.
- **pulldown-cmark:** Markdown parsing and rendering.
- **Bash Scripts:** Automation for markdown file creation and management.

## Key Features

- **Markdown Content Fetching:** Custom component to asynchronously fetch and
  render markdown content as HTML.
- **Blog Post Automation:** Bash scripts to simplify creating and managing
  markdown files for blog posts.

## Getting Started

To clone and run this project locally for development or personal use, follow
these steps:

### Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/amitrahman1026/personal-site.git
   ```
1. Navigate to the project directory:
   ```bash
   cd personal-site
   ```

## Development

### Install Dependencies:

1. Ensure Rust and wasm32-unknown-unknown target are installed. Install Trunk
   using Rust's package manager:

   ```bash
   cargo install trunk
   rustup target add wasm32-unknown-unknown

   ```

1. Run the Website Locally: Use the Makefile for convenient commands:

   ```bash
   make run
   ```

This serves the site at http://localhost:8080/personal-site.

### Create New Blog Posts

To create a new blog post, use the provided bash script for a streamlined
process:

```bash
make new-post
```

Follow the prompt to input your blog post title, and a template markdown file
will be created for you.

## Deployment

Deploy your site to GitHub Pages or your preferred hosting solution. A GitHub
Actions CI/CD pipeline can automate this process, or use the provided Makefile
script:

```bash
make deploy
```

### Contributing

Contributions are welcome! Feel free to submit pull requests or open issues for
bugs, features, or improvements. Check out the issues tab on GitHub.
