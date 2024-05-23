//! This module contains the new variants of the database models.
//!
//! This module is automatically generated. Do not write anything here.

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use super::*;

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewDerivedSample {
    pub parent_sample_id: Uuid,
    pub child_sample_id: Uuid,
}

impl Tabular for NewDerivedSample {
    const TABLE: Table = Table::DerivedSamples;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewObservation {
    pub id: Uuid,
    pub project_id: i32,
    pub individual_id: Option<Uuid>,
    pub notes: Option<String>,
    pub picture: Vec<u8>,
}

impl Tabular for NewObservation {
    const TABLE: Table = Table::Observations;
}
#[cfg(feature = "frontend")]
impl NewObservation {
    pub fn into_row(self, created_by: i32) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(created_by),
            gluesql::core::ast_builder::uuid(self.id.to_string()),
            gluesql::core::ast_builder::num(self.project_id),
            match self.individual_id {
                Some(individual_id) => gluesql::core::ast_builder::uuid(individual_id.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.notes {
                Some(notes) => gluesql::core::ast_builder::text(notes),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::bytea(self.picture),
            gluesql::core::ast_builder::num(created_by),
        ]
    }

    /// Insert the NewObservation into the database.
    ///
    /// # Arguments
    /// * `created_by` - The id of the user inserting the row.
    /// * `connection` - The connection to the database.
    ///
    /// # Returns
    /// The number of rows inserted in table NewObservation
    pub async fn insert<C>(
        self,
        created_by: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<super::Observation, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let id = self.id;
        table("observations")
            .insert()
            .columns("created_by,id,project_id,individual_id,notes,picture,updated_by")
            .values(vec![self.into_row(created_by)])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })?;
        super::Observation::get(id, connection).await.map(|maybe_row| maybe_row.unwrap())
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
        let mut update_row = table("observations")
            .update()        
.set("id", gluesql::core::ast_builder::uuid(self.id.to_string()))        
.set("project_id", gluesql::core::ast_builder::num(self.project_id))        
.set("picture", gluesql::core::ast_builder::bytea(self.picture))        
.set("updated_by", gluesql::core::ast_builder::num(user_id));
        if let Some(individual_id) = self.individual_id {
            update_row = update_row.set("individual_id", gluesql::core::ast_builder::uuid(individual_id.to_string()));
        }
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

impl Tabular for NewProject {
    const TABLE: Table = Table::Projects;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewProjectsTeamsRoleInvitation {
    pub table_id: i32,
    pub team_id: i32,
    pub role_id: i32,
}

impl Tabular for NewProjectsTeamsRoleInvitation {
    const TABLE: Table = Table::ProjectsTeamsRoleInvitations;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewProjectsTeamsRoleRequest {
    pub table_id: i32,
    pub team_id: i32,
    pub role_id: i32,
}

impl Tabular for NewProjectsTeamsRoleRequest {
    const TABLE: Table = Table::ProjectsTeamsRoleRequests;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewProjectsTeamsRole {
    pub table_id: i32,
    pub team_id: i32,
    pub role_id: i32,
}

impl Tabular for NewProjectsTeamsRole {
    const TABLE: Table = Table::ProjectsTeamsRoles;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewProjectsUsersRoleInvitation {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

impl Tabular for NewProjectsUsersRoleInvitation {
    const TABLE: Table = Table::ProjectsUsersRoleInvitations;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewProjectsUsersRoleRequest {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

impl Tabular for NewProjectsUsersRoleRequest {
    const TABLE: Table = Table::ProjectsUsersRoleRequests;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewProjectsUsersRole {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

impl Tabular for NewProjectsUsersRole {
    const TABLE: Table = Table::ProjectsUsersRoles;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSampleBioOttTaxonItem {
    pub sample_id: Uuid,
    pub taxon_id: i32,
}

impl Tabular for NewSampleBioOttTaxonItem {
    const TABLE: Table = Table::SampleBioOttTaxonItems;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSampleContainer {
    pub barcode: String,
    pub category_id: i32,
}

impl Tabular for NewSampleContainer {
    const TABLE: Table = Table::SampleContainers;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSampledIndividualBioOttTaxonItem {
    pub sampled_individual_id: Uuid,
    pub taxon_id: i32,
}

impl Tabular for NewSampledIndividualBioOttTaxonItem {
    const TABLE: Table = Table::SampledIndividualBioOttTaxonItems;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSampledIndividual {
    pub id: Uuid,
    pub notes: Option<String>,
    pub barcode: Option<String>,
    pub project_id: i32,
    pub picture: Vec<u8>,
}

impl Tabular for NewSampledIndividual {
    const TABLE: Table = Table::SampledIndividuals;
}
#[cfg(feature = "frontend")]
impl NewSampledIndividual {
    pub fn into_row(self, created_by: i32) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(created_by),
            gluesql::core::ast_builder::uuid(self.id.to_string()),
            match self.notes {
                Some(notes) => gluesql::core::ast_builder::text(notes),
                None => gluesql::core::ast_builder::null(),
            },
            match self.barcode {
                Some(barcode) => gluesql::core::ast_builder::text(barcode),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::num(self.project_id),
            gluesql::core::ast_builder::bytea(self.picture),
            gluesql::core::ast_builder::num(created_by),
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
            .columns("created_by,id,notes,barcode,project_id,picture,updated_by")
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
.set("project_id", gluesql::core::ast_builder::num(self.project_id))        
.set("picture", gluesql::core::ast_builder::bytea(self.picture))        
.set("updated_by", gluesql::core::ast_builder::num(user_id));
        if let Some(notes) = self.notes {
            update_row = update_row.set("notes", gluesql::core::ast_builder::text(notes));
        }
        if let Some(barcode) = self.barcode {
            update_row = update_row.set("barcode", gluesql::core::ast_builder::text(barcode));
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
    pub container_id: i32,
    pub notes: Option<String>,
    pub sampled_by: i32,
    pub state: i32,
}

impl Tabular for NewSample {
    const TABLE: Table = Table::Samples;
}
#[cfg(feature = "frontend")]
impl NewSample {
    pub fn into_row(self, created_by: i32) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(created_by),
            gluesql::core::ast_builder::uuid(self.id.to_string()),
            gluesql::core::ast_builder::num(self.container_id),
            match self.notes {
                Some(notes) => gluesql::core::ast_builder::text(notes),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::num(self.sampled_by),
            gluesql::core::ast_builder::num(self.state),
            gluesql::core::ast_builder::num(created_by),
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
            .columns("created_by,id,container_id,notes,sampled_by,state,updated_by")
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
        let mut update_row = table("samples")
            .update()        
.set("id", gluesql::core::ast_builder::uuid(self.id.to_string()))        
.set("container_id", gluesql::core::ast_builder::num(self.container_id))        
.set("sampled_by", gluesql::core::ast_builder::num(self.sampled_by))        
.set("state", gluesql::core::ast_builder::num(self.state))        
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
pub struct NewSpectraCollection {
    pub notes: Option<String>,
    pub sample_id: Uuid,
}

impl Tabular for NewSpectraCollection {
    const TABLE: Table = Table::SpectraCollections;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewTeam {
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
    pub parent_team_id: Option<i32>,
}

impl Tabular for NewTeam {
    const TABLE: Table = Table::Teams;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewTeamsTeamsRoleInvitation {
    pub table_id: i32,
    pub team_id: i32,
    pub role_id: i32,
}

impl Tabular for NewTeamsTeamsRoleInvitation {
    const TABLE: Table = Table::TeamsTeamsRoleInvitations;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewTeamsUsersRoleInvitation {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

impl Tabular for NewTeamsUsersRoleInvitation {
    const TABLE: Table = Table::TeamsUsersRoleInvitations;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewTeamsUsersRoleRequest {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

impl Tabular for NewTeamsUsersRoleRequest {
    const TABLE: Table = Table::TeamsUsersRoleRequests;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewTeamsUsersRole {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

impl Tabular for NewTeamsUsersRole {
    const TABLE: Table = Table::TeamsUsersRoles;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewUserEmail {
    pub email: String,
    pub login_provider_id: i32,
    pub primary_email: bool,
}

impl Tabular for NewUserEmail {
    const TABLE: Table = Table::UserEmails;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewUser {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub description: Option<String>,
    pub profile_picture: Vec<u8>,
}

impl Tabular for NewUser {
    const TABLE: Table = Table::Users;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewUsersUsersRoleInvitation {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

impl Tabular for NewUsersUsersRoleInvitation {
    const TABLE: Table = Table::UsersUsersRoleInvitations;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewUsersUsersRoleRequest {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

impl Tabular for NewUsersUsersRoleRequest {
    const TABLE: Table = Table::UsersUsersRoleRequests;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewUsersUsersRole {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

impl Tabular for NewUsersUsersRole {
    const TABLE: Table = Table::UsersUsersRoles;
}
