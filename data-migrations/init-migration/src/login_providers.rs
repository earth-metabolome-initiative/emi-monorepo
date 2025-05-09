//! Login providers migration

use core_structures::LoginProvider;
use diesel_async::AsyncPgConnection;
use web_common_traits::prelude::*;

/// Initialize the GitHub login provider if it does not exist
/// in the database. This function checks if a login provider with the
/// name "GitHub" exists. If it does not, it creates a new login provider
/// with the specified attributes and inserts it into the database.
///
/// # Arguments
///
/// * `portal_conn` - A mutable reference to an asynchronous `PostgreSQL`
///   connection.
///
/// # Errors
///
/// * If the login provider cannot be created or inserted into the database, an
///   error is returned.
async fn init_github_login_provider(
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    if LoginProvider::from_name("GitHub", portal_conn).await?.is_none() {
        let _provider = LoginProvider::new()
            .icon("github")?
            .name("GitHub")?
            .oauth_url("https://github.com/login/oauth/authorize")?
            .client_id(std::env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID"))?
            .redirect_uri(std::env::var("GITHUB_REDIRECT_URI").expect("GITHUB_REDIRECT_URI"))?
            .scope("read:user,user:email")?
            .build()?
            .backend_insert(portal_conn)
            .await?;
    }
    Ok(())
}

/// Initialize login providers in the database.
///
/// # Arguments
///
/// * `portal_conn` - A mutable reference to an asynchronous `PostgreSQL`
///   connection.
///
/// # Errors
///
/// * If the login provider cannot be created or inserted into the database, an
///   error is returned.
pub(crate) async fn init_login_providers(
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    init_github_login_provider(portal_conn).await
}
