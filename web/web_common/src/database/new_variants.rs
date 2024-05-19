//! This module contains the new variants of the database models.
//!
//! This module is automatically generated. Do not write anything here.

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewProject {
    pub name: String,
    pub description: String,
    pub public: bool,
    pub state_id: i32,
    pub icon_id: i32,
    pub color_id: i32,
    pub parent_project_id: Option<i32>,
    pub budget: Option<f64>,
    pub expenses: Option<f64>,
    pub expected_end_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSampledIndividual {
    pub id: Uuid,
    pub tagged: bool,
}

#[cfg(feature = "frontend")]
impl NewSampledIndividual {
    pub fn into_row(self, created_by: i32) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(created_by),
            gluesql::core::ast_builder::num(created_by),
            gluesql::core::ast_builder::uuid(self.id.to_string()),
            (self.tagged.into()),
        ]
    }

    /// Insert the NewSampledIndividual into the database.
    ///
    /// # Arguments
    /// * `created_by` - The id of the user inserting the row.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table NewSampledIndividual
    pub async fn insert<C>(
        self,
        created_by: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<super::SampledIndividual, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let id = self.id;
        table("sampled_individuals")
            .insert()
            .columns("created_by,updated_by,id,tagged")
            .values(vec![self.into_row(created_by)])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })?;
        super::SampledIndividual::get(id, connection).await.map(|maybe_row| maybe_row.unwrap())
    }

    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user who is updating the struct.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        user_id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("sampled_individuals")
            .update()        
.set("id", gluesql::core::ast_builder::uuid(self.id.to_string()))        
.set("tagged", self.tagged)        
.set("updated_by", gluesql::core::ast_builder::num(user_id))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSample {
    pub id: Uuid,
    pub sampled_by: i32,
    pub state: i32,
}

#[cfg(feature = "frontend")]
impl NewSample {
    pub fn into_row(self, created_by: i32) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(created_by),
            gluesql::core::ast_builder::num(created_by),
            gluesql::core::ast_builder::uuid(self.id.to_string()),
            gluesql::core::ast_builder::num(self.sampled_by),
            gluesql::core::ast_builder::num(self.state),
        ]
    }

    /// Insert the NewSample into the database.
    ///
    /// # Arguments
    /// * `created_by` - The id of the user inserting the row.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table NewSample
    pub async fn insert<C>(
        self,
        created_by: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<super::Sample, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let id = self.id;
        table("samples")
            .insert()
            .columns("created_by,updated_by,id,sampled_by,state")
            .values(vec![self.into_row(created_by)])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })?;
        super::Sample::get(id, connection).await.map(|maybe_row| maybe_row.unwrap())
    }

    /// Update the struct in the database.
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user who is updating the struct.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows updated.
    pub async fn update<C>(
        self,
        user_id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        table("samples")
            .update()        
.set("id", gluesql::core::ast_builder::uuid(self.id.to_string()))        
.set("sampled_by", gluesql::core::ast_builder::num(self.sampled_by))        
.set("state", gluesql::core::ast_builder::num(self.state))        
.set("updated_by", gluesql::core::ast_builder::num(user_id))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewTeam {
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
    pub parent_team_id: Option<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewUserEmail {
    pub email: String,
    pub login_provider_id: i32,
    pub primary_email: bool,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewUser {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub profile_picture: Vec<u8>,
}

