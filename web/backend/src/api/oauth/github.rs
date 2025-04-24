//! Login API for GitHub OAuth
use std::env;

use actix_web::{HttpResponse, get};
use core_structures::LoginProvider;
use redis::Client as RedisClient;
use reqwest::Client;
use serde::Deserialize;

use crate::{api::oauth::*, errors::BackendError};

/// Struct representing the GitHub OAuth2 configuration.
struct GitHubConfig {
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
    ///
    /// # Errors
    ///
    /// * If the environment variables are not set, an error is returned.
    /// * If the `LoginProvider` cannot be retrieved, an error is returned.
    async fn from_env(connection: &mut crate::Conn) -> Result<GitHubConfig, BackendError> {
        let provider = LoginProvider::from_name("GitHub", connection)
            .await?
            .ok_or_else(|| BackendError::UnknownLoginProvider("GitHub".to_string()))?;
        Ok(GitHubConfig { client_secret: env::var("GITHUB_CLIENT_SECRET")?, provider })
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
) -> HttpResponse {
    let code = &query.code;
    let state = &query.state;

    if code.is_empty() {
        return BackendError::Unauthorized.into();
    }

    let token_response = match get_github_oauth_token(code.as_str(), &pool).await {
        Ok(token_response) => token_response,
        Err(error) => {
            return error.into();
        }
    };

    // We retrieve the GitHub user emails
    let emails = match get_github_user_emails(token_response.access_token.as_str()).await {
        Ok(emails) => emails,
        Err(error) => {
            return error.into();
        }
    };

    let mut connection = match pool.get().await.map_err(BackendError::from) {
        Ok(connection) => connection,
        Err(error) => {
            return error.into();
        }
    };
    let github_config = match GitHubConfig::from_env(&mut connection).await {
        Ok(config) => config,
        Err(error) => {
            return error.into();
        }
    };

    todo!("Build the user from the emails and the GitHubConfig");

    // build_login_response(user.id, state, &redis_client).await
}

pub async fn get_github_oauth_token(
    authorization_code: &str,
    pool: &web::Data<crate::DBPool>,
) -> Result<GitHubOauthToken, BackendError> {
    let mut connection = pool.get().await?;
    let github_config = GitHubConfig::from_env(&mut connection).await?;

    let root_url = "https://github.com/login/oauth/access_token";

    let client = Client::new();

    let params = [
        ("client_id", github_config.provider.client_id.as_str()),
        ("code", authorization_code),
        ("client_secret", github_config.client_secret.as_str()),
    ];

    let response =
        client.post(root_url).header("Accept", "application/json").form(&params).send().await?;

    Ok(response.json::<GitHubOauthToken>().await?)
}

/// Returns the emails associated with the GitHub user.
async fn get_github_user_emails(
    authorization_code: &str,
) -> Result<Vec<GithubEmailMetadata>, BackendError> {
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
