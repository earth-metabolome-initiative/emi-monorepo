//! Transaction to create a new user.
//!
//! This transaction is used when a new user needs to be created, and includes:
//!
//! - Inserting the user into the users table
//! - Inserting the user's email into the user_emails table
//! - Inserting the user's primary email into the primary_user_emails table
//!

use crate::diesel::connection::SimpleConnection; // Required for batch_execute
use crate::model_implementations::*;
use crate::models::*;
use crate::transactions::renormalize_user_emails::Emails;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

/// Return a newly created user.
///
/// # Arguments
/// * `provider_id` - The ID of the login provider.
/// * `new_user` - The new user to be created.
/// * `user_emails` - The emails associated with the user.
/// * `pool` - The connection pool to the database.
///
/// # Implementative details
/// If multiple emails are provided, each one of them is inserted
/// separately into the user_emails table.
pub(crate) fn create_user(
    provider_id: i16,
    new_user: NewUser,
    user_emails: Emails,
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> QueryResult<User> {
    let mut conn = pool.get().unwrap();

    conn.batch_execute("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE;")
        .expect("Failed to set transaction isolation level");

    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        // Step 1: Insert the user into the users table
        log::debug!("Inserting new user in table from provider {}", provider_id);
        let user = diesel::insert_into(crate::schema::users::table)
            .values(&new_user)
            .get_result::<User>(conn)?;

        // Step 2: Insert the user's email into the user_emails table
        for email in user_emails.emails() {
            log::debug!("Inserting new user {} email {} in table for provider {}", user.id, email, provider_id);
            let new_user_email = NewUserEmail::new(email, user.id, provider_id);
            let email = new_user_email.insert(conn)?;
            // We check whether the email is the primary email
            if user_emails.is_primary(&email.email) {
                log::debug!("Inserting new user {} PRIMARY email {} in table for provider {}", user.id, email.email, provider_id);
                // Step 3: Insert the user's primary email into the primary_user_emails table
                let primary_user_email = NewPrimaryUserEmail::new(email.id);
                primary_user_email.insert(conn)?;
            }
        }

        Ok(user)
    })
}
