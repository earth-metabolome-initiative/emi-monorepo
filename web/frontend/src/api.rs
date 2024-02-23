use gloo_net::http::Request;
use web_common::login_provider::OAuth2LoginProvider;

pub async fn retrieve_login_providers() -> Result<Vec<OAuth2LoginProvider>, gloo_net::Error> {
    Request::get("/api/oauth/providers")
        .send()
        .await?
        .json()
        .await
}

pub async fn retrieve_logged_user_info() -> Result<web_common::user::User, gloo_net::Error> {
    Request::get("/api/logged_user_info")
        .send()
        .await?
        .json()
        .await
}