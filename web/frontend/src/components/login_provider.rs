//! HTML rendering of an OAuth login provider object.
use core_structures::LoginProvider as OAuthLoginProvider;
use yew::prelude::*;

use super::FAIcon;
use crate::{traits::AssignedComponent, utils::AssignedConnectorProps};

#[function_component(LoginProvider)]
/// Component for rendering a login provider.
pub fn login_provider(provider: &AssignedConnectorProps<OAuthLoginProvider>) -> Html {
    let root_url = provider.row.oauth_url.clone();

    let url = format!(
        "{root_url}?client_id={}&redirect_uri={}&scope={}&state={}",
        provider.row.client_id,
        provider.row.redirect_uri,
        provider.row.scope,
        option_env!("DOMAIN").unwrap_or_default(),
    );

    html! {
        <a class="login-provider" href={url}>
            <FAIcon icon={provider.row.icon.clone()} />
            {format!("Login with {}", provider.row.name)}
        </a>
    }
}

impl AssignedComponent for LoginProvider {
    type Row = OAuthLoginProvider;
}
