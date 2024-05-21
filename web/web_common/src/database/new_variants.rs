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
    pub tagged: bool,
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
            (self.tagged.into()),
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
            .columns("created_by,id,tagged,updated_by")
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
pub struct NewSampledIndividualsTeamsRoleInvitation {
    pub table_id: Uuid,
    pub team_id: i32,
    pub role_id: i32,
}

impl Tabular for NewSampledIndividualsTeamsRoleInvitation {
    const TABLE: Table = Table::SampledIndividualsTeamsRoleInvitations;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSampledIndividualsTeamsRoleRequest {
    pub table_id: Uuid,
    pub team_id: i32,
    pub role_id: i32,
}

impl Tabular for NewSampledIndividualsTeamsRoleRequest {
    const TABLE: Table = Table::SampledIndividualsTeamsRoleRequests;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSampledIndividualsTeamsRole {
    pub table_id: Uuid,
    pub team_id: i32,
    pub role_id: i32,
}

impl Tabular for NewSampledIndividualsTeamsRole {
    const TABLE: Table = Table::SampledIndividualsTeamsRoles;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSampledIndividualsUsersRoleInvitation {
    pub table_id: Uuid,
    pub user_id: i32,
    pub role_id: i32,
}

impl Tabular for NewSampledIndividualsUsersRoleInvitation {
    const TABLE: Table = Table::SampledIndividualsUsersRoleInvitations;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSampledIndividualsUsersRoleRequest {
    pub table_id: Uuid,
    pub user_id: i32,
    pub role_id: i32,
}

impl Tabular for NewSampledIndividualsUsersRoleRequest {
    const TABLE: Table = Table::SampledIndividualsUsersRoleRequests;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSampledIndividualsUsersRole {
    pub table_id: Uuid,
    pub user_id: i32,
    pub role_id: i32,
}

impl Tabular for NewSampledIndividualsUsersRole {
    const TABLE: Table = Table::SampledIndividualsUsersRoles;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSample {
    pub barcode_id: Uuid,
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
            gluesql::core::ast_builder::uuid(self.barcode_id.to_string()),
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
        let barcode_id = self.barcode_id;
        table("samples")
            .insert()
            .columns("created_by,barcode_id,sampled_by,state,updated_by")
            .values(vec![self.into_row(created_by)])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })?;
        super::Sample::get(barcode_id, connection).await.map(|maybe_row| maybe_row.unwrap())
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
.set("barcode_id", gluesql::core::ast_builder::uuid(self.barcode_id.to_string()))        
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
pub struct NewSamplesTeamsRoleInvitation {
    pub table_id: Uuid,
    pub team_id: i32,
    pub role_id: i32,
}

impl Tabular for NewSamplesTeamsRoleInvitation {
    const TABLE: Table = Table::SamplesTeamsRoleInvitations;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSamplesTeamsRoleRequest {
    pub table_id: Uuid,
    pub team_id: i32,
    pub role_id: i32,
}

impl Tabular for NewSamplesTeamsRoleRequest {
    const TABLE: Table = Table::SamplesTeamsRoleRequests;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSamplesTeamsRole {
    pub table_id: Uuid,
    pub team_id: i32,
    pub role_id: i32,
}

impl Tabular for NewSamplesTeamsRole {
    const TABLE: Table = Table::SamplesTeamsRoles;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSamplesUsersRoleInvitation {
    pub table_id: Uuid,
    pub user_id: i32,
    pub role_id: i32,
}

impl Tabular for NewSamplesUsersRoleInvitation {
    const TABLE: Table = Table::SamplesUsersRoleInvitations;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSamplesUsersRoleRequest {
    pub table_id: Uuid,
    pub user_id: i32,
    pub role_id: i32,
}

impl Tabular for NewSamplesUsersRoleRequest {
    const TABLE: Table = Table::SamplesUsersRoleRequests;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSamplesUsersRole {
    pub table_id: Uuid,
    pub user_id: i32,
    pub role_id: i32,
}

impl Tabular for NewSamplesUsersRole {
    const TABLE: Table = Table::SamplesUsersRoles;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSpectraCollection {
    pub sample_id: Uuid,
}

impl Tabular for NewSpectraCollection {
    const TABLE: Table = Table::SpectraCollections;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSpectraCollectionsTeamsRoleInvitation {
    pub table_id: i32,
    pub team_id: i32,
    pub role_id: i32,
}

impl Tabular for NewSpectraCollectionsTeamsRoleInvitation {
    const TABLE: Table = Table::SpectraCollectionsTeamsRoleInvitations;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSpectraCollectionsTeamsRoleRequest {
    pub table_id: i32,
    pub team_id: i32,
    pub role_id: i32,
}

impl Tabular for NewSpectraCollectionsTeamsRoleRequest {
    const TABLE: Table = Table::SpectraCollectionsTeamsRoleRequests;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSpectraCollectionsTeamsRole {
    pub table_id: i32,
    pub team_id: i32,
    pub role_id: i32,
}

impl Tabular for NewSpectraCollectionsTeamsRole {
    const TABLE: Table = Table::SpectraCollectionsTeamsRoles;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSpectraCollectionsUsersRoleInvitation {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

impl Tabular for NewSpectraCollectionsUsersRoleInvitation {
    const TABLE: Table = Table::SpectraCollectionsUsersRoleInvitations;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSpectraCollectionsUsersRoleRequest {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

impl Tabular for NewSpectraCollectionsUsersRoleRequest {
    const TABLE: Table = Table::SpectraCollectionsUsersRoleRequests;
}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct NewSpectraCollectionsUsersRole {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

impl Tabular for NewSpectraCollectionsUsersRole {
    const TABLE: Table = Table::SpectraCollectionsUsersRoles;
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
