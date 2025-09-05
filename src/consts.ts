import type { Metadata, Site } from "@types";

export const SITE: Site = {
  TITLE: "Amit Rahman",
  DESCRIPTION: "Personal website and blog of Amit Rahman",
  EMAIL: "contact@amitrahman.me",
  NUM_POSTS_ON_HOMEPAGE: 5,
  NUM_PUBLICATIONS_ON_HOMEPAGE: 3,
  SITEURL: 'https://amitrahman.me'
};

export const HIGHLIGHTAUTHOR = "Amit Rahman"

export const HOME: Metadata = {
  TITLE: "Home",
  DESCRIPTION: "Astro Micro is an accessible theme for Astro.",
};

export const BLOG: Metadata = {
  TITLE: "Blog",
  DESCRIPTION: "A collection of articles on topics I am passionate about.",
};

export const RESEARCH: Metadata = {
  TITLE: "Publications",
  DESCRIPTION:
    "A collection of my publications with links to paper, repositories and live demos.",
};

export const CV: Metadata = {
  TITLE: "CV",
  DESCRIPTION:
    "your cv",
};

export const TAGS: Metadata = {
  TITLE: "TAGS",
  DESCRIPTION:
    "blog tag filter",
};

export const ABOUT: Metadata = {
  TITLE: "ABOUT",
  DESCRIPTION:
    "A self-intro",
};