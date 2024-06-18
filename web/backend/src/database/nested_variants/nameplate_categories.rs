use crate::database::*;
use std::rc::Rc;

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct NestedNameplateCategory {
    pub inner: crate::database::flat_variants::NameplateCategory,
    pub permanence: crate::database::nested_variants::NestedPermanenceCategory,
    pub material: crate::database::nested_variants::NestedMaterial,
    pub icon: crate::database::flat_variants::FontAwesomeIcon,
    pub color: crate::database::flat_variants::Color,
}

unsafe impl Send for NestedNameplateCategory {}
unsafe impl Sync for NestedNameplateCategory {}
impl NestedNameplateCategory {
    /// Convert the flat struct to the nested struct.
    ///
    /// # Arguments
    /// * `flat_variant` - The flat struct.
    /// * `connection` - The database connection.
    fn from_flat(
        flat_variant: NameplateCategory,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::prelude::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        Ok(Self {
            permanence: crate::database::nested_variants::NestedPermanenceCategory::get(
                flat_variant.permanence_id,
                connection,
            )?,
            material: crate::database::nested_variants::NestedMaterial::get(
                flat_variant.material_id,
                connection,
            )?,
            icon: crate::database::flat_variants::FontAwesomeIcon::get(
                flat_variant.icon_id,
                connection,
            )?,
            color: crate::database::flat_variants::Color::get(flat_variant.color_id, connection)?,
            inner: flat_variant,
        })
    }
    /// Check whether the user can view the struct.
    pub fn can_view(&self) -> Result<bool, web_common::api::ApiError> {
        self.inner.can_view()
    }
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id() -> Result<bool, web_common::api::ApiError> {
        NameplateCategory::can_view_by_id()
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
        NameplateCategory::all_viewable(filter, limit, offset, connection)?
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
        filter: Option<&web_common::database::filter_variants::NameplateCategoryFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        NameplateCategory::all_viewable_sorted(filter, limit, offset, connection)?
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
        NameplateCategory::get(id, connection)
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
        filter: Option<&web_common::database::filter_variants::NameplateCategoryFilter>,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        NameplateCategory::strict_word_similarity_search_viewable(
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
        NameplateCategory::strict_word_similarity_search_with_score_viewable(
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
        NameplateCategory::can_update_by_id()
    }
    /// Check whether the user can admin the struct.
    pub fn can_admin(&self) -> Result<bool, web_common::api::ApiError> {
        self.inner.can_admin()
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    pub fn can_admin_by_id() -> Result<bool, web_common::api::ApiError> {
        NameplateCategory::can_admin_by_id()
    }
}
impl From<web_common::database::nested_variants::NestedNameplateCategory>
    for crate::database::nested_variants::NestedNameplateCategory
{
    fn from(item: web_common::database::nested_variants::NestedNameplateCategory) -> Self {
        Self {
            inner: crate::database::flat_variants::NameplateCategory::from(
                item.inner.as_ref().clone(),
            ),
            permanence: crate::database::nested_variants::NestedPermanenceCategory::from(
                item.permanence.as_ref().clone(),
            ),
            material: crate::database::nested_variants::NestedMaterial::from(
                item.material.as_ref().clone(),
            ),
            icon: crate::database::flat_variants::FontAwesomeIcon::from(item.icon.as_ref().clone()),
            color: crate::database::flat_variants::Color::from(item.color.as_ref().clone()),
        }
    }
}
impl From<crate::database::nested_variants::NestedNameplateCategory>
    for web_common::database::nested_variants::NestedNameplateCategory
{
    fn from(item: crate::database::nested_variants::NestedNameplateCategory) -> Self {
        Self {
            inner: Rc::from(
                web_common::database::flat_variants::NameplateCategory::from(item.inner),
            ),
            permanence: Rc::from(
                web_common::database::nested_variants::NestedPermanenceCategory::from(
                    item.permanence,
                ),
            ),
            material: Rc::from(web_common::database::nested_variants::NestedMaterial::from(
                item.material,
            )),
            icon: Rc::from(web_common::database::flat_variants::FontAwesomeIcon::from(
                item.icon,
            )),
            color: Rc::from(web_common::database::flat_variants::Color::from(item.color)),
        }
    }
}
