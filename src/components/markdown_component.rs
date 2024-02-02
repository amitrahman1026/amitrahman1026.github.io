use yew::prelude::*;
use web_sys::window;
use pulldown_cmark::{html, Options, Parser};
use std::rc::Rc;

#[derive(Properties, PartialEq)]
struct CallbackHelperProps {
    pub helper: Callback<String>,
}

#[derive(Properties, PartialEq)]
pub struct UrlProps {
    pub url: String,
}

#[function_component(CallbackHelper)]
fn callback_helper(props: &CallbackHelperProps) -> Html {
    props.helper.emit(String::from(""));
    html! { <></> }
}

#[function_component(MarkdownComponent)]
pub fn markdown_component(props: &UrlProps) -> Html {
    let inner = use_state(|| "Loading markdown file...".to_owned());
    let url = process_request(&props.url);
    let url_rc = Rc::new(url);

    let get_markdown = {
        let inner = inner.clone();
        let url_rc = url_rc.clone();
        Callback::from(move |_| {
            let url_rc_clone = url_rc.clone();
            let inner_clone = inner.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Err(e) = fetch_and_process_markdown(&url_rc_clone, &inner_clone).await {
                    println!("Error fetching markdown: {}", e);
                }
            });
        })
    };

    let test = Html::from_html_unchecked((*inner).clone().into());
    html! {
        <div>
            <CallbackHelper helper={get_markdown} />
            {test}
        </div>
    }
}

// Function to fetch and process markdown
async fn fetch_and_process_markdown(url: &str, inner: &UseStateHandle<String>) -> Result<(), reqwest::Error> {
    let content = reqwest::get(url).await?.text().await?;
    let html_content = markdown_to_html(&content);
    inner.set(html_content);
    Ok(())
}

fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(markdown, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    // add lazy loading to images
    html_output = html_output.replace("<img", "<img loading=\"lazy\" ");
    html_output
}

fn process_request(filename: &str) -> String {
    if let Some(window) = window() {
        if let Ok(root_url) = window.location().origin() {
            format!("{}{}", root_url, filename)
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}