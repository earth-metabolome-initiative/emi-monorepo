//! This file is generated automatically and should not be modified.
//!
//! Any edits you may apply to this document will be overwritten next time the
//! backend is generated. Refrain from making any changes to this file.

//! If you need to make changes to the backend, please modify the `generate_models`
//! document in the `migrations` folder.

use crate::database::*;
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
#[diesel(table_name = crate::database::schema::nameplate_categories)]
#[diesel(primary_key(id))]
pub struct NameplateCategory {
    pub id: i32,
    pub name: String,
    pub permanence_id: i32,
    pub material_id: i32,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

unsafe impl Send for NameplateCategory {}
unsafe impl Sync for NameplateCategory {}
impl From<web_common::database::flat_variants::NameplateCategory>
    for crate::database::flat_variants::NameplateCategory
{
    fn from(item: web_common::database::flat_variants::NameplateCategory) -> Self {
        Self {
            id: item.id,
            name: item.name,
            permanence_id: item.permanence_id,
            material_id: item.material_id,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl From<crate::database::flat_variants::NameplateCategory>
    for web_common::database::flat_variants::NameplateCategory
{
    fn from(item: crate::database::flat_variants::NameplateCategory) -> Self {
        Self {
            id: item.id,
            name: item.name,
            permanence_id: item.permanence_id,
            material_id: item.material_id,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl NameplateCategory {
    /// Check whether the user can view the struct.
    pub fn can_view(&self) -> Result<bool, web_common::api::ApiError> {
        Ok(true)
    }
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id() -> Result<bool, web_common::api::ApiError> {
        Ok(true)
    }
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_viewable(
        filter: Option<&web_common::database::filter_variants::NameplateCategoryFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::database::schema::nameplate_categories;
        let query = nameplate_categories::dsl::nameplate_categories
            .select(NameplateCategory::as_select())
            .order_by(nameplate_categories::dsl::id);
        let mut query = query.into_boxed();
        if let Some(permanence_id) = filter.and_then(|f| f.permanence_id) {
            query = query.filter(nameplate_categories::dsl::permanence_id.eq(permanence_id));
        }
        if let Some(material_id) = filter.and_then(|f| f.material_id) {
            query = query.filter(nameplate_categories::dsl::material_id.eq(material_id));
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(nameplate_categories::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(nameplate_categories::dsl::color_id.eq(color_id));
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
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_viewable_sorted(
        filter: Option<&web_common::database::filter_variants::NameplateCategoryFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        Self::all_viewable(filter, limit, offset, connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    pub fn get(
        id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        use crate::database::schema::nameplate_categories;
        nameplate_categories::dsl::nameplate_categories
            .filter(nameplate_categories::dsl::id.eq(id))
            .first::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn strict_word_similarity_search_viewable(
        filter: Option<&web_common::database::filter_variants::NameplateCategoryFilter>,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::database::schema::nameplate_categories;
        let mut query = nameplate_categories::dsl::nameplate_categories
            .select(NameplateCategory::as_select())
            .filter(
                crate::database::sql_function_bindings::concat_nameplate_categories_brand(
                    nameplate_categories::dsl::name,
                    nameplate_categories::dsl::description,
                )
                .strict_word_similarity_commutator_op(query)
                .or(
                    crate::database::sql_function_bindings::concat_nameplate_categories_brand(
                        nameplate_categories::dsl::name,
                        nameplate_categories::dsl::description,
                    )
                    .ilike(format!("%{}%", query)),
                ),
            )
            .order(
                crate::database::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::database::sql_function_bindings::concat_nameplate_categories_brand(
                        nameplate_categories::dsl::name,
                        nameplate_categories::dsl::description,
                    ),
                    query,
                ),
            )
            .into_boxed();
        if let Some(permanence_id) = filter.and_then(|f| f.permanence_id) {
            query = query.filter(nameplate_categories::dsl::permanence_id.eq(permanence_id));
        }
        if let Some(material_id) = filter.and_then(|f| f.material_id) {
            query = query.filter(nameplate_categories::dsl::material_id.eq(material_id));
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(nameplate_categories::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(nameplate_categories::dsl::color_id.eq(color_id));
        }
        query
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn strict_word_similarity_search_with_score_viewable(
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<(Self, f32)>, web_common::api::ApiError> {
        use crate::database::schema::nameplate_categories;
        nameplate_categories::dsl::nameplate_categories
            .select((
                NameplateCategory::as_select(),
                crate::database::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::database::sql_function_bindings::concat_nameplate_categories_brand(
                        nameplate_categories::dsl::name,
                        nameplate_categories::dsl::description,
                    ),
                    query,
                ),
            ))
            .filter(
                crate::database::sql_function_bindings::concat_nameplate_categories_brand(
                    nameplate_categories::dsl::name,
                    nameplate_categories::dsl::description,
                )
                .strict_word_similarity_commutator_op(query)
                .or(
                    crate::database::sql_function_bindings::concat_nameplate_categories_brand(
                        nameplate_categories::dsl::name,
                        nameplate_categories::dsl::description,
                    )
                    .ilike(format!("%{}%", query)),
                ),
            )
            .order(
                crate::database::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::database::sql_function_bindings::concat_nameplate_categories_brand(
                        nameplate_categories::dsl::name,
                        nameplate_categories::dsl::description,
                    ),
                    query,
                ),
            )
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<(Self, f32)>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    pub fn can_update(&self) -> Result<bool, web_common::api::ApiError> {
        Ok(false)
    }
    /// Check whether the user can update the struct associated to the provided ids.
    pub fn can_update_by_id() -> Result<bool, web_common::api::ApiError> {
        Ok(false)
    }
    /// Check whether the user can admin the struct.
    pub fn can_admin(&self) -> Result<bool, web_common::api::ApiError> {
        Ok(false)
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    pub fn can_admin_by_id() -> Result<bool, web_common::api::ApiError> {
        Ok(false)
    }
}
