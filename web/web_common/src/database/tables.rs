use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::Utc;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct ContainerHorizontalRule {
    pub id: i32,
    pub created_by: i32,
    pub name: String,
    pub item_type_id: i32,
    pub other_item_type_id: i32,
    pub minimum_temperature: Option<i32>,
    pub maximum_temperature: Option<i32>,
    pub minimum_humidity: Option<i32>,
    pub maximum_humidity: Option<i32>,
    pub minimum_pressure: Option<i32>,
    pub maximum_pressure: Option<i32>,
}
#[cfg(feature = "frontend")]
impl ContainerHorizontalRule {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::num(self.item_type_id),
            gluesql::core::ast_builder::num(self.other_item_type_id),
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
    /// The number of rows inserted in table ContainerHorizontalRule
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("container_horizontal_rules")
            .insert()
            .columns("id, created_by, name, item_type_id, other_item_type_id, minimum_temperature, maximum_temperature, minimum_humidity, maximum_humidity, minimum_pressure, maximum_pressure")
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
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("container_horizontal_rules")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, created_by, name, item_type_id, other_item_type_id, minimum_temperature, maximum_temperature, minimum_humidity, maximum_humidity, minimum_pressure, maximum_pressure")
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
        id: i32,
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
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("item_type_id", gluesql::core::ast_builder::num(self.item_type_id))        
.set("other_item_type_id", gluesql::core::ast_builder::num(self.other_item_type_id));
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
            .project("id, created_by, name, item_type_id, other_item_type_id, minimum_temperature, maximum_temperature, minimum_humidity, maximum_humidity, minimum_pressure, maximum_pressure")
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            item_type_id: match row.get("item_type_id").unwrap() {
                gluesql::prelude::Value::I32(item_type_id) => item_type_id.clone(),
                _ => unreachable!("Expected I32")
            },
            other_item_type_id: match row.get("other_item_type_id").unwrap() {
                gluesql::prelude::Value::I32(other_item_type_id) => other_item_type_id.clone(),
                _ => unreachable!("Expected I32")
            },
            minimum_temperature: match row.get("minimum_temperature").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(minimum_temperature) => Some(minimum_temperature.clone()),
                _ => unreachable!("Expected I32")
            },
            maximum_temperature: match row.get("maximum_temperature").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(maximum_temperature) => Some(maximum_temperature.clone()),
                _ => unreachable!("Expected I32")
            },
            minimum_humidity: match row.get("minimum_humidity").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(minimum_humidity) => Some(minimum_humidity.clone()),
                _ => unreachable!("Expected I32")
            },
            maximum_humidity: match row.get("maximum_humidity").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(maximum_humidity) => Some(maximum_humidity.clone()),
                _ => unreachable!("Expected I32")
            },
            minimum_pressure: match row.get("minimum_pressure").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(minimum_pressure) => Some(minimum_pressure.clone()),
                _ => unreachable!("Expected I32")
            },
            maximum_pressure: match row.get("maximum_pressure").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(maximum_pressure) => Some(maximum_pressure.clone()),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct ContainerVerticalRule {
    pub id: i32,
    pub created_by: i32,
    pub name: String,
    pub container_item_type_id: i32,
    pub contained_item_type_id: i32,
    pub minimum_temperature: Option<i32>,
    pub maximum_temperature: Option<i32>,
    pub minimum_humidity: Option<i32>,
    pub maximum_humidity: Option<i32>,
    pub minimum_pressure: Option<i32>,
    pub maximum_pressure: Option<i32>,
}
#[cfg(feature = "frontend")]
impl ContainerVerticalRule {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::num(self.container_item_type_id),
            gluesql::core::ast_builder::num(self.contained_item_type_id),
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
    /// The number of rows inserted in table ContainerVerticalRule
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("container_vertical_rules")
            .insert()
            .columns("id, created_by, name, container_item_type_id, contained_item_type_id, minimum_temperature, maximum_temperature, minimum_humidity, maximum_humidity, minimum_pressure, maximum_pressure")
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
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("container_vertical_rules")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, created_by, name, container_item_type_id, contained_item_type_id, minimum_temperature, maximum_temperature, minimum_humidity, maximum_humidity, minimum_pressure, maximum_pressure")
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
        id: i32,
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
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("container_item_type_id", gluesql::core::ast_builder::num(self.container_item_type_id))        
.set("contained_item_type_id", gluesql::core::ast_builder::num(self.contained_item_type_id));
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
            .project("id, created_by, name, container_item_type_id, contained_item_type_id, minimum_temperature, maximum_temperature, minimum_humidity, maximum_humidity, minimum_pressure, maximum_pressure")
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            container_item_type_id: match row.get("container_item_type_id").unwrap() {
                gluesql::prelude::Value::I32(container_item_type_id) => container_item_type_id.clone(),
                _ => unreachable!("Expected I32")
            },
            contained_item_type_id: match row.get("contained_item_type_id").unwrap() {
                gluesql::prelude::Value::I32(contained_item_type_id) => contained_item_type_id.clone(),
                _ => unreachable!("Expected I32")
            },
            minimum_temperature: match row.get("minimum_temperature").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(minimum_temperature) => Some(minimum_temperature.clone()),
                _ => unreachable!("Expected I32")
            },
            maximum_temperature: match row.get("maximum_temperature").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(maximum_temperature) => Some(maximum_temperature.clone()),
                _ => unreachable!("Expected I32")
            },
            minimum_humidity: match row.get("minimum_humidity").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(minimum_humidity) => Some(minimum_humidity.clone()),
                _ => unreachable!("Expected I32")
            },
            maximum_humidity: match row.get("maximum_humidity").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(maximum_humidity) => Some(maximum_humidity.clone()),
                _ => unreachable!("Expected I32")
            },
            minimum_pressure: match row.get("minimum_pressure").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(minimum_pressure) => Some(minimum_pressure.clone()),
                _ => unreachable!("Expected I32")
            },
            maximum_pressure: match row.get("maximum_pressure").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(maximum_pressure) => Some(maximum_pressure.clone()),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct ContinuousUnit {
    pub id: i32,
}
#[cfg(feature = "frontend")]
impl ContinuousUnit {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
        ]
    }

    /// Insert the ContinuousUnit into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table ContinuousUnit
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
        id: i32,
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
        id: i32,
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
        table("continuous_units")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))            .execute(connection)
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct DiscreteUnit {
    pub id: i32,
}
#[cfg(feature = "frontend")]
impl DiscreteUnit {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
        ]
    }

    /// Insert the DiscreteUnit into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table DiscreteUnit
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
        id: i32,
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
        id: i32,
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
        table("discrete_units")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))            .execute(connection)
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct DocumentFormat {
    pub id: i32,
    pub extension: String,
    pub mime_type: String,
}
#[cfg(feature = "frontend")]
impl DocumentFormat {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.extension),
            gluesql::core::ast_builder::text(self.mime_type),
        ]
    }

    /// Insert the DocumentFormat into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table DocumentFormat
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("document_formats")
            .insert()
            .columns("id, extension, mime_type")
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
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("document_formats")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, extension, mime_type")
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
        id: i32,
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
        table("document_formats")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("extension", gluesql::core::ast_builder::text(self.extension))        
.set("mime_type", gluesql::core::ast_builder::text(self.mime_type))            .execute(connection)
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
            .project("id, extension, mime_type")
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            extension: match row.get("extension").unwrap() {
                gluesql::prelude::Value::Str(extension) => extension.clone(),
                _ => unreachable!("Expected Str")
            },
            mime_type: match row.get("mime_type").unwrap() {
                gluesql::prelude::Value::Str(mime_type) => mime_type.clone(),
                _ => unreachable!("Expected Str")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct Document {
    pub id: Uuid,
    pub author_id: i32,
    pub path: String,
    pub format_id: i32,
    pub bytes: i32,
}
#[cfg(feature = "frontend")]
impl Document {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::uuid(self.id.to_string()),
            gluesql::core::ast_builder::num(self.author_id),
            gluesql::core::ast_builder::text(self.path),
            gluesql::core::ast_builder::num(self.format_id),
            gluesql::core::ast_builder::num(self.bytes),
        ]
    }

    /// Insert the Document into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table Document
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("documents")
            .insert()
            .columns("id, author_id, path, format_id, bytes")
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
        id: uuid::Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("documents")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, author_id, path, format_id, bytes")
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
        id: uuid::Uuid,
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
        table("documents")
            .update()        
