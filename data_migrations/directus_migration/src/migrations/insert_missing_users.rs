//! Submodule providing a method to migrate the users from the Directus database
//! to the new database.

use diesel::PgConnection;
use web_common_traits::database::BoundedRead;

use super::get_user;
use crate::codegen::DirectusUser;

pub fn insert_missing_users(
    directus_conn: &mut PgConnection,
    portal_conn: &mut PgConnection,
) -> Result<(), crate::error::Error> {
    let directus_users = DirectusUser::bounded_read(0, u16::MAX, directus_conn)?;
    for directus_user in directus_users {
        let _portal_user = match get_user(&directus_user, directus_conn, portal_conn) {
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
