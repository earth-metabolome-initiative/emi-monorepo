//! Not found error page.

use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct ErrorPageProps {
    pub error_code: usize,
    pub error_message: String,
}

#[function_component(ErrorPage)]
pub fn error_page(
    props: &ErrorPageProps,
) -> Html {
    html! {
        <div class="not-found-container">
            <div class="error-wrapper">
                <div class="error-code">
                    {props.error_code}
                </div>
                <div class="error-message">
                    {props.error_message.clone()}
                </div>
                <div class="back-to-home"><a href="/">{"Back to Home"}</a></div>
            </div>
        </div>
    }
}