.set("id", gluesql::core::ast_builder::uuid(self.id.to_string()))        
.set("author_id", gluesql::core::ast_builder::num(self.author_id))        
.set("path", gluesql::core::ast_builder::text(self.path))        
.set("format_id", gluesql::core::ast_builder::num(self.format_id))        
.set("bytes", gluesql::core::ast_builder::num(self.bytes))            .execute(connection)
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
            .project("id, author_id, path, format_id, bytes")
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
            author_id: match row.get("author_id").unwrap() {
                gluesql::prelude::Value::I32(author_id) => author_id.clone(),
                _ => unreachable!("Expected I32")
            },
            path: match row.get("path").unwrap() {
                gluesql::prelude::Value::Str(path) => path.clone(),
                _ => unreachable!("Expected Str")
            },
            format_id: match row.get("format_id").unwrap() {
                gluesql::prelude::Value::I32(format_id) => format_id.clone(),
                _ => unreachable!("Expected I32")
            },
            bytes: match row.get("bytes").unwrap() {
                gluesql::prelude::Value::I32(bytes) => bytes.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct ItemCategory {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_by: i32,
}
#[cfg(feature = "frontend")]
impl ItemCategory {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            gluesql::core::ast_builder::num(self.created_by),
        ]
    }

    /// Insert the ItemCategory into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table ItemCategory
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("item_categories")
            .insert()
            .columns("id, name, description, created_by")
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
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_categories")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, description, created_by")
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
        id: i32,
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
        table("item_categories")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("description", gluesql::core::ast_builder::text(self.description))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))            .execute(connection)
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
            .project("id, name, description, created_by")
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct ItemCategoryRelationship {
    pub id: i32,
    pub parent_id: i32,
    pub child_id: i32,
    pub added_by: i32,
}
#[cfg(feature = "frontend")]
impl ItemCategoryRelationship {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::num(self.parent_id),
            gluesql::core::ast_builder::num(self.child_id),
            gluesql::core::ast_builder::num(self.added_by),
        ]
    }

    /// Insert the ItemCategoryRelationship into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table ItemCategoryRelationship
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("item_category_relationships")
            .insert()
            .columns("id, parent_id, child_id, added_by")
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
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_category_relationships")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, parent_id, child_id, added_by")
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
        id: i32,
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
        table("item_category_relationships")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("parent_id", gluesql::core::ast_builder::num(self.parent_id))        
.set("child_id", gluesql::core::ast_builder::num(self.child_id))        
.set("added_by", gluesql::core::ast_builder::num(self.added_by))            .execute(connection)
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
            .project("id, parent_id, child_id, added_by")
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            parent_id: match row.get("parent_id").unwrap() {
                gluesql::prelude::Value::I32(parent_id) => parent_id.clone(),
                _ => unreachable!("Expected I32")
            },
            child_id: match row.get("child_id").unwrap() {
                gluesql::prelude::Value::I32(child_id) => child_id.clone(),
                _ => unreachable!("Expected I32")
            },
            added_by: match row.get("added_by").unwrap() {
                gluesql::prelude::Value::I32(added_by) => added_by.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct ItemCategoryUnit {
    pub id: i32,
    pub item_category_id: i32,
    pub unit_id: i32,
}
#[cfg(feature = "frontend")]
impl ItemCategoryUnit {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::num(self.item_category_id),
            gluesql::core::ast_builder::num(self.unit_id),
        ]
    }

    /// Insert the ItemCategoryUnit into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table ItemCategoryUnit
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
        id: i32,
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
        id: i32,
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
        table("item_category_units")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("item_category_id", gluesql::core::ast_builder::num(self.item_category_id))        
