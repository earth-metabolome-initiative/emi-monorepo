//! Login API for GitHub OAuth
use std::env;

use actix_web::{get, HttpResponse, Responder};
use core_structures::LoginProvider;
use redis::Client as RedisClient;
use reqwest::Client;
use serde::Deserialize;
use web_common::api::ApiError;

use super::jwt_cookies::build_login_response;
use crate::api::oauth::*;

/// Struct representing the GitHub OAuth2 configuration.
struct GitHubConfig {
    client_id: String,
    client_secret: String,
    provider_id: i16,
}

impl GitHubConfig {
    /// Function to retrieve the GitHub OAuth2 configuration from the
    /// environment.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `GitHubConfig` if the environment variables
    /// are set, or an error message if they are not.
    pub async fn from_env(
        connection: &mut web_common_traits::prelude::DBConn,
    ) -> Result<GitHubConfig, ApiError> {
        let Ok(client_secret) = env::var("GITHUB_CLIENT_SECRET") else {
            panic!("GITHUB_CLIENT_SECRET not set");
        };

        let Ok(client_id) = env::var("GITHUB_CLIENT_ID") else {
            panic!("GITHUB_CLIENT_ID not set");
        };

        // We retrieve the ID for the 'GitHub' provider from the database.
        let Some(provider) = LoginProvider::from_name("GitHub", connection).await? else {
            panic!("GitHub provider not found in the database");
        };

        Ok(GitHubConfig { client_id, client_secret, provider_id: provider.id })
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
    pool: web::Data<web_common_traits::prelude::DBPool>,
    redis_client: web::Data<RedisClient>,
) -> impl Responder {
    let code = &query.code;
    let state = &query.state;

    if code.is_empty() {
        return HttpResponse::Unauthorized().json(ApiError::unauthorized());
    }

    let Ok(token_response) = get_github_oauth_token(code.as_str(), &pool).await else {
        return HttpResponse::BadRequest().json(ApiError::internal_server_error());
    };

    // We retrieve the GitHub user emails
    let Ok(emails) = get_github_user_emails(token_response.access_token.as_str()).await else {
        return HttpResponse::BadRequest().json(ApiError::internal_server_error());
    };

    let Ok(mut connection) = pool.get().await else {
        return HttpResponse::InternalServerError().json(ApiError::internal_server_error());
    };
    let Ok(github_config) = GitHubConfig::from_env(&mut connection).await else {
        return HttpResponse::InternalServerError().json(ApiError::internal_server_error());
    };

    todo!("Build the user from the emails and the GitHubConfig");

    // build_login_response(user.id, state, &redis_client).await
}

pub async fn get_github_oauth_token(
    authorization_code: &str,
    pool: &web::Data<web_common_traits::prelude::DBPool>,
) -> Result<GitHubOauthToken, ApiError> {
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
) -> Result<Vec<GithubEmailMetadata>, ApiError> {
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
