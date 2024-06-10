use super::*;
use crate::database::*;
use std::rc::Rc;
use web_common::database::filter_structs::*;

#[derive(
    Eq, PartialEq, PartialOrd, Debug, Clone, serde::Serialize, serde::Deserialize, Default,
)]
pub struct NestedBioOttTaxonItem {
    pub inner: BioOttTaxonItem,
    pub ott_rank: NestedBioOttRank,
    pub domain: Option<BioOttTaxonItem>,
    pub kingdom: Option<BioOttTaxonItem>,
    pub phylum: Option<BioOttTaxonItem>,
    pub class: Option<BioOttTaxonItem>,
    pub order: Option<BioOttTaxonItem>,
    pub family: Option<BioOttTaxonItem>,
    pub genus: Option<BioOttTaxonItem>,
    pub parent: BioOttTaxonItem,
    pub icon: FontAwesomeIcon,
    pub color: Color,
}

unsafe impl Send for NestedBioOttTaxonItem {}
unsafe impl Sync for NestedBioOttTaxonItem {}
impl NestedBioOttTaxonItem {
    /// Convert the flat struct to the nested struct.
    ///
    /// # Arguments
    /// * `flat_variant` - The flat struct.
    /// * `connection` - The database connection.
    pub fn from_flat(
        flat_variant: BioOttTaxonItem,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::prelude::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        Ok(Self {
            ott_rank: NestedBioOttRank::get(flat_variant.ott_rank_id, connection)?,
            domain: flat_variant
                .domain_id
                .map(|domain_id| BioOttTaxonItem::get(domain_id, connection))
                .transpose()?,
            kingdom: flat_variant
                .kingdom_id
                .map(|kingdom_id| BioOttTaxonItem::get(kingdom_id, connection))
                .transpose()?,
            phylum: flat_variant
                .phylum_id
                .map(|phylum_id| BioOttTaxonItem::get(phylum_id, connection))
                .transpose()?,
            class: flat_variant
                .class_id
                .map(|class_id| BioOttTaxonItem::get(class_id, connection))
                .transpose()?,
            order: flat_variant
                .order_id
                .map(|order_id| BioOttTaxonItem::get(order_id, connection))
                .transpose()?,
            family: flat_variant
                .family_id
                .map(|family_id| BioOttTaxonItem::get(family_id, connection))
                .transpose()?,
            genus: flat_variant
                .genus_id
                .map(|genus_id| BioOttTaxonItem::get(genus_id, connection))
                .transpose()?,
            parent: BioOttTaxonItem::get(flat_variant.parent_id, connection)?,
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
        BioOttTaxonItem::can_view_by_id()
    }
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_viewable(
        filter: Option<&BioOttTaxonItemFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        BioOttTaxonItem::all_viewable(filter, limit, offset, connection)?
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
        filter: Option<&BioOttTaxonItemFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        BioOttTaxonItem::all_viewable_sorted(filter, limit, offset, connection)?
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
        BioOttTaxonItem::get(id, connection)
            .and_then(|flat_variant| Self::from_flat(flat_variant, connection))
    }
    /// Get the struct from the database by its ott_id.
    ///
    /// * `ott_id` - The ott_id of the struct to get.
    /// * `connection` - The connection to the database.
    pub fn from_ott_id(
        ott_id: &i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        BioOttTaxonItem::from_ott_id(ott_id, connection)
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
        filter: Option<&BioOttTaxonItemFilter>,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        BioOttTaxonItem::strict_word_similarity_search_viewable(
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
        BioOttTaxonItem::strict_word_similarity_search_with_score_viewable(
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
        BioOttTaxonItem::can_update_by_id()
    }
    /// Check whether the user can admin the struct.
    pub fn can_admin(&self) -> Result<bool, web_common::api::ApiError> {
        self.inner.can_admin()
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    pub fn can_admin_by_id() -> Result<bool, web_common::api::ApiError> {
        BioOttTaxonItem::can_admin_by_id()
    }
}
impl From<web_common::database::nested_variants::NestedBioOttTaxonItem> for NestedBioOttTaxonItem {
    fn from(item: web_common::database::nested_variants::NestedBioOttTaxonItem) -> Self {
        Self {
            inner: BioOttTaxonItem::from(item.inner.as_ref().clone()),
            ott_rank: NestedBioOttRank::from(item.ott_rank.as_ref().clone()),
            domain: item.domain.as_deref().cloned().map(BioOttTaxonItem::from),
            kingdom: item.kingdom.as_deref().cloned().map(BioOttTaxonItem::from),
            phylum: item.phylum.as_deref().cloned().map(BioOttTaxonItem::from),
            class: item.class.as_deref().cloned().map(BioOttTaxonItem::from),
            order: item.order.as_deref().cloned().map(BioOttTaxonItem::from),
            family: item.family.as_deref().cloned().map(BioOttTaxonItem::from),
            genus: item.genus.as_deref().cloned().map(BioOttTaxonItem::from),
            parent: BioOttTaxonItem::from(item.parent.as_ref().clone()),
            icon: FontAwesomeIcon::from(item.icon.as_ref().clone()),
            color: Color::from(item.color.as_ref().clone()),
        }
    }
}
impl From<NestedBioOttTaxonItem> for web_common::database::nested_variants::NestedBioOttTaxonItem {
    fn from(item: NestedBioOttTaxonItem) -> Self {
        Self {
            inner: Rc::from(web_common::database::BioOttTaxonItem::from(item.inner)),
            ott_rank: Rc::from(web_common::database::NestedBioOttRank::from(item.ott_rank)),
            domain: item
                .domain
                .map(web_common::database::BioOttTaxonItem::from)
                .map(Rc::from),
            kingdom: item
                .kingdom
                .map(web_common::database::BioOttTaxonItem::from)
                .map(Rc::from),
            phylum: item
                .phylum
                .map(web_common::database::BioOttTaxonItem::from)
                .map(Rc::from),
            class: item
                .class
                .map(web_common::database::BioOttTaxonItem::from)
                .map(Rc::from),
            order: item
                .order
                .map(web_common::database::BioOttTaxonItem::from)
                .map(Rc::from),
            family: item
                .family
                .map(web_common::database::BioOttTaxonItem::from)
                .map(Rc::from),
            genus: item
                .genus
                .map(web_common::database::BioOttTaxonItem::from)
                .map(Rc::from),
            parent: Rc::from(web_common::database::BioOttTaxonItem::from(item.parent)),
            icon: Rc::from(web_common::database::FontAwesomeIcon::from(item.icon)),
            color: Rc::from(web_common::database::Color::from(item.color)),
        }
    }
}
