//! This module contains the update variants of the database models.
//!
//! Some of the update variants would be identical to the new variants, //! and as such we do not generate them. You will find here the update variants //! only for the tables that have a primary key that is not a UUID.
//! This module is automatically generated. Do not write anything here.

use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use uuid::Uuid;
use super::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct UpdateProject {
    pub id: i32,
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

impl Tabular for UpdateProject {
    const TABLE: Table = Table::Projects;
}
#[cfg(feature = "frontend")]
impl UpdateProject {
    pub fn into_row(self, updated_by: i32) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(updated_by),
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            (self.public.into()),
            gluesql::core::ast_builder::num(self.state_id),
            gluesql::core::ast_builder::num(self.icon_id),
            gluesql::core::ast_builder::num(self.color_id),
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
        let mut update_row = table("projects")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("description", gluesql::core::ast_builder::text(self.description))        
.set("public", self.public)        
.set("state_id", gluesql::core::ast_builder::num(self.state_id))        
.set("icon_id", gluesql::core::ast_builder::num(self.icon_id))        
.set("color_id", gluesql::core::ast_builder::num(self.color_id))        
.set("updated_by", gluesql::core::ast_builder::num(user_id));
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

}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSpectraCollection {
    pub id: i32,
    pub notes: Option<String>,
    pub sample_id: Uuid,
}

impl Tabular for UpdateSpectraCollection {
    const TABLE: Table = Table::SpectraCollections;
}
#[cfg(feature = "frontend")]
impl UpdateSpectraCollection {
    pub fn into_row(self, updated_by: i32) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(updated_by),
            gluesql::core::ast_builder::num(self.id),
            match self.notes {
                Some(notes) => gluesql::core::ast_builder::text(notes),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::uuid(self.sample_id.to_string()),
        ]
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
        let mut update_row = table("spectra_collections")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("sample_id", gluesql::core::ast_builder::uuid(self.sample_id.to_string()))        
.set("updated_by", gluesql::core::ast_builder::num(user_id));
        if let Some(notes) = self.notes {
            update_row = update_row.set("notes", gluesql::core::ast_builder::text(notes));
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
pub struct UpdateTeam {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
    pub parent_team_id: Option<i32>,
}

impl Tabular for UpdateTeam {
    const TABLE: Table = Table::Teams;
}
#[cfg(feature = "frontend")]
impl UpdateTeam {
    pub fn into_row(self, updated_by: i32) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(updated_by),
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            gluesql::core::ast_builder::num(self.icon_id),
            gluesql::core::ast_builder::num(self.color_id),
            match self.parent_team_id {
                Some(parent_team_id) => gluesql::core::ast_builder::num(parent_team_id),
                None => gluesql::core::ast_builder::null(),
            },
        ]
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
        let mut update_row = table("teams")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("description", gluesql::core::ast_builder::text(self.description))        
.set("icon_id", gluesql::core::ast_builder::num(self.icon_id))        
.set("color_id", gluesql::core::ast_builder::num(self.color_id))        
.set("updated_by", gluesql::core::ast_builder::num(user_id));
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

}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct UpdateUser {
    pub id: i32,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub description: Option<String>,
    pub profile_picture: Vec<u8>,
}

impl Tabular for UpdateUser {
    const TABLE: Table = Table::Users;
}
#[cfg(feature = "frontend")]
impl UpdateUser {
    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.first_name),
            match self.middle_name {
                Some(middle_name) => gluesql::core::ast_builder::text(middle_name),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::text(self.last_name),
            match self.description {
                Some(description) => gluesql::core::ast_builder::text(description),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::bytea(self.profile_picture),
        ]
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
.set("profile_picture", gluesql::core::ast_builder::bytea(self.profile_picture));
        if let Some(middle_name) = self.middle_name {
            update_row = update_row.set("middle_name", gluesql::core::ast_builder::text(middle_name));
        }
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
