//! This module contains the new variants of the database models.
//!
//! This module is automatically generated. Do not write anything here.

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewContainerHorizontalRule {
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

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewContainerVerticalRule {
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

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewItemCategory {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewProcedure {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewProjectRequirement {
    pub project_id: i32,
    pub item_category_id: i32,
    pub quantity: i32,
    pub unit_id: Option<i32>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewProject {
    pub name: String,
    pub description: String,
    pub public: bool,
    pub state_id: i32,
    pub parent_project_id: Option<i32>,
    pub budget: Option<f64>,
    pub expenses: Option<f64>,
    pub expected_end_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSampledIndividual {
    pub id: Uuid,
    pub name: Option<String>,
    pub tagged: bool,
}

#[cfg(feature = "frontend")]
impl NewSampledIndividual {
    pub fn into_row(self, created_by: i32) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(created_by),
            gluesql::core::ast_builder::num(created_by),
            gluesql::core::ast_builder::uuid(self.id.to_string()),
            match self.name {
                Some(name) => gluesql::core::ast_builder::text(name),
                None => gluesql::core::ast_builder::null(),
            },
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
            .columns("created_by,updated_by,id,name,tagged")
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
        let mut update_row = table("sampled_individuals")
            .update()        
.set("id", gluesql::core::ast_builder::uuid(self.id.to_string()))        
.set("tagged", self.tagged)        
.set("updated_by", gluesql::core::ast_builder::num(user_id));
        if let Some(name) = self.name {
            update_row = update_row.set("name", gluesql::core::ast_builder::text(name));
        }
            update_row.execute(connection)
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
    pub procedure_id: Uuid,
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
            gluesql::core::ast_builder::uuid(self.procedure_id.to_string()),
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
            .columns("created_by,updated_by,id,sampled_by,procedure_id,state")
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
.set("procedure_id", gluesql::core::ast_builder::uuid(self.procedure_id.to_string()))        
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
pub struct NewSamplingProcedure {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[cfg(feature = "frontend")]
impl NewSamplingProcedure {
    pub fn into_row(self, created_by: i32) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(created_by),
            gluesql::core::ast_builder::num(created_by),
            gluesql::core::ast_builder::uuid(self.id.to_string()),
            gluesql::core::ast_builder::text(self.name),
            match self.description {
                Some(description) => gluesql::core::ast_builder::text(description),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the NewSamplingProcedure into the database.
    ///
    /// # Arguments
    /// * `created_by` - The id of the user inserting the row.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table NewSamplingProcedure
    pub async fn insert<C>(
        self,
        created_by: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<super::SamplingProcedure, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let id = self.id;
        table("sampling_procedures")
            .insert()
            .columns("created_by,updated_by,id,name,description")
            .values(vec![self.into_row(created_by)])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })?;
        super::SamplingProcedure::get(id, connection).await.map(|maybe_row| maybe_row.unwrap())
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
        let mut update_row = table("sampling_procedures")
            .update()        
.set("id", gluesql::core::ast_builder::uuid(self.id.to_string()))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("updated_by", gluesql::core::ast_builder::num(user_id));
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

}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewTeam {
    pub name: String,
    pub description: String,
    pub parent_team_id: Option<i32>,
}

