# Building a Personal Blog Site in 2024: A Rusty Adventure 🦀

Hey there! Let me tell you about how I built my blog site. It's 2024, and while
I could've gone the usual route (React, Next.js, you know the drill), I decided
to shake things up a bit. Enter Rust and WebAssembly. Yeah, that's right, I
built a blog with Rust. Why? Well, why not!

### What Was I Looking For?

#### Writing:

Markdown seemed like the ideal choice. It's like the Swiss Army knife of text
formatting. Pasting images is a breeze. I don't really need anything else (for
now).

#### Deploying:

I wanted my blog to be like those static site generators (Jekyll, Hugo...).
Super simple HTML, no unnecessary frills, no excess JavaScript. Just the
essentials.

#### Maintaining:

I'm a bit of a control freak when it comes to preserving my work. So, good
version control was a must. And hey, if I ever fall victim to shiny new
technology syndrome™, I want to be able to port my content with me, no strings
attached.

### SPA

I went with a Single Page Application (SPA) for this site. It's 2024, and
 wants to be caught lacking with slow-loading pages. The goal was to
keep things as light as possible and lazy load wherever I could.

### Fetch and Render Markdown

Here's where the magic happens. Wrote custom markdown component that fetches
markdown files from the void (my github repo). Then, using the pulldown-cmark
library, it transforms them into beautiful HTML right here accessing things via
the virtual DOM.

### Yew/Rust Frontend: A Rollercoaster

So, about Rust and Yew. Let's just say it was... an experience. Here's an honest
from someone who's not too familiar with Rust and can't web dev to save their
life. Should have really picked a struggle, but hey, I'm here now.

- **Productivity** Not quite on par with React and TypeScript, but had quite a
  familiar feel to it.
- **Integration abilities** Can't wait to mess around with physics simulations
  and graphics stuff in Wasm.
- **Weightclass** Feather-weight! Minimal dependencies are nice to look at. More
  importantly, stable dependencies are a plus.
- **Learning curve?** Let's just say my frontend skills got a reality check.
  Struggle bussed making a light/dark mode that actually remembers what you
  picked AND applies the themes everywhere.
- **Markdown to HTML?** Less difficult than I expected, thanks to the
  pulldown-cmark crate.

### Parting Thoughts

Building this site was like a wild ride through the amusement park of web
development. I've got new scars (thanks, light/dark mode and raw css), a 
new appreciation for minimalism, and a whole lot of respect for Rust.

If you're thinking of building your own blog or just messing around with Rust, I
say go for it. It's a challenge, but hey, what's life without a few good
challenges?

Catch you in the next post, have a few ideas in mind to go over some post
mortems of my projects. Stay tuned! 🚀
