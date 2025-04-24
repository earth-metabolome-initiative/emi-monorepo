//! Login page of the application.

use core_structures::LoginProvider as OAuthLoginProvider;
use web_common_traits::crud::Read;
use yew::prelude::*;
use yew_agent::worker::use_worker_bridge;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::login_provider::LoginProvider, router::AppRoute, stores::app_state::AppState,
    workers::DBWSWorker,
};

#[function_component(Login)]
pub fn login() -> HtmlResult {
    let navigator = use_navigator().unwrap();
    let (state, _) = use_store::<AppState>();

    if state.user().is_some() {
        navigator.push(&AppRoute::Home);
    }

    let login_providers = use_state(Vec::new);

    {
        let login_providers = login_providers.clone();
        let db = use_worker_bridge::<DBWSWorker, _>(move |db2c_message| {
            login_providers.set(db2c_message.into());
        });
        use_effect_with((), move |()| {
            db.send(OAuthLoginProvider::read_all(None, None).into());
        });
    }

    Ok(html! {
        <div class="fullscreen_center_app">
            <div class="login_box">
                <h2>{"Login"}</h2>
                <ul class="login_providers">
                    { for login_providers.iter().cloned().map(|provider| html! {
                        <li><LoginProvider ..provider/></li>
                    })}
                </ul>
            </div>
        </div>
    })
}
