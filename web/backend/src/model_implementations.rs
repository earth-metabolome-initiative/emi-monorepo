use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::r2d2::PooledConnection;
use email_address::*;
use uuid::Uuid;
use web_common::api::auth::users::name::Name;

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = user_emails)]
pub(crate) struct NewUserEmail {
    email: String,
    user_id: Uuid,
    login_provider_id: i16,
}

impl NewUserEmail {
    pub(crate) fn new(email: &str, user_id: Uuid, login_provider_id: i16) -> NewUserEmail {
        assert!(!email.is_empty());
        assert!(EmailAddress::is_valid(email));
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
        user_id: Uuid,
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
    /// * `user_email` - The email of the user.
    /// * `conn` - The database connection pool.
    pub fn get_user_email_from_email(
        &self,
        user_email: &str,
        conn: &mut PooledConnection<ConnectionManager<diesel::PgConnection>>,
    ) -> QueryResult<UserEmail> {
        use crate::schema::user_emails::dsl::*;
        user_emails
            .filter(email.eq(user_email))
            .filter(user_id.eq(self.id))
            .first::<UserEmail>(conn)
    }

    /// Returns the user's id.
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Updates the user's name in the database.
    ///
    /// # Arguments
    /// * `new_name` - The new name to set for the user.
    /// * `conn` - The database connection pool.
    pub async fn update_name(
        &self,
        new_name: Name,
        conn: &mut PooledConnection<ConnectionManager<diesel::PgConnection>>,
    ) -> Result<(), String> {
        use crate::schema::users::dsl::*;
        let update = diesel::update(users.filter(id.eq(self.id)))
            .set((
                first_name.eq(new_name.first_name().to_string()),
                middle_name.eq(new_name.middle_name().map(|s| s.to_string())),
                last_name.eq(new_name.last_name().to_string()),
            ))
            .execute(conn);
        match update {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to update user name".to_string()),
        }
    }

    pub fn profile_picture_path(&self) -> String {
        format!(
            "/api/users/{}/profile_picture/small.jpg",
            self.id.to_string().to_lowercase()
        )
    }

    pub fn has_profile_picture(&self, conn: &mut crate::DieselConn) -> bool {
        // In order to determine whether a user has a profile picture, we need to check whether
        // the user is the author, in the field created_by from the editables table, of any
        // document from the documents table as determined by the path column.
        let profile_picture_path = self.profile_picture_path();
        use crate::schema::documents::dsl::*;
        use crate::schema::editables::dsl::*;
        editables
            .inner_join(documents)
            .filter(created_by.eq(self.id))
            .filter(path.eq(profile_picture_path))
            .first::<(Editable, Document)>(conn)
            .is_ok()
    }

    /// Create a new web-commons User from a database User.
    pub fn to_web_common_user(
        &self,
        conn: &mut crate::DieselConn,
    ) -> web_common::api::auth::users::User {
        let name = Name::new(
            self.first_name.clone().unwrap_or_default(),
            self.middle_name.clone(),
            self.last_name.clone().unwrap_or_default(),
        )
        .ok();
        if self.has_profile_picture(conn) {
            web_common::api::auth::users::User::new(
                name,
                Some(self.profile_picture_path()),
                self.id,
            )
        } else {
            web_common::api::auth::users::User::new(name, None, self.id)
        }
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
