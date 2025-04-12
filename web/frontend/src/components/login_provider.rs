//! HTML rendering of an OAuth login provider object.
use api_path::api::oauth::providers::LoginProviderCredentials;
use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct OAuthLoginProviderProps {
    pub provider: LoginProviderCredentials,
}

#[function_component(LoginProvider)]
/// HTML rendering of an OAuth login provider object.
pub fn login_provider(props: &OAuthLoginProviderProps) -> Html {
    // We need to build the URL for the OAuth provider.

    let root_url = props.provider.oauth_url.clone();

    let state = web_sys::window().unwrap_throw().location().href().unwrap_throw();

    // We need to construct a GET request with the following parameters:
    // - client_id: The client ID of the OAuth application.
    // - redirect_uri: The URL to redirect to after the user logs in.
    // - scope: The scope of the OAuth application.
    // - state: The url from which the user is coming from.

    let options = format!(
        "client_id={}&redirect_uri={}&scope={}&state={}",
        props.provider.client_id, props.provider.redirect_uri, props.provider.scope, state
    );

    let url = format!("{}?{}", root_url, options);

    html! {
        <a class="login-provider" href={url}>
            <i class={format!("fab fa-{}", props.provider.icon)}></i>
            {format!("Login with {}", props.provider.name)}
        </a>
    }
}
