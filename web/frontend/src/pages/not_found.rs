//! Not found error page.

use yew::prelude::*;

use crate::components::ErrorPage;

pub struct NotFound {}

impl Component for NotFound {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        NotFound {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <ErrorPage error_code={404} error_message={"Oops! The page you are looking for does not exist."} />
        }
    }
}
