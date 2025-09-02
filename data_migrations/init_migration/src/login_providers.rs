//! Login providers migration

use core_structures::{LoginProvider, User, tables::insertables::LoginProviderBuildable};
use diesel::{OptionalExtension, PgConnection};
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
fn init_github_login_provider(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<LoginProvider> {
    if let Some(provider) = LoginProvider::from_name("GitHub", conn).optional()? {
        return Ok(provider);
    }

    Ok(LoginProvider::new()
        .icon("github")?
        .name("GitHub")?
        .oauth_url("https://github.com/login/oauth/authorize")?
        .client(std::env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID"))?
        .redirect_uri(std::env::var("GITHUB_REDIRECT_URI").expect("GITHUB_REDIRECT_URI"))?
        .scope("read:user,user:email")?
        .insert(user.id, conn)?)
}

/// Initialize login providers in the database.
///
/// # Arguments
///
/// * `user` - A reference to the user who is initializing the login providers.
/// * `conn` - A mutable reference to an hronous `PostgreSQL` connection.
///
/// # Errors
///
/// * If the login provider cannot be created or inserted into the database, an
///   error is returned.
pub(crate) fn init_login_providers(user: &User, conn: &mut PgConnection) -> anyhow::Result<()> {
    let _github = init_github_login_provider(user, conn)?;
    Ok(())
}
