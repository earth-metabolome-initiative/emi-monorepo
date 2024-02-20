//! Login API for GitHub OAuth
use crate::api::oauth::*;

use actix_web::{get, http::header::LOCATION, web, HttpResponse, Responder};

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::models::*;
use reqwest::Client;
use serde::Deserialize;
use std::env;
use std::error::Error;

/// Struct representing the GitHub OAuth2 configuration.
struct GitHubConfig {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    oauth_config: OauthConfig,
    provider_id: i16,
}

impl GitHubConfig {
    /// Function to retrieve the GitHub OAuth2 configuration from the environment.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `GitHubConfig` if the environment variables are set, or an error
    /// message if they are not.
    pub fn from_env(pool: &Pool<ConnectionManager<PgConnection>>) -> Result<GitHubConfig, String> {
        dotenvy::dotenv().ok();
        let client_id = env::var("GITHUB_CLIENT_ID");
        let client_secret = env::var("GITHUB_CLIENT_SECRET");
        let redirect_uri = env::var("GITHUB_REDIRECT_URI");

        if client_id.is_err() {
            return Err("GITHUB_CLIENT_ID not set".to_string());
        }

        if client_secret.is_err() {
            return Err("GITHUB_CLIENT_SECRET not set".to_string());
        }

        if redirect_uri.is_err() {
            return Err("GITHUB_REDIRECT_URI not set".to_string());
        }

        // We retrieve the ID for the 'GitHub' provider from the database.
        let provider_id = LoginProvider::get_provider_id("github", pool).unwrap();

        Ok(GitHubConfig {
            client_id: client_id.unwrap(),
            client_secret: client_secret.unwrap(),
            redirect_uri: redirect_uri.unwrap(),
            oauth_config: OauthConfig::from_env().unwrap(),
            provider_id,
        })
    }

    fn client_origin(&self) -> &str {
        self.oauth_config.client_origin.as_str()
    }

    fn jwt_secret(&self) -> &str {
        self.oauth_config.jwt_secret.as_str()
    }

    fn jwt_max_age(&self) -> i64 {
        self.oauth_config.jwt_max_age
    }
}

#[derive(Deserialize)]
pub struct GitHubOauthToken {
    pub access_token: String,
}

#[derive(Deserialize)]
pub struct GitHubUserMetadata {
    pub login: String,
    pub avatar_url: String,
    pub email: String,
}

#[get("/sessions/oauth/github")]
async fn github_oauth_handler(
    query: web::Query<QueryCode>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let code = &query.code;
    let state = &query.state;

    if code.is_empty() {
        return HttpResponse::Unauthorized().json(
            serde_json::json!({"status": "fail", "message": "Authorization code not provided!"}),
        );
    }

    let token_response = get_github_oauth_token(code.as_str(), &pool).await;
    if token_response.is_err() {
        let message = token_response.err().unwrap().to_string();
        return HttpResponse::BadGateway()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let token_response = token_response.unwrap();
    println!("Bearer {}", token_response.access_token);
    let github_user = get_github_user(&token_response.access_token).await;
    if github_user.is_err() {
        let message = github_user.err().unwrap().to_string();
        return HttpResponse::BadGateway()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let github_user = github_user.unwrap();

    let github_config = GitHubConfig::from_env(&pool).unwrap();

    // We query the database to see if the user already exists.
    let maybe_user_id =
        UserEmail::get_user_id(github_user.email.as_str(), github_config.provider_id, &pool);

    let cookie = if let Ok(user_id) = maybe_user_id {
        create_jwt_cookie(user_id, &github_config.oauth_config)
    } else {
        // If the user does not exist, we create a new user.
        let new_user = NewUser {
            name: github_user.login.as_str(),
            email: github_user.email.as_str(),
            avatar: github_user.avatar_url.as_str(),
        };

        let user_id = User::create_user(new_user, &pool).unwrap();
        let user_email = NewUserEmail {
            user_id,
            email: github_user.email.as_str(),
            provider_id: github_config.provider_id,
        };

        UserEmail::create_user_email(user_email, &pool).unwrap();

        create_jwt_cookie(user_id, &github_config.oauth_config)
    };

    let mut response = HttpResponse::Found();
    response.append_header((
        LOCATION,
        format!("{}{}", github_config.client_origin(), state),
    ));
    response.cookie(cookie);
    response.finish()
}

pub async fn get_github_oauth_token(
    authorization_code: &str,
    pool: &web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<GitHubOauthToken, Box<dyn Error>> {
    let github_config = GitHubConfig::from_env(pool)?;

    let root_url = "https://github.com/login/oauth/access_token";

    let client = Client::new();

    let params = [
        ("client_id", github_config.client_id.as_str()),
        ("code", authorization_code),
        ("client_secret", github_config.client_secret.as_str()),
    ];

    let response = client
        .post(root_url)
        .header("Accept", "application/json")
        .form(&params)
        .send()
        .await?;

    if response.status().is_success() {
        let oauth_response = response.json::<GitHubOauthToken>().await?;
        Ok(oauth_response)
    } else {
        let message = "An error occurred while trying to retrieve the access token.";
        Err(From::from(message))
    }
}

pub async fn get_github_user(access_token: &str) -> Result<GitHubUserMetadata, Box<dyn Error>> {
    let root_url = "https://api.github.com/user";

    let client = Client::new();

    let response = client
        .get(root_url)
        .bearer_auth(access_token)
        .send()
        .await?;

    if response.status().is_success() {
        let user_info = response.json::<GitHubUserMetadata>().await?;
        Ok(user_info)
    } else {
        let message = "An error occurred while trying to retrieve user information.";
        Err(From::from(message))
    }
}
