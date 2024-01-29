use yew::prelude::*;
mod components;
use components::{about_me::AboutMe};
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
    let theme = use_state(|| "light-theme".to_owned());

    let toggle_theme = {
        let theme = theme.clone();
        Callback::from(move |_| {
            let new_theme = if *theme == "light-theme" {
                "dark-theme"
            } else {
                "light-theme"
            };
            theme.set(new_theme.to_owned());

            // Use web_sys to access the document and body.
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");
            let body = document.body().expect("document should have a body");

            // Toggle the `dark-theme` class on the body.
            if new_theme == "dark-theme" {
                body.class_list().add_1("dark-theme").expect("unable to add class");
            } else {
                body.class_list().remove_1("dark-theme").expect("unable to remove class");
            }
        })
    };

    html! {
        <div class={format!("reader-mode {}", (*theme).clone())}>
            <div class="nav-container">
                <nav>
                    <a href="#/about_me">{"Amit Rahman"}</a>
                    <a href="#/resume">{"Resume"}</a>
                    <a href="#/projects">{"Projects"}</a>
                    <a href="#/contact">{"Contact"}</a>
                </nav>
                <div class="switch">
                    <input type="checkbox" id="theme-switch" onclick={toggle_theme} />
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
