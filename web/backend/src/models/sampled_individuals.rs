//! This file is generated automatically and should not be modified.
//!
//! Any edits you may apply to this document will be overwritten next time the
//! backend is generated. Refrain from making any changes to this file.

//! If you need to make changes to the backend, please modify the `generate_models`
//! document in the `migrations` folder.

use crate::schema::*;
use crate::sql_function_bindings::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use diesel::Identifiable;
use diesel::Insertable;
use diesel::Queryable;
use diesel::QueryableByName;
use diesel::Selectable;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
use web_common::database::filter_structs::*;

#[derive(
    Queryable,
    Debug,
    Identifiable,
    Eq,
    PartialEq,
    Clone,
    Serialize,
    Deserialize,
    Default,
    QueryableByName,
    Associations,
    Insertable,
    Selectable,
    AsChangeset,
)]
#[diesel(table_name = sampled_individuals)]
#[diesel(belongs_to(crate::models::nameplates::Nameplate, foreign_key = nameplate_id))]
#[diesel(belongs_to(crate::models::projects::Project, foreign_key = project_id))]
#[diesel(belongs_to(crate::models::users::User, foreign_key = created_by))]
#[diesel(primary_key(id))]
pub struct SampledIndividual {
    pub id: Uuid,
    pub notes: Option<String>,
    pub nameplate_id: Option<i32>,
    pub project_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
    pub picture: Vec<u8>,
}

impl From<SampledIndividual> for web_common::database::tables::SampledIndividual {
    fn from(item: SampledIndividual) -> Self {
        Self {
            id: item.id,
            notes: item.notes,
            nameplate_id: item.nameplate_id,
            project_id: item.project_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            picture: item.picture,
        }
    }
}

impl From<web_common::database::tables::SampledIndividual> for SampledIndividual {
    fn from(item: web_common::database::tables::SampledIndividual) -> Self {
        Self {
            id: item.id,
            notes: item.notes,
            nameplate_id: item.nameplate_id,
            project_id: item.project_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            picture: item.picture,
        }
    }
}

