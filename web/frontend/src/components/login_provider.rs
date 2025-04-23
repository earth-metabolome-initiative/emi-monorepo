//! HTML rendering of an OAuth login provider object.
use core_structures::LoginProvider as OAuthLoginProvider;
use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

#[function_component(LoginProvider)]
/// HTML rendering of an OAuth login provider object.
pub fn login_provider(provider: &OAuthLoginProvider) -> Html {
    // We need to build the URL for the OAuth provider.

    let root_url = provider.oauth_url.clone();

    let state = web_sys::window().unwrap_throw().location().href().unwrap_throw();

    let url = format!(
        "{root_url}?client_id={}&redirect_uri={}&scope={}&state={state}",
        provider.client_id, provider.redirect_uri, provider.scope,
    );

    html! {
        <a class="login-provider" href={url}>
            <i class={format!("fab fa-{}", provider.icon)}></i>
            {format!("Login with {}", provider.name)}
        </a>
    }
}
