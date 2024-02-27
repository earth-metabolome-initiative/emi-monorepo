//! Login page of the application.

use crate::api::oauth::providers::retrieve_login_providers;
use crate::components::login_provider::LoginProvider;
use crate::router::AppRoute;
use crate::stores::user_state::UserState;
use web_common::api::oauth::providers::OAuth2LoginProvider;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    let navigator = use_navigator().unwrap();
    let (user, _) = use_store::<UserState>();

    if user.is_logged_in() {
        navigator.push(&AppRoute::Home);
    }

    let login_providers = use_state(|| Vec::<OAuth2LoginProvider>::new());

    {
        let login_providers = login_providers.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let providers = retrieve_login_providers().await.unwrap();
                login_providers.set(providers);
            });
        });
    }

    html! {
        <div class="login_app">
            <div class="login_box">
                <h2>{"Login"}</h2>
                <ul class="login_providers">
                    { for login_providers.iter().map(|provider| html! {
                        <li><LoginProvider provider={provider.clone()} /></li>
                    })}
                </ul>
            </div>
        </div>
    }
}
