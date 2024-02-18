use yew::prelude::*; 
use crate::components::markdown_component::MarkdownComponent as Markdown;

#[function_component(AboutMe)]
pub fn about_me() -> Html{
    let url = "/personal-site/about_me.md";
    html! {
        <div>
           <Markdown url={url} /> 
        </div>
    }
}


