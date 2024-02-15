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
        <div class="flex h-screen bg-gray-100">
            <Sidebar />
            <div class="flex-1">
                <Navigator />
                <div class="p-4">
                    <h1 class="text-2xl font-bold">{"Dashboard"}</h1>
                    <p class="mt-4">{"Welcome to the dashboard"}</p>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}