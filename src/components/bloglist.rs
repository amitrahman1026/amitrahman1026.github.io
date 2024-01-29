use yew::prelude::*; 
use crate::components::markdown_component::MarkdownComponent as Markdown;
	
#[function_component(Bloglist)]
pub fn bloglist() -> Html{
    let url = "/personal-website/bloglist.md";
    html! {
        <div>
           <Markdown url={url} /> 
        </div>
    }
}


