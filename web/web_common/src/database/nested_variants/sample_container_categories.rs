//! This module contains the nested structs for the database tables.
//!
//! This file is automatically generated. Do not write anything here.

use std::rc::Rc;

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct NestedSampleContainerCategory {
    pub inner: Rc<crate::database::flat_variants::SampleContainerCategory>,
    pub material: Rc<crate::database::nested_variants::NestedMaterial>,
    pub icon: Rc<crate::database::flat_variants::FontAwesomeIcon>,
    pub color: Rc<crate::database::flat_variants::Color>,
}

unsafe impl Send for NestedSampleContainerCategory {}
unsafe impl Sync for NestedSampleContainerCategory {}
impl crate::database::Tabular for NestedSampleContainerCategory {
    const TABLE: crate::database::Table = crate::database::Table::SampleContainerCategories;
}
impl crate::database::Filtrable for NestedSampleContainerCategory {
    type Filter = crate::database::filter_variants::SampleContainerCategoryFilter;
}
impl crate::database::Describable for NestedSampleContainerCategory {
    fn description(&self) -> Option<&str> {
        self.inner.description()
    }
}
impl crate::database::Colorable for NestedSampleContainerCategory {
    fn color(&self) -> Option<&str> {
        Some(self.color.name.as_str())
    }
}
#[cfg(feature = "frontend")]
impl crate::database::AllRecords for NestedSampleContainerCategory {
    fn all_records<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&<Self as crate::database::Filtrable>::Filter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> impl std::future::Future<Output = Result<Vec<Self>, crate::api::ApiError>> {
        Self::all(filter, limit, offset, connection)
    }
}
#[cfg(feature = "frontend")]
impl NestedSampleContainerCategory {
    /// Convert the flat struct to the nested struct.
    ///
    /// # Arguments
    /// * `flat_variant` - The flat struct.
    /// * `connection` - The database connection.
    pub(crate) async fn from_flat(
        flat_variant: crate::database::flat_variants::SampleContainerCategory,
        connection: &mut gluesql::prelude::Glue<
            impl gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
        >,
    ) -> Result<Self, crate::api::ApiError> {
        Ok(Self {
            material: Rc::from(
                crate::database::nested_variants::NestedMaterial::get(
                    flat_variant.material_id,
                    connection,
                )
                .await?
                .unwrap(),
            ),
            icon: Rc::from(
                crate::database::flat_variants::FontAwesomeIcon::get(
                    flat_variant.icon_id,
                    connection,
                )
                .await?
                .unwrap(),
            ),
            color: Rc::from(
                crate::database::flat_variants::Color::get(flat_variant.color_id, connection)
                    .await?
                    .unwrap(),
            ),
            inner: Rc::from(flat_variant),
        })
    }
    /// Get the id attribute.
    pub fn get_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_id()
    }
    /// Get the name attribute.
    pub fn get_name<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.inner.get_name()
    }
    /// Get the volume attribute.
    pub fn get_volume<E>(&self) -> &E
    where
        f64: AsRef<E>,
    {
        self.inner.get_volume()
    }
    /// Get the unit attribute.
    pub fn get_unit<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.inner.get_unit()
    }
    /// Get the material_id attribute.
    pub fn get_material_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_material_id()
    }
    /// Get the description attribute.
    pub fn get_description<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.inner.get_description()
    }
    /// Get the icon_id attribute.
    pub fn get_icon_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_icon_id()
    }
    /// Get the color_id attribute.
    pub fn get_color_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_color_id()
    }
    /// Insert the SampleContainerCategory into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        self.inner.as_ref().clone().insert(connection).await
    }
    /// Get the SampleContainerCategory from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `connection` - The connection to the database.
    pub async fn get<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, crate::api::ApiError> {
        if let Some(flat_variant) =
            crate::database::flat_variants::SampleContainerCategory::get(id, connection).await?
        {
            Ok(Some(Self::from_flat(flat_variant, connection).await?))
        } else {
            Ok(None)
        }
    }
    /// Delete the SampleContainerCategory from the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn delete<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        self.inner.as_ref().clone().delete(connection).await
    }
    /// Delete the SampleContainerCategory from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    pub async fn delete_from_id<
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    >(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        crate::database::flat_variants::SampleContainerCategory::delete_from_id(id, connection)
            .await
    }
    /// Update the struct in the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn update<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        self.inner.as_ref().clone().update(connection).await
    }
    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// * `connection` - The connection to the database.
    pub async fn update_or_insert<
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    >(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        self.inner
            .as_ref()
            .clone()
            .update_or_insert(connection)
            .await
    }
    /// Get all SampleContainerCategory from the database.
    ///
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    pub async fn all<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&crate::database::filter_variants::SampleContainerCategoryFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, crate::api::ApiError> {
        let mut sample_container_categories = Vec::new();
        for flat_variant in crate::database::flat_variants::SampleContainerCategory::all(
            filter, limit, offset, connection,
        )
        .await?
        .into_iter()
        {
            sample_container_categories.push(Self::from_flat(flat_variant, connection).await?);
        }
        Ok(sample_container_categories)
    }
}