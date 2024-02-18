# Building a Personal Blog Site in 2024: A Rusty Adventure 🦀

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

#### Maintaining:

I'm a bit of a control freak when it comes to preserving my work. Been
traumatised ever since my pc crashed on MS Word and I hadn't hit save in the
past 30ms. So, good version control was a must. And hey, if I ever fall victim
to shiny new technology syndrome™, I want to be able to port my content with me,
no strings attached.

### SPA

I went with a Single Page Application (SPA) for this site. It's 2024, and wants
to be caught lacking with slow-loading pages. The goal was to keep things as
light as possible and lazy load wherever I could.

### Fetch and Render Markdown

Here's where the magic happens. Wrote custom markdown component that fetches
markdown files from the void (my github repo). Then, using the pulldown-cmark
library, it transforms them into beautiful HTML right here accessing things via
the virtual DOM.

### Yew/Rust Frontend: A Rollercoaster

So, about Rust and Yew. Let's just say it was... an experience. Here's an honest
from someone who's not too familiar with Rust and can't web dev to save their
life. Should have really picked a struggle, but hey, I'm here now.

Here's the tldr:

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

## Exploring Yew and the Virtual DOM

When embarking on this journey to build a personal blog site, I dived deep into
the world of Single Page Applications (SPA) and stumbled upon Yew. Yew is a
modern Rust framework for creating multi-threaded front-end web apps using
WebAssembly. It stands out by leveraging Rust's performance, safety, and
WebAssembly's power to bring a new level of efficiency and reliability to web
development.

### Yew and the Virtual DOM

Yew uses a concept called the Virtual DOM, similar to React in the JavaScript
ecosystem. The Virtual DOM is an in-memory representation of the actual DOM. It
allows Yew to minimize direct DOM manipulation by batching changes and applying
them in the most efficient way possible. This approach reduces the amount of
computation and rendering required, leading to faster, more responsive
applications.

### Comparison with Other SPA Generators

- **Static Site Generators (e.g., Hugo, Jekyll):** These tools excel at
  generating static websites quickly and are incredibly efficient for blogs or
  documentation sites where content doesn't change dynamically. However, they
  lack the interactivity and responsiveness of a SPA built with Yew or
  JavaScript frameworks. Yew offers more flexibility and dynamism for
  applications requiring user interaction without sacrificing performance.

- **JavaScript Frameworks (e.g., React, Vue.js):** JavaScript has long been the
  king of the web, and frameworks like React and Vue.js have made building
  complex SPAs more accessible. These frameworks offer vast ecosystems, numerous
  libraries, and community support. However, they can suffer from slower
  performance due to JavaScript's dynamic nature. Yew, leveraging Rust and
  WebAssembly, provides a compelling alternative with its promise of near-native
  performance, type safety, and lower runtime errors.

## The nitty gritty of making this site look presentable

### Implementing Persistent Light and Dark Mode

One of the challenges I faced while building my blog was ensuring a seamless
user experience through persistent light and dark modes. Usually I'd use some
pre made component in react or something, but this time I actually had to hand
roll something and raw dog the css. After getting the css down I realised I took
persistent storage for granted because my site seemed to have amnesia everytime
i refreshed the page.

Here's how I approached state management by hand - The implementation revolves
around utilizing the browser's localStorage to store the user's theme preference
and applying this preference every time the page loads. This ensures the
selected theme persists across sessions.

```rust
// Initializing theme state with user's last preference
let theme = use_state(|| "Is it sunny or are we in dark mode?".to_string());

// When the page loads...
use_effect_with_deps(move |_| {
    // ...we check if the user has a preferred theme...
    let stored_theme = window.local_storage.get("theme");

    // ...and apply it, making sure our site remembers its manners.
    if stored_theme.is_some() {
        apply_theme(stored_theme.unwrap()); // Apply stored theme
    }
}, []);


// Flipping the switch between light and dark
let toggle_theme = Callback::new(move || {
    let new_theme = if theme is "light-theme" { "dark-theme" } else { "light-theme" };
    set_stored_theme(new_theme); // Update localStorage
    apply_theme(new_theme); // Apply new theme
});

// Function to get the initial theme, checking localStorage first
fn get_initial_theme() -> String {
    localStorage.get("theme").unwrap_or_else(|| "light-theme".to_string())
}

// The magic that applies the theme and ensures the site knows what it's wearing
fn apply_theme(theme: String) {
    document.body().class_list().set_theme_class(theme);
}

// Function to store the current theme in localStorage
fn set_stored_theme(theme: String) {
    localStorage.set("theme", theme);
}
```

As you can see, I had to dive deep into the Rusty depths of state management and
local storage to ensure that your retinas are spared when reading my blog at 3
AM.

