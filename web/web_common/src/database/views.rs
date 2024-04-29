use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
use chrono::NaiveDateTime;
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct PublicUser {
    pub id: i32,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub thumbnail_id: Option<Uuid>,
    pub picture_id: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl PublicUser {
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
            match self.thumbnail_id {
                Some(thumbnail_id) => gluesql::core::ast_builder::uuid(thumbnail_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.picture_id {
                Some(picture_id) => gluesql::core::ast_builder::uuid(picture_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the PublicUser into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table PublicUser
    pub async fn insert<C>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("public_users")
            .insert()
            .columns("id, first_name, middle_name, last_name, created_at, updated_at, thumbnail_id, picture_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })
    }

    /// Get PublicUser from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of PublicUser to get.
    /// * `connection` - The connection to the database.
    ///
    pub async fn get<C>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("public_users")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, first_name, middle_name, last_name, created_at, updated_at, thumbnail_id, picture_id")
            .limit(1)
            .execute(connection)
            .await?;
         Ok(select_row.select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete PublicUser from the database.
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
        table("public_users")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                 _ => unreachable!("Payload must be a Delete"),
             })
    }

    /// Delete the current instance of PublicUser from the database.
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
        let mut update_row = table("public_users")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("first_name", gluesql::core::ast_builder::text(self.first_name))        
.set("last_name", gluesql::core::ast_builder::text(self.last_name))        
.set("created_at", gluesql::core::ast_builder::timestamp(self.created_at.to_string()))        
.set("updated_at", gluesql::core::ast_builder::timestamp(self.updated_at.to_string()));
        if let Some(middle_name) = self.middle_name {
            update_row = update_row.set("middle_name", gluesql::core::ast_builder::text(middle_name));
        }
        if let Some(thumbnail_id) = self.thumbnail_id {
            update_row = update_row.set("thumbnail_id", gluesql::core::ast_builder::uuid(thumbnail_id.to_string()));
        }
        if let Some(picture_id) = self.picture_id {
            update_row = update_row.set("picture_id", gluesql::core::ast_builder::uuid(picture_id.to_string()));
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
    /// Get all PublicUser from the database.
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
        let select_row = table("public_users")
            .select()
            .project("id, first_name, middle_name, last_name, created_at, updated_at, thumbnail_id, picture_id")
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
            thumbnail_id: match row.get("thumbnail_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(thumbnail_id) => Some(Uuid::from_u128(*thumbnail_id)),
                _ => unreachable!("Expected Uuid"),
            },
            picture_id: match row.get("picture_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(picture_id) => Some(Uuid::from_u128(*picture_id)),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
