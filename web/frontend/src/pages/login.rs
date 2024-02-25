//! Login page of the application.

use crate::api::retrieve_login_providers;
use crate::components::login_provider::LoginProvider;
use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    let login_providers = use_state(|| Vec::new());

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
