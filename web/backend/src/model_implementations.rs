use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::r2d2::PooledConnection;
use diesel::PgConnection;
use email_address::*;

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = user_emails)]
pub(crate) struct NewUserEmail {
    email: String,
    user_id: i32,
    login_provider_id: i16,
}

impl NewUserEmail {
    pub(crate) fn new(email: &str, user_id: i32, login_provider_id: i16) -> NewUserEmail {
        assert!(!email.is_empty());
        assert!(EmailAddress::is_valid(email));
        assert!(user_id > 0);
        assert!(login_provider_id > 0);

        NewUserEmail {
            email: email.to_string(),
            user_id,
            login_provider_id,
        }
    }

    pub fn insert(
        &self,
        conn: &mut PooledConnection<ConnectionManager<diesel::PgConnection>>,
    ) -> Result<UserEmail, diesel::result::Error> {
        use crate::schema::user_emails::dsl::*;
        let result = diesel::insert_into(user_emails).values(self).execute(conn);
        match result {
            Ok(_) => user_emails
                .filter(email.eq(&self.email))
                .filter(user_id.eq(self.user_id))
                .filter(login_provider_id.eq(self.login_provider_id))
                .first::<UserEmail>(conn),
            Err(e) => Err(e),
        }
    }
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = primary_user_emails)]
pub(crate) struct NewPrimaryUserEmail {
    id: i32,
}

impl NewPrimaryUserEmail {
    pub(crate) fn new(id: i32) -> NewPrimaryUserEmail {
        assert!(id > 0);

        NewPrimaryUserEmail { id }
    }

    pub(crate) fn insert(
        &self,
        conn: &mut PooledConnection<ConnectionManager<diesel::PgConnection>>,
    ) -> QueryResult<usize> {
        use crate::schema::primary_user_emails::dsl::*;
        diesel::insert_into(primary_user_emails)
            .values(self)
            .execute(conn)
    }
}

impl UserEmail {
    pub(crate) fn provider_id(&self) -> i16 {
        self.login_provider_id
    }

    pub(crate) fn user_id(&self) -> i32 {
        self.user_id
    }

    pub(crate) fn email(&self) -> &str {
        &self.email
    }
}

impl LoginProvider {
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

    pub fn from_provider_name(
        provider_name: &str,
        pool: &Pool<ConnectionManager<PgConnection>>,
    ) -> Result<LoginProvider, String> {
        use crate::schema::login_providers::dsl::*;
        let mut conn = pool.get().unwrap();
        let provider = login_providers
            .filter(name.eq(provider_name))
            .first::<LoginProvider>(&mut conn);
        match provider {
            Ok(provider) => Ok(provider),
            Err(_) => Err(format!("No provider with name {} found", provider_name)),
        }
    }
}

#[derive(Queryable, Insertable, Debug, Default)]
#[diesel(table_name = users)]
pub(crate) struct NewUser {
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
}

impl User {
    /// Returns the user with the given ID.
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user.
    /// * `pool` - The database connection pool.
    pub fn get(
        user_id: i32,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<User, String> {
        use crate::schema::users::dsl::*;
        let user = users.filter(id.eq(user_id)).first::<User>(conn);
        match user {
            Ok(user) => Ok(user),
            Err(_) => Err(format!("No user with id {} found", user_id)),
        }
    }

    /// Returns the UserMail associated to the provided email if it is associated to the current user.
    ///
    /// # Arguments
    /// * `email` - The email of the user.
    /// * `conn` - The database connection pool.
    pub fn get_user_email_from_email(
        &self,
        email: &str,
        conn: &mut PooledConnection<ConnectionManager<diesel::PgConnection>>,
    ) -> QueryResult<UserEmail> {
        use crate::schema::user_emails::dsl::*;
        user_emails
            .filter(email.eq(email))
            .filter(user_id.eq(self.id))
            .first::<UserEmail>(conn)
    }

    /// Returns the user's id.
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn is_website_admin(
        &self,
        conn: &mut PooledConnection<ConnectionManager<diesel::PgConnection>>,
    ) -> bool {
        use crate::schema::primary_user_emails::dsl::*;
        
    }
}

impl LoginProvider {
    /// Returns list of available login providers.
    ///
    /// # Arguments
    /// * `pool` - The database connection pool.
    pub fn get_all(
        pool: &Pool<ConnectionManager<PgConnection>>,
    ) -> Result<Vec<LoginProvider>, String> {
        use crate::schema::login_providers::dsl::*;
        let mut conn = pool.get().unwrap();
        let providers = login_providers.load::<LoginProvider>(&mut conn);
        match providers {
            Ok(providers) => Ok(providers),
            Err(_) => Err("Failed to retrieve login providers".to_string()),
        }
    }
}
