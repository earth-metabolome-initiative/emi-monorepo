mod api;
mod components;
mod pages;
mod router;
mod store;

use crate::router::{switch, AppRoute};
use crate::components::*;
use web_common::user::User;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
fn App() -> Html {
    let user_state: UseStateHandle<Option<User>> = use_state(|| None);

    html! {
        <ContextProvider<Option<User>> context={(*user_state).clone()}>
            <BrowserRouter>
                <components::Navigator />
                <div class="app">
                    <Switch<AppRoute> render={switch} />
                </div>
            </BrowserRouter>
            <Footer />
        </ContextProvider<Option<User>>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
