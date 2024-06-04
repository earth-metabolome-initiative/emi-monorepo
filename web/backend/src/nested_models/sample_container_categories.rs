use super::*;
use crate::models::*;
use serde::Deserialize;
use serde::Serialize;
use std::rc::Rc;
use web_common::database::filter_structs::*;

#[derive(PartialEq, PartialOrd, Debug, Clone, Serialize, Deserialize, Default)]
pub struct NestedSampleContainerCategory {
    pub inner: SampleContainerCategory,
    pub material: NestedMaterial,
    pub icon: FontAwesomeIcon,
    pub color: Color,
}

impl NestedSampleContainerCategory {
    /// Convert the flat struct to the nested struct.
    ///
    /// # Arguments
    /// * `flat_variant` - The flat struct.
    /// * `connection` - The database connection.
    pub fn from_flat(
        flat_variant: SampleContainerCategory,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::prelude::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        Ok(Self {
            material: NestedMaterial::get(flat_variant.material_id, connection)?,
            icon: FontAwesomeIcon::get(flat_variant.icon_id, connection)?,
            color: Color::get(flat_variant.color_id, connection)?,
            inner: flat_variant,
        })
    }
    /// Check whether the user can view the struct.
    pub fn can_view(&self) -> Result<bool, web_common::api::ApiError> {
        self.inner.can_view()
    }
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id() -> Result<bool, web_common::api::ApiError> {
        SampleContainerCategory::can_view_by_id()
    }
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_viewable(
        filter: Option<&SampleContainerCategoryFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        SampleContainerCategory::all_viewable(filter, limit, offset, connection)?
            .into_iter()
            .map(|flat_variant| Self::from_flat(flat_variant, connection))
            .collect()
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_viewable_sorted(
        filter: Option<&SampleContainerCategoryFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        SampleContainerCategory::all_viewable_sorted(filter, limit, offset, connection)?
            .into_iter()
            .map(|flat_variant| Self::from_flat(flat_variant, connection))
            .collect()
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
        SampleContainerCategory::get(id, connection)
            .and_then(|flat_variant| Self::from_flat(flat_variant, connection))
    }
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn strict_word_similarity_search_viewable(
        filter: Option<&SampleContainerCategoryFilter>,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        SampleContainerCategory::strict_word_similarity_search_viewable(
            filter, query, limit, offset, connection,
        )?
        .into_iter()
        .map(|flat_variant| Self::from_flat(flat_variant, connection))
        .collect()
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
        SampleContainerCategory::strict_word_similarity_search_with_score_viewable(
            query, limit, offset, connection,
        )?
        .into_iter()
        .map(|(flat_variant, score)| Ok((Self::from_flat(flat_variant, connection)?, score)))
        .collect()
    }
    /// Check whether the user can update the struct.
    pub fn can_update(&self) -> Result<bool, web_common::api::ApiError> {
        self.inner.can_update()
    }
    /// Check whether the user can update the struct associated to the provided ids.
    pub fn can_update_by_id() -> Result<bool, web_common::api::ApiError> {
        SampleContainerCategory::can_update_by_id()
    }
    /// Check whether the user can admin the struct.
    pub fn can_admin(&self) -> Result<bool, web_common::api::ApiError> {
        self.inner.can_admin()
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    pub fn can_admin_by_id() -> Result<bool, web_common::api::ApiError> {
        SampleContainerCategory::can_admin_by_id()
    }
}
impl From<web_common::database::nested_models::NestedSampleContainerCategory>
    for NestedSampleContainerCategory
{
    fn from(item: web_common::database::nested_models::NestedSampleContainerCategory) -> Self {
        Self {
            inner: SampleContainerCategory::from(item.inner.as_ref().clone()),
            material: NestedMaterial::from(item.material.as_ref().clone()),
            icon: FontAwesomeIcon::from(item.icon.as_ref().clone()),
            color: Color::from(item.color.as_ref().clone()),
        }
    }
}
impl From<NestedSampleContainerCategory>
    for web_common::database::nested_models::NestedSampleContainerCategory
{
    fn from(item: NestedSampleContainerCategory) -> Self {
        Self {
            inner: Rc::from(web_common::database::SampleContainerCategory::from(
                item.inner,
            )),
            material: Rc::from(web_common::database::NestedMaterial::from(item.material)),
            icon: Rc::from(web_common::database::FontAwesomeIcon::from(item.icon)),
            color: Rc::from(web_common::database::Color::from(item.color)),
        }
    }
}
