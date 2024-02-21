//! Login API for GitHub OAuth
use crate::api::oauth::*;

use actix_web::{get, http::header::LOCATION, web, HttpResponse, Responder};

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::model_implementations::*;
use crate::models::*;
use reqwest::Client;
use serde::Deserialize;
use std::env;
use std::error::Error;

/// Struct representing the GitHub OAuth2 configuration.
struct GitHubConfig {
    client_id: String,
    client_secret: String,
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

        if client_id.is_err() {
            return Err("GITHUB_CLIENT_ID not set".to_string());
        }

        if client_secret.is_err() {
            return Err("GITHUB_CLIENT_SECRET not set".to_string());
        }

        // We retrieve the ID for the 'GitHub' provider from the database.
        let provider_id = LoginProvider::get_provider_id("github", pool).unwrap();

        Ok(GitHubConfig {
            client_id: client_id.unwrap(),
            client_secret: client_secret.unwrap(),
            oauth_config: OauthConfig::from_env().unwrap(),
            provider_id,
        })
    }

    fn client_origin(&self) -> &str {
        self.oauth_config.client_origin.as_str()
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

#[get("/oauth/github")]
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

    // We query the database to see if there is already an user with
    // the provided email. If so, we will not have to create a new user,
    // and we need to check whether the provide ID
    // used for registering the email is the same as the one used for the
    // current login provider. If the login provider is different, it means
    // that the user is trying to login with a different provider, and we
    // need to insert a new user email with the new provider ID.
    // Conversely, if the user does not exist, we create a new user and
    // insert a new user email.
    let mail_row = UserEmail::get_row_from_email(github_user.email.as_str(), &pool);

    let user_id = match mail_row {
        Ok(mail_row) => {
            // If the email is already present in the database, we check
            // whether it was registered with the same provider.
            // If not, we insert a new user email with the new provider ID.
            if mail_row.login_provider_id != github_config.provider_id {
                let new_user_email = NewUserEmail::new(
                    github_user.email.as_str(),
                    mail_row.user_id,
                    github_config.provider_id,
                );

                if new_user_email.is_err() {
                    let message = new_user_email.err().unwrap().to_string();
                    return HttpResponse::BadGateway()
                        .json(serde_json::json!({"status": "fail", "message": message}));
                }

                let query_result = new_user_email.unwrap().insert(&pool);

                if query_result.is_err() {
                    let message = query_result.err().unwrap().to_string();
                    return HttpResponse::BadGateway()
                        .json(serde_json::json!({"status": "fail", "message": message}));
                }
            }

            mail_row.user_id
        }
        Err(_) => {
            // If we cannot find the user email, we create a new user.
            let new_user_query = NewUser::insert_default(&pool);

            if new_user_query.is_err() {
                let message = new_user_query.err().unwrap().to_string();
                return HttpResponse::BadGateway()
                    .json(serde_json::json!({"status": "fail", "message": message}));
            }

            let new_user_id = new_user_query.unwrap();

            let new_user_email = NewUserEmail::new(
                github_user.email.as_str(),
                new_user_id,
                github_config.provider_id,
            );

            if new_user_email.is_err() {
                let message = new_user_email.err().unwrap().to_string();
                return HttpResponse::BadGateway()
                    .json(serde_json::json!({"status": "fail", "message": message}));
            }

            let query_result = new_user_email.unwrap().insert(&pool);

            if query_result.is_err() {
                let message = query_result.err().unwrap().to_string();
                return HttpResponse::BadGateway()
                    .json(serde_json::json!({"status": "fail", "message": message}));
            }

            new_user_id
        }
    };

    let cookie = create_jwt_cookie(user_id, &github_config.oauth_config);

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