.set("unit_id", gluesql::core::ast_builder::num(self.unit_id))            .execute(connection)
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            item_category_id: match row.get("item_category_id").unwrap() {
                gluesql::prelude::Value::I32(item_category_id) => item_category_id.clone(),
                _ => unreachable!("Expected I32")
            },
            unit_id: match row.get("unit_id").unwrap() {
                gluesql::prelude::Value::I32(unit_id) => unit_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct ItemContinuousQuantity {
    pub id: Uuid,
    pub item_id: Uuid,
    pub amount: i32,
    pub unit_id: i32,
    pub sensor_id: Option<Uuid>,
    pub measured_at: NaiveDateTime,
    pub measured_by: Option<i32>,
}
#[cfg(feature = "frontend")]
impl ItemContinuousQuantity {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::uuid(self.id.to_string()),
            gluesql::core::ast_builder::uuid(self.item_id.to_string()),
            gluesql::core::ast_builder::num(self.amount),
            gluesql::core::ast_builder::num(self.unit_id),
            match self.sensor_id {
                Some(sensor_id) => gluesql::core::ast_builder::uuid(sensor_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::timestamp(self.measured_at.to_string()),
            match self.measured_by {
                Some(measured_by) => gluesql::core::ast_builder::num(measured_by),
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
    /// The number of rows inserted in table ItemContinuousQuantity
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("item_continuous_quantities")
            .insert()
            .columns("id, item_id, amount, unit_id, sensor_id, measured_at, measured_by")
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
        id: uuid::Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_continuous_quantities")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, item_id, amount, unit_id, sensor_id, measured_at, measured_by")
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
        id: uuid::Uuid,
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
            .update()        
.set("id", gluesql::core::ast_builder::uuid(self.id.to_string()))        
.set("item_id", gluesql::core::ast_builder::uuid(self.item_id.to_string()))        
.set("amount", gluesql::core::ast_builder::num(self.amount))        
.set("unit_id", gluesql::core::ast_builder::num(self.unit_id))        
.set("measured_at", gluesql::core::ast_builder::timestamp(self.measured_at.to_string()));
        if let Some(sensor_id) = self.sensor_id {
            update_row = update_row.set("sensor_id", gluesql::core::ast_builder::uuid(sensor_id.to_string()));
        }
        if let Some(measured_by) = self.measured_by {
            update_row = update_row.set("measured_by", gluesql::core::ast_builder::num(measured_by));
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
            .project("id, item_id, amount, unit_id, sensor_id, measured_at, measured_by")
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
            amount: match row.get("amount").unwrap() {
                gluesql::prelude::Value::I32(amount) => amount.clone(),
                _ => unreachable!("Expected I32")
            },
            unit_id: match row.get("unit_id").unwrap() {
                gluesql::prelude::Value::I32(unit_id) => unit_id.clone(),
                _ => unreachable!("Expected I32")
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
                gluesql::prelude::Value::I32(measured_by) => Some(measured_by.clone()),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct ItemDiscreteQuantity {
    pub id: Uuid,
    pub item_id: Uuid,
    pub quantity: i32,
    pub unit_id: i32,
    pub measured_at: NaiveDateTime,
    pub measured_by: Option<i32>,
}
#[cfg(feature = "frontend")]
impl ItemDiscreteQuantity {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::uuid(self.id.to_string()),
            gluesql::core::ast_builder::uuid(self.item_id.to_string()),
            gluesql::core::ast_builder::num(self.quantity),
            gluesql::core::ast_builder::num(self.unit_id),
            gluesql::core::ast_builder::timestamp(self.measured_at.to_string()),
            match self.measured_by {
                Some(measured_by) => gluesql::core::ast_builder::num(measured_by),
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
    /// The number of rows inserted in table ItemDiscreteQuantity
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
        id: uuid::Uuid,
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
        id: uuid::Uuid,
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
            .update()        
.set("id", gluesql::core::ast_builder::uuid(self.id.to_string()))        
.set("item_id", gluesql::core::ast_builder::uuid(self.item_id.to_string()))        
.set("quantity", gluesql::core::ast_builder::num(self.quantity))        
.set("unit_id", gluesql::core::ast_builder::num(self.unit_id))        
.set("measured_at", gluesql::core::ast_builder::timestamp(self.measured_at.to_string()));
        if let Some(measured_by) = self.measured_by {
            update_row = update_row.set("measured_by", gluesql::core::ast_builder::num(measured_by));
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
                gluesql::prelude::Value::Uuid(item_id) => Uuid::from_u128(*item_id),
                _ => unreachable!("Expected Uuid"),
            },
            quantity: match row.get("quantity").unwrap() {
                gluesql::prelude::Value::I32(quantity) => quantity.clone(),
                _ => unreachable!("Expected I32")
            },
            unit_id: match row.get("unit_id").unwrap() {
                gluesql::prelude::Value::I32(unit_id) => unit_id.clone(),
                _ => unreachable!("Expected I32")
            },
            measured_at: match row.get("measured_at").unwrap() {
                gluesql::prelude::Value::Timestamp(measured_at) => measured_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            measured_by: match row.get("measured_by").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(measured_by) => Some(measured_by.clone()),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct ItemLocation {
    pub id: Uuid,
    pub item_id: Option<Uuid>,
    pub located_by: Option<i32>,
    pub located_at: NaiveDateTime,
    pub location_id: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl ItemLocation {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::uuid(self.id.to_string()),
            match self.item_id {
                Some(item_id) => gluesql::core::ast_builder::uuid(item_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.located_by {
                Some(located_by) => gluesql::core::ast_builder::num(located_by),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::timestamp(self.located_at.to_string()),
            match self.location_id {
                Some(location_id) => gluesql::core::ast_builder::uuid(location_id.to_string()),
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
    /// The number of rows inserted in table ItemLocation
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("item_locations")
            .insert()
            .columns("id, item_id, located_by, located_at, location_id")
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
        id: uuid::Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_locations")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, item_id, located_by, located_at, location_id")
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
        id: uuid::Uuid,
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
            .update()        
.set("id", gluesql::core::ast_builder::uuid(self.id.to_string()))        
.set("located_at", gluesql::core::ast_builder::timestamp(self.located_at.to_string()));
        if let Some(item_id) = self.item_id {
            update_row = update_row.set("item_id", gluesql::core::ast_builder::uuid(item_id.to_string()));
        }
        if let Some(located_by) = self.located_by {
            update_row = update_row.set("located_by", gluesql::core::ast_builder::num(located_by));
        }
        if let Some(location_id) = self.location_id {
            update_row = update_row.set("location_id", gluesql::core::ast_builder::uuid(location_id.to_string()));
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
            .project("id, item_id, located_by, located_at, location_id")
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
            located_by: match row.get("located_by").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(located_by) => Some(located_by.clone()),
                _ => unreachable!("Expected I32")
            },
            located_at: match row.get("located_at").unwrap() {
                gluesql::prelude::Value::Timestamp(located_at) => located_at.clone(),
                _ => unreachable!("Expected Timestamp")
            },
            location_id: match row.get("location_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(location_id) => Some(Uuid::from_u128(*location_id)),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct ItemUnit {
    pub id: Uuid,
    pub item_id: Uuid,
    pub unit_id: i32,
}
#[cfg(feature = "frontend")]
impl ItemUnit {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::uuid(self.id.to_string()),
            gluesql::core::ast_builder::uuid(self.item_id.to_string()),
            gluesql::core::ast_builder::num(self.unit_id),
        ]
    }

    /// Insert the ItemUnit into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table ItemUnit
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
        id: uuid::Uuid,
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
        id: uuid::Uuid,
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
        table("item_units")
            .update()        
.set("id", gluesql::core::ast_builder::uuid(self.id.to_string()))        
.set("item_id", gluesql::core::ast_builder::uuid(self.item_id.to_string()))        
.set("unit_id", gluesql::core::ast_builder::num(self.unit_id))            .execute(connection)
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
                gluesql::prelude::Value::I32(unit_id) => unit_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct Item {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl Item {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::uuid(self.id.to_string()),
            match self.parent_id {
                Some(parent_id) => gluesql::core::ast_builder::uuid(parent_id.to_string()),
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
    /// The number of rows inserted in table Item
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
        id: uuid::Uuid,
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
        id: uuid::Uuid,
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
            .update()        
.set("id", gluesql::core::ast_builder::uuid(self.id.to_string()));
        if let Some(parent_id) = self.parent_id {
            update_row = update_row.set("parent_id", gluesql::core::ast_builder::uuid(parent_id.to_string()));
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
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct Location {
    pub id: Uuid,
    pub latitude_degrees: Option<i32>,
    pub latitude_minutes: Option<i32>,
    pub latitude_seconds: Option<i32>,
    pub longitude_degrees: Option<i32>,
    pub longitude_minutes: Option<i32>,
    pub longitude_seconds: Option<i32>,
    pub altitude: Option<i32>,
    pub address: Option<String>,
    pub geolocalization_device_id: Option<Uuid>,
    pub altitude_device_id: Option<Uuid>,
    pub parent_location_id: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl Location {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::uuid(self.id.to_string()),
            match self.latitude_degrees {
                Some(latitude_degrees) => gluesql::core::ast_builder::num(latitude_degrees),
                None => gluesql::core::ast_builder::null(),
            },
            match self.latitude_minutes {
                Some(latitude_minutes) => gluesql::core::ast_builder::num(latitude_minutes),
                None => gluesql::core::ast_builder::null(),
            },
            match self.latitude_seconds {
                Some(latitude_seconds) => gluesql::core::ast_builder::num(latitude_seconds),
                None => gluesql::core::ast_builder::null(),
            },
            match self.longitude_degrees {
                Some(longitude_degrees) => gluesql::core::ast_builder::num(longitude_degrees),
                None => gluesql::core::ast_builder::null(),
            },
            match self.longitude_minutes {
                Some(longitude_minutes) => gluesql::core::ast_builder::num(longitude_minutes),
                None => gluesql::core::ast_builder::null(),
            },
            match self.longitude_seconds {
                Some(longitude_seconds) => gluesql::core::ast_builder::num(longitude_seconds),
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
                Some(geolocalization_device_id) => gluesql::core::ast_builder::uuid(geolocalization_device_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.altitude_device_id {
                Some(altitude_device_id) => gluesql::core::ast_builder::uuid(altitude_device_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.parent_location_id {
                Some(parent_location_id) => gluesql::core::ast_builder::uuid(parent_location_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the Location into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table Location
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("locations")
            .insert()
            .columns("id, latitude_degrees, latitude_minutes, latitude_seconds, longitude_degrees, longitude_minutes, longitude_seconds, altitude, address, geolocalization_device_id, altitude_device_id, parent_location_id")
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
        id: uuid::Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("locations")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, latitude_degrees, latitude_minutes, latitude_seconds, longitude_degrees, longitude_minutes, longitude_seconds, altitude, address, geolocalization_device_id, altitude_device_id, parent_location_id")
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
        id: uuid::Uuid,
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
            .update()        
.set("id", gluesql::core::ast_builder::uuid(self.id.to_string()));
        if let Some(latitude_degrees) = self.latitude_degrees {
            update_row = update_row.set("latitude_degrees", gluesql::core::ast_builder::num(latitude_degrees));
        }
        if let Some(latitude_minutes) = self.latitude_minutes {
            update_row = update_row.set("latitude_minutes", gluesql::core::ast_builder::num(latitude_minutes));
        }
        if let Some(latitude_seconds) = self.latitude_seconds {
            update_row = update_row.set("latitude_seconds", gluesql::core::ast_builder::num(latitude_seconds));
        }
        if let Some(longitude_degrees) = self.longitude_degrees {
            update_row = update_row.set("longitude_degrees", gluesql::core::ast_builder::num(longitude_degrees));
        }
        if let Some(longitude_minutes) = self.longitude_minutes {
            update_row = update_row.set("longitude_minutes", gluesql::core::ast_builder::num(longitude_minutes));
        }
        if let Some(longitude_seconds) = self.longitude_seconds {
            update_row = update_row.set("longitude_seconds", gluesql::core::ast_builder::num(longitude_seconds));
        }
        if let Some(altitude) = self.altitude {
            update_row = update_row.set("altitude", gluesql::core::ast_builder::num(altitude));
        }
        if let Some(address) = self.address {
            update_row = update_row.set("address", gluesql::core::ast_builder::text(address));
        }
        if let Some(geolocalization_device_id) = self.geolocalization_device_id {
            update_row = update_row.set("geolocalization_device_id", gluesql::core::ast_builder::uuid(geolocalization_device_id.to_string()));
        }
        if let Some(altitude_device_id) = self.altitude_device_id {
            update_row = update_row.set("altitude_device_id", gluesql::core::ast_builder::uuid(altitude_device_id.to_string()));
        }
        if let Some(parent_location_id) = self.parent_location_id {
            update_row = update_row.set("parent_location_id", gluesql::core::ast_builder::uuid(parent_location_id.to_string()));
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
            .project("id, latitude_degrees, latitude_minutes, latitude_seconds, longitude_degrees, longitude_minutes, longitude_seconds, altitude, address, geolocalization_device_id, altitude_device_id, parent_location_id")
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
            latitude_degrees: match row.get("latitude_degrees").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(latitude_degrees) => Some(latitude_degrees.clone()),
                _ => unreachable!("Expected I32")
            },
            latitude_minutes: match row.get("latitude_minutes").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(latitude_minutes) => Some(latitude_minutes.clone()),
                _ => unreachable!("Expected I32")
            },
            latitude_seconds: match row.get("latitude_seconds").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(latitude_seconds) => Some(latitude_seconds.clone()),
                _ => unreachable!("Expected I32")
            },
            longitude_degrees: match row.get("longitude_degrees").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(longitude_degrees) => Some(longitude_degrees.clone()),
                _ => unreachable!("Expected I32")
            },
            longitude_minutes: match row.get("longitude_minutes").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(longitude_minutes) => Some(longitude_minutes.clone()),
                _ => unreachable!("Expected I32")
            },
            longitude_seconds: match row.get("longitude_seconds").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(longitude_seconds) => Some(longitude_seconds.clone()),
                _ => unreachable!("Expected I32")
            },
            altitude: match row.get("altitude").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(altitude) => Some(altitude.clone()),
                _ => unreachable!("Expected I32")
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
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct LoginProvider {
    pub id: i32,
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
            gluesql::core::ast_builder::num(self.id),
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
    /// The number of rows inserted in table LoginProvider
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
        id: i32,
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
        id: i32,
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
        table("login_providers")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("font_awesome_icon", gluesql::core::ast_builder::text(self.font_awesome_icon))        
.set("client_id_var_name", gluesql::core::ast_builder::text(self.client_id_var_name))        
.set("redirect_uri_var_name", gluesql::core::ast_builder::text(self.redirect_uri_var_name))        
.set("oauth_url", gluesql::core::ast_builder::text(self.oauth_url))        
.set("scope", gluesql::core::ast_builder::text(self.scope))            .execute(connection)
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
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
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct ManufacturedItemCategory {
    pub id: i32,
    pub cost: i32,
    pub cost_per_day: i32,
    pub currency: String,
    pub manifacturer_id: i32,
}
#[cfg(feature = "frontend")]
impl ManufacturedItemCategory {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::num(self.cost),
            gluesql::core::ast_builder::num(self.cost_per_day),
            gluesql::core::ast_builder::text(self.currency),
            gluesql::core::ast_builder::num(self.manifacturer_id),
        ]
    }

    /// Insert the ManufacturedItemCategory into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table ManufacturedItemCategory
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
        id: i32,
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
        id: i32,
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
        table("manufactured_item_categories")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("cost", gluesql::core::ast_builder::num(self.cost))        
.set("cost_per_day", gluesql::core::ast_builder::num(self.cost_per_day))        
.set("currency", gluesql::core::ast_builder::text(self.currency))        
.set("manifacturer_id", gluesql::core::ast_builder::num(self.manifacturer_id))            .execute(connection)
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            cost: match row.get("cost").unwrap() {
                gluesql::prelude::Value::I32(cost) => cost.clone(),
                _ => unreachable!("Expected I32")
            },
            cost_per_day: match row.get("cost_per_day").unwrap() {
                gluesql::prelude::Value::I32(cost_per_day) => cost_per_day.clone(),
                _ => unreachable!("Expected I32")
            },
            currency: match row.get("currency").unwrap() {
                gluesql::prelude::Value::Str(currency) => currency.clone(),
                _ => unreachable!("Expected Str")
            },
            manifacturer_id: match row.get("manifacturer_id").unwrap() {
                gluesql::prelude::Value::I32(manifacturer_id) => manifacturer_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct Notification {
    pub id: i32,
    pub user_id: i32,
    pub operation: String,
    pub table_name: String,
    pub read: bool,
}
#[cfg(feature = "frontend")]
impl Notification {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::num(self.user_id),
            gluesql::core::ast_builder::text(self.operation),
            gluesql::core::ast_builder::text(self.table_name),
            (self.read.into()),
        ]
    }

    /// Insert the Notification into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table Notification
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("notifications")
            .insert()
            .columns("id, user_id, operation, table_name, read")
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
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("notifications")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, user_id, operation, table_name, read")
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
        id: i32,
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
        table("notifications")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("user_id", gluesql::core::ast_builder::num(self.user_id))        
.set("operation", gluesql::core::ast_builder::text(self.operation))        
.set("table_name", gluesql::core::ast_builder::text(self.table_name))        
.set("read", self.read)            .execute(connection)
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
            .project("id, user_id, operation, table_name, read")
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            user_id: match row.get("user_id").unwrap() {
                gluesql::prelude::Value::I32(user_id) => user_id.clone(),
                _ => unreachable!("Expected I32")
            },
            operation: match row.get("operation").unwrap() {
                gluesql::prelude::Value::Str(operation) => operation.clone(),
                _ => unreachable!("Expected Str")
            },
            table_name: match row.get("table_name").unwrap() {
                gluesql::prelude::Value::Str(table_name) => table_name.clone(),
                _ => unreachable!("Expected Str")
            },
            read: match row.get("read").unwrap() {
                gluesql::prelude::Value::Bool(read) => read.clone(),
                _ => unreachable!("Expected Bool")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct Organization {
    pub id: i32,
    pub parent_organization_id: Option<i32>,
    pub name: String,
}
#[cfg(feature = "frontend")]
impl Organization {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            match self.parent_organization_id {
                Some(parent_organization_id) => gluesql::core::ast_builder::num(parent_organization_id),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::text(self.name),
        ]
    }

    /// Insert the Organization into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table Organization
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("organizations")
            .insert()
            .columns("id, parent_organization_id, name")
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
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("organizations")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, parent_organization_id, name")
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
        id: i32,
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
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name));
        if let Some(parent_organization_id) = self.parent_organization_id {
            update_row = update_row.set("parent_organization_id", gluesql::core::ast_builder::num(parent_organization_id));
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
            .project("id, parent_organization_id, name")
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            parent_organization_id: match row.get("parent_organization_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(parent_organization_id) => Some(parent_organization_id.clone()),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct PrimaryUserEmail {
    pub id: i32,
}
#[cfg(feature = "frontend")]
impl PrimaryUserEmail {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
        ]
    }

    /// Insert the PrimaryUserEmail into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table PrimaryUserEmail
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
        id: i32,
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
        id: i32,
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
        table("primary_user_emails")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))            .execute(connection)
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct ProcedureContinuousRequirement {
    pub id: i32,
    pub created_by: i32,
    pub procedure_id: i32,
    pub item_category_id: i32,
    pub quantity: i32,
    pub unit_id: Option<i32>,
}
#[cfg(feature = "frontend")]
impl ProcedureContinuousRequirement {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::num(self.procedure_id),
            gluesql::core::ast_builder::num(self.item_category_id),
            gluesql::core::ast_builder::num(self.quantity),
            match self.unit_id {
                Some(unit_id) => gluesql::core::ast_builder::num(unit_id),
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
    /// The number of rows inserted in table ProcedureContinuousRequirement
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("procedure_continuous_requirements")
            .insert()
            .columns("id, created_by, procedure_id, item_category_id, quantity, unit_id")
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
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("procedure_continuous_requirements")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, created_by, procedure_id, item_category_id, quantity, unit_id")
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
        id: i32,
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
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("procedure_id", gluesql::core::ast_builder::num(self.procedure_id))        
.set("item_category_id", gluesql::core::ast_builder::num(self.item_category_id))        
.set("quantity", gluesql::core::ast_builder::num(self.quantity));
        if let Some(unit_id) = self.unit_id {
            update_row = update_row.set("unit_id", gluesql::core::ast_builder::num(unit_id));
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
            .project("id, created_by, procedure_id, item_category_id, quantity, unit_id")
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            procedure_id: match row.get("procedure_id").unwrap() {
                gluesql::prelude::Value::I32(procedure_id) => procedure_id.clone(),
                _ => unreachable!("Expected I32")
            },
            item_category_id: match row.get("item_category_id").unwrap() {
                gluesql::prelude::Value::I32(item_category_id) => item_category_id.clone(),
                _ => unreachable!("Expected I32")
            },
            quantity: match row.get("quantity").unwrap() {
                gluesql::prelude::Value::I32(quantity) => quantity.clone(),
                _ => unreachable!("Expected I32")
            },
            unit_id: match row.get("unit_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(unit_id) => Some(unit_id.clone()),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct ProcedureDiscreteRequirement {
    pub id: i32,
    pub created_by: i32,
    pub procedure_id: i32,
    pub item_category_id: i32,
    pub quantity: i32,
    pub unit_id: Option<i32>,
}
#[cfg(feature = "frontend")]
impl ProcedureDiscreteRequirement {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::num(self.procedure_id),
            gluesql::core::ast_builder::num(self.item_category_id),
            gluesql::core::ast_builder::num(self.quantity),
            match self.unit_id {
                Some(unit_id) => gluesql::core::ast_builder::num(unit_id),
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
    /// The number of rows inserted in table ProcedureDiscreteRequirement
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("procedure_discrete_requirements")
            .insert()
            .columns("id, created_by, procedure_id, item_category_id, quantity, unit_id")
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
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("procedure_discrete_requirements")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, created_by, procedure_id, item_category_id, quantity, unit_id")
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
        id: i32,
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
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("procedure_id", gluesql::core::ast_builder::num(self.procedure_id))        
.set("item_category_id", gluesql::core::ast_builder::num(self.item_category_id))        
.set("quantity", gluesql::core::ast_builder::num(self.quantity));
        if let Some(unit_id) = self.unit_id {
            update_row = update_row.set("unit_id", gluesql::core::ast_builder::num(unit_id));
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
            .project("id, created_by, procedure_id, item_category_id, quantity, unit_id")
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            procedure_id: match row.get("procedure_id").unwrap() {
                gluesql::prelude::Value::I32(procedure_id) => procedure_id.clone(),
                _ => unreachable!("Expected I32")
            },
            item_category_id: match row.get("item_category_id").unwrap() {
                gluesql::prelude::Value::I32(item_category_id) => item_category_id.clone(),
                _ => unreachable!("Expected I32")
            },
            quantity: match row.get("quantity").unwrap() {
                gluesql::prelude::Value::I32(quantity) => quantity.clone(),
                _ => unreachable!("Expected I32")
            },
            unit_id: match row.get("unit_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(unit_id) => Some(unit_id.clone()),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct Procedure {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_by: Option<i32>,
}
#[cfg(feature = "frontend")]
impl Procedure {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            match self.description {
                Some(description) => gluesql::core::ast_builder::text(description),
                None => gluesql::core::ast_builder::null(),
            },
            match self.created_by {
                Some(created_by) => gluesql::core::ast_builder::num(created_by),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the Procedure into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table Procedure
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("procedures")
            .insert()
            .columns("id, name, description, created_by")
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
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("procedures")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, description, created_by")
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
        id: i32,
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
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name));
        if let Some(description) = self.description {
            update_row = update_row.set("description", gluesql::core::ast_builder::text(description));
        }
        if let Some(created_by) = self.created_by {
            update_row = update_row.set("created_by", gluesql::core::ast_builder::num(created_by));
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
            .project("id, name, description, created_by")
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
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
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(created_by) => Some(created_by.clone()),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct ProjectRequirement {
    pub id: i32,
    pub created_by: i32,
    pub project_id: i32,
    pub item_category_id: i32,
    pub quantity: i32,
    pub unit_id: Option<i32>,
}
#[cfg(feature = "frontend")]
impl ProjectRequirement {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::num(self.project_id),
            gluesql::core::ast_builder::num(self.item_category_id),
            gluesql::core::ast_builder::num(self.quantity),
            match self.unit_id {
                Some(unit_id) => gluesql::core::ast_builder::num(unit_id),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the ProjectRequirement into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table ProjectRequirement
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("project_requirements")
            .insert()
            .columns("id, created_by, project_id, item_category_id, quantity, unit_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get ProjectRequirement from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of ProjectRequirement to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("project_requirements")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, created_by, project_id, item_category_id, quantity, unit_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete ProjectRequirement from the database.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete_from_id<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("project_requirements")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of ProjectRequirement from the database.
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
        let mut update_row = table("project_requirements")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("project_id", gluesql::core::ast_builder::num(self.project_id))        
.set("item_category_id", gluesql::core::ast_builder::num(self.item_category_id))        
.set("quantity", gluesql::core::ast_builder::num(self.quantity));
        if let Some(unit_id) = self.unit_id {
            update_row = update_row.set("unit_id", gluesql::core::ast_builder::num(unit_id));
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
    /// Get all ProjectRequirement from the database.
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
        let select_row = table("project_requirements")
            .select()
            .project("id, created_by, project_id, item_category_id, quantity, unit_id")
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            project_id: match row.get("project_id").unwrap() {
                gluesql::prelude::Value::I32(project_id) => project_id.clone(),
                _ => unreachable!("Expected I32")
            },
            item_category_id: match row.get("item_category_id").unwrap() {
                gluesql::prelude::Value::I32(item_category_id) => item_category_id.clone(),
                _ => unreachable!("Expected I32")
            },
            quantity: match row.get("quantity").unwrap() {
                gluesql::prelude::Value::I32(quantity) => quantity.clone(),
                _ => unreachable!("Expected I32")
            },
            unit_id: match row.get("unit_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(unit_id) => Some(unit_id.clone()),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct ProjectState {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub font_awesome_icon: String,
    pub icon_color: String,
}
#[cfg(feature = "frontend")]
impl ProjectState {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
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
    /// The number of rows inserted in table ProjectState
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
        id: i32,
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
        id: i32,
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
        table("project_states")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("description", gluesql::core::ast_builder::text(self.description))        
.set("font_awesome_icon", gluesql::core::ast_builder::text(self.font_awesome_icon))        
.set("icon_color", gluesql::core::ast_builder::text(self.icon_color))            .execute(connection)
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
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
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub public: bool,
    pub state_id: i32,
    pub parent_project_id: Option<i32>,
    pub budget: Option<i64>,
    pub expenses: Option<i64>,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub expected_end_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
}
#[cfg(feature = "frontend")]
impl Project {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            (self.public.into()),
            gluesql::core::ast_builder::num(self.state_id),
            match self.parent_project_id {
                Some(parent_project_id) => gluesql::core::ast_builder::num(parent_project_id),
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
            gluesql::core::ast_builder::num(self.created_by),
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
    /// The number of rows inserted in table Project
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
        id: i32,
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
        id: i32,
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
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("description", gluesql::core::ast_builder::text(self.description))        
.set("public", self.public)        
.set("state_id", gluesql::core::ast_builder::num(self.state_id))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()));
        if let Some(parent_project_id) = self.parent_project_id {
            update_row = update_row.set("parent_project_id", gluesql::core::ast_builder::num(parent_project_id));
        }
        if let Some(budget) = self.budget {
            update_row = update_row.set("budget", gluesql::core::ast_builder::num(budget));
        }
        if let Some(expenses) = self.expenses {
            update_row = update_row.set("expenses", gluesql::core::ast_builder::num(expenses));
        }
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
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
                gluesql::prelude::Value::I32(state_id) => state_id.clone(),
                _ => unreachable!("Expected I32")
            },
            parent_project_id: match row.get("parent_project_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(parent_project_id) => Some(parent_project_id.clone()),
                _ => unreachable!("Expected I32")
            },
            budget: match row.get("budget").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I64(budget) => Some(budget.clone()),
                _ => unreachable!("Expected I64")
            },
            expenses: match row.get("expenses").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I64(expenses) => Some(expenses.clone()),
                _ => unreachable!("Expected I64")
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
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
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct Role {
    pub id: i32,
    pub name: String,
}
#[cfg(feature = "frontend")]
impl Role {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
        ]
    }

    /// Insert the Role into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table Role
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("roles")
            .insert()
            .columns("id, name")
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
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("roles")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name")
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
        id: i32,
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
        table("roles")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))            .execute(connection)
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
            .project("id, name")
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct SampleTaxa {
    pub id: Uuid,
    pub created_by: i32,
    pub sample_id: Uuid,
    pub taxon_id: i32,
}
#[cfg(feature = "frontend")]
impl SampleTaxa {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::uuid(self.id.to_string()),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::uuid(self.sample_id.to_string()),
            gluesql::core::ast_builder::num(self.taxon_id),
        ]
    }

    /// Insert the SampleTaxa into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table SampleTaxa
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sample_taxa")
            .insert()
            .columns("id, created_by, sample_id, taxon_id")
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
        id: uuid::Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sample_taxa")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, created_by, sample_id, taxon_id")
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
        id: uuid::Uuid,
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
        table("sample_taxa")
            .update()        
.set("id", gluesql::core::ast_builder::uuid(self.id.to_string()))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("sample_id", gluesql::core::ast_builder::uuid(self.sample_id.to_string()))        
.set("taxon_id", gluesql::core::ast_builder::num(self.taxon_id))            .execute(connection)
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
            .project("id, created_by, sample_id, taxon_id")
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
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            sample_id: match row.get("sample_id").unwrap() {
                gluesql::prelude::Value::Uuid(sample_id) => Uuid::from_u128(*sample_id),
                _ => unreachable!("Expected Uuid"),
            },
            taxon_id: match row.get("taxon_id").unwrap() {
                gluesql::prelude::Value::I32(taxon_id) => taxon_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct SampledIndividualTaxa {
    pub id: Uuid,
    pub created_by: i32,
    pub sampled_individual_id: Uuid,
    pub taxon_id: i32,
}
#[cfg(feature = "frontend")]
impl SampledIndividualTaxa {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::uuid(self.id.to_string()),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::uuid(self.sampled_individual_id.to_string()),
            gluesql::core::ast_builder::num(self.taxon_id),
        ]
    }

    /// Insert the SampledIndividualTaxa into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table SampledIndividualTaxa
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sampled_individual_taxa")
            .insert()
            .columns("id, created_by, sampled_individual_id, taxon_id")
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
        id: uuid::Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sampled_individual_taxa")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, created_by, sampled_individual_id, taxon_id")
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
        id: uuid::Uuid,
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
        table("sampled_individual_taxa")
            .update()        
.set("id", gluesql::core::ast_builder::uuid(self.id.to_string()))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))        
.set("sampled_individual_id", gluesql::core::ast_builder::uuid(self.sampled_individual_id.to_string()))        
.set("taxon_id", gluesql::core::ast_builder::num(self.taxon_id))            .execute(connection)
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
            .project("id, created_by, sampled_individual_id, taxon_id")
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
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
            sampled_individual_id: match row.get("sampled_individual_id").unwrap() {
                gluesql::prelude::Value::Uuid(sampled_individual_id) => Uuid::from_u128(*sampled_individual_id),
                _ => unreachable!("Expected Uuid"),
            },
            taxon_id: match row.get("taxon_id").unwrap() {
                gluesql::prelude::Value::I32(taxon_id) => taxon_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct SampledIndividual {
    pub id: Uuid,
}
#[cfg(feature = "frontend")]
impl SampledIndividual {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::uuid(self.id.to_string()),
        ]
    }

    /// Insert the SampledIndividual into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table SampledIndividual
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
        id: uuid::Uuid,
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
        id: uuid::Uuid,
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
        table("sampled_individuals")
            .update()        
.set("id", gluesql::core::ast_builder::uuid(self.id.to_string()))            .execute(connection)
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
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct Sample {
    pub id: Uuid,
    pub created_by: Option<i32>,
    pub derived_from: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl Sample {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::uuid(self.id.to_string()),
            match self.created_by {
                Some(created_by) => gluesql::core::ast_builder::num(created_by),
                None => gluesql::core::ast_builder::null(),
            },
            match self.derived_from {
                Some(derived_from) => gluesql::core::ast_builder::uuid(derived_from.to_string()),
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
    /// The number of rows inserted in table Sample
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("samples")
            .insert()
            .columns("id, created_by, derived_from")
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
        id: uuid::Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("samples")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, created_by, derived_from")
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
        id: uuid::Uuid,
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
            .update()        
.set("id", gluesql::core::ast_builder::uuid(self.id.to_string()));
        if let Some(created_by) = self.created_by {
            update_row = update_row.set("created_by", gluesql::core::ast_builder::num(created_by));
        }
        if let Some(derived_from) = self.derived_from {
            update_row = update_row.set("derived_from", gluesql::core::ast_builder::uuid(derived_from.to_string()));
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
            .project("id, created_by, derived_from")
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
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(created_by) => Some(created_by.clone()),
                _ => unreachable!("Expected I32")
            },
            derived_from: match row.get("derived_from").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(derived_from) => Some(Uuid::from_u128(*derived_from)),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct Spectra {
    pub id: i32,
    pub spectra_collection_id: i32,
}
#[cfg(feature = "frontend")]
impl Spectra {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::num(self.spectra_collection_id),
        ]
    }

    /// Insert the Spectra into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table Spectra
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
        id: i32,
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
        id: i32,
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
        table("spectra")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("spectra_collection_id", gluesql::core::ast_builder::num(self.spectra_collection_id))            .execute(connection)
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            spectra_collection_id: match row.get("spectra_collection_id").unwrap() {
                gluesql::prelude::Value::I32(spectra_collection_id) => spectra_collection_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct SpectraCollection {
    pub id: i32,
    pub sample_id: Uuid,
    pub created_by: i32,
}
#[cfg(feature = "frontend")]
impl SpectraCollection {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::uuid(self.sample_id.to_string()),
            gluesql::core::ast_builder::num(self.created_by),
        ]
    }

    /// Insert the SpectraCollection into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table SpectraCollection
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("spectra_collection")
            .insert()
            .columns("id, sample_id, created_by")
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
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("spectra_collection")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, sample_id, created_by")
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
        id: i32,
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
        table("spectra_collection")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("sample_id", gluesql::core::ast_builder::uuid(self.sample_id.to_string()))        
.set("created_by", gluesql::core::ast_builder::num(self.created_by))            .execute(connection)
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
            .project("id, sample_id, created_by")
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            sample_id: match row.get("sample_id").unwrap() {
                gluesql::prelude::Value::Uuid(sample_id) => Uuid::from_u128(*sample_id),
                _ => unreachable!("Expected Uuid"),
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct Taxa {
    pub id: i32,
    pub name: String,
    pub ncbi_taxon_id: Option<i32>,
}
#[cfg(feature = "frontend")]
impl Taxa {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
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
    /// The number of rows inserted in table Taxa
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
        id: i32,
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
        id: i32,
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
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name));
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
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
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct Team {
    pub id: i32,
    pub parent_team_id: Option<i32>,
}
#[cfg(feature = "frontend")]
impl Team {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            match self.parent_team_id {
                Some(parent_team_id) => gluesql::core::ast_builder::num(parent_team_id),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the Team into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table Team
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("teams")
            .insert()
            .columns("id, parent_team_id")
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
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("teams")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, parent_team_id")
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
        id: i32,
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
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id));
        if let Some(parent_team_id) = self.parent_team_id {
            update_row = update_row.set("parent_team_id", gluesql::core::ast_builder::num(parent_team_id));
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
            .project("id, parent_team_id")
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            parent_team_id: match row.get("parent_team_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(parent_team_id) => Some(parent_team_id.clone()),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct Unit {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub symbol: String,
}
#[cfg(feature = "frontend")]
impl Unit {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            gluesql::core::ast_builder::text(self.symbol),
        ]
    }

    /// Insert the Unit into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table Unit
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("units")
            .insert()
            .columns("id, name, description, symbol")
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
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("units")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, description, symbol")
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
        id: i32,
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
        table("units")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("description", gluesql::core::ast_builder::text(self.description))        
.set("symbol", gluesql::core::ast_builder::text(self.symbol))            .execute(connection)
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
            .project("id, name, description, symbol")
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str")
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str")
            },
            symbol: match row.get("symbol").unwrap() {
                gluesql::prelude::Value::Str(symbol) => symbol.clone(),
                _ => unreachable!("Expected Str")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct UserEmail {
    pub id: i32,
    pub email: String,
    pub user_id: i32,
    pub login_provider_id: i32,
}
#[cfg(feature = "frontend")]
impl UserEmail {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.email),
            gluesql::core::ast_builder::num(self.user_id),
            gluesql::core::ast_builder::num(self.login_provider_id),
        ]
    }

    /// Insert the UserEmail into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table UserEmail
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
        id: i32,
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
        id: i32,
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
        table("user_emails")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("email", gluesql::core::ast_builder::text(self.email))        
.set("user_id", gluesql::core::ast_builder::num(self.user_id))        
.set("login_provider_id", gluesql::core::ast_builder::num(self.login_provider_id))            .execute(connection)
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
            },
            email: match row.get("email").unwrap() {
                gluesql::prelude::Value::Str(email) => email.clone(),
                _ => unreachable!("Expected Str")
            },
            user_id: match row.get("user_id").unwrap() {
                gluesql::prelude::Value::I32(user_id) => user_id.clone(),
                _ => unreachable!("Expected I32")
            },
            login_provider_id: match row.get("login_provider_id").unwrap() {
                gluesql::prelude::Value::I32(login_provider_id) => login_provider_id.clone(),
                _ => unreachable!("Expected I32")
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct User {
    pub id: i32,
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
            gluesql::core::ast_builder::num(self.id),
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
    /// The number of rows inserted in table User
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
        id: i32,
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
        id: i32,
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
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("first_name", gluesql::core::ast_builder::text(self.first_name))        
.set("last_name", gluesql::core::ast_builder::text(self.last_name))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))        
.set("updated_at", gluesql::core::ast_builder::timestamp(self.updated_at.to_string()));
        if let Some(middle_name) = self.middle_name {
            update_row = update_row.set("middle_name", gluesql::core::ast_builder::text(middle_name));
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
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32")
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
    ContainerHorizontalRule,
    ContainerVerticalRule,
    ContinuousUnit,
    DiscreteUnit,
    DocumentFormat,
    Document,
    ItemCategory,
    ItemCategoryRelationship,
    ItemCategoryUnit,
    ItemContinuousQuantity,
    ItemDiscreteQuantity,
    ItemLocation,
    ItemUnit,
    Item,
    Location,
    LoginProvider,
    ManufacturedItemCategory,
    Notification,
    Organization,
    PrimaryUserEmail,
    ProcedureContinuousRequirement,
    ProcedureDiscreteRequirement,
    Procedure,
    ProjectRequirement,
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
    Team,
    Unit,
    UserEmail,
    User,
}

impl Table {
    pub fn name(&self) -> &'static str {
        match self {
            Table::ContainerHorizontalRule => "container_horizontal_rules",
            Table::ContainerVerticalRule => "container_vertical_rules",
            Table::ContinuousUnit => "continuous_units",
            Table::DiscreteUnit => "discrete_units",
            Table::DocumentFormat => "document_formats",
            Table::Document => "documents",
            Table::ItemCategory => "item_categories",
            Table::ItemCategoryRelationship => "item_category_relationships",
            Table::ItemCategoryUnit => "item_category_units",
            Table::ItemContinuousQuantity => "item_continuous_quantities",
            Table::ItemDiscreteQuantity => "item_discrete_quantities",
            Table::ItemLocation => "item_locations",
            Table::ItemUnit => "item_units",
            Table::Item => "items",
            Table::Location => "locations",
            Table::LoginProvider => "login_providers",
            Table::ManufacturedItemCategory => "manufactured_item_categories",
            Table::Notification => "notifications",
            Table::Organization => "organizations",
            Table::PrimaryUserEmail => "primary_user_emails",
            Table::ProcedureContinuousRequirement => "procedure_continuous_requirements",
            Table::ProcedureDiscreteRequirement => "procedure_discrete_requirements",
            Table::Procedure => "procedures",
            Table::ProjectRequirement => "project_requirements",
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
            Table::Team => "teams",
            Table::Unit => "units",
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
    ContainerHorizontalRule(ContainerHorizontalRule),
    ContainerVerticalRule(ContainerVerticalRule),
    ContinuousUnit(ContinuousUnit),
    DiscreteUnit(DiscreteUnit),
    DocumentFormat(DocumentFormat),
    Document(Document),
    ItemCategory(ItemCategory),
    ItemCategoryRelationship(ItemCategoryRelationship),
    ItemCategoryUnit(ItemCategoryUnit),
    ItemContinuousQuantity(ItemContinuousQuantity),
    ItemDiscreteQuantity(ItemDiscreteQuantity),
    ItemLocation(ItemLocation),
    ItemUnit(ItemUnit),
    Item(Item),
    Location(Location),
    LoginProvider(LoginProvider),
    ManufacturedItemCategory(ManufacturedItemCategory),
    Notification(Notification),
    Organization(Organization),
    PrimaryUserEmail(PrimaryUserEmail),
    ProcedureContinuousRequirement(ProcedureContinuousRequirement),
    ProcedureDiscreteRequirement(ProcedureDiscreteRequirement),
    Procedure(Procedure),
    ProjectRequirement(ProjectRequirement),
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
    Team(Team),
    Unit(Unit),
    UserEmail(UserEmail),
    User(User),
}

impl From<ContainerHorizontalRule> for TableRow {
    fn from(item: ContainerHorizontalRule) -> Self {
        TableRow::ContainerHorizontalRule(item)
    }
}
impl std::convert::TryFrom<TableRow> for ContainerHorizontalRule {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::ContainerHorizontalRule(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<ContainerVerticalRule> for TableRow {
    fn from(item: ContainerVerticalRule) -> Self {
        TableRow::ContainerVerticalRule(item)
    }
}
impl std::convert::TryFrom<TableRow> for ContainerVerticalRule {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::ContainerVerticalRule(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<ContinuousUnit> for TableRow {
    fn from(item: ContinuousUnit) -> Self {
        TableRow::ContinuousUnit(item)
    }
}
impl std::convert::TryFrom<TableRow> for ContinuousUnit {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::ContinuousUnit(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<DiscreteUnit> for TableRow {
    fn from(item: DiscreteUnit) -> Self {
        TableRow::DiscreteUnit(item)
    }
}
impl std::convert::TryFrom<TableRow> for DiscreteUnit {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::DiscreteUnit(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<DocumentFormat> for TableRow {
    fn from(item: DocumentFormat) -> Self {
        TableRow::DocumentFormat(item)
    }
}
impl std::convert::TryFrom<TableRow> for DocumentFormat {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::DocumentFormat(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<Document> for TableRow {
    fn from(item: Document) -> Self {
        TableRow::Document(item)
    }
}
impl std::convert::TryFrom<TableRow> for Document {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::Document(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<ItemCategory> for TableRow {
    fn from(item: ItemCategory) -> Self {
        TableRow::ItemCategory(item)
    }
}
impl std::convert::TryFrom<TableRow> for ItemCategory {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::ItemCategory(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<ItemCategoryRelationship> for TableRow {
    fn from(item: ItemCategoryRelationship) -> Self {
        TableRow::ItemCategoryRelationship(item)
    }
}
impl std::convert::TryFrom<TableRow> for ItemCategoryRelationship {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::ItemCategoryRelationship(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<ItemCategoryUnit> for TableRow {
    fn from(item: ItemCategoryUnit) -> Self {
        TableRow::ItemCategoryUnit(item)
    }
}
impl std::convert::TryFrom<TableRow> for ItemCategoryUnit {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::ItemCategoryUnit(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<ItemContinuousQuantity> for TableRow {
    fn from(item: ItemContinuousQuantity) -> Self {
        TableRow::ItemContinuousQuantity(item)
    }
}
impl std::convert::TryFrom<TableRow> for ItemContinuousQuantity {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::ItemContinuousQuantity(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<ItemDiscreteQuantity> for TableRow {
    fn from(item: ItemDiscreteQuantity) -> Self {
        TableRow::ItemDiscreteQuantity(item)
    }
}
impl std::convert::TryFrom<TableRow> for ItemDiscreteQuantity {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::ItemDiscreteQuantity(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<ItemLocation> for TableRow {
    fn from(item: ItemLocation) -> Self {
        TableRow::ItemLocation(item)
    }
}
impl std::convert::TryFrom<TableRow> for ItemLocation {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::ItemLocation(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<ItemUnit> for TableRow {
    fn from(item: ItemUnit) -> Self {
        TableRow::ItemUnit(item)
    }
}
impl std::convert::TryFrom<TableRow> for ItemUnit {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::ItemUnit(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<Item> for TableRow {
    fn from(item: Item) -> Self {
        TableRow::Item(item)
    }
}
impl std::convert::TryFrom<TableRow> for Item {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::Item(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<Location> for TableRow {
    fn from(item: Location) -> Self {
        TableRow::Location(item)
    }
}
impl std::convert::TryFrom<TableRow> for Location {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::Location(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<LoginProvider> for TableRow {
    fn from(item: LoginProvider) -> Self {
        TableRow::LoginProvider(item)
    }
}
impl std::convert::TryFrom<TableRow> for LoginProvider {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::LoginProvider(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<ManufacturedItemCategory> for TableRow {
    fn from(item: ManufacturedItemCategory) -> Self {
        TableRow::ManufacturedItemCategory(item)
    }
}
impl std::convert::TryFrom<TableRow> for ManufacturedItemCategory {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::ManufacturedItemCategory(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<Notification> for TableRow {
    fn from(item: Notification) -> Self {
        TableRow::Notification(item)
    }
}
impl std::convert::TryFrom<TableRow> for Notification {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::Notification(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<Organization> for TableRow {
    fn from(item: Organization) -> Self {
        TableRow::Organization(item)
    }
}
impl std::convert::TryFrom<TableRow> for Organization {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::Organization(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<PrimaryUserEmail> for TableRow {
    fn from(item: PrimaryUserEmail) -> Self {
        TableRow::PrimaryUserEmail(item)
    }
}
impl std::convert::TryFrom<TableRow> for PrimaryUserEmail {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::PrimaryUserEmail(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<ProcedureContinuousRequirement> for TableRow {
    fn from(item: ProcedureContinuousRequirement) -> Self {
        TableRow::ProcedureContinuousRequirement(item)
    }
}
impl std::convert::TryFrom<TableRow> for ProcedureContinuousRequirement {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::ProcedureContinuousRequirement(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<ProcedureDiscreteRequirement> for TableRow {
    fn from(item: ProcedureDiscreteRequirement) -> Self {
        TableRow::ProcedureDiscreteRequirement(item)
    }
}
impl std::convert::TryFrom<TableRow> for ProcedureDiscreteRequirement {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::ProcedureDiscreteRequirement(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<Procedure> for TableRow {
    fn from(item: Procedure) -> Self {
        TableRow::Procedure(item)
    }
}
impl std::convert::TryFrom<TableRow> for Procedure {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::Procedure(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<ProjectRequirement> for TableRow {
    fn from(item: ProjectRequirement) -> Self {
        TableRow::ProjectRequirement(item)
    }
}
impl std::convert::TryFrom<TableRow> for ProjectRequirement {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::ProjectRequirement(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<ProjectState> for TableRow {
    fn from(item: ProjectState) -> Self {
        TableRow::ProjectState(item)
    }
}
impl std::convert::TryFrom<TableRow> for ProjectState {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::ProjectState(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<Project> for TableRow {
    fn from(item: Project) -> Self {
        TableRow::Project(item)
    }
}
impl std::convert::TryFrom<TableRow> for Project {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::Project(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<Role> for TableRow {
    fn from(item: Role) -> Self {
        TableRow::Role(item)
    }
}
impl std::convert::TryFrom<TableRow> for Role {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::Role(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<SampleTaxa> for TableRow {
    fn from(item: SampleTaxa) -> Self {
        TableRow::SampleTaxa(item)
    }
}
impl std::convert::TryFrom<TableRow> for SampleTaxa {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::SampleTaxa(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<SampledIndividualTaxa> for TableRow {
    fn from(item: SampledIndividualTaxa) -> Self {
        TableRow::SampledIndividualTaxa(item)
    }
}
impl std::convert::TryFrom<TableRow> for SampledIndividualTaxa {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::SampledIndividualTaxa(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<SampledIndividual> for TableRow {
    fn from(item: SampledIndividual) -> Self {
        TableRow::SampledIndividual(item)
    }
}
impl std::convert::TryFrom<TableRow> for SampledIndividual {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::SampledIndividual(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<Sample> for TableRow {
    fn from(item: Sample) -> Self {
        TableRow::Sample(item)
    }
}
impl std::convert::TryFrom<TableRow> for Sample {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::Sample(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<Spectra> for TableRow {
    fn from(item: Spectra) -> Self {
        TableRow::Spectra(item)
    }
}
impl std::convert::TryFrom<TableRow> for Spectra {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::Spectra(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<SpectraCollection> for TableRow {
    fn from(item: SpectraCollection) -> Self {
        TableRow::SpectraCollection(item)
    }
}
impl std::convert::TryFrom<TableRow> for SpectraCollection {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::SpectraCollection(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<Taxa> for TableRow {
    fn from(item: Taxa) -> Self {
        TableRow::Taxa(item)
    }
}
impl std::convert::TryFrom<TableRow> for Taxa {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::Taxa(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<Team> for TableRow {
    fn from(item: Team) -> Self {
        TableRow::Team(item)
    }
}
impl std::convert::TryFrom<TableRow> for Team {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::Team(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<Unit> for TableRow {
    fn from(item: Unit) -> Self {
        TableRow::Unit(item)
    }
}
impl std::convert::TryFrom<TableRow> for Unit {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::Unit(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<UserEmail> for TableRow {
    fn from(item: UserEmail) -> Self {
        TableRow::UserEmail(item)
    }
}
impl std::convert::TryFrom<TableRow> for UserEmail {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::UserEmail(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<User> for TableRow {
    fn from(item: User) -> Self {
        TableRow::User(item)
    }
}
impl std::convert::TryFrom<TableRow> for User {
    type Error = &'static str;
    fn try_from(item: TableRow) -> Result<Self, Self::Error> {
        match item {
            TableRow::User(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<&str> for Table {
    fn from(item: &str) -> Self {
        match item {
            "container_horizontal_rules" => Table::ContainerHorizontalRule,
            "container_vertical_rules" => Table::ContainerVerticalRule,
            "continuous_units" => Table::ContinuousUnit,
            "discrete_units" => Table::DiscreteUnit,
            "document_formats" => Table::DocumentFormat,
            "documents" => Table::Document,
            "item_categories" => Table::ItemCategory,
            "item_category_relationships" => Table::ItemCategoryRelationship,
            "item_category_units" => Table::ItemCategoryUnit,
            "item_continuous_quantities" => Table::ItemContinuousQuantity,
            "item_discrete_quantities" => Table::ItemDiscreteQuantity,
            "item_locations" => Table::ItemLocation,
            "item_units" => Table::ItemUnit,
            "items" => Table::Item,
            "locations" => Table::Location,
            "login_providers" => Table::LoginProvider,
            "manufactured_item_categories" => Table::ManufacturedItemCategory,
            "notifications" => Table::Notification,
            "organizations" => Table::Organization,
            "primary_user_emails" => Table::PrimaryUserEmail,
            "procedure_continuous_requirements" => Table::ProcedureContinuousRequirement,
            "procedure_discrete_requirements" => Table::ProcedureDiscreteRequirement,
            "procedures" => Table::Procedure,
            "project_requirements" => Table::ProjectRequirement,
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
            "teams" => Table::Team,
            "units" => Table::Unit,
            "user_emails" => Table::UserEmail,
            "users" => Table::User,
            _ => panic!("Unknown table name"),
        }
    }
}
impl TableRow {
    pub fn table(&self) -> &'static Table {
        match self {
            TableRow::ContainerHorizontalRule(_) => &Table::ContainerHorizontalRule,
            TableRow::ContainerVerticalRule(_) => &Table::ContainerVerticalRule,
            TableRow::ContinuousUnit(_) => &Table::ContinuousUnit,
            TableRow::DiscreteUnit(_) => &Table::DiscreteUnit,
            TableRow::DocumentFormat(_) => &Table::DocumentFormat,
            TableRow::Document(_) => &Table::Document,
            TableRow::ItemCategory(_) => &Table::ItemCategory,
            TableRow::ItemCategoryRelationship(_) => &Table::ItemCategoryRelationship,
            TableRow::ItemCategoryUnit(_) => &Table::ItemCategoryUnit,
            TableRow::ItemContinuousQuantity(_) => &Table::ItemContinuousQuantity,
            TableRow::ItemDiscreteQuantity(_) => &Table::ItemDiscreteQuantity,
            TableRow::ItemLocation(_) => &Table::ItemLocation,
            TableRow::ItemUnit(_) => &Table::ItemUnit,
            TableRow::Item(_) => &Table::Item,
            TableRow::Location(_) => &Table::Location,
            TableRow::LoginProvider(_) => &Table::LoginProvider,
            TableRow::ManufacturedItemCategory(_) => &Table::ManufacturedItemCategory,
            TableRow::Notification(_) => &Table::Notification,
            TableRow::Organization(_) => &Table::Organization,
            TableRow::PrimaryUserEmail(_) => &Table::PrimaryUserEmail,
            TableRow::ProcedureContinuousRequirement(_) => &Table::ProcedureContinuousRequirement,
            TableRow::ProcedureDiscreteRequirement(_) => &Table::ProcedureDiscreteRequirement,
            TableRow::Procedure(_) => &Table::Procedure,
            TableRow::ProjectRequirement(_) => &Table::ProjectRequirement,
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
            TableRow::Team(_) => &Table::Team,
            TableRow::Unit(_) => &Table::Unit,
            TableRow::UserEmail(_) => &Table::UserEmail,
            TableRow::User(_) => &Table::User,
        }
    }

    pub fn table_name(&self) -> &'static str {
        self.table().name()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, )]
pub enum SearchableTableRow {
    ProjectState(ProjectState),
    Project(Project),
    Taxa(Taxa),
    User(User),
}

impl From<ProjectState> for SearchableTableRow {
    fn from(item: ProjectState) -> Self {
        SearchableTableRow::ProjectState(item)
    }
}
impl std::convert::TryFrom<SearchableTableRow> for ProjectState {
    type Error = &'static str;
    fn try_from(item: SearchableTableRow) -> Result<Self, Self::Error> {
        match item {
            SearchableTableRow::ProjectState(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<Project> for SearchableTableRow {
    fn from(item: Project) -> Self {
        SearchableTableRow::Project(item)
    }
}
impl std::convert::TryFrom<SearchableTableRow> for Project {
    type Error = &'static str;
    fn try_from(item: SearchableTableRow) -> Result<Self, Self::Error> {
        match item {
            SearchableTableRow::Project(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<Taxa> for SearchableTableRow {
    fn from(item: Taxa) -> Self {
        SearchableTableRow::Taxa(item)
    }
}
impl std::convert::TryFrom<SearchableTableRow> for Taxa {
    type Error = &'static str;
    fn try_from(item: SearchableTableRow) -> Result<Self, Self::Error> {
        match item {
            SearchableTableRow::Taxa(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
impl From<User> for SearchableTableRow {
    fn from(item: User) -> Self {
        SearchableTableRow::User(item)
    }
}
impl std::convert::TryFrom<SearchableTableRow> for User {
    type Error = &'static str;
    fn try_from(item: SearchableTableRow) -> Result<Self, Self::Error> {
        match item {
            SearchableTableRow::User(item) => Ok(item),
            _ => Err("Invalid conversion"),
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Copy, Eq, )]
pub enum SearchableTable {
    ProjectState,
    Project,
    Taxa,
    User,
}

impl SearchableTable {
    /// Search the table or view by the query.
    ///
    /// # Arguments
    /// * `query` - The query to search.
    /// * `number_of_results` - The number of results to return.
    pub fn search(&self, query: String, number_of_results: usize) -> super::selects::Select {
        match self {
            SearchableTable::ProjectState => super::selects::Select::search(SearchableTable::ProjectState, query, number_of_results),
            SearchableTable::Project => super::selects::Select::search(SearchableTable::Project, query, number_of_results),
            SearchableTable::Taxa => super::selects::Select::search(SearchableTable::Taxa, query, number_of_results),
            SearchableTable::User => super::selects::Select::search(SearchableTable::User, query, number_of_results),
        }
    }
}
pub trait SearchableTableName {
    /// Returns the variant of the table or view.
    fn parent_enum() -> SearchableTable;
}
impl SearchableTableName for ProjectState {
    fn parent_enum() -> SearchableTable {
        SearchableTable::ProjectState
    }
}
impl SearchableTableName for Project {
    fn parent_enum() -> SearchableTable {
        SearchableTable::Project
    }
}
impl SearchableTableName for Taxa {
    fn parent_enum() -> SearchableTable {
        SearchableTable::Taxa
    }
}
impl SearchableTableName for User {
    fn parent_enum() -> SearchableTable {
        SearchableTable::User
    }
}
pub trait SearchTable {
    /// Search the table or view by the query.
    ///
    /// # Arguments
    /// * `query` - The query to search.
    /// * `number_of_results` - The number of results to return.
    fn search(query: String, number_of_results: usize) -> super::selects::Select;
}
impl<T> SearchTable for T where T: SearchableTableName {
    fn search(query: String, number_of_results: usize) -> super::selects::Select {
        Self::parent_enum().search(query, number_of_results)
    }
}
