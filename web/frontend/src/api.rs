use gloo_net::http::Request;
use web_common::login_provider::OAuth2LoginProvider;

pub async fn retrieve_login_providers() -> Result<Vec<OAuth2LoginProvider>, gloo_net::Error> {
    Request::get("/api/oauth/providers")
        .send()
        .await?
        .json()
        .await
}
