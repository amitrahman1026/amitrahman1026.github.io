use yew::prelude::*; 
use crate::components::markdown_component::MarkdownComponent as Markdown;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
}

#[function_component(Blogpost)]
pub fn blogpost(props: &Props) -> Html{
    let post_title = &props.title.clone();
    let post_title = format!("/personal-website/blogposts/{}",post_title);
    html! {
        <div>
            <Markdown url={post_title} />
        </div>
    }
}