impl SampledIndividual {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_view(
        &self,
        author_user_id: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(self.id, author_user_id, connection)
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_view_by_id(
        id: Uuid,
        author_user_id: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(can_view_sampled_individuals(author_user_id, id))
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
        filter: Option<&SampledIndividualFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::sampled_individuals;
        let mut query = sampled_individuals::dsl::sampled_individuals.into_boxed();
        if let Some(nameplate_id) = filter.and_then(|f| f.nameplate_id) {
            query = query.filter(sampled_individuals::dsl::nameplate_id.eq(nameplate_id));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(sampled_individuals::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sampled_individuals::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(sampled_individuals::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_view_sampled_individuals(
                author_user_id,
                sampled_individuals::dsl::id,
            ))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
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
        filter: Option<&SampledIndividualFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::sampled_individuals;
        let mut query = sampled_individuals::dsl::sampled_individuals.into_boxed();
        if let Some(nameplate_id) = filter.and_then(|f| f.nameplate_id) {
            query = query.filter(sampled_individuals::dsl::nameplate_id.eq(nameplate_id));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(sampled_individuals::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sampled_individuals::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(sampled_individuals::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_view_sampled_individuals(
                author_user_id,
                sampled_individuals::dsl::id,
            ))
            .order_by(sampled_individuals::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    pub fn get(
        id: Uuid,
        author_user_id: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Self, web_common::api::ApiError> {
        if !Self::can_view_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::sampled_individuals;
        sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::id.eq(id))
            .first::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its nameplate_id.
    ///
    /// * `nameplate_id` - The nameplate_id of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    pub fn from_nameplate_id(
        nameplate_id: Option<&i32>,
        author_user_id: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Self, web_common::api::ApiError> {
        use crate::schema::sampled_individuals;
        let flat_variant = sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::nameplate_id.eq(nameplate_id))
            .first::<Self>(connection)?;
        if !flat_variant.can_view(author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        Ok(flat_variant)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn similarity_search_viewable(
        filter: Option<&SampledIndividualFilter>,
        author_user_id: Option<i32>,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sampled_individuals;
        if filter
            .map(|f| f.project_id.is_some() && f.created_by.is_some() && f.updated_by.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::project_id.eq(project_id))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_view_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    similarity_op(nameplates::dsl::barcode, query).or(similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
                )
                .order(
                    similarity_dist(nameplates::dsl::barcode, query)
                        + similarity_dist(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::created_by.eq(created_by))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_view_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    similarity_op(nameplates::dsl::barcode, query).or(similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
                )
                .order(
                    similarity_dist(nameplates::dsl::barcode, query)
                        + similarity_dist(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::updated_by.eq(updated_by))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_view_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    similarity_op(nameplates::dsl::barcode, query).or(similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
                )
                .order(
                    similarity_dist(nameplates::dsl::barcode, query)
                        + similarity_dist(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        sampled_individuals::dsl::sampled_individuals
            .select(SampledIndividual::as_select())
            // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
            .left_join(
                nameplates::dsl::nameplates
                    .on(sampled_individuals::dsl::nameplate_id.eq(nameplates::dsl::id.nullable())),
            )
            // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
            .inner_join(
                projects::dsl::projects
                    .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
            )
            .filter(sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)))
            .filter(can_view_sampled_individuals(
                author_user_id,
                sampled_individuals::dsl::id,
            ))
            .filter(
                similarity_op(nameplates::dsl::barcode, query).or(similarity_op(
                    concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    ),
                    query,
                )),
            )
            .order(
                similarity_dist(nameplates::dsl::barcode, query)
                    + similarity_dist(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
            )
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn word_similarity_search_viewable(
        filter: Option<&SampledIndividualFilter>,
        author_user_id: Option<i32>,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sampled_individuals;
        if filter
            .map(|f| f.project_id.is_some() && f.created_by.is_some() && f.updated_by.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::project_id.eq(project_id))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_view_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    word_similarity_op(nameplates::dsl::barcode, query).or(word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
                )
                .order(
                    word_similarity_dist_op(nameplates::dsl::barcode, query)
                        + word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::created_by.eq(created_by))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_view_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    word_similarity_op(nameplates::dsl::barcode, query).or(word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
                )
                .order(
                    word_similarity_dist_op(nameplates::dsl::barcode, query)
                        + word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::updated_by.eq(updated_by))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_view_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    word_similarity_op(nameplates::dsl::barcode, query).or(word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
                )
                .order(
                    word_similarity_dist_op(nameplates::dsl::barcode, query)
                        + word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        sampled_individuals::dsl::sampled_individuals
            .select(SampledIndividual::as_select())
            // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
            .left_join(
                nameplates::dsl::nameplates
                    .on(sampled_individuals::dsl::nameplate_id.eq(nameplates::dsl::id.nullable())),
            )
            // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
            .inner_join(
                projects::dsl::projects
                    .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
            )
            .filter(sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)))
            .filter(can_view_sampled_individuals(
                author_user_id,
                sampled_individuals::dsl::id,
            ))
            .filter(
                word_similarity_op(nameplates::dsl::barcode, query).or(word_similarity_op(
                    concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    ),
                    query,
                )),
            )
            .order(
                word_similarity_dist_op(nameplates::dsl::barcode, query)
                    + word_similarity_dist_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
            )
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn strict_word_similarity_search_viewable(
        filter: Option<&SampledIndividualFilter>,
        author_user_id: Option<i32>,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sampled_individuals;
        if filter
            .map(|f| f.project_id.is_some() && f.created_by.is_some() && f.updated_by.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::project_id.eq(project_id))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_view_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    strict_word_similarity_op(nameplates::dsl::barcode, query).or(
                        strict_word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                    ),
                )
                .order(
                    strict_word_similarity_dist_op(nameplates::dsl::barcode, query)
                        + strict_word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::created_by.eq(created_by))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_view_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    strict_word_similarity_op(nameplates::dsl::barcode, query).or(
                        strict_word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                    ),
                )
                .order(
                    strict_word_similarity_dist_op(nameplates::dsl::barcode, query)
                        + strict_word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::updated_by.eq(updated_by))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_view_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    strict_word_similarity_op(nameplates::dsl::barcode, query).or(
                        strict_word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                    ),
                )
                .order(
                    strict_word_similarity_dist_op(nameplates::dsl::barcode, query)
                        + strict_word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        sampled_individuals::dsl::sampled_individuals
            .select(SampledIndividual::as_select())
            // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
            .left_join(
                nameplates::dsl::nameplates
                    .on(sampled_individuals::dsl::nameplate_id.eq(nameplates::dsl::id.nullable())),
            )
            // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
            .inner_join(
                projects::dsl::projects
                    .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
            )
            .filter(sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)))
            .filter(can_view_sampled_individuals(
                author_user_id,
                sampled_individuals::dsl::id,
            ))
            .filter(
                strict_word_similarity_op(nameplates::dsl::barcode, query).or(
                    strict_word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                ),
            )
            .order(
                strict_word_similarity_dist_op(nameplates::dsl::barcode, query)
                    + strict_word_similarity_dist_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
            )
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_update(
        &self,
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(self.id, author_user_id, connection)
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_update_by_id(
        id: Uuid,
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(can_update_sampled_individuals(author_user_id, id))
            .get_result(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_updatable(
        filter: Option<&SampledIndividualFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::sampled_individuals;
        let mut query = sampled_individuals::dsl::sampled_individuals.into_boxed();
        if let Some(nameplate_id) = filter.and_then(|f| f.nameplate_id) {
            query = query.filter(sampled_individuals::dsl::nameplate_id.eq(nameplate_id));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(sampled_individuals::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sampled_individuals::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(sampled_individuals::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_update_sampled_individuals(
                author_user_id,
                sampled_individuals::dsl::id,
            ))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
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
        filter: Option<&SampledIndividualFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::sampled_individuals;
        let mut query = sampled_individuals::dsl::sampled_individuals.into_boxed();
        if let Some(nameplate_id) = filter.and_then(|f| f.nameplate_id) {
            query = query.filter(sampled_individuals::dsl::nameplate_id.eq(nameplate_id));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(sampled_individuals::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sampled_individuals::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(sampled_individuals::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_update_sampled_individuals(
                author_user_id,
                sampled_individuals::dsl::id,
            ))
            .order_by(sampled_individuals::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the updatable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn similarity_search_updatable(
        filter: Option<&SampledIndividualFilter>,
        author_user_id: i32,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sampled_individuals;
        if filter
            .map(|f| f.project_id.is_some() && f.created_by.is_some() && f.updated_by.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::project_id.eq(project_id))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_update_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    similarity_op(nameplates::dsl::barcode, query).or(similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
                )
                .order(
                    similarity_dist(nameplates::dsl::barcode, query)
                        + similarity_dist(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::created_by.eq(created_by))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_update_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    similarity_op(nameplates::dsl::barcode, query).or(similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
                )
                .order(
                    similarity_dist(nameplates::dsl::barcode, query)
                        + similarity_dist(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::updated_by.eq(updated_by))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_update_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    similarity_op(nameplates::dsl::barcode, query).or(similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
                )
                .order(
                    similarity_dist(nameplates::dsl::barcode, query)
                        + similarity_dist(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        sampled_individuals::dsl::sampled_individuals
            .select(SampledIndividual::as_select())
            // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
            .left_join(
                nameplates::dsl::nameplates
                    .on(sampled_individuals::dsl::nameplate_id.eq(nameplates::dsl::id.nullable())),
            )
            // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
            .inner_join(
                projects::dsl::projects
                    .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
            )
            .filter(sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)))
            .filter(can_update_sampled_individuals(
                author_user_id,
                sampled_individuals::dsl::id,
            ))
            .filter(
                similarity_op(nameplates::dsl::barcode, query).or(similarity_op(
                    concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    ),
                    query,
                )),
            )
            .order(
                similarity_dist(nameplates::dsl::barcode, query)
                    + similarity_dist(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
            )
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the updatable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn word_similarity_search_updatable(
        filter: Option<&SampledIndividualFilter>,
        author_user_id: i32,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sampled_individuals;
        if filter
            .map(|f| f.project_id.is_some() && f.created_by.is_some() && f.updated_by.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::project_id.eq(project_id))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_update_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    word_similarity_op(nameplates::dsl::barcode, query).or(word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
                )
                .order(
                    word_similarity_dist_op(nameplates::dsl::barcode, query)
                        + word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::created_by.eq(created_by))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_update_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    word_similarity_op(nameplates::dsl::barcode, query).or(word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
                )
                .order(
                    word_similarity_dist_op(nameplates::dsl::barcode, query)
                        + word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::updated_by.eq(updated_by))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_update_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    word_similarity_op(nameplates::dsl::barcode, query).or(word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
                )
                .order(
                    word_similarity_dist_op(nameplates::dsl::barcode, query)
                        + word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        sampled_individuals::dsl::sampled_individuals
            .select(SampledIndividual::as_select())
            // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
            .left_join(
                nameplates::dsl::nameplates
                    .on(sampled_individuals::dsl::nameplate_id.eq(nameplates::dsl::id.nullable())),
            )
            // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
            .inner_join(
                projects::dsl::projects
                    .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
            )
            .filter(sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)))
            .filter(can_update_sampled_individuals(
                author_user_id,
                sampled_individuals::dsl::id,
            ))
            .filter(
                word_similarity_op(nameplates::dsl::barcode, query).or(word_similarity_op(
                    concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    ),
                    query,
                )),
            )
            .order(
                word_similarity_dist_op(nameplates::dsl::barcode, query)
                    + word_similarity_dist_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
            )
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the updatable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn strict_word_similarity_search_updatable(
        filter: Option<&SampledIndividualFilter>,
        author_user_id: i32,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sampled_individuals;
        if filter
            .map(|f| f.project_id.is_some() && f.created_by.is_some() && f.updated_by.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::project_id.eq(project_id))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_update_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    strict_word_similarity_op(nameplates::dsl::barcode, query).or(
                        strict_word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                    ),
                )
                .order(
                    strict_word_similarity_dist_op(nameplates::dsl::barcode, query)
                        + strict_word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::created_by.eq(created_by))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_update_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    strict_word_similarity_op(nameplates::dsl::barcode, query).or(
                        strict_word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                    ),
                )
                .order(
                    strict_word_similarity_dist_op(nameplates::dsl::barcode, query)
                        + strict_word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::updated_by.eq(updated_by))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_update_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    strict_word_similarity_op(nameplates::dsl::barcode, query).or(
                        strict_word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                    ),
                )
                .order(
                    strict_word_similarity_dist_op(nameplates::dsl::barcode, query)
                        + strict_word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        sampled_individuals::dsl::sampled_individuals
            .select(SampledIndividual::as_select())
            // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
            .left_join(
                nameplates::dsl::nameplates
                    .on(sampled_individuals::dsl::nameplate_id.eq(nameplates::dsl::id.nullable())),
            )
            // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
            .inner_join(
                projects::dsl::projects
                    .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
            )
            .filter(sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)))
            .filter(can_update_sampled_individuals(
                author_user_id,
                sampled_individuals::dsl::id,
            ))
            .filter(
                strict_word_similarity_op(nameplates::dsl::barcode, query).or(
                    strict_word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                ),
            )
            .order(
                strict_word_similarity_dist_op(nameplates::dsl::barcode, query)
                    + strict_word_similarity_dist_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
            )
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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(self.id, author_user_id, connection)
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_admin_by_id(
        id: Uuid,
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(can_admin_sampled_individuals(author_user_id, id))
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
        filter: Option<&SampledIndividualFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::sampled_individuals;
        let mut query = sampled_individuals::dsl::sampled_individuals.into_boxed();
        if let Some(nameplate_id) = filter.and_then(|f| f.nameplate_id) {
            query = query.filter(sampled_individuals::dsl::nameplate_id.eq(nameplate_id));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(sampled_individuals::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sampled_individuals::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(sampled_individuals::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_admin_sampled_individuals(
                author_user_id,
                sampled_individuals::dsl::id,
            ))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
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
        filter: Option<&SampledIndividualFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::sampled_individuals;
        let mut query = sampled_individuals::dsl::sampled_individuals.into_boxed();
        if let Some(nameplate_id) = filter.and_then(|f| f.nameplate_id) {
            query = query.filter(sampled_individuals::dsl::nameplate_id.eq(nameplate_id));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(sampled_individuals::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sampled_individuals::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(sampled_individuals::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_admin_sampled_individuals(
                author_user_id,
                sampled_individuals::dsl::id,
            ))
            .order_by(sampled_individuals::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the administrable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn similarity_search_administrable(
        filter: Option<&SampledIndividualFilter>,
        author_user_id: i32,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sampled_individuals;
        if filter
            .map(|f| f.project_id.is_some() && f.created_by.is_some() && f.updated_by.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::project_id.eq(project_id))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_admin_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    similarity_op(nameplates::dsl::barcode, query).or(similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
                )
                .order(
                    similarity_dist(nameplates::dsl::barcode, query)
                        + similarity_dist(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::created_by.eq(created_by))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_admin_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    similarity_op(nameplates::dsl::barcode, query).or(similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
                )
                .order(
                    similarity_dist(nameplates::dsl::barcode, query)
                        + similarity_dist(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::updated_by.eq(updated_by))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_admin_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    similarity_op(nameplates::dsl::barcode, query).or(similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
                )
                .order(
                    similarity_dist(nameplates::dsl::barcode, query)
                        + similarity_dist(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        sampled_individuals::dsl::sampled_individuals
            .select(SampledIndividual::as_select())
            // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
            .left_join(
                nameplates::dsl::nameplates
                    .on(sampled_individuals::dsl::nameplate_id.eq(nameplates::dsl::id.nullable())),
            )
            // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
            .inner_join(
                projects::dsl::projects
                    .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
            )
            .filter(sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)))
            .filter(can_admin_sampled_individuals(
                author_user_id,
                sampled_individuals::dsl::id,
            ))
            .filter(
                similarity_op(nameplates::dsl::barcode, query).or(similarity_op(
                    concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    ),
                    query,
                )),
            )
            .order(
                similarity_dist(nameplates::dsl::barcode, query)
                    + similarity_dist(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
            )
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the administrable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn word_similarity_search_administrable(
        filter: Option<&SampledIndividualFilter>,
        author_user_id: i32,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sampled_individuals;
        if filter
            .map(|f| f.project_id.is_some() && f.created_by.is_some() && f.updated_by.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::project_id.eq(project_id))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_admin_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    word_similarity_op(nameplates::dsl::barcode, query).or(word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
                )
                .order(
                    word_similarity_dist_op(nameplates::dsl::barcode, query)
                        + word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::created_by.eq(created_by))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_admin_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    word_similarity_op(nameplates::dsl::barcode, query).or(word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
                )
                .order(
                    word_similarity_dist_op(nameplates::dsl::barcode, query)
                        + word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::updated_by.eq(updated_by))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_admin_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    word_similarity_op(nameplates::dsl::barcode, query).or(word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
                )
                .order(
                    word_similarity_dist_op(nameplates::dsl::barcode, query)
                        + word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        sampled_individuals::dsl::sampled_individuals
            .select(SampledIndividual::as_select())
            // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
            .left_join(
                nameplates::dsl::nameplates
                    .on(sampled_individuals::dsl::nameplate_id.eq(nameplates::dsl::id.nullable())),
            )
            // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
            .inner_join(
                projects::dsl::projects
                    .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
            )
            .filter(sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)))
            .filter(can_admin_sampled_individuals(
                author_user_id,
                sampled_individuals::dsl::id,
            ))
            .filter(
                word_similarity_op(nameplates::dsl::barcode, query).or(word_similarity_op(
                    concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    ),
                    query,
                )),
            )
            .order(
                word_similarity_dist_op(nameplates::dsl::barcode, query)
                    + word_similarity_dist_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
            )
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the administrable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn strict_word_similarity_search_administrable(
        filter: Option<&SampledIndividualFilter>,
        author_user_id: i32,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sampled_individuals;
        if filter
            .map(|f| f.project_id.is_some() && f.created_by.is_some() && f.updated_by.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::project_id.eq(project_id))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_admin_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    strict_word_similarity_op(nameplates::dsl::barcode, query).or(
                        strict_word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                    ),
                )
                .order(
                    strict_word_similarity_dist_op(nameplates::dsl::barcode, query)
                        + strict_word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::created_by.eq(created_by))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_admin_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    strict_word_similarity_op(nameplates::dsl::barcode, query).or(
                        strict_word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                    ),
                )
                .order(
                    strict_word_similarity_dist_op(nameplates::dsl::barcode, query)
                        + strict_word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            return sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::updated_by.eq(updated_by))
                .select(SampledIndividual::as_select())
                // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
                .left_join(
                    nameplates::dsl::nameplates
                        .on(sampled_individuals::dsl::nameplate_id
                            .eq(nameplates::dsl::id.nullable())),
                )
                // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
                .inner_join(
                    projects::dsl::projects
                        .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
                )
                .filter(
                    sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)),
                )
                .filter(can_admin_sampled_individuals(
                    author_user_id,
                    sampled_individuals::dsl::id,
                ))
                .filter(
                    strict_word_similarity_op(nameplates::dsl::barcode, query).or(
                        strict_word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                    ),
                )
                .order(
                    strict_word_similarity_dist_op(nameplates::dsl::barcode, query)
                        + strict_word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        sampled_individuals::dsl::sampled_individuals
            .select(SampledIndividual::as_select())
            // This operation is defined by a first order index linking sampled_individuals.nameplate_id to nameplates.
            .left_join(
                nameplates::dsl::nameplates
                    .on(sampled_individuals::dsl::nameplate_id.eq(nameplates::dsl::id.nullable())),
            )
            // This operation is defined by a first order index linking sampled_individuals.project_id to projects.
            .inner_join(
                projects::dsl::projects
                    .on(sampled_individuals::dsl::project_id.eq(projects::dsl::id)),
            )
            .filter(sampled_individuals::dsl::nameplate_id.eq(filter.and_then(|f| f.nameplate_id)))
            .filter(can_admin_sampled_individuals(
                author_user_id,
                sampled_individuals::dsl::id,
            ))
            .filter(
                strict_word_similarity_op(nameplates::dsl::barcode, query).or(
                    strict_word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                ),
            )
            .order(
                strict_word_similarity_dist_op(nameplates::dsl::barcode, query)
                    + strict_word_similarity_dist_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
            )
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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<usize, web_common::api::ApiError> {
        Self::delete_by_id(self.id, author_user_id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    pub fn delete_by_id(
        id: Uuid,
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<usize, web_common::api::ApiError> {
        if !Self::can_admin_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(
            sampled_individuals::dsl::sampled_individuals
                .filter(sampled_individuals::dsl::id.eq(id)),
        )
        .execute(connection)
        .map_err(web_common::api::ApiError::from)
    }
}