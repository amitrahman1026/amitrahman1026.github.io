use yew::prelude::*; 
use crate::components::markdown_component::MarkdownComponent as Markdown;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
}

#[function_component(Blogpost)]
pub fn blogpost(props: &Props) -> Html{
    let encoded_title = utf8_percent_encode(&props.title, NON_ALPHANUMERIC).to_string();
    let post_url = format!("/personal-site/blogposts/{}", encoded_title);

    html! {
        <div>
            <Markdown url={post_url} />
        </div>
    }
}


