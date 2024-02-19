mod api;
mod components;
mod store;

use components::{Navigator, Sidebar};
use store::Store;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
fn App() -> Html {
    let (store, _) = use_store::<Store>();

    html! {
        <div class="app">
            <Navigator />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}