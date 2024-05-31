//! This file is generated automatically and should not be modified.
//!
//! Any edits you may apply to this document will be overwritten next time the
//! backend is generated. Refrain from making any changes to this file.

//! If you need to make changes to the backend, please modify the `generate_models`
//! document in the `migrations` folder.

use diesel::Queryable;
use diesel::QueryableByName;
use diesel::Identifiable;
use diesel::Insertable;
use crate::schema::*;
use crate::sql_function_bindings::*;
use diesel::Selectable;
use serde::Deserialize;
use serde::Serialize;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use diesel::prelude::*;
use web_common::database::filter_structs::*;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = materials)]
#[diesel(belongs_to(crate::models::font_awesome_icons::FontAwesomeIcon, foreign_key = icon_id))]
#[diesel(belongs_to(crate::models::colors::Color, foreign_key = color_id))]
#[diesel(primary_key(id))]
pub struct Material {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}

impl From<Material> for web_common::database::tables::Material {
    fn from(item: Material) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl From<web_common::database::tables::Material> for Material {
    fn from(item: web_common::database::tables::Material) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl Material {
    /// Check whether the user can view the struct.
    pub fn can_view(
        &self,
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id(
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&MaterialFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::materials;
        let mut query = materials::dsl::materials
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(materials::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(materials::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&MaterialFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::materials;
        let mut query = materials::dsl::materials
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(materials::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(materials::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::materials;
        materials::dsl::materials
            .filter(materials::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its name.
    ///
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
name: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::materials;
        let flat_variant = materials::dsl::materials
            .filter(materials::dsl::name.eq(name))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
}
