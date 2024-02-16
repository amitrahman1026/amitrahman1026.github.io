use yew::prelude::*;
mod components;
use components::{
    about_me::AboutMe, bloglist::Bloglist, blogpost::Blogpost, contact::ContactMe,
    projects::Projects, resume::Resume,
};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_name_entry: Callback<String>,
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about_me")]
    AboutMe,
    #[at("/blog")]
    Blog,
    #[at("/blogposts/:title")]
    Blogpost { title: String },
    #[at("/resume")]
    Resume,
    #[at("/projects")]
    Projects,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <AboutMe />
        },
        Route::AboutMe => html! {
            <AboutMe />
        },
        Route::Blog => html! {
            <Bloglist />
        },
        Route::Blogpost { title } => html! {
            <Blogpost title={title} />
        },
        Route::Resume => html! {
            <Resume />
        },
        Route::Projects => html! {
            <Projects />
        },
        Route::Contact => html! {
            <ContactMe />
        },
        Route::NotFound => html! {
            <div class="reader-mode">
                <h1>{ "404" }</h1>
                <p>{"Page doesn't exist. How did you get here?"}</p>
            </div>
        },
    }
}

#[function_component]
fn App() -> Html {
    let theme = use_state(|| "light-theme".to_string());

    // Effect for initializing the theme on mount
    {
        let theme = theme.clone();
        use_effect_with_deps(
            move |_| {
                let window = web_sys::window().expect("no global `window` exists");
                let local_storage = window
                    .local_storage()
                    .expect("local storage not available")
                    .expect("cannot access local storage");
                let document = window.document().expect("should have a document on window");
                let body = document.body().expect("document should have a body");

                if let Ok(Some(stored_theme)) = local_storage.get_item("theme") {
                    theme.set(stored_theme.clone());
                    body.class_list()
                        .add_1(&stored_theme)
                        .expect("unable to add class");
                    // Set the highlight.js theme
                    let highlight_style = document
                        .get_element_by_id("highlight-style")
                        .expect("highlight-style element not found");
                    let highlight_theme = if stored_theme == "dark-theme" {
                        "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/night-owl.min.css"
                    } else {
                        "//cdn.jsdelivr.net/gh/highlightjs/cdn-release@10.3.2/build/styles/github.min.css"
                    };
                    highlight_style
                        .set_attribute("href", highlight_theme)
                        .expect("Failed to set highlight.js theme");
                } else {
                    // Apply default theme if none is stored
                    body.class_list()
                        .add_1("light-theme")
                        .expect("unable to add class");
                    // Set default highlight.js theme
                    let highlight_style = document
                        .get_element_by_id("highlight-style")
                        .expect("highlight-style element not found");
                    highlight_style
                    .set_attribute("href", "//cdn.jsdelivr.net/gh/highlightjs/cdn-release@10.3.2/build/styles/github.min.css")
                    .expect("Failed to set default highlight.js theme");
                }

                || ()
            },
            (),
        );
    }

    let toggle_theme = {
        let theme = theme.clone();
        Callback::from(move |_| {
            let new_theme = if *theme == "light-theme" {
                "dark-theme"
            } else {
                "light-theme"
            };
            theme.set(new_theme.to_string());

            // Set the theme in local storage
            let window = web_sys::window().expect("no global `window` exists");
            let local_storage = window
                .local_storage()
                .expect("local storage not available")
                .expect("cannot access local storage");
            local_storage
                .set_item("theme", new_theme)
                .expect("unable to set item in local storage");

            // Apply the theme class to the body
            let document = window.document().expect("should have a document on window");
            let body = document.body().expect("document should have a body");
            body.class_list()
                .remove_1("light-theme")
                .expect("unable to remove class");
            body.class_list()
                .remove_1("dark-theme")
                .expect("unable to remove class");
            body.class_list()
                .add_1(new_theme)
                .expect("unable to add class");
            // Set highlight.js theme
            let highlight_style = document
                .get_element_by_id("highlight-style")
                .expect("highlight-style element not found");
            let new_highlight_theme = if new_theme == "dark-theme" {
                "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/night-owl.min.css"
                // "//cdn.jsdelivr.net/gh/highlightjs/cdn-release@10.3.2/build/styles/monokai-sublime.min.css"
            } else {
                "//cdn.jsdelivr.net/gh/highlightjs/cdn-release@10.3.2/build/styles/github.min.css"
            };
            highlight_style
                .set_attribute("href", new_highlight_theme)
                .expect("Failed to set highlight.js theme");
        })
    };

    let is_dark_theme = (*theme) == "dark-theme";

    html! {
        <div class={format!("reader-mode {}", *theme)}>
            <div class="nav-container">
                <nav>
                    <a href="#/about_me">{"Amit Rahman"}</a>
                    <a href="#/blog">{"Blog"}</a>
                    <a href="#/resume">{"Resume"}</a>
                    <a href="#/projects">{"Projects"}</a>
                    <a href="#/contact">{"Contact"}</a>
                </nav>
                <div class="switch">
                    <input type="checkbox" id="theme-switch" checked={is_dark_theme} onclick={toggle_theme} />
                    <label for="theme-switch" class="slider"></label>
                </div>
            </div>
            <HashRouter basename="/personal-website">
                <Switch<Route> render={switch} />
            </HashRouter>
            <br/>
            <hr/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
