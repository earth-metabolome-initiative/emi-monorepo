//! Submodule providing a method to migrate the users from the Directus database
//! to the new database.

use diesel_async::AsyncPgConnection;
use web_common_traits::database::AsyncRead;

use super::get_user;
use crate::codegen::DirectusUser;

pub async fn insert_missing_users(
    directus_conn: &mut AsyncPgConnection,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let directus_users = DirectusUser::load_all(directus_conn).await?;
    for directus_user in directus_users {
        let _portal_user = match get_user(&directus_user, directus_conn, portal_conn).await {
            Ok(user) => user,
            Err(crate::error::Error::UserNeverLoggedIn(user)) => {
                println!(
                    "User never logged in: {} {}",
                    user.first_name.unwrap(),
                    user.last_name.unwrap()
                );
                continue;
            }
            Err(error) => {
                return Err(error);
            }
        };
    }

    Ok(())
}
