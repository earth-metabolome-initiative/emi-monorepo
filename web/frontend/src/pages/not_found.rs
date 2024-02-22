//! Not found error page.

use crate::components::ErrorPage;
use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <ErrorPage error_code={404} error_message={"Oops! The page you are looking for does not exist."} />
    }
}
