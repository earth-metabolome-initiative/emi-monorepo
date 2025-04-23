//! Login page of the application.

use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::login_provider::LoginProvider, router::AppRoute, stores::app_state::AppState,
};

#[function_component(Login)]
pub fn login() -> Html {
    let navigator = use_navigator().unwrap();
    let (state, _) = use_store::<AppState>();

    if state.user().is_some() {
        navigator.push(&AppRoute::Home);
    }

    let login_providers = use_state(|| Vec::<LoginProviderCredentials>::new());

    // {
    //     let login_providers = login_providers.clone();
    //     use_effect_with((), move |_| {
    //         wasm_bindgen_futures::spawn_local(async move {
    //             let providers = retrieve_login_providers().await.unwrap();
    //             login_providers.set(providers);
    //         });
    //     });
    // }

    html! {
        <div class="fullscreen_center_app">
            <div class="login_box">
                <h2>{"Login"}</h2>
                <ul class="login_providers">
                    // { for login_providers.iter().map(|provider| html! {
                    //     <li><LoginProvider provider={provider.clone()} /></li>
                    // })}
                </ul>
            </div>
        </div>
    }
}
