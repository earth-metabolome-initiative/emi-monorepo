//! Login providers migration

use core_structures::LoginProvider;
use diesel::PgConnection;
use web_common_traits::prelude::*;

/// Initialize the GitHub login provider if it does not exist
/// in the database. This function checks if a login provider with the
/// name "GitHub" exists. If it does not, it creates a new login provider
/// with the specified attributes and inserts it into the database.
///
/// # Arguments
///
/// * `conn` - A mutable reference to an hronous `PostgreSQL` connection.
///
/// # Errors
///
/// * If the login provider cannot be created or inserted into the database, an
///   error is returned.
fn init_github_login_provider(conn: &mut PgConnection) -> Result<(), crate::error::Error> {
    if LoginProvider::from_name("GitHub", conn)?.is_none() {
        let _provider = LoginProvider::new()
            .icon("github")?
            .name("GitHub")?
            .oauth_url("https://github.com/login/oauth/authorize")?
            .client_id(std::env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID"))?
            .redirect_uri(std::env::var("GITHUB_REDIRECT_URI").expect("GITHUB_REDIRECT_URI"))?
            .scope("read:user,user:email")?
            .unchecked_insert(conn)?;
    }
    Ok(())
}

/// Initialize login providers in the database.
///
/// # Arguments
///
/// * `conn` - A mutable reference to an hronous `PostgreSQL` connection.
///
/// # Errors
///
/// * If the login provider cannot be created or inserted into the database, an
///   error is returned.
pub(crate) fn init_login_providers(conn: &mut PgConnection) -> Result<(), crate::error::Error> {
    init_github_login_provider(conn)
}
