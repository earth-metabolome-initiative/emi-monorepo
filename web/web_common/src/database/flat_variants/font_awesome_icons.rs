//! This file is automatically generated, do not modify it directly.
use super::*;
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct FontAwesomeIcon {
    pub id: i32,
    pub name: String,
    pub description: String,
}

unsafe impl Send for FontAwesomeIcon {}
unsafe impl Sync for FontAwesomeIcon {}
impl Tabular for FontAwesomeIcon {
    const TABLE: crate::database::Table = crate::database::Table::FontAwesomeIcons;
}
impl Describable for FontAwesomeIcon {
    fn description(&self) -> Option<&str> {
        Some(self.description.as_str())
    }
}
impl Colorable for FontAwesomeIcon {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for crate::database::flat_variants::FontAwesomeIcon {
    type Filter = crate::database::EmptyFilter;
}
#[cfg(feature = "frontend")]
impl AllRecords for FontAwesomeIcon {
    fn all_records<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        _filter: Option<&<Self as Filtrable>::Filter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> impl std::future::Future<Output = Result<Vec<Self>, crate::api::ApiError>> {
        Self::all(limit, offset, connection)
    }
}
#[cfg(feature = "frontend")]
impl FontAwesomeIcon {
    /// Get the id attribute.
    pub fn get_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.id.as_ref()
    }

    /// Get the name attribute.
    pub fn get_name<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.name.as_ref()
    }

    /// Get the description attribute.
    pub fn get_description<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.description.as_ref()
    }

    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
        ]
    }

    /// Insert the FontAwesomeIcon into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        Ok(table("font_awesome_icons")
            .insert()
            .columns("id, name, description")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
            .map(|payload| match payload {
                gluesql::prelude::Payload::Insert(number_of_inserted_rows) => {
                    number_of_inserted_rows
                }
                _ => unreachable!("Payload must be an Insert"),
            })?)
    }

    /// Get the FontAwesomeIcon from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `connection` - The connection to the database.
    pub async fn get<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        let select_row = table("font_awesome_icons")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, description")
            .limit(1)
            .execute(connection)
            .await?;
        Ok(select_row
            .select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete the FontAwesomeIcon from the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn delete<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        Self::delete_from_id(self.id, connection).await
    }

    /// Delete the FontAwesomeIcon from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    pub async fn delete_from_id<
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    >(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        Ok(table("font_awesome_icons")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
            .map(|payload| match payload {
                gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                _ => unreachable!("Payload must be a Delete"),
            })?)
    }

    /// Update the struct in the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn update<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        table("font_awesome_icons")
            .update()
            .set("id", gluesql::core::ast_builder::num(self.id))
            .set("name", gluesql::core::ast_builder::text(self.name))
            .set(
                "description",
                gluesql::core::ast_builder::text(self.description),
            )
            .execute(connection)
            .await
            .map(|payload| match payload {
                gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                _ => unreachable!("Expected Payload::Update"),
            })
            .map_err(crate::api::ApiError::from)
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
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all FontAwesomeIcon from the database.
    ///
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    pub async fn all<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        let select_row = table("font_awesome_icons")
            .select()
            .project("id, name, description")
            .order_by("description DESC")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row
            .select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32"),
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str"),
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str"),
            },
        }
    }
}