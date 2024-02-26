use gloo_net::http::Request;
use web_common::api::oauth::providers::*;

pub async fn retrieve_login_providers() -> Result<Vec<OAuth2LoginProvider>, gloo_net::Error> {
    Request::get(FULL_ENDPOINT).send().await?.json().await
}
