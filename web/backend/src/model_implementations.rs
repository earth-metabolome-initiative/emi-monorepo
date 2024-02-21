use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;
use email_address::*;

#[derive(Queryable, Insertable, Debug)]
#[table_name = "user_emails"]
pub(crate) struct NewUserEmail {
    email: String,
    user_id: i32,
    login_provider_id: i16,
}

impl NewUserEmail {
    pub fn new(email: &str, user_id: i32, login_provider_id: i16) -> Result<NewUserEmail, String> {
        if email.is_empty() {
            return Err("Email cannot be empty".to_string());
        }

        // We check that the provided email is valid.
        if !EmailAddress::is_valid(email) {
            return Err(format!("Email {} is not valid", email));
        }

        if user_id < 0 {
            return Err(format!(
                "User ID must be greater than 0, received {}",
                user_id
            ));
        }

        if login_provider_id < 0 {
            return Err(format!(
                "Login provider ID must be greater than 0, received {}",
                login_provider_id
            ));
        }

        Ok(NewUserEmail {
            email: email.to_string(),
            user_id,
            login_provider_id,
        })
    }

    pub fn insert(&self, pool: &Pool<ConnectionManager<PgConnection>>) -> Result<(), String> {
        use crate::schema::user_emails::dsl::*;
        let mut conn = pool.get().unwrap();
        let result = diesel::insert_into(user_emails)
            .values(self)
            .execute(&mut conn);
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Failed to insert user email {}", self.email)),
        }
    }
}

impl UserEmail {
    /// Returns the user id for a given email.
    ///
    /// # Arguments
    /// * `email` - The email of the user.
    /// * `pool` - The database connection pool.
    pub fn get_row_from_email(
        email: &str,
        pool: &Pool<ConnectionManager<PgConnection>>,
    ) -> Result<UserEmail, String> {
        let mut conn = pool.get().unwrap();
        let row = user_emails::dsl::user_emails
            .filter(user_emails::email.eq(email))
            .first::<UserEmail>(&mut conn);
        match row {
            Ok(user_email) => Ok(user_email),
            Err(_) => Err(format!("No user with email {} found", email)),
        }
    }

    /// Returns the user id for a given email and provider.
    ///
    /// # Arguments
    /// * `user_email` - The email of the user.
    /// * `provider_id` - The ID of the provider.
    /// * `pool` - The database connection pool.
    pub fn get_user_id_from_user_email_and_provider(
        user_email: &str,
        provider_id: i16,
        pool: &Pool<ConnectionManager<PgConnection>>,
    ) -> Result<i32, String> {
        use crate::schema::user_emails::dsl::*;
        let mut conn = pool.get().unwrap();
        let row = user_emails
            .filter(email.eq(user_email))
            .filter(login_provider_id.eq(provider_id))
            .first::<UserEmail>(&mut conn);
        match row {
            Ok(user_email) => Ok(user_email.user_id),
            Err(_) => {
                // We retrieve the name of the provider from the database.
                let provider = LoginProvider::get(provider_id, pool);
                match provider {
                    Ok(provider) => Err(format!(
                        "No user with email {} and provider {} found",
                        user_email, provider.name
                    )),
                    Err(_) => Err(format!(
                        "No user with email {} and provider {} found",
                        user_email, provider_id
                    )),
                }
            }
        }
    }
}

impl LoginProvider {
    pub fn get_provider_id(
        provider_name: &str,
        pool: &Pool<ConnectionManager<PgConnection>>,
    ) -> Result<i16, String> {
        use crate::schema::login_providers::dsl::*;
        let mut conn = pool.get().unwrap();
        let provider = login_providers
            .filter(name.eq(provider_name))
            .first::<LoginProvider>(&mut conn);
        match provider {
            Ok(provider) => Ok(provider.id),
            Err(_) => Err(format!("No provider with name {} found", provider_name)),
        }
    }

    pub fn get(
        provider_id: i16,
        pool: &Pool<ConnectionManager<PgConnection>>,
    ) -> Result<LoginProvider, String> {
        use crate::schema::login_providers::dsl::*;
        let mut conn = pool.get().unwrap();
        let provider = login_providers
            .filter(id.eq(provider_id))
            .first::<LoginProvider>(&mut conn);
        match provider {
            Ok(provider) => Ok(provider),
            Err(_) => Err(format!("No provider with id {} found", provider_id)),
        }
    }
}

#[derive(Queryable, Insertable, Debug, Default)]
#[table_name = "users"]
pub(crate) struct NewUser {
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
}

impl NewUser {
    pub fn insert_default(pool: &Pool<ConnectionManager<PgConnection>>) -> Result<i32, String> {
        use crate::schema::users::dsl::*;
        let mut conn = pool.get().unwrap();
        let result = diesel::insert_into(users)
            .default_values()
            .returning(id)
            .get_result::<i32>(&mut conn);
        match result {
            Ok(user_id) => Ok(user_id),
            Err(_) => Err("Failed to insert user".to_string()),
        }
    }
}

impl User {
    /// Returns the user with the given ID.
    /// 
    /// # Arguments
    /// * `user_id` - The ID of the user.
    /// * `pool` - The database connection pool.
    pub fn get(user_id: i32, pool: &Pool<ConnectionManager<PgConnection>>) -> Result<User, String> {
        use crate::schema::users::dsl::*;
        let mut conn = pool.get().unwrap();
        let user = users.filter(id.eq(user_id)).first::<User>(&mut conn);
        match user {
            Ok(user) => Ok(user),
            Err(_) => Err(format!("No user with id {} found", user_id)),
        }
    }

    /// Returns whether a user with the given ID exists.
    /// 
    /// # Arguments
    /// * `user_id` - The ID of the user.
    /// * `pool` - The database connection pool.
    pub fn exists(user_id: i32, pool: &Pool<ConnectionManager<PgConnection>>) -> bool {
        User::get(user_id, pool).is_ok()
    }
}