//! Login API for GitHub OAuth
use std::env;

use actix_web::{HttpResponse, Responder, get};
use backend_request_errors::BackendRequestError;
use core_structures::LoginProvider;
use redis::Client as RedisClient;
use reqwest::Client;
use serde::Deserialize;

use super::jwt_cookies::build_login_response;
use crate::{api::oauth::*, errors::BackendError};

/// Struct representing the GitHub OAuth2 configuration.
struct GitHubConfig {
    client_id: String,
    client_secret: String,
    provider: LoginProvider,
}

impl GitHubConfig {
    /// Function to retrieve the GitHub OAuth2 configuration from the
    /// environment.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `GitHubConfig` if the environment variables
    /// are set, or an error message if they are not.
    pub async fn from_env(connection: &mut crate::Conn) -> Result<GitHubConfig, BackendError> {
        Ok(GitHubConfig {
            client_id: env::var("GITHUB_CLIENT_ID")?,
            client_secret: env::var("GITHUB_CLIENT_SECRET")?,
            provider: LoginProvider::from_name("GitHub", connection).await?,
        })
    }
}

#[derive(Deserialize)]
pub struct GitHubOauthToken {
    pub access_token: String,
}

#[derive(Debug, Deserialize)]
struct GithubEmailMetadata {
    email: String,
    verified: bool,
    primary: bool,
    #[serde(rename = "visibility")]
    _visibility: Option<String>,
}

#[get("/github")]
async fn github_oauth_handler(
    query: web::Query<QueryCode>,
    pool: web::Data<crate::DBPool>,
    redis_client: web::Data<RedisClient>,
) -> impl Responder {
    let code = &query.code;
    let state = &query.state;

    if code.is_empty() {
        return HttpResponse::Unauthorized().json(Error::Unauthorized);
    }

    let Ok(token_response) = get_github_oauth_token(code.as_str(), &pool).await else {
        return HttpResponse::BadRequest().json(Error::DieselError);
    };

    // We retrieve the GitHub user emails
    let Ok(emails) = get_github_user_emails(token_response.access_token.as_str()).await else {
        return HttpResponse::BadRequest().json(Error::DieselError);
    };

    let Ok(mut connection) = pool.get().await else {
        return HttpResponse::InternalServerError().json(Error::DieselError);
    };
    let Ok(github_config) = GitHubConfig::from_env(&mut connection).await else {
        return HttpResponse::InternalServerError().json(Error::DieselError);
    };

    todo!("Build the user from the emails and the GitHubConfig");

    // build_login_response(user.id, state, &redis_client).await
}

pub async fn get_github_oauth_token(
    authorization_code: &str,
    pool: &web::Data<crate::DBPool>,
) -> Result<GitHubOauthToken, Error> {
    let mut connection = pool.get().await?;
    let github_config = GitHubConfig::from_env(&mut connection).await?;

    let root_url = "https://github.com/login/oauth/access_token";

    let client = Client::new();

    let params = [
        ("client_id", github_config.client_id.as_str()),
        ("code", authorization_code),
        ("client_secret", github_config.client_secret.as_str()),
    ];

    let response =
        client.post(root_url).header("Accept", "application/json").form(&params).send().await?;

    Ok(response.json::<GitHubOauthToken>().await?)
}

/// Returns the emails associated with the GitHub user.
pub async fn get_github_user_emails(
    authorization_code: &str,
) -> Result<Vec<GithubEmailMetadata>, Error> {
    let root_url = "https://api.github.com/user/emails";

    let client = Client::new();

    let response = client
        .get(root_url)
        .header("Accept", "application/json")
        .header("User-Agent", "EarthMetabolomeInitiativePortal")
        .bearer_auth(authorization_code)
        .send()
        .await?;

    Ok(response.json::<Vec<GithubEmailMetadata>>().await?)
}
