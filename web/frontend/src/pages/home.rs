//! Home page of the application.

use yew::prelude::*;

pub struct Home {}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Home {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="home">
                <h1>{ "Welcome to the Home Page!" }</h1>
                <p>{ "This is the home page of the application." }</p>
            </div>
        }
    }
}
