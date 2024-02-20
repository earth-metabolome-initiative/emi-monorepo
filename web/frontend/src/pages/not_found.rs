//! Not found error page.

use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div>
            <h1>{"Not Found"}</h1>
        </div>
    }
}