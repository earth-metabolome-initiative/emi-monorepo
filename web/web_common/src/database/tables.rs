use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
use chrono::NaiveDateTime;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct Archivable {
    pub id: Uuid,
    pub archived_at: NaiveDateTime,
    pub archived_by: Uuid,
}
#[cfg(feature = "frontend")]
impl Archivable {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::timestamp(self.archived_at.to_string()),
            gluesql::core::ast_builder::expr(self.archived_by.to_string()),
        ]
    }

    /// Insert the Archivable into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table archivables
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("archivables")
            .insert()
            .columns("id, archived_at, archived_by")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Archivable from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of Archivable to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("archivables")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, archived_at, archived_by")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Archivable from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("archivables")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Archivable from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("archivables")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("archived_at", gluesql::core::ast_builder::timestamp(self.archived_at.to_string()));
        update_row = update_row.set("archived_by", gluesql::core::ast_builder::expr(self.archived_by.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Archivable from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("archivables")
            .select()
            .project("id, archived_at, archived_by")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            archived_at: match row.get("archived_at").unwrap() {
                gluesql::prelude::Value::Timestamp(archived_at) => archived_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            archived_by: match row.get("archived_by").unwrap() {
                gluesql::prelude::Value::Uuid(archived_by) => Uuid::from_u128(*archived_by),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct ContainerHorizontalRule {
    pub id: Uuid,
    pub item_type_id: Option<Uuid>,
    pub other_item_type_id: Option<Uuid>,
    pub minimum_temperature: Option<f64>,
    pub maximum_temperature: Option<f64>,
    pub minimum_humidity: Option<f64>,
    pub maximum_humidity: Option<f64>,
    pub minimum_pressure: Option<f64>,
    pub maximum_pressure: Option<f64>,
}
#[cfg(feature = "frontend")]
impl ContainerHorizontalRule {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            match self.item_type_id {
                Some(item_type_id) => gluesql::core::ast_builder::expr(item_type_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.other_item_type_id {
                Some(other_item_type_id) => gluesql::core::ast_builder::expr(other_item_type_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.minimum_temperature {
                Some(minimum_temperature) => gluesql::core::ast_builder::num(minimum_temperature),
                None => gluesql::core::ast_builder::null(),
            },
            match self.maximum_temperature {
                Some(maximum_temperature) => gluesql::core::ast_builder::num(maximum_temperature),
                None => gluesql::core::ast_builder::null(),
            },
            match self.minimum_humidity {
                Some(minimum_humidity) => gluesql::core::ast_builder::num(minimum_humidity),
                None => gluesql::core::ast_builder::null(),
            },
            match self.maximum_humidity {
                Some(maximum_humidity) => gluesql::core::ast_builder::num(maximum_humidity),
                None => gluesql::core::ast_builder::null(),
            },
            match self.minimum_pressure {
                Some(minimum_pressure) => gluesql::core::ast_builder::num(minimum_pressure),
                None => gluesql::core::ast_builder::null(),
            },
            match self.maximum_pressure {
                Some(maximum_pressure) => gluesql::core::ast_builder::num(maximum_pressure),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the ContainerHorizontalRule into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table container_horizontal_rules
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("container_horizontal_rules")
            .insert()
            .columns("id, item_type_id, other_item_type_id, minimum_temperature, maximum_temperature, minimum_humidity, maximum_humidity, minimum_pressure, maximum_pressure")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ContainerHorizontalRule from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of ContainerHorizontalRule to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("container_horizontal_rules")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, item_type_id, other_item_type_id, minimum_temperature, maximum_temperature, minimum_humidity, maximum_humidity, minimum_pressure, maximum_pressure")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ContainerHorizontalRule from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("container_horizontal_rules")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ContainerHorizontalRule from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("container_horizontal_rules")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        if let Some(item_type_id) = self.item_type_id {
            update_row = update_row.set("item_type_id", gluesql::core::ast_builder::expr(item_type_id.to_string()));
        }
        if let Some(other_item_type_id) = self.other_item_type_id {
            update_row = update_row.set("other_item_type_id", gluesql::core::ast_builder::expr(other_item_type_id.to_string()));
        }
        if let Some(minimum_temperature) = self.minimum_temperature {
            update_row = update_row.set("minimum_temperature", gluesql::core::ast_builder::num(minimum_temperature));
        }
        if let Some(maximum_temperature) = self.maximum_temperature {
            update_row = update_row.set("maximum_temperature", gluesql::core::ast_builder::num(maximum_temperature));
        }
        if let Some(minimum_humidity) = self.minimum_humidity {
            update_row = update_row.set("minimum_humidity", gluesql::core::ast_builder::num(minimum_humidity));
        }
        if let Some(maximum_humidity) = self.maximum_humidity {
            update_row = update_row.set("maximum_humidity", gluesql::core::ast_builder::num(maximum_humidity));
        }
        if let Some(minimum_pressure) = self.minimum_pressure {
            update_row = update_row.set("minimum_pressure", gluesql::core::ast_builder::num(minimum_pressure));
        }
        if let Some(maximum_pressure) = self.maximum_pressure {
            update_row = update_row.set("maximum_pressure", gluesql::core::ast_builder::num(maximum_pressure));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ContainerHorizontalRule from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("container_horizontal_rules")
            .select()
            .project("id, item_type_id, other_item_type_id, minimum_temperature, maximum_temperature, minimum_humidity, maximum_humidity, minimum_pressure, maximum_pressure")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            item_type_id: match row.get("item_type_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(item_type_id) => Some(Uuid::from_u128(*item_type_id)),
                _ => unreachable!("Expected Uuid"),
            },
            other_item_type_id: match row.get("other_item_type_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(other_item_type_id) => Some(Uuid::from_u128(*other_item_type_id)),
                _ => unreachable!("Expected Uuid"),
            },
            minimum_temperature: match row.get("minimum_temperature").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(minimum_temperature) => Some(minimum_temperature.clone()),
                _ => unreachable!("Expected F64")
            },
            maximum_temperature: match row.get("maximum_temperature").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(maximum_temperature) => Some(maximum_temperature.clone()),
                _ => unreachable!("Expected F64")
            },
            minimum_humidity: match row.get("minimum_humidity").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(minimum_humidity) => Some(minimum_humidity.clone()),
                _ => unreachable!("Expected F64")
            },
            maximum_humidity: match row.get("maximum_humidity").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(maximum_humidity) => Some(maximum_humidity.clone()),
                _ => unreachable!("Expected F64")
            },
            minimum_pressure: match row.get("minimum_pressure").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(minimum_pressure) => Some(minimum_pressure.clone()),
                _ => unreachable!("Expected F64")
            },
            maximum_pressure: match row.get("maximum_pressure").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(maximum_pressure) => Some(maximum_pressure.clone()),
                _ => unreachable!("Expected F64")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct ContainerVerticalRule {
    pub id: Uuid,
    pub container_item_type_id: Option<Uuid>,
    pub contained_item_type_id: Option<Uuid>,
    pub minimum_temperature: Option<f64>,
    pub maximum_temperature: Option<f64>,
    pub minimum_humidity: Option<f64>,
    pub maximum_humidity: Option<f64>,
    pub minimum_pressure: Option<f64>,
    pub maximum_pressure: Option<f64>,
}
#[cfg(feature = "frontend")]
impl ContainerVerticalRule {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            match self.container_item_type_id {
                Some(container_item_type_id) => gluesql::core::ast_builder::expr(container_item_type_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.contained_item_type_id {
                Some(contained_item_type_id) => gluesql::core::ast_builder::expr(contained_item_type_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.minimum_temperature {
                Some(minimum_temperature) => gluesql::core::ast_builder::num(minimum_temperature),
                None => gluesql::core::ast_builder::null(),
            },
            match self.maximum_temperature {
                Some(maximum_temperature) => gluesql::core::ast_builder::num(maximum_temperature),
                None => gluesql::core::ast_builder::null(),
            },
            match self.minimum_humidity {
                Some(minimum_humidity) => gluesql::core::ast_builder::num(minimum_humidity),
                None => gluesql::core::ast_builder::null(),
            },
            match self.maximum_humidity {
                Some(maximum_humidity) => gluesql::core::ast_builder::num(maximum_humidity),
                None => gluesql::core::ast_builder::null(),
            },
            match self.minimum_pressure {
                Some(minimum_pressure) => gluesql::core::ast_builder::num(minimum_pressure),
                None => gluesql::core::ast_builder::null(),
            },
            match self.maximum_pressure {
                Some(maximum_pressure) => gluesql::core::ast_builder::num(maximum_pressure),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the ContainerVerticalRule into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table container_vertical_rules
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("container_vertical_rules")
            .insert()
            .columns("id, container_item_type_id, contained_item_type_id, minimum_temperature, maximum_temperature, minimum_humidity, maximum_humidity, minimum_pressure, maximum_pressure")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ContainerVerticalRule from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of ContainerVerticalRule to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("container_vertical_rules")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, container_item_type_id, contained_item_type_id, minimum_temperature, maximum_temperature, minimum_humidity, maximum_humidity, minimum_pressure, maximum_pressure")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ContainerVerticalRule from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("container_vertical_rules")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ContainerVerticalRule from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("container_vertical_rules")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        if let Some(container_item_type_id) = self.container_item_type_id {
            update_row = update_row.set("container_item_type_id", gluesql::core::ast_builder::expr(container_item_type_id.to_string()));
        }
        if let Some(contained_item_type_id) = self.contained_item_type_id {
            update_row = update_row.set("contained_item_type_id", gluesql::core::ast_builder::expr(contained_item_type_id.to_string()));
        }
        if let Some(minimum_temperature) = self.minimum_temperature {
            update_row = update_row.set("minimum_temperature", gluesql::core::ast_builder::num(minimum_temperature));
        }
        if let Some(maximum_temperature) = self.maximum_temperature {
            update_row = update_row.set("maximum_temperature", gluesql::core::ast_builder::num(maximum_temperature));
        }
        if let Some(minimum_humidity) = self.minimum_humidity {
            update_row = update_row.set("minimum_humidity", gluesql::core::ast_builder::num(minimum_humidity));
        }
        if let Some(maximum_humidity) = self.maximum_humidity {
            update_row = update_row.set("maximum_humidity", gluesql::core::ast_builder::num(maximum_humidity));
        }
        if let Some(minimum_pressure) = self.minimum_pressure {
            update_row = update_row.set("minimum_pressure", gluesql::core::ast_builder::num(minimum_pressure));
        }
        if let Some(maximum_pressure) = self.maximum_pressure {
            update_row = update_row.set("maximum_pressure", gluesql::core::ast_builder::num(maximum_pressure));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ContainerVerticalRule from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("container_vertical_rules")
            .select()
            .project("id, container_item_type_id, contained_item_type_id, minimum_temperature, maximum_temperature, minimum_humidity, maximum_humidity, minimum_pressure, maximum_pressure")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            container_item_type_id: match row.get("container_item_type_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(container_item_type_id) => Some(Uuid::from_u128(*container_item_type_id)),
                _ => unreachable!("Expected Uuid"),
            },
            contained_item_type_id: match row.get("contained_item_type_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(contained_item_type_id) => Some(Uuid::from_u128(*contained_item_type_id)),
                _ => unreachable!("Expected Uuid"),
            },
            minimum_temperature: match row.get("minimum_temperature").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(minimum_temperature) => Some(minimum_temperature.clone()),
                _ => unreachable!("Expected F64")
            },
            maximum_temperature: match row.get("maximum_temperature").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(maximum_temperature) => Some(maximum_temperature.clone()),
                _ => unreachable!("Expected F64")
            },
            minimum_humidity: match row.get("minimum_humidity").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(minimum_humidity) => Some(minimum_humidity.clone()),
                _ => unreachable!("Expected F64")
            },
            maximum_humidity: match row.get("maximum_humidity").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(maximum_humidity) => Some(maximum_humidity.clone()),
                _ => unreachable!("Expected F64")
            },
            minimum_pressure: match row.get("minimum_pressure").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(minimum_pressure) => Some(minimum_pressure.clone()),
                _ => unreachable!("Expected F64")
            },
            maximum_pressure: match row.get("maximum_pressure").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(maximum_pressure) => Some(maximum_pressure.clone()),
                _ => unreachable!("Expected F64")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct ContinuousUnit {
    pub id: Uuid,
}
#[cfg(feature = "frontend")]
impl ContinuousUnit {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
        ]
    }

    /// Insert the ContinuousUnit into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table continuous_units
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("continuous_units")
            .insert()
            .columns("id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ContinuousUnit from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of ContinuousUnit to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("continuous_units")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ContinuousUnit from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("continuous_units")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ContinuousUnit from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("continuous_units")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ContinuousUnit from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("continuous_units")
            .select()
            .project("id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct Describable {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}
#[cfg(feature = "frontend")]
impl Describable {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::text(self.name),
            match self.description {
                Some(description) => gluesql::core::ast_builder::text(description),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the Describable into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table describables
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("describables")
            .insert()
            .columns("id, name, description")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Describable from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of Describable to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("describables")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, description")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Describable from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("describables")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Describable from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("describables")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("name", gluesql::core::ast_builder::text(self.name));
        if let Some(description) = self.description {
            update_row = update_row.set("description", gluesql::core::ast_builder::text(description));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Describable from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("describables")
            .select()
            .project("id, name, description")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Str(description) => Some(description.clone()),
                _ => unreachable!("Expected Str")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct DiscreteUnit {
    pub id: Uuid,
}
#[cfg(feature = "frontend")]
impl DiscreteUnit {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
        ]
    }

    /// Insert the DiscreteUnit into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table discrete_units
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("discrete_units")
            .insert()
            .columns("id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get DiscreteUnit from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of DiscreteUnit to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("discrete_units")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete DiscreteUnit from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("discrete_units")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of DiscreteUnit from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("discrete_units")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all DiscreteUnit from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("discrete_units")
            .select()
            .project("id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct DocumentFormat {
    pub id: Uuid,
    pub mime_type: String,
}
#[cfg(feature = "frontend")]
impl DocumentFormat {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::text(self.mime_type),
        ]
    }

    /// Insert the DocumentFormat into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table document_formats
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("document_formats")
            .insert()
            .columns("id, mime_type")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get DocumentFormat from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of DocumentFormat to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("document_formats")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, mime_type")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete DocumentFormat from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("document_formats")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of DocumentFormat from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("document_formats")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("mime_type", gluesql::core::ast_builder::text(self.mime_type));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all DocumentFormat from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("document_formats")
            .select()
            .project("id, mime_type")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            mime_type: match row.get("mime_type").unwrap() {
                gluesql::prelude::Value::Str(mime_type) => mime_type.clone(),
                _ => unreachable!("Expected Str")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct Document {
    pub id: Uuid,
    pub path: String,
    pub format_id: Uuid,
    pub bytes: i32,
}
#[cfg(feature = "frontend")]
impl Document {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::text(self.path),
            gluesql::core::ast_builder::expr(self.format_id.to_string()),
            gluesql::core::ast_builder::num(self.bytes),
        ]
    }

    /// Insert the Document into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table documents
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("documents")
            .insert()
            .columns("id, path, format_id, bytes")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Document from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of Document to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("documents")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, path, format_id, bytes")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Document from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("documents")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Document from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("documents")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("path", gluesql::core::ast_builder::text(self.path));
        update_row = update_row.set("format_id", gluesql::core::ast_builder::expr(self.format_id.to_string()));
        update_row = update_row.set("bytes", gluesql::core::ast_builder::num(self.bytes));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Document from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("documents")
            .select()
            .project("id, path, format_id, bytes")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            path: match row.get("path").unwrap() {
                gluesql::prelude::Value::Str(path) => path.clone(),
                _ => unreachable!("Expected Str")
            },
            format_id: match row.get("format_id").unwrap() {
                gluesql::prelude::Value::Uuid(format_id) => Uuid::from_u128(*format_id),
                _ => unreachable!("Expected Uuid"),
            },
            bytes: match row.get("bytes").unwrap() {
                gluesql::prelude::Value::I32(bytes) => bytes.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct Editable {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub created_by: Uuid,
}
#[cfg(feature = "frontend")]
impl Editable {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::expr(self.created_by.to_string()),
        ]
    }

    /// Insert the Editable into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table editables
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("editables")
            .insert()
            .columns("id, created_at, created_by")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Editable from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of Editable to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("editables")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, created_at, created_by")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Editable from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("editables")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Editable from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("editables")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()));
        update_row = update_row.set("created_by", gluesql::core::ast_builder::expr(self.created_by.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Editable from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("editables")
            .select()
            .project("id, created_at, created_by")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::Uuid(created_by) => Uuid::from_u128(*created_by),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct Edit {
    pub id: Uuid,
    pub editable_id: Uuid,
}
#[cfg(feature = "frontend")]
impl Edit {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::expr(self.editable_id.to_string()),
        ]
    }

    /// Insert the Edit into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table edits
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("edits")
            .insert()
            .columns("id, editable_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Edit from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of Edit to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("edits")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, editable_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Edit from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("edits")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Edit from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("edits")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("editable_id", gluesql::core::ast_builder::expr(self.editable_id.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Edit from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("edits")
            .select()
            .project("id, editable_id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            editable_id: match row.get("editable_id").unwrap() {
                gluesql::prelude::Value::Uuid(editable_id) => Uuid::from_u128(*editable_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct ItemCategory {
    pub id: Uuid,
}
#[cfg(feature = "frontend")]
impl ItemCategory {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
        ]
    }

    /// Insert the ItemCategory into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table item_categories
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("item_categories")
            .insert()
            .columns("id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ItemCategory from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of ItemCategory to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_categories")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ItemCategory from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("item_categories")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ItemCategory from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("item_categories")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ItemCategory from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_categories")
            .select()
            .project("id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct ItemCategoryRelationship {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub child_id: Uuid,
}
#[cfg(feature = "frontend")]
impl ItemCategoryRelationship {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::expr(self.parent_id.to_string()),
            gluesql::core::ast_builder::expr(self.child_id.to_string()),
        ]
    }

    /// Insert the ItemCategoryRelationship into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table item_category_relationships
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("item_category_relationships")
            .insert()
            .columns("id, parent_id, child_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ItemCategoryRelationship from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of ItemCategoryRelationship to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_category_relationships")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, parent_id, child_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ItemCategoryRelationship from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("item_category_relationships")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ItemCategoryRelationship from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("item_category_relationships")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("parent_id", gluesql::core::ast_builder::expr(self.parent_id.to_string()));
        update_row = update_row.set("child_id", gluesql::core::ast_builder::expr(self.child_id.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ItemCategoryRelationship from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_category_relationships")
            .select()
            .project("id, parent_id, child_id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            parent_id: match row.get("parent_id").unwrap() {
                gluesql::prelude::Value::Uuid(parent_id) => Uuid::from_u128(*parent_id),
                _ => unreachable!("Expected Uuid"),
            },
            child_id: match row.get("child_id").unwrap() {
                gluesql::prelude::Value::Uuid(child_id) => Uuid::from_u128(*child_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct ItemCategoryUnit {
    pub id: Uuid,
    pub item_category_id: Uuid,
    pub unit_id: Uuid,
}
#[cfg(feature = "frontend")]
impl ItemCategoryUnit {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::expr(self.item_category_id.to_string()),
            gluesql::core::ast_builder::expr(self.unit_id.to_string()),
        ]
    }

    /// Insert the ItemCategoryUnit into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table item_category_units
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("item_category_units")
            .insert()
            .columns("id, item_category_id, unit_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ItemCategoryUnit from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of ItemCategoryUnit to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_category_units")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, item_category_id, unit_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ItemCategoryUnit from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("item_category_units")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ItemCategoryUnit from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("item_category_units")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("item_category_id", gluesql::core::ast_builder::expr(self.item_category_id.to_string()));
        update_row = update_row.set("unit_id", gluesql::core::ast_builder::expr(self.unit_id.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ItemCategoryUnit from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_category_units")
            .select()
            .project("id, item_category_id, unit_id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            item_category_id: match row.get("item_category_id").unwrap() {
                gluesql::prelude::Value::Uuid(item_category_id) => Uuid::from_u128(*item_category_id),
                _ => unreachable!("Expected Uuid"),
            },
            unit_id: match row.get("unit_id").unwrap() {
                gluesql::prelude::Value::Uuid(unit_id) => Uuid::from_u128(*unit_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct ItemContinuousQuantity {
    pub id: Uuid,
    pub item_id: Option<Uuid>,
    pub weight: f64,
    pub unit_id: Option<Uuid>,
    pub sensor_id: Option<Uuid>,
    pub measured_at: NaiveDateTime,
    pub measured_by: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl ItemContinuousQuantity {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            match self.item_id {
                Some(item_id) => gluesql::core::ast_builder::expr(item_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::num(self.weight),
            match self.unit_id {
                Some(unit_id) => gluesql::core::ast_builder::expr(unit_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.sensor_id {
                Some(sensor_id) => gluesql::core::ast_builder::expr(sensor_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::timestamp(self.measured_at.to_string()),
            match self.measured_by {
                Some(measured_by) => gluesql::core::ast_builder::expr(measured_by.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the ItemContinuousQuantity into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table item_continuous_quantities
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("item_continuous_quantities")
            .insert()
            .columns("id, item_id, weight, unit_id, sensor_id, measured_at, measured_by")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ItemContinuousQuantity from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of ItemContinuousQuantity to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_continuous_quantities")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, item_id, weight, unit_id, sensor_id, measured_at, measured_by")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ItemContinuousQuantity from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("item_continuous_quantities")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ItemContinuousQuantity from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("item_continuous_quantities")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        if let Some(item_id) = self.item_id {
            update_row = update_row.set("item_id", gluesql::core::ast_builder::expr(item_id.to_string()));
        }
        update_row = update_row.set("weight", gluesql::core::ast_builder::num(self.weight));
        if let Some(unit_id) = self.unit_id {
            update_row = update_row.set("unit_id", gluesql::core::ast_builder::expr(unit_id.to_string()));
        }
        if let Some(sensor_id) = self.sensor_id {
            update_row = update_row.set("sensor_id", gluesql::core::ast_builder::expr(sensor_id.to_string()));
        }
        update_row = update_row.set("measured_at", gluesql::core::ast_builder::timestamp(self.measured_at.to_string()));
        if let Some(measured_by) = self.measured_by {
            update_row = update_row.set("measured_by", gluesql::core::ast_builder::expr(measured_by.to_string()));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ItemContinuousQuantity from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_continuous_quantities")
            .select()
            .project("id, item_id, weight, unit_id, sensor_id, measured_at, measured_by")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            item_id: match row.get("item_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(item_id) => Some(Uuid::from_u128(*item_id)),
                _ => unreachable!("Expected Uuid"),
            },
            weight: match row.get("weight").unwrap() {
                gluesql::prelude::Value::F64(weight) => weight.clone(),
                _ => unreachable!("Expected F64")
            },
            unit_id: match row.get("unit_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(unit_id) => Some(Uuid::from_u128(*unit_id)),
                _ => unreachable!("Expected Uuid"),
            },
            sensor_id: match row.get("sensor_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(sensor_id) => Some(Uuid::from_u128(*sensor_id)),
                _ => unreachable!("Expected Uuid"),
            },
            measured_at: match row.get("measured_at").unwrap() {
                gluesql::prelude::Value::Timestamp(measured_at) => measured_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            measured_by: match row.get("measured_by").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(measured_by) => Some(Uuid::from_u128(*measured_by)),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct ItemDiscreteQuantity {
    pub id: Uuid,
    pub item_id: Option<Uuid>,
    pub quantity: i32,
    pub unit_id: Option<Uuid>,
    pub measured_at: NaiveDateTime,
    pub measured_by: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl ItemDiscreteQuantity {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            match self.item_id {
                Some(item_id) => gluesql::core::ast_builder::expr(item_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::num(self.quantity),
            match self.unit_id {
                Some(unit_id) => gluesql::core::ast_builder::expr(unit_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::timestamp(self.measured_at.to_string()),
            match self.measured_by {
                Some(measured_by) => gluesql::core::ast_builder::expr(measured_by.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the ItemDiscreteQuantity into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table item_discrete_quantities
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("item_discrete_quantities")
            .insert()
            .columns("id, item_id, quantity, unit_id, measured_at, measured_by")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ItemDiscreteQuantity from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of ItemDiscreteQuantity to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_discrete_quantities")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, item_id, quantity, unit_id, measured_at, measured_by")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ItemDiscreteQuantity from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("item_discrete_quantities")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ItemDiscreteQuantity from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("item_discrete_quantities")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        if let Some(item_id) = self.item_id {
            update_row = update_row.set("item_id", gluesql::core::ast_builder::expr(item_id.to_string()));
        }
        update_row = update_row.set("quantity", gluesql::core::ast_builder::num(self.quantity));
        if let Some(unit_id) = self.unit_id {
            update_row = update_row.set("unit_id", gluesql::core::ast_builder::expr(unit_id.to_string()));
        }
        update_row = update_row.set("measured_at", gluesql::core::ast_builder::timestamp(self.measured_at.to_string()));
        if let Some(measured_by) = self.measured_by {
            update_row = update_row.set("measured_by", gluesql::core::ast_builder::expr(measured_by.to_string()));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ItemDiscreteQuantity from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_discrete_quantities")
            .select()
            .project("id, item_id, quantity, unit_id, measured_at, measured_by")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            item_id: match row.get("item_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(item_id) => Some(Uuid::from_u128(*item_id)),
                _ => unreachable!("Expected Uuid"),
            },
            quantity: match row.get("quantity").unwrap() {
                gluesql::prelude::Value::I32(quantity) => quantity.clone(),
                _ => unreachable!("Expected I32")
            },
            unit_id: match row.get("unit_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(unit_id) => Some(Uuid::from_u128(*unit_id)),
                _ => unreachable!("Expected Uuid"),
            },
            measured_at: match row.get("measured_at").unwrap() {
                gluesql::prelude::Value::Timestamp(measured_at) => measured_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            measured_by: match row.get("measured_by").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(measured_by) => Some(Uuid::from_u128(*measured_by)),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct ItemLocation {
    pub id: Uuid,
    pub item_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub previous_location_id: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl ItemLocation {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            match self.item_id {
                Some(item_id) => gluesql::core::ast_builder::expr(item_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.location_id {
                Some(location_id) => gluesql::core::ast_builder::expr(location_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.previous_location_id {
                Some(previous_location_id) => gluesql::core::ast_builder::expr(previous_location_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the ItemLocation into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table item_locations
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("item_locations")
            .insert()
            .columns("id, item_id, location_id, previous_location_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ItemLocation from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of ItemLocation to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_locations")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, item_id, location_id, previous_location_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ItemLocation from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("item_locations")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ItemLocation from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("item_locations")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        if let Some(item_id) = self.item_id {
            update_row = update_row.set("item_id", gluesql::core::ast_builder::expr(item_id.to_string()));
        }
        if let Some(location_id) = self.location_id {
            update_row = update_row.set("location_id", gluesql::core::ast_builder::expr(location_id.to_string()));
        }
        if let Some(previous_location_id) = self.previous_location_id {
            update_row = update_row.set("previous_location_id", gluesql::core::ast_builder::expr(previous_location_id.to_string()));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ItemLocation from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_locations")
            .select()
            .project("id, item_id, location_id, previous_location_id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            item_id: match row.get("item_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(item_id) => Some(Uuid::from_u128(*item_id)),
                _ => unreachable!("Expected Uuid"),
            },
            location_id: match row.get("location_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(location_id) => Some(Uuid::from_u128(*location_id)),
                _ => unreachable!("Expected Uuid"),
            },
            previous_location_id: match row.get("previous_location_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(previous_location_id) => Some(Uuid::from_u128(*previous_location_id)),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct ItemUnit {
    pub id: Uuid,
    pub item_id: Uuid,
    pub unit_id: Uuid,
}
#[cfg(feature = "frontend")]
impl ItemUnit {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::expr(self.item_id.to_string()),
            gluesql::core::ast_builder::expr(self.unit_id.to_string()),
        ]
    }

    /// Insert the ItemUnit into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table item_units
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("item_units")
            .insert()
            .columns("id, item_id, unit_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ItemUnit from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of ItemUnit to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_units")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, item_id, unit_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ItemUnit from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("item_units")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ItemUnit from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("item_units")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("item_id", gluesql::core::ast_builder::expr(self.item_id.to_string()));
        update_row = update_row.set("unit_id", gluesql::core::ast_builder::expr(self.unit_id.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ItemUnit from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_units")
            .select()
            .project("id, item_id, unit_id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            item_id: match row.get("item_id").unwrap() {
                gluesql::prelude::Value::Uuid(item_id) => Uuid::from_u128(*item_id),
                _ => unreachable!("Expected Uuid"),
            },
            unit_id: match row.get("unit_id").unwrap() {
                gluesql::prelude::Value::Uuid(unit_id) => Uuid::from_u128(*unit_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct Item {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl Item {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            match self.parent_id {
                Some(parent_id) => gluesql::core::ast_builder::expr(parent_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the Item into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table items
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("items")
            .insert()
            .columns("id, parent_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Item from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of Item to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("items")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, parent_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Item from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("items")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Item from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("items")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        if let Some(parent_id) = self.parent_id {
            update_row = update_row.set("parent_id", gluesql::core::ast_builder::expr(parent_id.to_string()));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Item from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("items")
            .select()
            .project("id, parent_id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            parent_id: match row.get("parent_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(parent_id) => Some(Uuid::from_u128(*parent_id)),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct LocationState {
    pub id: Uuid,
    pub font_awesome_icon: Option<String>,
}
#[cfg(feature = "frontend")]
impl LocationState {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            match self.font_awesome_icon {
                Some(font_awesome_icon) => gluesql::core::ast_builder::text(font_awesome_icon),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the LocationState into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table location_states
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("location_states")
            .insert()
            .columns("id, font_awesome_icon")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get LocationState from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of LocationState to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("location_states")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, font_awesome_icon")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete LocationState from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("location_states")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of LocationState from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("location_states")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        if let Some(font_awesome_icon) = self.font_awesome_icon {
            update_row = update_row.set("font_awesome_icon", gluesql::core::ast_builder::text(font_awesome_icon));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all LocationState from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("location_states")
            .select()
            .project("id, font_awesome_icon")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            font_awesome_icon: match row.get("font_awesome_icon").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Str(font_awesome_icon) => Some(font_awesome_icon.clone()),
                _ => unreachable!("Expected Str")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct Location {
    pub id: Uuid,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f64>,
    pub address: Option<String>,
    pub geolocalization_device_id: Option<Uuid>,
    pub altitude_device_id: Option<Uuid>,
    pub parent_location_id: Option<Uuid>,
    pub state_id: Uuid,
}
#[cfg(feature = "frontend")]
impl Location {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            match self.latitude {
                Some(latitude) => gluesql::core::ast_builder::num(latitude),
                None => gluesql::core::ast_builder::null(),
            },
            match self.longitude {
                Some(longitude) => gluesql::core::ast_builder::num(longitude),
                None => gluesql::core::ast_builder::null(),
            },
            match self.altitude {
                Some(altitude) => gluesql::core::ast_builder::num(altitude),
                None => gluesql::core::ast_builder::null(),
            },
            match self.address {
                Some(address) => gluesql::core::ast_builder::text(address),
                None => gluesql::core::ast_builder::null(),
            },
            match self.geolocalization_device_id {
                Some(geolocalization_device_id) => gluesql::core::ast_builder::expr(geolocalization_device_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.altitude_device_id {
                Some(altitude_device_id) => gluesql::core::ast_builder::expr(altitude_device_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.parent_location_id {
                Some(parent_location_id) => gluesql::core::ast_builder::expr(parent_location_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::expr(self.state_id.to_string()),
        ]
    }

    /// Insert the Location into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table locations
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("locations")
            .insert()
            .columns("id, latitude, longitude, altitude, address, geolocalization_device_id, altitude_device_id, parent_location_id, state_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Location from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of Location to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("locations")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, latitude, longitude, altitude, address, geolocalization_device_id, altitude_device_id, parent_location_id, state_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Location from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("locations")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Location from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("locations")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        if let Some(latitude) = self.latitude {
            update_row = update_row.set("latitude", gluesql::core::ast_builder::num(latitude));
        }
        if let Some(longitude) = self.longitude {
            update_row = update_row.set("longitude", gluesql::core::ast_builder::num(longitude));
        }
        if let Some(altitude) = self.altitude {
            update_row = update_row.set("altitude", gluesql::core::ast_builder::num(altitude));
        }
        if let Some(address) = self.address {
            update_row = update_row.set("address", gluesql::core::ast_builder::text(address));
        }
        if let Some(geolocalization_device_id) = self.geolocalization_device_id {
            update_row = update_row.set("geolocalization_device_id", gluesql::core::ast_builder::expr(geolocalization_device_id.to_string()));
        }
        if let Some(altitude_device_id) = self.altitude_device_id {
            update_row = update_row.set("altitude_device_id", gluesql::core::ast_builder::expr(altitude_device_id.to_string()));
        }
        if let Some(parent_location_id) = self.parent_location_id {
            update_row = update_row.set("parent_location_id", gluesql::core::ast_builder::expr(parent_location_id.to_string()));
        }
        update_row = update_row.set("state_id", gluesql::core::ast_builder::expr(self.state_id.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Location from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("locations")
            .select()
            .project("id, latitude, longitude, altitude, address, geolocalization_device_id, altitude_device_id, parent_location_id, state_id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            latitude: match row.get("latitude").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(latitude) => Some(latitude.clone()),
                _ => unreachable!("Expected F64")
            },
            longitude: match row.get("longitude").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(longitude) => Some(longitude.clone()),
                _ => unreachable!("Expected F64")
            },
            altitude: match row.get("altitude").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(altitude) => Some(altitude.clone()),
                _ => unreachable!("Expected F64")
            },
            address: match row.get("address").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Str(address) => Some(address.clone()),
                _ => unreachable!("Expected Str")
            },
            geolocalization_device_id: match row.get("geolocalization_device_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(geolocalization_device_id) => Some(Uuid::from_u128(*geolocalization_device_id)),
                _ => unreachable!("Expected Uuid"),
            },
            altitude_device_id: match row.get("altitude_device_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(altitude_device_id) => Some(Uuid::from_u128(*altitude_device_id)),
                _ => unreachable!("Expected Uuid"),
            },
            parent_location_id: match row.get("parent_location_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(parent_location_id) => Some(Uuid::from_u128(*parent_location_id)),
                _ => unreachable!("Expected Uuid"),
            },
            state_id: match row.get("state_id").unwrap() {
                gluesql::prelude::Value::Uuid(state_id) => Uuid::from_u128(*state_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct LoginProvider {
    pub id: Uuid,
    pub name: String,
    pub font_awesome_icon: String,
    pub client_id_var_name: String,
    pub redirect_uri_var_name: String,
    pub oauth_url: String,
    pub scope: String,
}
#[cfg(feature = "frontend")]
impl LoginProvider {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.font_awesome_icon),
            gluesql::core::ast_builder::text(self.client_id_var_name),
            gluesql::core::ast_builder::text(self.redirect_uri_var_name),
            gluesql::core::ast_builder::text(self.oauth_url),
            gluesql::core::ast_builder::text(self.scope),
        ]
    }

    /// Insert the LoginProvider into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table login_providers
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("login_providers")
            .insert()
            .columns("id, name, font_awesome_icon, client_id_var_name, redirect_uri_var_name, oauth_url, scope")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get LoginProvider from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of LoginProvider to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("login_providers")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, font_awesome_icon, client_id_var_name, redirect_uri_var_name, oauth_url, scope")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete LoginProvider from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("login_providers")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of LoginProvider from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("login_providers")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("name", gluesql::core::ast_builder::text(self.name));
        update_row = update_row.set("font_awesome_icon", gluesql::core::ast_builder::text(self.font_awesome_icon));
        update_row = update_row.set("client_id_var_name", gluesql::core::ast_builder::text(self.client_id_var_name));
        update_row = update_row.set("redirect_uri_var_name", gluesql::core::ast_builder::text(self.redirect_uri_var_name));
        update_row = update_row.set("oauth_url", gluesql::core::ast_builder::text(self.oauth_url));
        update_row = update_row.set("scope", gluesql::core::ast_builder::text(self.scope));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all LoginProvider from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("login_providers")
            .select()
            .project("id, name, font_awesome_icon, client_id_var_name, redirect_uri_var_name, oauth_url, scope")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            font_awesome_icon: match row.get("font_awesome_icon").unwrap() {
                gluesql::prelude::Value::Str(font_awesome_icon) => font_awesome_icon.clone(),
                _ => unreachable!("Expected Str")
            },
            client_id_var_name: match row.get("client_id_var_name").unwrap() {
                gluesql::prelude::Value::Str(client_id_var_name) => client_id_var_name.clone(),
                _ => unreachable!("Expected Str")
            },
            redirect_uri_var_name: match row.get("redirect_uri_var_name").unwrap() {
                gluesql::prelude::Value::Str(redirect_uri_var_name) => redirect_uri_var_name.clone(),
                _ => unreachable!("Expected Str")
            },
            oauth_url: match row.get("oauth_url").unwrap() {
                gluesql::prelude::Value::Str(oauth_url) => oauth_url.clone(),
                _ => unreachable!("Expected Str")
            },
            scope: match row.get("scope").unwrap() {
                gluesql::prelude::Value::Str(scope) => scope.clone(),
                _ => unreachable!("Expected Str")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct ManufacturedItemCategory {
    pub id: Uuid,
    pub cost: f64,
    pub cost_per_day: f64,
    pub currency: String,
    pub manifacturer_id: Uuid,
}
#[cfg(feature = "frontend")]
impl ManufacturedItemCategory {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::num(self.cost),
            gluesql::core::ast_builder::num(self.cost_per_day),
            gluesql::core::ast_builder::text(self.currency),
            gluesql::core::ast_builder::expr(self.manifacturer_id.to_string()),
        ]
    }

    /// Insert the ManufacturedItemCategory into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table manufactured_item_categories
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("manufactured_item_categories")
            .insert()
            .columns("id, cost, cost_per_day, currency, manifacturer_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ManufacturedItemCategory from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of ManufacturedItemCategory to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("manufactured_item_categories")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, cost, cost_per_day, currency, manifacturer_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ManufacturedItemCategory from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("manufactured_item_categories")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ManufacturedItemCategory from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("manufactured_item_categories")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("cost", gluesql::core::ast_builder::num(self.cost));
        update_row = update_row.set("cost_per_day", gluesql::core::ast_builder::num(self.cost_per_day));
        update_row = update_row.set("currency", gluesql::core::ast_builder::text(self.currency));
        update_row = update_row.set("manifacturer_id", gluesql::core::ast_builder::expr(self.manifacturer_id.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ManufacturedItemCategory from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("manufactured_item_categories")
            .select()
            .project("id, cost, cost_per_day, currency, manifacturer_id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            cost: match row.get("cost").unwrap() {
                gluesql::prelude::Value::F64(cost) => cost.clone(),
                _ => unreachable!("Expected F64")
            },
            cost_per_day: match row.get("cost_per_day").unwrap() {
                gluesql::prelude::Value::F64(cost_per_day) => cost_per_day.clone(),
                _ => unreachable!("Expected F64")
            },
            currency: match row.get("currency").unwrap() {
                gluesql::prelude::Value::Str(currency) => currency.clone(),
                _ => unreachable!("Expected Str")
            },
            manifacturer_id: match row.get("manifacturer_id").unwrap() {
                gluesql::prelude::Value::Uuid(manifacturer_id) => Uuid::from_u128(*manifacturer_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub operation: String,
    pub table_name: String,
    pub row_id: Option<Uuid>,
    pub read: bool,
}
#[cfg(feature = "frontend")]
impl Notification {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::expr(self.user_id.to_string()),
            gluesql::core::ast_builder::text(self.operation),
            gluesql::core::ast_builder::text(self.table_name),
            match self.row_id {
                Some(row_id) => gluesql::core::ast_builder::expr(row_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            (self.read.into()),
        ]
    }

    /// Insert the Notification into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table notifications
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("notifications")
            .insert()
            .columns("id, user_id, operation, table_name, row_id, read")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Notification from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of Notification to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("notifications")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, user_id, operation, table_name, row_id, read")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Notification from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("notifications")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Notification from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("notifications")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("user_id", gluesql::core::ast_builder::expr(self.user_id.to_string()));
        update_row = update_row.set("operation", gluesql::core::ast_builder::text(self.operation));
        update_row = update_row.set("table_name", gluesql::core::ast_builder::text(self.table_name));
        if let Some(row_id) = self.row_id {
            update_row = update_row.set("row_id", gluesql::core::ast_builder::expr(row_id.to_string()));
        }
        update_row = update_row.set("read", self.read);
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Notification from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("notifications")
            .select()
            .project("id, user_id, operation, table_name, row_id, read")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            user_id: match row.get("user_id").unwrap() {
                gluesql::prelude::Value::Uuid(user_id) => Uuid::from_u128(*user_id),
                _ => unreachable!("Expected Uuid"),
            },
            operation: match row.get("operation").unwrap() {
                gluesql::prelude::Value::Str(operation) => operation.clone(),
                _ => unreachable!("Expected Str")
            },
            table_name: match row.get("table_name").unwrap() {
                gluesql::prelude::Value::Str(table_name) => table_name.clone(),
                _ => unreachable!("Expected Str")
            },
            row_id: match row.get("row_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(row_id) => Some(Uuid::from_u128(*row_id)),
                _ => unreachable!("Expected Uuid"),
            },
            read: match row.get("read").unwrap() {
                gluesql::prelude::Value::Bool(read) => read.clone(),
                _ => unreachable!("Expected Bool")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct OrganizationAuthorization {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub editable_id: Uuid,
    pub role_id: Uuid,
}
#[cfg(feature = "frontend")]
impl OrganizationAuthorization {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::expr(self.organization_id.to_string()),
            gluesql::core::ast_builder::expr(self.editable_id.to_string()),
            gluesql::core::ast_builder::expr(self.role_id.to_string()),
        ]
    }

    /// Insert the OrganizationAuthorization into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table organization_authorizations
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("organization_authorizations")
            .insert()
            .columns("id, organization_id, editable_id, role_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get OrganizationAuthorization from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of OrganizationAuthorization to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("organization_authorizations")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, organization_id, editable_id, role_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete OrganizationAuthorization from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("organization_authorizations")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of OrganizationAuthorization from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("organization_authorizations")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("organization_id", gluesql::core::ast_builder::expr(self.organization_id.to_string()));
        update_row = update_row.set("editable_id", gluesql::core::ast_builder::expr(self.editable_id.to_string()));
        update_row = update_row.set("role_id", gluesql::core::ast_builder::expr(self.role_id.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all OrganizationAuthorization from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("organization_authorizations")
            .select()
            .project("id, organization_id, editable_id, role_id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            organization_id: match row.get("organization_id").unwrap() {
                gluesql::prelude::Value::Uuid(organization_id) => Uuid::from_u128(*organization_id),
                _ => unreachable!("Expected Uuid"),
            },
            editable_id: match row.get("editable_id").unwrap() {
                gluesql::prelude::Value::Uuid(editable_id) => Uuid::from_u128(*editable_id),
                _ => unreachable!("Expected Uuid"),
            },
            role_id: match row.get("role_id").unwrap() {
                gluesql::prelude::Value::Uuid(role_id) => Uuid::from_u128(*role_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct OrganizationLocation {
    pub id: Uuid,
    pub organization_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub previous_location_id: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl OrganizationLocation {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            match self.organization_id {
                Some(organization_id) => gluesql::core::ast_builder::expr(organization_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.location_id {
                Some(location_id) => gluesql::core::ast_builder::expr(location_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.previous_location_id {
                Some(previous_location_id) => gluesql::core::ast_builder::expr(previous_location_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the OrganizationLocation into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table organization_locations
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("organization_locations")
            .insert()
            .columns("id, organization_id, location_id, previous_location_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get OrganizationLocation from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of OrganizationLocation to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("organization_locations")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, organization_id, location_id, previous_location_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete OrganizationLocation from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("organization_locations")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of OrganizationLocation from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("organization_locations")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        if let Some(organization_id) = self.organization_id {
            update_row = update_row.set("organization_id", gluesql::core::ast_builder::expr(organization_id.to_string()));
        }
        if let Some(location_id) = self.location_id {
            update_row = update_row.set("location_id", gluesql::core::ast_builder::expr(location_id.to_string()));
        }
        if let Some(previous_location_id) = self.previous_location_id {
            update_row = update_row.set("previous_location_id", gluesql::core::ast_builder::expr(previous_location_id.to_string()));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all OrganizationLocation from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("organization_locations")
            .select()
            .project("id, organization_id, location_id, previous_location_id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            organization_id: match row.get("organization_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(organization_id) => Some(Uuid::from_u128(*organization_id)),
                _ => unreachable!("Expected Uuid"),
            },
            location_id: match row.get("location_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(location_id) => Some(Uuid::from_u128(*location_id)),
                _ => unreachable!("Expected Uuid"),
            },
            previous_location_id: match row.get("previous_location_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(previous_location_id) => Some(Uuid::from_u128(*previous_location_id)),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct OrganizationState {
    pub id: Uuid,
    pub font_awesome_icon: Option<String>,
}
#[cfg(feature = "frontend")]
impl OrganizationState {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            match self.font_awesome_icon {
                Some(font_awesome_icon) => gluesql::core::ast_builder::text(font_awesome_icon),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the OrganizationState into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table organization_states
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("organization_states")
            .insert()
            .columns("id, font_awesome_icon")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get OrganizationState from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of OrganizationState to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("organization_states")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, font_awesome_icon")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete OrganizationState from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("organization_states")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of OrganizationState from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("organization_states")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        if let Some(font_awesome_icon) = self.font_awesome_icon {
            update_row = update_row.set("font_awesome_icon", gluesql::core::ast_builder::text(font_awesome_icon));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all OrganizationState from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("organization_states")
            .select()
            .project("id, font_awesome_icon")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            font_awesome_icon: match row.get("font_awesome_icon").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Str(font_awesome_icon) => Some(font_awesome_icon.clone()),
                _ => unreachable!("Expected Str")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct Organization {
    pub id: Uuid,
    pub state_id: Option<Uuid>,
    pub parent_organization_id: Option<Uuid>,
    pub logo_id: Option<Uuid>,
    pub website_url: Option<String>,
}
#[cfg(feature = "frontend")]
impl Organization {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            match self.state_id {
                Some(state_id) => gluesql::core::ast_builder::expr(state_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.parent_organization_id {
                Some(parent_organization_id) => gluesql::core::ast_builder::expr(parent_organization_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.logo_id {
                Some(logo_id) => gluesql::core::ast_builder::expr(logo_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.website_url {
                Some(website_url) => gluesql::core::ast_builder::text(website_url),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the Organization into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table organizations
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("organizations")
            .insert()
            .columns("id, state_id, parent_organization_id, logo_id, website_url")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Organization from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of Organization to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("organizations")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, state_id, parent_organization_id, logo_id, website_url")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Organization from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("organizations")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Organization from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("organizations")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        if let Some(state_id) = self.state_id {
            update_row = update_row.set("state_id", gluesql::core::ast_builder::expr(state_id.to_string()));
        }
        if let Some(parent_organization_id) = self.parent_organization_id {
            update_row = update_row.set("parent_organization_id", gluesql::core::ast_builder::expr(parent_organization_id.to_string()));
        }
        if let Some(logo_id) = self.logo_id {
            update_row = update_row.set("logo_id", gluesql::core::ast_builder::expr(logo_id.to_string()));
        }
        if let Some(website_url) = self.website_url {
            update_row = update_row.set("website_url", gluesql::core::ast_builder::text(website_url));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Organization from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("organizations")
            .select()
            .project("id, state_id, parent_organization_id, logo_id, website_url")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            state_id: match row.get("state_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(state_id) => Some(Uuid::from_u128(*state_id)),
                _ => unreachable!("Expected Uuid"),
            },
            parent_organization_id: match row.get("parent_organization_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(parent_organization_id) => Some(Uuid::from_u128(*parent_organization_id)),
                _ => unreachable!("Expected Uuid"),
            },
            logo_id: match row.get("logo_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(logo_id) => Some(Uuid::from_u128(*logo_id)),
                _ => unreachable!("Expected Uuid"),
            },
            website_url: match row.get("website_url").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Str(website_url) => Some(website_url.clone()),
                _ => unreachable!("Expected Str")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct PrimaryUserEmail {
    pub id: Uuid,
}
#[cfg(feature = "frontend")]
impl PrimaryUserEmail {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
        ]
    }

    /// Insert the PrimaryUserEmail into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table primary_user_emails
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("primary_user_emails")
            .insert()
            .columns("id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get PrimaryUserEmail from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of PrimaryUserEmail to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("primary_user_emails")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete PrimaryUserEmail from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("primary_user_emails")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of PrimaryUserEmail from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("primary_user_emails")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all PrimaryUserEmail from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("primary_user_emails")
            .select()
            .project("id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct ProcedureContinuousRequirement {
    pub id: Uuid,
    pub procedure_id: Uuid,
    pub item_category_id: Uuid,
    pub quantity: f64,
    pub unit_id: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl ProcedureContinuousRequirement {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::expr(self.procedure_id.to_string()),
            gluesql::core::ast_builder::expr(self.item_category_id.to_string()),
            gluesql::core::ast_builder::num(self.quantity),
            match self.unit_id {
                Some(unit_id) => gluesql::core::ast_builder::expr(unit_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the ProcedureContinuousRequirement into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table procedure_continuous_requirements
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("procedure_continuous_requirements")
            .insert()
            .columns("id, procedure_id, item_category_id, quantity, unit_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ProcedureContinuousRequirement from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of ProcedureContinuousRequirement to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("procedure_continuous_requirements")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, procedure_id, item_category_id, quantity, unit_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ProcedureContinuousRequirement from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("procedure_continuous_requirements")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ProcedureContinuousRequirement from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("procedure_continuous_requirements")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("procedure_id", gluesql::core::ast_builder::expr(self.procedure_id.to_string()));
        update_row = update_row.set("item_category_id", gluesql::core::ast_builder::expr(self.item_category_id.to_string()));
        update_row = update_row.set("quantity", gluesql::core::ast_builder::num(self.quantity));
        if let Some(unit_id) = self.unit_id {
            update_row = update_row.set("unit_id", gluesql::core::ast_builder::expr(unit_id.to_string()));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ProcedureContinuousRequirement from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("procedure_continuous_requirements")
            .select()
            .project("id, procedure_id, item_category_id, quantity, unit_id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            procedure_id: match row.get("procedure_id").unwrap() {
                gluesql::prelude::Value::Uuid(procedure_id) => Uuid::from_u128(*procedure_id),
                _ => unreachable!("Expected Uuid"),
            },
            item_category_id: match row.get("item_category_id").unwrap() {
                gluesql::prelude::Value::Uuid(item_category_id) => Uuid::from_u128(*item_category_id),
                _ => unreachable!("Expected Uuid"),
            },
            quantity: match row.get("quantity").unwrap() {
                gluesql::prelude::Value::F64(quantity) => quantity.clone(),
                _ => unreachable!("Expected F64")
            },
            unit_id: match row.get("unit_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(unit_id) => Some(Uuid::from_u128(*unit_id)),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct ProcedureDiscreteRequirement {
    pub id: Uuid,
    pub procedure_id: Uuid,
    pub item_category_id: Uuid,
    pub quantity: i32,
    pub unit_id: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl ProcedureDiscreteRequirement {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::expr(self.procedure_id.to_string()),
            gluesql::core::ast_builder::expr(self.item_category_id.to_string()),
            gluesql::core::ast_builder::num(self.quantity),
            match self.unit_id {
                Some(unit_id) => gluesql::core::ast_builder::expr(unit_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the ProcedureDiscreteRequirement into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table procedure_discrete_requirements
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("procedure_discrete_requirements")
            .insert()
            .columns("id, procedure_id, item_category_id, quantity, unit_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ProcedureDiscreteRequirement from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of ProcedureDiscreteRequirement to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("procedure_discrete_requirements")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, procedure_id, item_category_id, quantity, unit_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ProcedureDiscreteRequirement from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("procedure_discrete_requirements")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ProcedureDiscreteRequirement from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("procedure_discrete_requirements")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("procedure_id", gluesql::core::ast_builder::expr(self.procedure_id.to_string()));
        update_row = update_row.set("item_category_id", gluesql::core::ast_builder::expr(self.item_category_id.to_string()));
        update_row = update_row.set("quantity", gluesql::core::ast_builder::num(self.quantity));
        if let Some(unit_id) = self.unit_id {
            update_row = update_row.set("unit_id", gluesql::core::ast_builder::expr(unit_id.to_string()));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ProcedureDiscreteRequirement from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("procedure_discrete_requirements")
            .select()
            .project("id, procedure_id, item_category_id, quantity, unit_id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            procedure_id: match row.get("procedure_id").unwrap() {
                gluesql::prelude::Value::Uuid(procedure_id) => Uuid::from_u128(*procedure_id),
                _ => unreachable!("Expected Uuid"),
            },
            item_category_id: match row.get("item_category_id").unwrap() {
                gluesql::prelude::Value::Uuid(item_category_id) => Uuid::from_u128(*item_category_id),
                _ => unreachable!("Expected Uuid"),
            },
            quantity: match row.get("quantity").unwrap() {
                gluesql::prelude::Value::I32(quantity) => quantity.clone(),
                _ => unreachable!("Expected I32")
            },
            unit_id: match row.get("unit_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(unit_id) => Some(Uuid::from_u128(*unit_id)),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct Procedure {
    pub id: Uuid,
}
#[cfg(feature = "frontend")]
impl Procedure {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
        ]
    }

    /// Insert the Procedure into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table procedures
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("procedures")
            .insert()
            .columns("id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Procedure from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of Procedure to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("procedures")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Procedure from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("procedures")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Procedure from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("procedures")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Procedure from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("procedures")
            .select()
            .project("id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct ProjectContinuousRequirement {
    pub id: Uuid,
    pub project_id: Uuid,
    pub item_id: Uuid,
    pub quantity: f64,
    pub unit_id: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl ProjectContinuousRequirement {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::expr(self.project_id.to_string()),
            gluesql::core::ast_builder::expr(self.item_id.to_string()),
            gluesql::core::ast_builder::num(self.quantity),
            match self.unit_id {
                Some(unit_id) => gluesql::core::ast_builder::expr(unit_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the ProjectContinuousRequirement into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table project_continuous_requirements
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("project_continuous_requirements")
            .insert()
            .columns("id, project_id, item_id, quantity, unit_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ProjectContinuousRequirement from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of ProjectContinuousRequirement to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("project_continuous_requirements")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, project_id, item_id, quantity, unit_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ProjectContinuousRequirement from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("project_continuous_requirements")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ProjectContinuousRequirement from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("project_continuous_requirements")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("project_id", gluesql::core::ast_builder::expr(self.project_id.to_string()));
        update_row = update_row.set("item_id", gluesql::core::ast_builder::expr(self.item_id.to_string()));
        update_row = update_row.set("quantity", gluesql::core::ast_builder::num(self.quantity));
        if let Some(unit_id) = self.unit_id {
            update_row = update_row.set("unit_id", gluesql::core::ast_builder::expr(unit_id.to_string()));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ProjectContinuousRequirement from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("project_continuous_requirements")
            .select()
            .project("id, project_id, item_id, quantity, unit_id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            project_id: match row.get("project_id").unwrap() {
                gluesql::prelude::Value::Uuid(project_id) => Uuid::from_u128(*project_id),
                _ => unreachable!("Expected Uuid"),
            },
            item_id: match row.get("item_id").unwrap() {
                gluesql::prelude::Value::Uuid(item_id) => Uuid::from_u128(*item_id),
                _ => unreachable!("Expected Uuid"),
            },
            quantity: match row.get("quantity").unwrap() {
                gluesql::prelude::Value::F64(quantity) => quantity.clone(),
                _ => unreachable!("Expected F64")
            },
            unit_id: match row.get("unit_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(unit_id) => Some(Uuid::from_u128(*unit_id)),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct ProjectDiscreteRequirement {
    pub id: Uuid,
    pub project_id: Uuid,
    pub item_id: Uuid,
    pub quantity: f64,
    pub unit_id: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl ProjectDiscreteRequirement {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::expr(self.project_id.to_string()),
            gluesql::core::ast_builder::expr(self.item_id.to_string()),
            gluesql::core::ast_builder::num(self.quantity),
            match self.unit_id {
                Some(unit_id) => gluesql::core::ast_builder::expr(unit_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the ProjectDiscreteRequirement into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table project_discrete_requirements
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("project_discrete_requirements")
            .insert()
            .columns("id, project_id, item_id, quantity, unit_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ProjectDiscreteRequirement from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of ProjectDiscreteRequirement to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("project_discrete_requirements")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, project_id, item_id, quantity, unit_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ProjectDiscreteRequirement from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("project_discrete_requirements")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ProjectDiscreteRequirement from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("project_discrete_requirements")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("project_id", gluesql::core::ast_builder::expr(self.project_id.to_string()));
        update_row = update_row.set("item_id", gluesql::core::ast_builder::expr(self.item_id.to_string()));
        update_row = update_row.set("quantity", gluesql::core::ast_builder::num(self.quantity));
        if let Some(unit_id) = self.unit_id {
            update_row = update_row.set("unit_id", gluesql::core::ast_builder::expr(unit_id.to_string()));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ProjectDiscreteRequirement from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("project_discrete_requirements")
            .select()
            .project("id, project_id, item_id, quantity, unit_id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            project_id: match row.get("project_id").unwrap() {
                gluesql::prelude::Value::Uuid(project_id) => Uuid::from_u128(*project_id),
                _ => unreachable!("Expected Uuid"),
            },
            item_id: match row.get("item_id").unwrap() {
                gluesql::prelude::Value::Uuid(item_id) => Uuid::from_u128(*item_id),
                _ => unreachable!("Expected Uuid"),
            },
            quantity: match row.get("quantity").unwrap() {
                gluesql::prelude::Value::F64(quantity) => quantity.clone(),
                _ => unreachable!("Expected F64")
            },
            unit_id: match row.get("unit_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(unit_id) => Some(Uuid::from_u128(*unit_id)),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct ProjectMilestone {
    pub id: Uuid,
    pub project_id: Uuid,
    pub due_date: NaiveDateTime,
    pub completed_at: Option<NaiveDateTime>,
}
#[cfg(feature = "frontend")]
impl ProjectMilestone {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::expr(self.project_id.to_string()),
            gluesql::core::ast_builder::timestamp(self.due_date.to_string()),
            match self.completed_at {
                Some(completed_at) => gluesql::core::ast_builder::timestamp(completed_at.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the ProjectMilestone into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table project_milestones
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("project_milestones")
            .insert()
            .columns("id, project_id, due_date, completed_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ProjectMilestone from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of ProjectMilestone to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("project_milestones")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, project_id, due_date, completed_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ProjectMilestone from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("project_milestones")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ProjectMilestone from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("project_milestones")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("project_id", gluesql::core::ast_builder::expr(self.project_id.to_string()));
        update_row = update_row.set("due_date", gluesql::core::ast_builder::timestamp(self.due_date.to_string()));
        if let Some(completed_at) = self.completed_at {
            update_row = update_row.set("completed_at", gluesql::core::ast_builder::timestamp(completed_at.to_string()));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ProjectMilestone from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("project_milestones")
            .select()
            .project("id, project_id, due_date, completed_at")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            project_id: match row.get("project_id").unwrap() {
                gluesql::prelude::Value::Uuid(project_id) => Uuid::from_u128(*project_id),
                _ => unreachable!("Expected Uuid"),
            },
            due_date: match row.get("due_date").unwrap() {
                gluesql::prelude::Value::Timestamp(due_date) => due_date.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            completed_at: match row.get("completed_at").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Timestamp(completed_at) => Some(completed_at.clone()),
                _ => unreachable!("Expected Timestamp")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct ProjectState {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub font_awesome_icon: String,
    pub icon_color: String,
}
#[cfg(feature = "frontend")]
impl ProjectState {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            gluesql::core::ast_builder::text(self.font_awesome_icon),
            gluesql::core::ast_builder::text(self.icon_color),
        ]
    }

    /// Insert the ProjectState into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table project_states
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("project_states")
            .insert()
            .columns("id, name, description, font_awesome_icon, icon_color")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ProjectState from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of ProjectState to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("project_states")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, description, font_awesome_icon, icon_color")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ProjectState from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("project_states")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ProjectState from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("project_states")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("name", gluesql::core::ast_builder::text(self.name));
        update_row = update_row.set("description", gluesql::core::ast_builder::text(self.description));
        update_row = update_row.set("font_awesome_icon", gluesql::core::ast_builder::text(self.font_awesome_icon));
        update_row = update_row.set("icon_color", gluesql::core::ast_builder::text(self.icon_color));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all ProjectState from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("project_states")
            .select()
            .project("id, name, description, font_awesome_icon, icon_color")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str")
            },
            font_awesome_icon: match row.get("font_awesome_icon").unwrap() {
                gluesql::prelude::Value::Str(font_awesome_icon) => font_awesome_icon.clone(),
                _ => unreachable!("Expected Str")
            },
            icon_color: match row.get("icon_color").unwrap() {
                gluesql::prelude::Value::Str(icon_color) => icon_color.clone(),
                _ => unreachable!("Expected Str")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub public: bool,
    pub state_id: Uuid,
    pub parent_project_id: Option<Uuid>,
    pub budget: Option<f64>,
    pub expenses: Option<f64>,
    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
    pub expected_end_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
}
#[cfg(feature = "frontend")]
impl Project {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            (self.public.into()),
            gluesql::core::ast_builder::expr(self.state_id.to_string()),
            match self.parent_project_id {
                Some(parent_project_id) => gluesql::core::ast_builder::expr(parent_project_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.budget {
                Some(budget) => gluesql::core::ast_builder::num(budget),
                None => gluesql::core::ast_builder::null(),
            },
            match self.expenses {
                Some(expenses) => gluesql::core::ast_builder::num(expenses),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::expr(self.created_by.to_string()),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            match self.expected_end_date {
                Some(expected_end_date) => gluesql::core::ast_builder::timestamp(expected_end_date.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.end_date {
                Some(end_date) => gluesql::core::ast_builder::timestamp(end_date.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the Project into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table projects
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("projects")
            .insert()
            .columns("id, name, description, public, state_id, parent_project_id, budget, expenses, created_by, created_at, expected_end_date, end_date")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Project from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of Project to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, description, public, state_id, parent_project_id, budget, expenses, created_by, created_at, expected_end_date, end_date")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Project from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("projects")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Project from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("projects")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("name", gluesql::core::ast_builder::text(self.name));
        update_row = update_row.set("description", gluesql::core::ast_builder::text(self.description));
        update_row = update_row.set("public", self.public);
        update_row = update_row.set("state_id", gluesql::core::ast_builder::expr(self.state_id.to_string()));
        if let Some(parent_project_id) = self.parent_project_id {
            update_row = update_row.set("parent_project_id", gluesql::core::ast_builder::expr(parent_project_id.to_string()));
        }
        if let Some(budget) = self.budget {
            update_row = update_row.set("budget", gluesql::core::ast_builder::num(budget));
        }
        if let Some(expenses) = self.expenses {
            update_row = update_row.set("expenses", gluesql::core::ast_builder::num(expenses));
        }
        update_row = update_row.set("created_by", gluesql::core::ast_builder::expr(self.created_by.to_string()));
        update_row = update_row.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()));
        if let Some(expected_end_date) = self.expected_end_date {
            update_row = update_row.set("expected_end_date", gluesql::core::ast_builder::timestamp(expected_end_date.to_string()));
        }
        if let Some(end_date) = self.end_date {
            update_row = update_row.set("end_date", gluesql::core::ast_builder::timestamp(end_date.to_string()));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Project from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects")
            .select()
            .project("id, name, description, public, state_id, parent_project_id, budget, expenses, created_by, created_at, expected_end_date, end_date")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str")
            },
            public: match row.get("public").unwrap() {
                gluesql::prelude::Value::Bool(public) => public.clone(),
                _ => unreachable!("Expected Bool")
            },
            state_id: match row.get("state_id").unwrap() {
                gluesql::prelude::Value::Uuid(state_id) => Uuid::from_u128(*state_id),
                _ => unreachable!("Expected Uuid"),
            },
            parent_project_id: match row.get("parent_project_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(parent_project_id) => Some(Uuid::from_u128(*parent_project_id)),
                _ => unreachable!("Expected Uuid"),
            },
            budget: match row.get("budget").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(budget) => Some(budget.clone()),
                _ => unreachable!("Expected F64")
            },
            expenses: match row.get("expenses").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(expenses) => Some(expenses.clone()),
                _ => unreachable!("Expected F64")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::Uuid(created_by) => Uuid::from_u128(*created_by),
                _ => unreachable!("Expected Uuid"),
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            expected_end_date: match row.get("expected_end_date").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Timestamp(expected_end_date) => Some(expected_end_date.clone()),
                _ => unreachable!("Expected Timestamp")
            },
            end_date: match row.get("end_date").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Timestamp(end_date) => Some(end_date.clone()),
                _ => unreachable!("Expected Timestamp")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct Role {
    pub id: Uuid,
}
#[cfg(feature = "frontend")]
impl Role {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
        ]
    }

    /// Insert the Role into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table roles
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("roles")
            .insert()
            .columns("id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Role from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of Role to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("roles")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Role from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("roles")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Role from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("roles")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Role from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("roles")
            .select()
            .project("id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct SampleTaxa {
    pub id: Uuid,
    pub sample_id: Uuid,
    pub taxon_id: Uuid,
}
#[cfg(feature = "frontend")]
impl SampleTaxa {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::expr(self.sample_id.to_string()),
            gluesql::core::ast_builder::expr(self.taxon_id.to_string()),
        ]
    }

    /// Insert the SampleTaxa into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table sample_taxa
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sample_taxa")
            .insert()
            .columns("id, sample_id, taxon_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get SampleTaxa from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of SampleTaxa to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sample_taxa")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, sample_id, taxon_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete SampleTaxa from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sample_taxa")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of SampleTaxa from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("sample_taxa")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("sample_id", gluesql::core::ast_builder::expr(self.sample_id.to_string()));
        update_row = update_row.set("taxon_id", gluesql::core::ast_builder::expr(self.taxon_id.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all SampleTaxa from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sample_taxa")
            .select()
            .project("id, sample_id, taxon_id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            sample_id: match row.get("sample_id").unwrap() {
                gluesql::prelude::Value::Uuid(sample_id) => Uuid::from_u128(*sample_id),
                _ => unreachable!("Expected Uuid"),
            },
            taxon_id: match row.get("taxon_id").unwrap() {
                gluesql::prelude::Value::Uuid(taxon_id) => Uuid::from_u128(*taxon_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct SampledIndividualTaxa {
    pub id: Uuid,
    pub sampled_individual_id: Uuid,
    pub taxon_id: Uuid,
}
#[cfg(feature = "frontend")]
impl SampledIndividualTaxa {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::expr(self.sampled_individual_id.to_string()),
            gluesql::core::ast_builder::expr(self.taxon_id.to_string()),
        ]
    }

    /// Insert the SampledIndividualTaxa into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table sampled_individual_taxa
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sampled_individual_taxa")
            .insert()
            .columns("id, sampled_individual_id, taxon_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get SampledIndividualTaxa from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of SampledIndividualTaxa to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sampled_individual_taxa")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, sampled_individual_id, taxon_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete SampledIndividualTaxa from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sampled_individual_taxa")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of SampledIndividualTaxa from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("sampled_individual_taxa")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("sampled_individual_id", gluesql::core::ast_builder::expr(self.sampled_individual_id.to_string()));
        update_row = update_row.set("taxon_id", gluesql::core::ast_builder::expr(self.taxon_id.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all SampledIndividualTaxa from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sampled_individual_taxa")
            .select()
            .project("id, sampled_individual_id, taxon_id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            sampled_individual_id: match row.get("sampled_individual_id").unwrap() {
                gluesql::prelude::Value::Uuid(sampled_individual_id) => Uuid::from_u128(*sampled_individual_id),
                _ => unreachable!("Expected Uuid"),
            },
            taxon_id: match row.get("taxon_id").unwrap() {
                gluesql::prelude::Value::Uuid(taxon_id) => Uuid::from_u128(*taxon_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct SampledIndividual {
    pub id: Uuid,
}
#[cfg(feature = "frontend")]
impl SampledIndividual {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
        ]
    }

    /// Insert the SampledIndividual into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table sampled_individuals
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sampled_individuals")
            .insert()
            .columns("id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get SampledIndividual from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of SampledIndividual to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sampled_individuals")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete SampledIndividual from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sampled_individuals")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of SampledIndividual from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("sampled_individuals")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all SampledIndividual from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sampled_individuals")
            .select()
            .project("id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct Sample {
    pub id: Uuid,
    pub derived_from: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl Sample {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            match self.derived_from {
                Some(derived_from) => gluesql::core::ast_builder::expr(derived_from.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the Sample into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table samples
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("samples")
            .insert()
            .columns("id, derived_from")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Sample from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of Sample to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("samples")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, derived_from")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Sample from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("samples")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Sample from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("samples")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        if let Some(derived_from) = self.derived_from {
            update_row = update_row.set("derived_from", gluesql::core::ast_builder::expr(derived_from.to_string()));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Sample from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("samples")
            .select()
            .project("id, derived_from")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            derived_from: match row.get("derived_from").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(derived_from) => Some(Uuid::from_u128(*derived_from)),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct Spectra {
    pub id: Uuid,
    pub spectra_collection_id: Uuid,
}
#[cfg(feature = "frontend")]
impl Spectra {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::expr(self.spectra_collection_id.to_string()),
        ]
    }

    /// Insert the Spectra into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table spectra
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("spectra")
            .insert()
            .columns("id, spectra_collection_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Spectra from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of Spectra to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("spectra")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, spectra_collection_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Spectra from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("spectra")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Spectra from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("spectra")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("spectra_collection_id", gluesql::core::ast_builder::expr(self.spectra_collection_id.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Spectra from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("spectra")
            .select()
            .project("id, spectra_collection_id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            spectra_collection_id: match row.get("spectra_collection_id").unwrap() {
                gluesql::prelude::Value::Uuid(spectra_collection_id) => Uuid::from_u128(*spectra_collection_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct SpectraCollection {
    pub id: Uuid,
    pub sample_id: Uuid,
}
#[cfg(feature = "frontend")]
impl SpectraCollection {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::expr(self.sample_id.to_string()),
        ]
    }

    /// Insert the SpectraCollection into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table spectra_collection
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("spectra_collection")
            .insert()
            .columns("id, sample_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get SpectraCollection from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of SpectraCollection to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("spectra_collection")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, sample_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete SpectraCollection from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("spectra_collection")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of SpectraCollection from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("spectra_collection")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("sample_id", gluesql::core::ast_builder::expr(self.sample_id.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all SpectraCollection from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("spectra_collection")
            .select()
            .project("id, sample_id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            sample_id: match row.get("sample_id").unwrap() {
                gluesql::prelude::Value::Uuid(sample_id) => Uuid::from_u128(*sample_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct Taxa {
    pub id: Uuid,
    pub name: String,
    pub ncbi_taxon_id: Option<i32>,
}
#[cfg(feature = "frontend")]
impl Taxa {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::text(self.name),
            match self.ncbi_taxon_id {
                Some(ncbi_taxon_id) => gluesql::core::ast_builder::num(ncbi_taxon_id),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the Taxa into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table taxa
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("taxa")
            .insert()
            .columns("id, name, ncbi_taxon_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Taxa from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of Taxa to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("taxa")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, ncbi_taxon_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Taxa from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("taxa")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Taxa from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("taxa")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("name", gluesql::core::ast_builder::text(self.name));
        if let Some(ncbi_taxon_id) = self.ncbi_taxon_id {
            update_row = update_row.set("ncbi_taxon_id", gluesql::core::ast_builder::num(ncbi_taxon_id));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Taxa from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("taxa")
            .select()
            .project("id, name, ncbi_taxon_id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            ncbi_taxon_id: match row.get("ncbi_taxon_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(ncbi_taxon_id) => Some(ncbi_taxon_id.clone()),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct TeamAuthorization {
    pub id: Uuid,
    pub team_id: Uuid,
    pub editable_id: Uuid,
    pub role_id: Uuid,
}
#[cfg(feature = "frontend")]
impl TeamAuthorization {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::expr(self.team_id.to_string()),
            gluesql::core::ast_builder::expr(self.editable_id.to_string()),
            gluesql::core::ast_builder::expr(self.role_id.to_string()),
        ]
    }

    /// Insert the TeamAuthorization into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table team_authorizations
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("team_authorizations")
            .insert()
            .columns("id, team_id, editable_id, role_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get TeamAuthorization from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of TeamAuthorization to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("team_authorizations")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, team_id, editable_id, role_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete TeamAuthorization from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("team_authorizations")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of TeamAuthorization from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("team_authorizations")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("team_id", gluesql::core::ast_builder::expr(self.team_id.to_string()));
        update_row = update_row.set("editable_id", gluesql::core::ast_builder::expr(self.editable_id.to_string()));
        update_row = update_row.set("role_id", gluesql::core::ast_builder::expr(self.role_id.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all TeamAuthorization from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("team_authorizations")
            .select()
            .project("id, team_id, editable_id, role_id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            team_id: match row.get("team_id").unwrap() {
                gluesql::prelude::Value::Uuid(team_id) => Uuid::from_u128(*team_id),
                _ => unreachable!("Expected Uuid"),
            },
            editable_id: match row.get("editable_id").unwrap() {
                gluesql::prelude::Value::Uuid(editable_id) => Uuid::from_u128(*editable_id),
                _ => unreachable!("Expected Uuid"),
            },
            role_id: match row.get("role_id").unwrap() {
                gluesql::prelude::Value::Uuid(role_id) => Uuid::from_u128(*role_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct TeamState {
    pub id: Uuid,
    pub font_awesome_icon: String,
}
#[cfg(feature = "frontend")]
impl TeamState {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::text(self.font_awesome_icon),
        ]
    }

    /// Insert the TeamState into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table team_states
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("team_states")
            .insert()
            .columns("id, font_awesome_icon")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get TeamState from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of TeamState to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("team_states")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, font_awesome_icon")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete TeamState from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("team_states")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of TeamState from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("team_states")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("font_awesome_icon", gluesql::core::ast_builder::text(self.font_awesome_icon));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all TeamState from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("team_states")
            .select()
            .project("id, font_awesome_icon")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            font_awesome_icon: match row.get("font_awesome_icon").unwrap() {
                gluesql::prelude::Value::Str(font_awesome_icon) => font_awesome_icon.clone(),
                _ => unreachable!("Expected Str")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct Team {
    pub id: Uuid,
    pub parent_team_id: Option<Uuid>,
    pub team_state_id: Uuid,
}
#[cfg(feature = "frontend")]
impl Team {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            match self.parent_team_id {
                Some(parent_team_id) => gluesql::core::ast_builder::expr(parent_team_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::expr(self.team_state_id.to_string()),
        ]
    }

    /// Insert the Team into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table teams
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("teams")
            .insert()
            .columns("id, parent_team_id, team_state_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Team from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of Team to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("teams")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, parent_team_id, team_state_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Team from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("teams")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Team from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("teams")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        if let Some(parent_team_id) = self.parent_team_id {
            update_row = update_row.set("parent_team_id", gluesql::core::ast_builder::expr(parent_team_id.to_string()));
        }
        update_row = update_row.set("team_state_id", gluesql::core::ast_builder::expr(self.team_state_id.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Team from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("teams")
            .select()
            .project("id, parent_team_id, team_state_id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            parent_team_id: match row.get("parent_team_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(parent_team_id) => Some(Uuid::from_u128(*parent_team_id)),
                _ => unreachable!("Expected Uuid"),
            },
            team_state_id: match row.get("team_state_id").unwrap() {
                gluesql::prelude::Value::Uuid(team_state_id) => Uuid::from_u128(*team_state_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct Unit {
    pub id: Uuid,
    pub symbol: String,
}
#[cfg(feature = "frontend")]
impl Unit {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::text(self.symbol),
        ]
    }

    /// Insert the Unit into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table units
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("units")
            .insert()
            .columns("id, symbol")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get Unit from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of Unit to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("units")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, symbol")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete Unit from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("units")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of Unit from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("units")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("symbol", gluesql::core::ast_builder::text(self.symbol));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Unit from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("units")
            .select()
            .project("id, symbol")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            symbol: match row.get("symbol").unwrap() {
                gluesql::prelude::Value::Str(symbol) => symbol.clone(),
                _ => unreachable!("Expected Str")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct UserAuthorization {
    pub id: Uuid,
    pub user_id: Uuid,
    pub editable_id: Uuid,
    pub role_id: Uuid,
}
#[cfg(feature = "frontend")]
impl UserAuthorization {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::expr(self.user_id.to_string()),
            gluesql::core::ast_builder::expr(self.editable_id.to_string()),
            gluesql::core::ast_builder::expr(self.role_id.to_string()),
        ]
    }

    /// Insert the UserAuthorization into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table user_authorizations
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("user_authorizations")
            .insert()
            .columns("id, user_id, editable_id, role_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get UserAuthorization from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of UserAuthorization to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("user_authorizations")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, user_id, editable_id, role_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete UserAuthorization from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("user_authorizations")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of UserAuthorization from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("user_authorizations")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("user_id", gluesql::core::ast_builder::expr(self.user_id.to_string()));
        update_row = update_row.set("editable_id", gluesql::core::ast_builder::expr(self.editable_id.to_string()));
        update_row = update_row.set("role_id", gluesql::core::ast_builder::expr(self.role_id.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all UserAuthorization from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("user_authorizations")
            .select()
            .project("id, user_id, editable_id, role_id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            user_id: match row.get("user_id").unwrap() {
                gluesql::prelude::Value::Uuid(user_id) => Uuid::from_u128(*user_id),
                _ => unreachable!("Expected Uuid"),
            },
            editable_id: match row.get("editable_id").unwrap() {
                gluesql::prelude::Value::Uuid(editable_id) => Uuid::from_u128(*editable_id),
                _ => unreachable!("Expected Uuid"),
            },
            role_id: match row.get("role_id").unwrap() {
                gluesql::prelude::Value::Uuid(role_id) => Uuid::from_u128(*role_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct UserEmail {
    pub id: Uuid,
    pub email: String,
    pub user_id: Uuid,
    pub login_provider_id: Uuid,
}
#[cfg(feature = "frontend")]
impl UserEmail {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::text(self.email),
            gluesql::core::ast_builder::expr(self.user_id.to_string()),
            gluesql::core::ast_builder::expr(self.login_provider_id.to_string()),
        ]
    }

    /// Insert the UserEmail into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table user_emails
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("user_emails")
            .insert()
            .columns("id, email, user_id, login_provider_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get UserEmail from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of UserEmail to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("user_emails")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, email, user_id, login_provider_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete UserEmail from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("user_emails")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of UserEmail from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("user_emails")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("email", gluesql::core::ast_builder::text(self.email));
        update_row = update_row.set("user_id", gluesql::core::ast_builder::expr(self.user_id.to_string()));
        update_row = update_row.set("login_provider_id", gluesql::core::ast_builder::expr(self.login_provider_id.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all UserEmail from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("user_emails")
            .select()
            .project("id, email, user_id, login_provider_id")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            email: match row.get("email").unwrap() {
                gluesql::prelude::Value::Str(email) => email.clone(),
                _ => unreachable!("Expected Str")
            },
            user_id: match row.get("user_id").unwrap() {
                gluesql::prelude::Value::Uuid(user_id) => Uuid::from_u128(*user_id),
                _ => unreachable!("Expected Uuid"),
            },
            login_provider_id: match row.get("login_provider_id").unwrap() {
                gluesql::prelude::Value::Uuid(login_provider_id) => Uuid::from_u128(*login_provider_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#[cfg(feature = "frontend")]
impl User {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::expr(self.id.to_string()),
            gluesql::core::ast_builder::text(self.first_name),
            match self.middle_name {
                Some(middle_name) => gluesql::core::ast_builder::text(middle_name),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::text(self.last_name),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
        ]
    }

    /// Insert the User into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table users
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("users")
            .insert()
            .columns("id, first_name, middle_name, last_name, created_at, updated_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get User from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of User to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("users")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, first_name, middle_name, last_name, created_at, updated_at")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete User from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("users")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of User from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Self::delete_from_id(self.id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("users")
            .update();
        update_row = update_row.set("id", gluesql::core::ast_builder::expr(self.id.to_string()));
        update_row = update_row.set("first_name", gluesql::core::ast_builder::text(self.first_name));
        if let Some(middle_name) = self.middle_name {
            update_row = update_row.set("middle_name", gluesql::core::ast_builder::text(middle_name));
        }
        update_row = update_row.set("last_name", gluesql::core::ast_builder::text(self.last_name));
        update_row = update_row.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()));
        update_row = update_row.set("updated_at", gluesql::core::ast_builder::timestamp(self.updated_at.to_string()));
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated or inserted.
    pub async fn update_or_insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all User from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub async fn all<C>(
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("users")
            .select()
            .project("id, first_name, middle_name, last_name, created_at, updated_at")
            .execute(connection)
            .await?;
        Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            first_name: match row.get("first_name").unwrap() {
                gluesql::prelude::Value::Str(first_name) => first_name.clone(),
                _ => unreachable!("Expected Str")
            },
            middle_name: match row.get("middle_name").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Str(middle_name) => Some(middle_name.clone()),
                _ => unreachable!("Expected Str")
            },
            last_name: match row.get("last_name").unwrap() {
                gluesql::prelude::Value::Str(last_name) => last_name.clone(),
                _ => unreachable!("Expected Str")
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            updated_at: match row.get("updated_at").unwrap() {
                gluesql::prelude::Value::Timestamp(updated_at) => updated_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Copy, Eq, )]
pub enum Table {
    Archivable,
    ContainerHorizontalRule,
    ContainerVerticalRule,
    ContinuousUnit,
    Describable,
    DiscreteUnit,
    DocumentFormat,
    Document,
    Editable,
    Edit,
    ItemCategory,
    ItemCategoryRelationship,
    ItemCategoryUnit,
    ItemContinuousQuantity,
    ItemDiscreteQuantity,
    ItemLocation,
    ItemUnit,
    Item,
    LocationState,
    Location,
    LoginProvider,
    ManufacturedItemCategory,
    Notification,
    OrganizationAuthorization,
    OrganizationLocation,
    OrganizationState,
    Organization,
    PrimaryUserEmail,
    ProcedureContinuousRequirement,
    ProcedureDiscreteRequirement,
    Procedure,
    ProjectContinuousRequirement,
    ProjectDiscreteRequirement,
    ProjectMilestone,
    ProjectState,
    Project,
    Role,
    SampleTaxa,
    SampledIndividualTaxa,
    SampledIndividual,
    Sample,
    Spectra,
    SpectraCollection,
    Taxa,
    TeamAuthorization,
    TeamState,
    Team,
    Unit,
    UserAuthorization,
    UserEmail,
    User,
}

impl Table {
    pub fn name(&self) -> &'static str {
        match self {
            Table::Archivable => "archivables",
            Table::ContainerHorizontalRule => "container_horizontal_rules",
            Table::ContainerVerticalRule => "container_vertical_rules",
            Table::ContinuousUnit => "continuous_units",
            Table::Describable => "describables",
            Table::DiscreteUnit => "discrete_units",
            Table::DocumentFormat => "document_formats",
            Table::Document => "documents",
            Table::Editable => "editables",
            Table::Edit => "edits",
            Table::ItemCategory => "item_categories",
            Table::ItemCategoryRelationship => "item_category_relationships",
            Table::ItemCategoryUnit => "item_category_units",
            Table::ItemContinuousQuantity => "item_continuous_quantities",
            Table::ItemDiscreteQuantity => "item_discrete_quantities",
            Table::ItemLocation => "item_locations",
            Table::ItemUnit => "item_units",
            Table::Item => "items",
            Table::LocationState => "location_states",
            Table::Location => "locations",
            Table::LoginProvider => "login_providers",
            Table::ManufacturedItemCategory => "manufactured_item_categories",
            Table::Notification => "notifications",
            Table::OrganizationAuthorization => "organization_authorizations",
            Table::OrganizationLocation => "organization_locations",
            Table::OrganizationState => "organization_states",
            Table::Organization => "organizations",
            Table::PrimaryUserEmail => "primary_user_emails",
            Table::ProcedureContinuousRequirement => "procedure_continuous_requirements",
            Table::ProcedureDiscreteRequirement => "procedure_discrete_requirements",
            Table::Procedure => "procedures",
            Table::ProjectContinuousRequirement => "project_continuous_requirements",
            Table::ProjectDiscreteRequirement => "project_discrete_requirements",
            Table::ProjectMilestone => "project_milestones",
            Table::ProjectState => "project_states",
            Table::Project => "projects",
            Table::Role => "roles",
            Table::SampleTaxa => "sample_taxa",
            Table::SampledIndividualTaxa => "sampled_individual_taxa",
            Table::SampledIndividual => "sampled_individuals",
            Table::Sample => "samples",
            Table::Spectra => "spectra",
            Table::SpectraCollection => "spectra_collection",
            Table::Taxa => "taxa",
            Table::TeamAuthorization => "team_authorizations",
            Table::TeamState => "team_states",
            Table::Team => "teams",
            Table::Unit => "units",
            Table::UserAuthorization => "user_authorizations",
            Table::UserEmail => "user_emails",
            Table::User => "users",
        }
    }
}
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub enum TableRow {
    Archivable(Archivable),
    ContainerHorizontalRule(ContainerHorizontalRule),
    ContainerVerticalRule(ContainerVerticalRule),
    ContinuousUnit(ContinuousUnit),
    Describable(Describable),
    DiscreteUnit(DiscreteUnit),
    DocumentFormat(DocumentFormat),
    Document(Document),
    Editable(Editable),
    Edit(Edit),
    ItemCategory(ItemCategory),
    ItemCategoryRelationship(ItemCategoryRelationship),
    ItemCategoryUnit(ItemCategoryUnit),
    ItemContinuousQuantity(ItemContinuousQuantity),
    ItemDiscreteQuantity(ItemDiscreteQuantity),
    ItemLocation(ItemLocation),
    ItemUnit(ItemUnit),
    Item(Item),
    LocationState(LocationState),
    Location(Location),
    LoginProvider(LoginProvider),
    ManufacturedItemCategory(ManufacturedItemCategory),
    Notification(Notification),
    OrganizationAuthorization(OrganizationAuthorization),
    OrganizationLocation(OrganizationLocation),
    OrganizationState(OrganizationState),
    Organization(Organization),
    PrimaryUserEmail(PrimaryUserEmail),
    ProcedureContinuousRequirement(ProcedureContinuousRequirement),
    ProcedureDiscreteRequirement(ProcedureDiscreteRequirement),
    Procedure(Procedure),
    ProjectContinuousRequirement(ProjectContinuousRequirement),
    ProjectDiscreteRequirement(ProjectDiscreteRequirement),
    ProjectMilestone(ProjectMilestone),
    ProjectState(ProjectState),
    Project(Project),
    Role(Role),
    SampleTaxa(SampleTaxa),
    SampledIndividualTaxa(SampledIndividualTaxa),
    SampledIndividual(SampledIndividual),
    Sample(Sample),
    Spectra(Spectra),
    SpectraCollection(SpectraCollection),
    Taxa(Taxa),
    TeamAuthorization(TeamAuthorization),
    TeamState(TeamState),
    Team(Team),
    Unit(Unit),
    UserAuthorization(UserAuthorization),
    UserEmail(UserEmail),
    User(User),
}

impl From<&str> for Table {
    fn from(item: &str) -> Self {
        match item {
            "archivables" => Table::Archivable,
            "container_horizontal_rules" => Table::ContainerHorizontalRule,
            "container_vertical_rules" => Table::ContainerVerticalRule,
            "continuous_units" => Table::ContinuousUnit,
            "describables" => Table::Describable,
            "discrete_units" => Table::DiscreteUnit,
            "document_formats" => Table::DocumentFormat,
            "documents" => Table::Document,
            "editables" => Table::Editable,
            "edits" => Table::Edit,
            "item_categories" => Table::ItemCategory,
            "item_category_relationships" => Table::ItemCategoryRelationship,
            "item_category_units" => Table::ItemCategoryUnit,
            "item_continuous_quantities" => Table::ItemContinuousQuantity,
            "item_discrete_quantities" => Table::ItemDiscreteQuantity,
            "item_locations" => Table::ItemLocation,
            "item_units" => Table::ItemUnit,
            "items" => Table::Item,
            "location_states" => Table::LocationState,
            "locations" => Table::Location,
            "login_providers" => Table::LoginProvider,
            "manufactured_item_categories" => Table::ManufacturedItemCategory,
            "notifications" => Table::Notification,
            "organization_authorizations" => Table::OrganizationAuthorization,
            "organization_locations" => Table::OrganizationLocation,
            "organization_states" => Table::OrganizationState,
            "organizations" => Table::Organization,
            "primary_user_emails" => Table::PrimaryUserEmail,
            "procedure_continuous_requirements" => Table::ProcedureContinuousRequirement,
            "procedure_discrete_requirements" => Table::ProcedureDiscreteRequirement,
            "procedures" => Table::Procedure,
            "project_continuous_requirements" => Table::ProjectContinuousRequirement,
            "project_discrete_requirements" => Table::ProjectDiscreteRequirement,
            "project_milestones" => Table::ProjectMilestone,
            "project_states" => Table::ProjectState,
            "projects" => Table::Project,
            "roles" => Table::Role,
            "sample_taxa" => Table::SampleTaxa,
            "sampled_individual_taxa" => Table::SampledIndividualTaxa,
            "sampled_individuals" => Table::SampledIndividual,
            "samples" => Table::Sample,
            "spectra" => Table::Spectra,
            "spectra_collection" => Table::SpectraCollection,
            "taxa" => Table::Taxa,
            "team_authorizations" => Table::TeamAuthorization,
            "team_states" => Table::TeamState,
            "teams" => Table::Team,
            "units" => Table::Unit,
            "user_authorizations" => Table::UserAuthorization,
            "user_emails" => Table::UserEmail,
            "users" => Table::User,
            _ => panic!("Unknown table name"),
        }
    }
}
impl TableRow {
    pub fn table(&self) -> &'static Table {
        match self {
            TableRow::Archivable(_) => &Table::Archivable,
            TableRow::ContainerHorizontalRule(_) => &Table::ContainerHorizontalRule,
            TableRow::ContainerVerticalRule(_) => &Table::ContainerVerticalRule,
            TableRow::ContinuousUnit(_) => &Table::ContinuousUnit,
            TableRow::Describable(_) => &Table::Describable,
            TableRow::DiscreteUnit(_) => &Table::DiscreteUnit,
            TableRow::DocumentFormat(_) => &Table::DocumentFormat,
            TableRow::Document(_) => &Table::Document,
            TableRow::Editable(_) => &Table::Editable,
            TableRow::Edit(_) => &Table::Edit,
            TableRow::ItemCategory(_) => &Table::ItemCategory,
            TableRow::ItemCategoryRelationship(_) => &Table::ItemCategoryRelationship,
            TableRow::ItemCategoryUnit(_) => &Table::ItemCategoryUnit,
            TableRow::ItemContinuousQuantity(_) => &Table::ItemContinuousQuantity,
            TableRow::ItemDiscreteQuantity(_) => &Table::ItemDiscreteQuantity,
            TableRow::ItemLocation(_) => &Table::ItemLocation,
            TableRow::ItemUnit(_) => &Table::ItemUnit,
            TableRow::Item(_) => &Table::Item,
            TableRow::LocationState(_) => &Table::LocationState,
            TableRow::Location(_) => &Table::Location,
            TableRow::LoginProvider(_) => &Table::LoginProvider,
            TableRow::ManufacturedItemCategory(_) => &Table::ManufacturedItemCategory,
            TableRow::Notification(_) => &Table::Notification,
            TableRow::OrganizationAuthorization(_) => &Table::OrganizationAuthorization,
            TableRow::OrganizationLocation(_) => &Table::OrganizationLocation,
            TableRow::OrganizationState(_) => &Table::OrganizationState,
            TableRow::Organization(_) => &Table::Organization,
            TableRow::PrimaryUserEmail(_) => &Table::PrimaryUserEmail,
            TableRow::ProcedureContinuousRequirement(_) => &Table::ProcedureContinuousRequirement,
            TableRow::ProcedureDiscreteRequirement(_) => &Table::ProcedureDiscreteRequirement,
            TableRow::Procedure(_) => &Table::Procedure,
            TableRow::ProjectContinuousRequirement(_) => &Table::ProjectContinuousRequirement,
            TableRow::ProjectDiscreteRequirement(_) => &Table::ProjectDiscreteRequirement,
            TableRow::ProjectMilestone(_) => &Table::ProjectMilestone,
            TableRow::ProjectState(_) => &Table::ProjectState,
            TableRow::Project(_) => &Table::Project,
            TableRow::Role(_) => &Table::Role,
            TableRow::SampleTaxa(_) => &Table::SampleTaxa,
            TableRow::SampledIndividualTaxa(_) => &Table::SampledIndividualTaxa,
            TableRow::SampledIndividual(_) => &Table::SampledIndividual,
            TableRow::Sample(_) => &Table::Sample,
            TableRow::Spectra(_) => &Table::Spectra,
            TableRow::SpectraCollection(_) => &Table::SpectraCollection,
            TableRow::Taxa(_) => &Table::Taxa,
            TableRow::TeamAuthorization(_) => &Table::TeamAuthorization,
            TableRow::TeamState(_) => &Table::TeamState,
            TableRow::Team(_) => &Table::Team,
            TableRow::Unit(_) => &Table::Unit,
            TableRow::UserAuthorization(_) => &Table::UserAuthorization,
            TableRow::UserEmail(_) => &Table::UserEmail,
            TableRow::User(_) => &Table::User,
        }
    }

    pub fn table_name(&self) -> &'static str {
        self.table().name()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Copy, Eq, )]
pub enum SearcheableTable {
    ProjectState,
    Project,
    Taxa,
    User,
}

