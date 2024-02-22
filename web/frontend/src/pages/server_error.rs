//! Not found error page.

use crate::components::ErrorPage;
use yew::prelude::*;

#[function_component(ServerErrorPage)]
pub fn server_error_page() -> Html {
    html! {
        <ErrorPage error_code={500} error_message={"Oops! Something went wrong."} />
    }
}
