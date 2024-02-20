//! Not found error page.

use yew::prelude::*;
use crate::components::ErrorPage;

#[function_component(ServerErrorPage)]
pub fn server_error_page() -> Html {
    html! {
        <ErrorPage error_code={500} error_message={"Oops! Something went wrong."} />
    }
}