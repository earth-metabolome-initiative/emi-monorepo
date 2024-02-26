use crate::diesel_enums::OrganizationUserRole as OrganizationUserRoleEnum;
use crate::diesel_enums::ProjectUserRole as ProjectUserRoleEnum;
use crate::diesel_enums::WebsiteRole;
use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::r2d2::PooledConnection;
use email_address::*;
use uuid::Uuid;

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

    pub fn is_website_admin(
        &self,
        conn: &mut PooledConnection<ConnectionManager<diesel::PgConnection>>,
    ) -> bool {
        use crate::schema::describables::dsl::*;
        use crate::schema::website_user_roles::dsl::*;
        // Check if the user has the admin role
        website_user_roles
            .filter(user_id.eq(self.id))
            // We execute an inner join on the website_role_id from the website_user_roles table
            // and the id from the describables table, to get the actual role name.
            .inner_join(describables.on(crate::schema::describables::id.eq(website_role_id)))
            .filter(name.eq(WebsiteRole::Admin.to_string()))
            .select(crate::schema::website_user_roles::id)
            .first::<i64>(conn)
            .is_ok()
    }

    pub fn is_organization_admin(
        &self,
        organization_id: i64,
        conn: &mut PooledConnection<ConnectionManager<diesel::PgConnection>>,
    ) -> bool {
        use crate::schema::describables::dsl::*;
        crate::schema::organization_users::dsl::organization_users
            .filter(crate::schema::organization_users::user_id.eq(self.id))
            .filter(crate::schema::organization_users::organization_id.eq(organization_id))
            .inner_join(
                describables
                    .on(crate::schema::describables::id
                        .eq(crate::schema::organization_users::role_id)),
            )
            .filter(name.eq(OrganizationUserRoleEnum::Admin.to_string()))
            .select(crate::schema::organization_users::id)
            .first::<i64>(conn)
            .is_ok()
    }

    pub fn is_project_admin(
        &self,
        project_id: i64,
        conn: &mut PooledConnection<ConnectionManager<diesel::PgConnection>>,
    ) -> bool {
        use crate::schema::describables::dsl::*;
        // Check if the user has the admin role
        crate::schema::project_users::dsl::project_users
            .filter(crate::schema::project_users::dsl::user_id.eq(self.id))
            .filter(crate::schema::project_users::dsl::project_id.eq(project_id))
            .inner_join(
                describables
                    .on(crate::schema::describables::id
                        .eq(crate::schema::project_users::dsl::role_id)),
            )
            .filter(name.eq(ProjectUserRoleEnum::Admin.to_string()))
            .select(crate::schema::project_users::id)
            .first::<i64>(conn)
            .is_ok()
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