### A Tale of Unintended Recursion

<div style="text-align: center;">
    <img src="/personal-website/images/1_ozymandias.png" style="width: 50%;" alt="Relevant xkcd">
</div>

In my quest to make this site as dynamic as possible for something born outta
markdown, I accidentally turned it into a hyperactive child that kept refreshing
itself over and over again. The culprit? A misplaced Callback that triggered a
fetch operation on every render, resulting in an unintentional infinite loop.

This bug stemmed from a misunderstanding of Yew's effect handling and the
importance of proper scoping in Rust. Let's delve into the problem and the
solution that resolved it.

Initially, my approach to fetching and displaying markdown content involved
creating a Callback within my component that would fetch the content whenever
the component was rendered. The code looked something like this:

```rust
#[function_component(MarkdownComponent)]
pub fn markdown_component(props: &UrlProps) -> Html {
    let inner = use_state(|| "Loading markdown file...".to_owned());
    let url = process_request(&props.url);
    let url_rc = Rc::new(url);

    let get_markdown = {
        let inner = inner.clone();
        let url_rc = url_rc.clone();
        Callback::from(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                // Attempt to fetch markdown content
            });
        })
    };

    html! {
        <div>
            <div>{ "Contents appear (many many times)!" }</div>
        </div>
    }
}
```

#### The Light: Use of use_effect_with_deps

The solution came in the form of the use_effect_with_deps hook, which allowed me
to fetch content only when the URL changes, thus breaking the cycle of endless
refreshes:

To solve this issue, I refactored the component to utilize Yew's
use_effect_with_deps hook, which allows for side effects to be run in response
to changes in specified dependencies. This method ensured that the content
fetching logic was only executed when the component's URL prop changed,
preventing the re-render loop:

```rust
#[function_component(MarkdownComponent)]
pub fn markdown_component(props: &UrlProps) -> Html {
    let content = use_state(|| "Loading markdown file...".to_owned());
    
    {
        let content = content.clone();
        let url = props.url.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                // Fetch markdown content and update state
            });
            || ()
        }, props.url.clone()); // Dependency array
    }

    html! {
        <div>
            <div>{ "Voilà! Contents appear (only once this time)!" }</div>
        </div>
    }
}
```

### Post processing for visual goodies

#### Lazy loading images

Cmark pulldown didn't really have options to make images lazy load and maybe one
day I'd like to fork the crate for my own needs but for now —

```rust
fn markdown_to_html(markdown: &str) -> String {
    // Parsing markdown to HTML
    let mut html_output = String::new();
    html::push_html(&mut html_output, pulldown_cmark::Parser::new_ext(markdown, options));
    
    // Adding lazy loading attribute to each <img> tag
    html_output.replace("<img", "<img loading=\"lazy\"");
}
```

#### Syntax Highilighting

So far you've been reading code blocks with nice syntax highlighting.. this
wasnt always the case. After digging through some defunct syntax crates and
weighing my options of writing my own implementation like the chad rust
programmer I am (literally only started a few weeks ago) the choice was obvious:
cheat using some sneaky javascript hehe This involved injecting a custom script
into the document to apply Highlight.js to all `<pre><code>` blocks after the
content was set:

```rust
async fn fetch_and_process_markdown(url: &str, inner: &UseStateHandle<String>) -> Result<(), reqwest::Error> {
    // Fetching and processing markdown to HTML
    let content = reqwest::get(url).await?.text().await?;
    inner.set(markdown_to_html(&content));
    
    // Injecting Highlight.js script for syntax highlighting
    if let Some(document) = web_sys::window().unwrap().document() {
        append_highlight_script(&document);
    }
}

fn append_highlight_script(document: &Document) {
    let script = document.create_element("script").expect("Failed to create script element");
    script.set_inner_html(
        "setTimeout(() => { document.querySelectorAll('pre code').forEach((block) => { hljs.highlightElement(block); }); }, 0);"
    );
    document.body().expect("No body element").append_child(&script).expect("Failed to append script element");
}
```

Now that there are more colours I have successfully cornered the market for the
ADHD demographic.

All in all frontend is way harder than I thought and somedays I wish I was a
React Andy.

### Parting Thoughts

Building this site was like a wild ride through the amusement park of web
development. I've got new scars (thanks, light/dark mode and raw css), a new
appreciation for minimalism, and a whole lot of respect for Rust.

If you're thinking of building your own blog or just messing around with Rust, I
say go for it. It's a challenge for sure and a great way to learn more about how
the DOM works, because not everything is abtracted behind
`import { ThemeProvider, createTheme } from '@mui/material/styles';`

Catch you in the next post, have a few ideas in mind to go over some post
mortems of my projects. Stay tuned! 🚀
