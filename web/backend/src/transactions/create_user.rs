//! Transaction to create a new user.
//!
//! This transaction is used when a new user needs to be created, and includes:
//!
//! - Inserting the user into the users table
//! - Inserting the user's email into the user_emails table
//! - Inserting the user's primary email into the primary_user_emails table

use crate::models::*;
use crate::new_variants::InsertRow;
use crate::transactions::renormalize_user_emails::Emails;
use diesel::connection::SimpleConnection; // Required for batch_execute
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use identicon_rs::Identicon;
use std::io::Cursor;
use web_common::database::{NewUser, NewUserEmail};

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
    provider_id: i32,
    user_emails: Emails,
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> QueryResult<User> {
    let chained_emails = user_emails.emails().join(", ");
    let identicon = Identicon::new(&chained_emails);
    let dynamic_image = identicon.generate_image().unwrap();

    // Write the image data into the buffer as PNG format
    let mut png_buffer = Cursor::new(Vec::new());

    dynamic_image
        .write_to(&mut png_buffer, image::ImageOutputFormat::Png)
        .unwrap();

    let png_buffer = png_buffer.into_inner();

    let new_user = NewUser {
        first_name: "".to_string(),
        middle_name: None,
        description: None,
        last_name: "".to_string(),
        profile_picture: png_buffer,
    };

    let mut conn = pool.get().unwrap();

    conn.batch_execute("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE;")
        .expect("Failed to set transaction isolation level");

    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        // Step 1: Insert the user into the users table
        log::debug!("Inserting new user in table from provider {}", provider_id);
        let user = new_user.insert(0, conn)?;

        // Step 2: Insert the user's email into the user_emails table
        for email in user_emails.emails() {
            log::debug!(
                "Inserting new user {} email {} in table for provider {}",
                user.id,
                email,
                provider_id
            );
            let new_user_email = NewUserEmail {
                email: email.clone(),
                login_provider_id: provider_id,
                primary_email: true,
            };

            let _ = new_user_email.insert(user.id, conn)?;
        }

        Ok(user)
    })
}
