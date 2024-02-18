use yew::prelude::*;

#[function_component(ContactMe)]
pub fn contact() -> Html {
    html! {
        <div class="contact-container">
            <h1>{ "Contact Me" }</h1>
            <p>{ "Feel free to reach out to me via email or connect with me on GitHub or LinkedIn." }</p>
            <ul>
                <li>{ "Email: " }<a href="mailto:amit.amit.rahman@gmail.com">{"amit.amit.rahman@gmail.com"}</a></li>
                <li>{ "GitHub: " }<a href="https://www.github.com/amitrahman1026" target="_blank">{"github.com/amitrahman1026"}</a></li>
                <li>{ "LinkedIn: " }<a href="https://www.linkedin.com/in/amitrahman1026" target="_blank">{"linkedin.com/in/amitrahman1026"}</a></li>
                <li>{ "Phone: +1 (551) 998-3381" }</li>
            </ul>
        </div>
    }
}

