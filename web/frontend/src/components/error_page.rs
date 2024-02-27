//! Not found error page.

use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::AppRoute;

#[derive(Clone, Properties, PartialEq)]
pub struct ErrorPageProps {
    pub error_code: usize,
    pub error_message: String,
}

#[function_component(ErrorPage)]
pub fn error_page(props: &ErrorPageProps) -> Html {
    let go_back_home = {
        let navigator = use_navigator().unwrap();
        Callback::from(move |_| {
            navigator.push(&AppRoute::Home);
        })
    };

    html! {
        <div class="fullscreen_center_app">
            <div class="error-wrapper">
                <div class="error-code">
                    {props.error_code}
                </div>
                <div class="error-message">
                    {props.error_message.clone()}
                </div>
                <div class="back-to-home">
                    <button onclick={go_back_home}>
                        {"Back to home"}
                    </button>
                </div>
            </div>
        </div>
    }
}
