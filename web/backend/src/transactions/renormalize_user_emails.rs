//! Tooling to maintain user consistency when email addresses lead to user merger.
//!
//! When a user logs in with one or more new email addresses, we need to check if the email address is already
//! associated with another user. If it is, we need to merge the two users.
//!
//! The procedure first checks whether any of the emails provided are already associated with a user. If None of them
//! are, we can create a new user account, with the provided provider ID and email address. If one of the emails is
//! already associated with a user but NOT with the current login provider, we simply insert the new email address
//! into the user_emails table, and associate it with the user and the current login provider. If the email is already
//! associated with the current login provider, we do nothing.
//! If more than one of the emails is associated with a user, we merge the users, and associate the new email address
//! with the user that is kept.
//!
//! This last operation is the most complex, as it requires updating all foreign keys that point to the user that is
//! deleted, and updating the user_id of all the posts that the user has made. This is done in a transaction, to ensure
//! that the database is in a consistent state at all times.
//!
//! A transaction, in SQL speech, is a sequence of operations performed as a single unit of work. A logical unit of work
//! must exhibit four properties, called the ACID (Atomicity, Consistency, Isolation, and Durability) properties, to
//! qualify as a transaction. For example, a transfer of funds from one bank account to another, even involving multiple
//! changes such as debiting one account and crediting another, is a single transaction.
//!
//! The transaction is started by calling the `transaction` method on the connection, and passing a closure that
//! contains the operations that should be performed as part of the transaction. If the closure returns an error, the
//! transaction is rolled back, and the error is returned. If the closure returns Ok, the transaction is committed, and
//! the Ok value is returned.
use crate::database::new_variants::InsertRow;
use crate::database::*;
use crate::transactions::create_user::create_user;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sql_query;
use diesel::sql_types::Text;
use email_address::EmailAddress;
use web_common::api::ApiError;
use web_common::database::NewUserEmail;

pub(crate) struct Emails {
    emails: Vec<String>,
}

impl Emails {
    pub(crate) fn new(emails: Vec<String>, primary: String) -> Result<Emails, ApiError> {
        if !emails.contains(&primary) {
            Err("Primary email not in list of emails")?;
        }

        // We check if any of the emails provided are not valid email addresses.
        if emails.iter().any(|email| !EmailAddress::is_valid(email)) {
            Err("Invalid email address")?;
        }

        Ok(Emails { emails })
    }

    pub(crate) fn emails(&self) -> &[String] {
        &self.emails
    }
}

fn get_unique_users_from_emails(
    emails: &[String],
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> QueryResult<Vec<User>> {
    use crate::database::schema::user_emails::dsl::*;
    use crate::database::schema::users::dsl::*;

    let mut conn = pool.get().unwrap();

    user_emails
        .filter(email.eq_any(emails))
        .inner_join(users)
        .select(User::as_select())
        .distinct()
        .load::<User>(&mut conn)
}

#[derive(QueryableByName)]
struct ForeignKeyInfo {
    #[diesel(sql_type = Text)]
    table_name: String,
    #[diesel(sql_type = Text)]
    column_name: String,
}

/// Update foreign keys that reference the old user ID to reference the new user ID.
///
/// # Arguments
/// * `old_user_id` - The ID of the user that is being deleted.
/// * `new_user_id` - The ID of the user that is kept.
/// * `conn` - The connection to the database.
fn update_foreign_user_id_keys(
    old_user_id: i32,
    new_user_id: i32,
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> QueryResult<()> {
    let mut conn = pool.get().unwrap();

    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        // Step 2: Update foreign keys
        // Query to obtain tables and columns with foreign key constraints referencing the users table
        let foreign_key_tables_query = "
            SELECT
                tc.table_name,
                kcu.column_name
            FROM
                information_schema.table_constraints AS tc
                JOIN information_schema.key_column_usage AS kcu
                ON tc.constraint_name = kcu.constraint_name
            WHERE
                tc.constraint_type = 'FOREIGN KEY' AND
                kcu.referenced_table_name = 'users'
        ";

        // Execute the query and iterate over the result
        let foreign_key_tables: Vec<ForeignKeyInfo> =
            sql_query(foreign_key_tables_query).load(conn)?;

        for info in foreign_key_tables {
            // Generate and execute UPDATE statement for each table
            let update_statement = format!(
                "UPDATE {} SET {} = $1 WHERE {} = $2",
                info.table_name, info.column_name, info.column_name
            );

            sql_query(&update_statement)
                .bind::<diesel::sql_types::Integer, _>(new_user_id)
                .bind::<diesel::sql_types::Integer, _>(old_user_id)
                .execute(conn)?;
        }

        Ok(())
    })
}

fn insert_user_emails(
    user: &User,
    provider_id: i32,
    emails: &[String],
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> QueryResult<()> {
    let mut conn = pool.get().unwrap();

    for email in emails {
        // If the email is not associated with the user, we insert it.
        let user_email = UserEmail::from_email_and_login_provider_id(
            email,
            &provider_id,
            Some(user.id),
            &mut conn,
        );

        if user_email.is_err() {
            let new_user_email = NewUserEmail {
                email: email.clone(),
                login_provider_id: provider_id,
                primary_email: true,
            };
            new_user_email.insert(user.id, &mut conn)?;
        }
    }

    Ok(())
}

pub(crate) fn renormalize_user_emails(
    provider_id: i32,
    emails: Emails,
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> QueryResult<User> {
    use crate::database::schema::users::dsl::*;

    // First, we need to identify how many of the emails are already associated with users.
    // 1) If none of them are, we can create a new user account, with the provided provider ID and email address.
    // 2) If only one of the emails is already associated with a user, we insert all of the emails with the new provider
    // ID, including the first one if the provider ID of the relationship is different from the one we are trying to
    // insert.
    // 3) If more than one of the emails is associated with a user, we merge the users, and associate the new email
    // address with the user that is kept. This means also updating all of the other foreign keys that point to the
    // user that is deleted, and updating the user_id of all the tables that reference the user that is deleted.
    // Particular attention needs to be paid to future proofing this last operation, as when we add new tables that
    // reference the user, we need to remember to update this operation to include the new tables.

    // We start with identifying the users that are associated with the emails provided.
    let mut mail_users = get_unique_users_from_emails(emails.emails(), pool)?;

    match mail_users.len() {
        0 => {
            log::info!("Creating the new user for provider {}", provider_id);
            // Case 0: If no mails are associated with a user, we can create a new user account.
            create_user(provider_id, emails, pool)
        }
        1 => {
            log::info!("Extending emails from provider {}", provider_id);
            // Case 1: If only one email is associated with a user, we insert all of the emails with the new provider ID.
            let this_user = mail_users.pop().unwrap();

            insert_user_emails(&this_user, provider_id, emails.emails(), pool)?;

            Ok(this_user)
        }
        number_of_emails => {
            log::info!(
                "Attempting to merge {} from provider {}",
                number_of_emails,
                provider_id
            );
            // Case 3, user merger: If more than one email is associated with a user, we merge the users.
            // We keep the first user, and merge the others into it.
            let user_to_keep = mail_users.pop().unwrap();

            let mut conn = pool.get().unwrap();

            for mail_user in mail_users {
                update_foreign_user_id_keys(mail_user.id, user_to_keep.id, pool)?;
                diesel::delete(users.filter(id.eq(mail_user.id))).execute(&mut conn)?;
            }

            // And now that we have merged the users, we find ourselves in the same situation as in case 1.
            insert_user_emails(&user_to_keep, provider_id, emails.emails(), pool)?;

            Ok(user_to_keep)
        }
    }
}
