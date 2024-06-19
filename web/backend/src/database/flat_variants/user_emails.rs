//! This file is generated automatically and should not be modified.
//!
//! Any edits you may apply to this document will be overwritten next time the
//! backend is generated. Refrain from making any changes to this file.

//! If you need to make changes to the backend, please modify the `generate_models`
//! document in the `migrations` folder.

use diesel::prelude::*;
use diesel::Identifiable;
use diesel::Insertable;
use diesel::Queryable;
use diesel::QueryableByName;
use diesel::Selectable;

#[derive(
    PartialEq,
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Identifiable,
    QueryableByName,
    Queryable,
    Insertable,
    Selectable,
    AsChangeset,
)]
#[diesel(table_name = crate::database::schema::user_emails)]
#[diesel(primary_key(id))]
pub struct UserEmail {
    pub id: i32,
    pub email: String,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
    pub login_provider_id: i32,
    pub primary_email: bool,
}

unsafe impl Send for UserEmail {}
unsafe impl Sync for UserEmail {}
impl From<web_common::database::flat_variants::UserEmail>
    for crate::database::flat_variants::UserEmail
{
    fn from(item: web_common::database::flat_variants::UserEmail) -> Self {
        Self {
            id: item.id,
            email: item.email,
            created_by: item.created_by,
            created_at: item.created_at,
            login_provider_id: item.login_provider_id,
            primary_email: item.primary_email,
        }
    }
}

impl From<crate::database::flat_variants::UserEmail>
    for web_common::database::flat_variants::UserEmail
{
    fn from(item: crate::database::flat_variants::UserEmail) -> Self {
        Self {
            id: item.id,
            email: item.email,
            created_by: item.created_by,
            created_at: item.created_at,
            login_provider_id: item.login_provider_id,
            primary_email: item.primary_email,
        }
    }
}

