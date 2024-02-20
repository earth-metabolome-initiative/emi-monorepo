mod api;
mod components;
mod pages;
mod router;
mod store;

use crate::router::{switch, AppRoute};
use components::Navigator;
use web_common::user::User;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
fn App() -> Html {
    let user_state: UseStateHandle<Option<User>> = use_state(|| None);

    html! {
        <ContextProvider<Option<User>> context={(*user_state).clone()}>
            <Navigator />
            <div class="app">
                <BrowserRouter>
                    <Switch<AppRoute> render={switch} />
                </BrowserRouter>
            </div>
        </ContextProvider<Option<User>>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
