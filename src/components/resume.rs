use yew::prelude::*;

#[function_component(Resume)]
pub fn resume() -> Html{
    html! {
        <div>
            <h1>{ "Resume" }</h1>

            <embed src="/personal_website/resume/resume.html.pdf" width="100%" height="600px" 
 type="application/pdf" />

        </div>
    }
}