impl UserEmail {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_view(
        &self,
        author_user_id: Option<i32>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(self.id, author_user_id, connection)
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_view_by_id(
        id: i32,
        author_user_id: Option<i32>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(
            crate::database::sql_function_bindings::can_view_user_emails(author_user_id, id),
        )
        .get_result(connection)
        .map_err(web_common::api::ApiError::from)
    }
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_viewable(
        filter: Option<&web_common::database::filter_variants::UserEmailFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::database::schema::user_emails;
        let query = user_emails::dsl::user_emails
            .select(UserEmail::as_select())
            .filter(
                crate::database::sql_function_bindings::can_view_user_emails(
                    author_user_id,
                    user_emails::dsl::id,
                ),
            )
            .order_by(user_emails::dsl::id);
        let mut query = query.into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(user_emails::dsl::created_by.eq(created_by));
        }
        if let Some(login_provider_id) = filter.and_then(|f| f.login_provider_id) {
            query = query.filter(user_emails::dsl::login_provider_id.eq(login_provider_id));
        }
        query
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_viewable_sorted(
        filter: Option<&web_common::database::filter_variants::UserEmailFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::database::schema::user_emails;
        let query = user_emails::dsl::user_emails
            .select(UserEmail::as_select())
            .filter(
                crate::database::sql_function_bindings::can_view_user_emails(
                    author_user_id,
                    user_emails::dsl::id,
                ),
            )
            .order_by(user_emails::dsl::created_at.desc());
        let mut query = query.into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(user_emails::dsl::created_by.eq(created_by));
        }
        if let Some(login_provider_id) = filter.and_then(|f| f.login_provider_id) {
            query = query.filter(user_emails::dsl::login_provider_id.eq(login_provider_id));
        }
        query
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    pub fn get(
        id: i32,
        author_user_id: Option<i32>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        if !Self::can_view_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::database::schema::user_emails;
        user_emails::dsl::user_emails
            .filter(user_emails::dsl::id.eq(id))
            .first::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its email and login_provider_id.
    ///
    /// * `email` - The email of the struct to get.
    /// * `login_provider_id` - The login_provider_id of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    pub fn from_email_and_login_provider_id(
        email: &str,
        login_provider_id: &i32,
        author_user_id: Option<i32>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        use crate::database::schema::user_emails;
        let flat_variant = user_emails::dsl::user_emails
            .filter(user_emails::dsl::email.eq(email))
            .filter(user_emails::dsl::login_provider_id.eq(login_provider_id))
            .first::<Self>(connection)?;
        if !flat_variant.can_view(author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        Ok(flat_variant)
    }
    /// Check whether the user can update the struct.
    pub fn can_update(&self) -> Result<bool, web_common::api::ApiError> {
        Ok(false)
    }
    /// Check whether the user can update the struct associated to the provided ids.
    pub fn can_update_by_id() -> Result<bool, web_common::api::ApiError> {
        Ok(false)
    }
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_updatable(
        filter: Option<&web_common::database::filter_variants::UserEmailFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::database::schema::user_emails;
        let query = user_emails::dsl::user_emails
            .select(UserEmail::as_select())
            .order_by(user_emails::dsl::id);
        let mut query = query.into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(user_emails::dsl::created_by.eq(created_by));
        }
        if let Some(login_provider_id) = filter.and_then(|f| f.login_provider_id) {
            query = query.filter(user_emails::dsl::login_provider_id.eq(login_provider_id));
        }
        query
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_updatable_sorted(
        filter: Option<&web_common::database::filter_variants::UserEmailFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::database::schema::user_emails;
        let query = user_emails::dsl::user_emails
            .select(UserEmail::as_select())
            .order_by(user_emails::dsl::created_at.desc());
        let mut query = query.into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(user_emails::dsl::created_by.eq(created_by));
        }
        if let Some(login_provider_id) = filter.and_then(|f| f.login_provider_id) {
            query = query.filter(user_emails::dsl::login_provider_id.eq(login_provider_id));
        }
        query
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_admin(
        &self,
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(self.id, author_user_id, connection)
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_admin_by_id(
        id: i32,
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(
            crate::database::sql_function_bindings::can_admin_user_emails(author_user_id, id),
        )
        .get_result(connection)
        .map_err(web_common::api::ApiError::from)
    }
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_administrable(
        filter: Option<&web_common::database::filter_variants::UserEmailFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::database::schema::user_emails;
        let query = user_emails::dsl::user_emails
            .select(UserEmail::as_select())
            .filter(
                crate::database::sql_function_bindings::can_admin_user_emails(
                    author_user_id,
                    user_emails::dsl::id,
                ),
            )
            .order_by(user_emails::dsl::id);
        let mut query = query.into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(user_emails::dsl::created_by.eq(created_by));
        }
        if let Some(login_provider_id) = filter.and_then(|f| f.login_provider_id) {
            query = query.filter(user_emails::dsl::login_provider_id.eq(login_provider_id));
        }
        query
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_administrable_sorted(
        filter: Option<&web_common::database::filter_variants::UserEmailFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::database::schema::user_emails;
        let query = user_emails::dsl::user_emails
            .select(UserEmail::as_select())
            .filter(
                crate::database::sql_function_bindings::can_admin_user_emails(
                    author_user_id,
                    user_emails::dsl::id,
                ),
            )
            .order_by(user_emails::dsl::created_at.desc());
        let mut query = query.into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(user_emails::dsl::created_by.eq(created_by));
        }
        if let Some(login_provider_id) = filter.and_then(|f| f.login_provider_id) {
            query = query.filter(user_emails::dsl::login_provider_id.eq(login_provider_id));
        }
        query
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    pub fn delete(
        &self,
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<usize, web_common::api::ApiError> {
        Self::delete_by_id(self.id, author_user_id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    pub fn delete_by_id(
        id: i32,
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<usize, web_common::api::ApiError> {
        if !Self::can_admin_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(
            crate::database::user_emails::dsl::user_emails
                .filter(crate::database::user_emails::dsl::id.eq(id)),
        )
        .execute(connection)
        .map_err(web_common::api::ApiError::from)
    }
}
