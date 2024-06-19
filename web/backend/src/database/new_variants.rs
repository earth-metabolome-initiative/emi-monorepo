//! This module contains the new variants of the database models.
//!
//! This module is automatically generated. Do not write anything here.

use crate::database::*;
use diesel::prelude::*;
/// Trait providing the insert method for the new variants.
pub(crate) trait InsertRow {
    /// The intermediate representation of the row.
    type Intermediate;

    /// The flat variant of the new variant.
    type Flat;

    /// Convert the new variant into the intermediate representation.
    fn to_intermediate(self, user_id: i32) -> Self::Intermediate;

    /// Insert the row into the database.
    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError>;
}

/// Intermediate representation of the new variant NewDerivedSample.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::derived_samples)]
pub(crate) struct IntermediateNewDerivedSample {
    created_by: i32,
    parent_sample_id: uuid::Uuid,
    child_sample_id: uuid::Uuid,
    quantity: f64,
    unit_id: i32,
    updated_by: i32,
}

impl InsertRow for web_common::database::NewDerivedSample {
    type Intermediate = IntermediateNewDerivedSample;
    type Flat = crate::database::flat_variants::DerivedSample;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewDerivedSample {
            created_by: user_id,
            parent_sample_id: self.parent_sample_id,
            child_sample_id: self.child_sample_id,
            quantity: self.quantity,
            unit_id: self.unit_id,
            updated_by: user_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::derived_samples;
        Ok(diesel::insert_into(derived_samples::dsl::derived_samples)
            .values(InsertRow::to_intermediate(self, user_id))
            .get_result(connection)?)
    }
}

/// Intermediate representation of the new variant NewNameplate.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::nameplates)]
pub(crate) struct IntermediateNewNameplate {
    created_by: i32,
    barcode: String,
    project_id: i32,
    category_id: i32,
    geolocation: postgis_diesel::types::Point,
    updated_by: i32,
}

impl InsertRow for web_common::database::NewNameplate {
    type Intermediate = IntermediateNewNameplate;
    type Flat = crate::database::flat_variants::Nameplate;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewNameplate {
            created_by: user_id,
            barcode: self.barcode,
            project_id: self.project_id,
            category_id: self.category_id,
            geolocation: self.geolocation.convert(),
            updated_by: user_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::nameplates;
        Ok(diesel::insert_into(nameplates::dsl::nameplates)
            .values(InsertRow::to_intermediate(self, user_id))
            .get_result(connection)?)
    }
}

/// Intermediate representation of the new variant NewObservation.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::observations)]
pub(crate) struct IntermediateNewObservation {
    created_by: i32,
    id: uuid::Uuid,
    parent_observation_id: Option<uuid::Uuid>,
    project_id: i32,
    organism_id: Option<uuid::Uuid>,
    sample_id: Option<uuid::Uuid>,
    subject_id: i32,
    notes: Option<String>,
    picture: crate::database::diesel_types::JPEG,
    updated_by: i32,
}

impl InsertRow for web_common::database::NewObservation {
    type Intermediate = IntermediateNewObservation;
    type Flat = crate::database::flat_variants::Observation;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewObservation {
            created_by: user_id,
            id: self.id,
            parent_observation_id: self.parent_observation_id,
            project_id: self.project_id,
            organism_id: self.organism_id,
            sample_id: self.sample_id,
            subject_id: self.subject_id,
            notes: self.notes,
            picture: self.picture.convert(),
            updated_by: user_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::observations;
        Ok(diesel::insert_into(observations::dsl::observations)
            .values(InsertRow::to_intermediate(self, user_id))
            .get_result(connection)?)
    }
}

/// Intermediate representation of the new variant NewOrganismBioOttTaxonItem.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::organism_bio_ott_taxon_items)]
pub(crate) struct IntermediateNewOrganismBioOttTaxonItem {
    created_by: i32,
    organism_id: uuid::Uuid,
    taxon_id: i32,
}

impl InsertRow for web_common::database::NewOrganismBioOttTaxonItem {
    type Intermediate = IntermediateNewOrganismBioOttTaxonItem;
    type Flat = crate::database::flat_variants::OrganismBioOttTaxonItem;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewOrganismBioOttTaxonItem {
            created_by: user_id,
            organism_id: self.organism_id,
            taxon_id: self.taxon_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::organism_bio_ott_taxon_items;
        Ok(
            diesel::insert_into(organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items)
                .values(InsertRow::to_intermediate(self, user_id))
                .get_result(connection)?,
        )
    }
}

/// Intermediate representation of the new variant NewOrganism.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::organisms)]
pub(crate) struct IntermediateNewOrganism {
    created_by: i32,
    id: uuid::Uuid,
    host_organism_id: Option<uuid::Uuid>,
    sample_id: Option<uuid::Uuid>,
    notes: Option<String>,
    wild: bool,
    nameplate_id: i32,
    project_id: i32,
    updated_by: i32,
}

impl InsertRow for web_common::database::NewOrganism {
    type Intermediate = IntermediateNewOrganism;
    type Flat = crate::database::flat_variants::Organism;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewOrganism {
            created_by: user_id,
            id: self.id,
            host_organism_id: self.host_organism_id,
            sample_id: self.sample_id,
            notes: self.notes,
            wild: self.wild,
            nameplate_id: self.nameplate_id,
            project_id: self.project_id,
            updated_by: user_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::organisms;
        Ok(diesel::insert_into(organisms::dsl::organisms)
            .values(InsertRow::to_intermediate(self, user_id))
            .get_result(connection)?)
    }
}

/// Intermediate representation of the new variant NewProject.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::projects)]
pub(crate) struct IntermediateNewProject {
    created_by: i32,
    name: String,
    description: String,
    public: bool,
    state_id: i32,
    icon_id: i32,
    color_id: i32,
    parent_project_id: Option<i32>,
    budget: Option<f64>,
    expenses: Option<f64>,
    expected_end_date: Option<chrono::NaiveDateTime>,
    end_date: Option<chrono::NaiveDateTime>,
    updated_by: i32,
}

impl InsertRow for web_common::database::NewProject {
    type Intermediate = IntermediateNewProject;
    type Flat = crate::database::flat_variants::Project;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewProject {
            created_by: user_id,
            name: self.name,
            description: self.description,
            public: self.public,
            state_id: self.state_id,
            icon_id: self.icon_id,
            color_id: self.color_id,
            parent_project_id: self.parent_project_id,
            budget: self.budget,
            expenses: self.expenses,
            expected_end_date: self.expected_end_date,
            end_date: self.end_date,
            updated_by: user_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::projects;
        Ok(diesel::insert_into(projects::dsl::projects)
            .values(InsertRow::to_intermediate(self, user_id))
            .get_result(connection)?)
    }
}

/// Intermediate representation of the new variant NewProjectsTeamsRoleInvitation.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::projects_teams_role_invitations)]
pub(crate) struct IntermediateNewProjectsTeamsRoleInvitation {
    created_by: i32,
    table_id: i32,
    team_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewProjectsTeamsRoleInvitation {
    type Intermediate = IntermediateNewProjectsTeamsRoleInvitation;
    type Flat = crate::database::flat_variants::ProjectsTeamsRoleInvitation;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewProjectsTeamsRoleInvitation {
            created_by: user_id,
            table_id: self.table_id,
            team_id: self.team_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::projects_teams_role_invitations;
        Ok(diesel::insert_into(
            projects_teams_role_invitations::dsl::projects_teams_role_invitations,
        )
        .values(InsertRow::to_intermediate(self, user_id))
        .get_result(connection)?)
    }
}

/// Intermediate representation of the new variant NewProjectsTeamsRoleRequest.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::projects_teams_role_requests)]
pub(crate) struct IntermediateNewProjectsTeamsRoleRequest {
    created_by: i32,
    table_id: i32,
    team_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewProjectsTeamsRoleRequest {
    type Intermediate = IntermediateNewProjectsTeamsRoleRequest;
    type Flat = crate::database::flat_variants::ProjectsTeamsRoleRequest;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewProjectsTeamsRoleRequest {
            created_by: user_id,
            table_id: self.table_id,
            team_id: self.team_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::projects_teams_role_requests;
        Ok(
            diesel::insert_into(projects_teams_role_requests::dsl::projects_teams_role_requests)
                .values(InsertRow::to_intermediate(self, user_id))
                .get_result(connection)?,
        )
    }
}

/// Intermediate representation of the new variant NewProjectsTeamsRole.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::projects_teams_roles)]
pub(crate) struct IntermediateNewProjectsTeamsRole {
    created_by: i32,
    table_id: i32,
    team_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewProjectsTeamsRole {
    type Intermediate = IntermediateNewProjectsTeamsRole;
    type Flat = crate::database::flat_variants::ProjectsTeamsRole;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewProjectsTeamsRole {
            created_by: user_id,
            table_id: self.table_id,
            team_id: self.team_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::projects_teams_roles;
        Ok(
            diesel::insert_into(projects_teams_roles::dsl::projects_teams_roles)
                .values(InsertRow::to_intermediate(self, user_id))
                .get_result(connection)?,
        )
    }
}

/// Intermediate representation of the new variant NewProjectsUsersRoleInvitation.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::projects_users_role_invitations)]
pub(crate) struct IntermediateNewProjectsUsersRoleInvitation {
    created_by: i32,
    table_id: i32,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewProjectsUsersRoleInvitation {
    type Intermediate = IntermediateNewProjectsUsersRoleInvitation;
    type Flat = crate::database::flat_variants::ProjectsUsersRoleInvitation;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewProjectsUsersRoleInvitation {
            created_by: user_id,
            table_id: self.table_id,
            user_id: self.user_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::projects_users_role_invitations;
        Ok(diesel::insert_into(
            projects_users_role_invitations::dsl::projects_users_role_invitations,
        )
        .values(InsertRow::to_intermediate(self, user_id))
        .get_result(connection)?)
    }
}

/// Intermediate representation of the new variant NewProjectsUsersRoleRequest.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::projects_users_role_requests)]
pub(crate) struct IntermediateNewProjectsUsersRoleRequest {
    created_by: i32,
    table_id: i32,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewProjectsUsersRoleRequest {
    type Intermediate = IntermediateNewProjectsUsersRoleRequest;
    type Flat = crate::database::flat_variants::ProjectsUsersRoleRequest;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewProjectsUsersRoleRequest {
            created_by: user_id,
            table_id: self.table_id,
            user_id: self.user_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::projects_users_role_requests;
        Ok(
            diesel::insert_into(projects_users_role_requests::dsl::projects_users_role_requests)
                .values(InsertRow::to_intermediate(self, user_id))
                .get_result(connection)?,
        )
    }
}

/// Intermediate representation of the new variant NewProjectsUsersRole.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::projects_users_roles)]
pub(crate) struct IntermediateNewProjectsUsersRole {
    created_by: i32,
    table_id: i32,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewProjectsUsersRole {
    type Intermediate = IntermediateNewProjectsUsersRole;
    type Flat = crate::database::flat_variants::ProjectsUsersRole;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewProjectsUsersRole {
            created_by: user_id,
            table_id: self.table_id,
            user_id: self.user_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::projects_users_roles;
        Ok(
            diesel::insert_into(projects_users_roles::dsl::projects_users_roles)
                .values(InsertRow::to_intermediate(self, user_id))
                .get_result(connection)?,
        )
    }
}

/// Intermediate representation of the new variant NewSampleBioOttTaxonItem.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::sample_bio_ott_taxon_items)]
pub(crate) struct IntermediateNewSampleBioOttTaxonItem {
    created_by: i32,
    sample_id: uuid::Uuid,
    taxon_id: i32,
}

impl InsertRow for web_common::database::NewSampleBioOttTaxonItem {
    type Intermediate = IntermediateNewSampleBioOttTaxonItem;
    type Flat = crate::database::flat_variants::SampleBioOttTaxonItem;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSampleBioOttTaxonItem {
            created_by: user_id,
            sample_id: self.sample_id,
            taxon_id: self.taxon_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::sample_bio_ott_taxon_items;
        Ok(
            diesel::insert_into(sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items)
                .values(InsertRow::to_intermediate(self, user_id))
                .get_result(connection)?,
        )
    }
}

/// Intermediate representation of the new variant NewSampleContainer.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::sample_containers)]
pub(crate) struct IntermediateNewSampleContainer {
    created_by: i32,
    barcode: String,
    project_id: i32,
    category_id: i32,
    updated_by: i32,
}

impl InsertRow for web_common::database::NewSampleContainer {
    type Intermediate = IntermediateNewSampleContainer;
    type Flat = crate::database::flat_variants::SampleContainer;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSampleContainer {
            created_by: user_id,
            barcode: self.barcode,
            project_id: self.project_id,
            category_id: self.category_id,
            updated_by: user_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::sample_containers;
        Ok(
            diesel::insert_into(sample_containers::dsl::sample_containers)
                .values(InsertRow::to_intermediate(self, user_id))
                .get_result(connection)?,
        )
    }
}

/// Intermediate representation of the new variant NewSample.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::samples)]
pub(crate) struct IntermediateNewSample {
    created_by: i32,
    id: uuid::Uuid,
    container_id: i32,
    notes: Option<String>,
    project_id: i32,
    sampled_by: i32,
    state_id: i32,
    updated_by: i32,
}

impl InsertRow for web_common::database::NewSample {
    type Intermediate = IntermediateNewSample;
    type Flat = crate::database::flat_variants::Sample;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSample {
            created_by: user_id,
            id: self.id,
            container_id: self.container_id,
            notes: self.notes,
            project_id: self.project_id,
            sampled_by: self.sampled_by,
            state_id: self.state_id,
            updated_by: user_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::samples;
        Ok(diesel::insert_into(samples::dsl::samples)
            .values(InsertRow::to_intermediate(self, user_id))
            .get_result(connection)?)
    }
}

/// Intermediate representation of the new variant NewSpectraCollection.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::spectra_collections)]
pub(crate) struct IntermediateNewSpectraCollection {
    created_by: i32,
    notes: Option<String>,
    sample_id: uuid::Uuid,
    updated_by: i32,
}

impl InsertRow for web_common::database::NewSpectraCollection {
    type Intermediate = IntermediateNewSpectraCollection;
    type Flat = crate::database::flat_variants::SpectraCollection;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSpectraCollection {
            created_by: user_id,
            notes: self.notes,
            sample_id: self.sample_id,
            updated_by: user_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::spectra_collections;
        Ok(
            diesel::insert_into(spectra_collections::dsl::spectra_collections)
                .values(InsertRow::to_intermediate(self, user_id))
                .get_result(connection)?,
        )
    }
}

/// Intermediate representation of the new variant NewTeam.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::teams)]
pub(crate) struct IntermediateNewTeam {
    created_by: i32,
    name: String,
    description: String,
    icon_id: i32,
    color_id: i32,
    state_id: i32,
    parent_team_id: Option<i32>,
    updated_by: i32,
}

impl InsertRow for web_common::database::NewTeam {
    type Intermediate = IntermediateNewTeam;
    type Flat = crate::database::flat_variants::Team;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewTeam {
            created_by: user_id,
            name: self.name,
            description: self.description,
            icon_id: self.icon_id,
            color_id: self.color_id,
            state_id: self.state_id,
            parent_team_id: self.parent_team_id,
            updated_by: user_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::teams;
        Ok(diesel::insert_into(teams::dsl::teams)
            .values(InsertRow::to_intermediate(self, user_id))
            .get_result(connection)?)
    }
}

/// Intermediate representation of the new variant NewTeamsTeamsRoleInvitation.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::teams_teams_role_invitations)]
pub(crate) struct IntermediateNewTeamsTeamsRoleInvitation {
    created_by: i32,
    table_id: i32,
    team_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewTeamsTeamsRoleInvitation {
    type Intermediate = IntermediateNewTeamsTeamsRoleInvitation;
    type Flat = crate::database::flat_variants::TeamsTeamsRoleInvitation;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewTeamsTeamsRoleInvitation {
            created_by: user_id,
            table_id: self.table_id,
            team_id: self.team_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::teams_teams_role_invitations;
        Ok(
            diesel::insert_into(teams_teams_role_invitations::dsl::teams_teams_role_invitations)
                .values(InsertRow::to_intermediate(self, user_id))
                .get_result(connection)?,
        )
    }
}

/// Intermediate representation of the new variant NewTeamsUsersRoleInvitation.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::teams_users_role_invitations)]
pub(crate) struct IntermediateNewTeamsUsersRoleInvitation {
    created_by: i32,
    table_id: i32,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewTeamsUsersRoleInvitation {
    type Intermediate = IntermediateNewTeamsUsersRoleInvitation;
    type Flat = crate::database::flat_variants::TeamsUsersRoleInvitation;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewTeamsUsersRoleInvitation {
            created_by: user_id,
            table_id: self.table_id,
            user_id: self.user_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::teams_users_role_invitations;
        Ok(
            diesel::insert_into(teams_users_role_invitations::dsl::teams_users_role_invitations)
                .values(InsertRow::to_intermediate(self, user_id))
                .get_result(connection)?,
        )
    }
}

/// Intermediate representation of the new variant NewTeamsUsersRoleRequest.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::teams_users_role_requests)]
pub(crate) struct IntermediateNewTeamsUsersRoleRequest {
    created_by: i32,
    table_id: i32,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewTeamsUsersRoleRequest {
    type Intermediate = IntermediateNewTeamsUsersRoleRequest;
    type Flat = crate::database::flat_variants::TeamsUsersRoleRequest;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewTeamsUsersRoleRequest {
            created_by: user_id,
            table_id: self.table_id,
            user_id: self.user_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::teams_users_role_requests;
        Ok(
            diesel::insert_into(teams_users_role_requests::dsl::teams_users_role_requests)
                .values(InsertRow::to_intermediate(self, user_id))
                .get_result(connection)?,
        )
    }
}

/// Intermediate representation of the new variant NewTeamsUsersRole.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::teams_users_roles)]
pub(crate) struct IntermediateNewTeamsUsersRole {
    created_by: i32,
    table_id: i32,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewTeamsUsersRole {
    type Intermediate = IntermediateNewTeamsUsersRole;
    type Flat = crate::database::flat_variants::TeamsUsersRole;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewTeamsUsersRole {
            created_by: user_id,
            table_id: self.table_id,
            user_id: self.user_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::teams_users_roles;
        Ok(
            diesel::insert_into(teams_users_roles::dsl::teams_users_roles)
                .values(InsertRow::to_intermediate(self, user_id))
                .get_result(connection)?,
        )
    }
}

/// Intermediate representation of the new variant NewUserEmail.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::user_emails)]
pub(crate) struct IntermediateNewUserEmail {
    created_by: i32,
    email: String,
    login_provider_id: i32,
    primary_email: bool,
}

impl InsertRow for web_common::database::NewUserEmail {
    type Intermediate = IntermediateNewUserEmail;
    type Flat = crate::database::flat_variants::UserEmail;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewUserEmail {
            created_by: user_id,
            email: self.email,
            login_provider_id: self.login_provider_id,
            primary_email: self.primary_email,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::user_emails;
        Ok(diesel::insert_into(user_emails::dsl::user_emails)
            .values(InsertRow::to_intermediate(self, user_id))
            .get_result(connection)?)
    }
}

/// Intermediate representation of the new variant NewUser.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::users)]
pub(crate) struct IntermediateNewUser {
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    description: Option<String>,
    organization_id: Option<i32>,
    picture: crate::database::diesel_types::JPEG,
}

impl InsertRow for web_common::database::NewUser {
    type Intermediate = IntermediateNewUser;
    type Flat = crate::database::flat_variants::User;

    fn to_intermediate(self, _user_id: i32) -> Self::Intermediate {
        IntermediateNewUser {
            first_name: self.first_name,
            middle_name: self.middle_name,
            last_name: self.last_name,
            description: self.description,
            organization_id: self.organization_id,
            picture: self.picture.convert(),
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::users;
        assert_eq!(user_id, 0);
        Ok(diesel::insert_into(users::dsl::users)
            .values(InsertRow::to_intermediate(self, 0))
            .get_result(connection)?)
    }
}

/// Intermediate representation of the new variant NewUsersUsersRoleInvitation.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::users_users_role_invitations)]
pub(crate) struct IntermediateNewUsersUsersRoleInvitation {
    created_by: i32,
    table_id: i32,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewUsersUsersRoleInvitation {
    type Intermediate = IntermediateNewUsersUsersRoleInvitation;
    type Flat = crate::database::flat_variants::UsersUsersRoleInvitation;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewUsersUsersRoleInvitation {
            created_by: user_id,
            table_id: self.table_id,
            user_id: self.user_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::users_users_role_invitations;
        Ok(
            diesel::insert_into(users_users_role_invitations::dsl::users_users_role_invitations)
                .values(InsertRow::to_intermediate(self, user_id))
                .get_result(connection)?,
        )
    }
}

/// Intermediate representation of the new variant NewUsersUsersRoleRequest.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::users_users_role_requests)]
pub(crate) struct IntermediateNewUsersUsersRoleRequest {
    created_by: i32,
    table_id: i32,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewUsersUsersRoleRequest {
    type Intermediate = IntermediateNewUsersUsersRoleRequest;
    type Flat = crate::database::flat_variants::UsersUsersRoleRequest;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewUsersUsersRoleRequest {
            created_by: user_id,
            table_id: self.table_id,
            user_id: self.user_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::users_users_role_requests;
        Ok(
            diesel::insert_into(users_users_role_requests::dsl::users_users_role_requests)
                .values(InsertRow::to_intermediate(self, user_id))
                .get_result(connection)?,
        )
    }
}

/// Intermediate representation of the new variant NewUsersUsersRole.
#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::users_users_roles)]
pub(crate) struct IntermediateNewUsersUsersRole {
    created_by: i32,
    table_id: i32,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewUsersUsersRole {
    type Intermediate = IntermediateNewUsersUsersRole;
    type Flat = crate::database::flat_variants::UsersUsersRole;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewUsersUsersRole {
            created_by: user_id,
            table_id: self.table_id,
            user_id: self.user_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, web_common::api::ApiError> {
        use crate::database::schema::users_users_roles;
        Ok(
            diesel::insert_into(users_users_roles::dsl::users_users_roles)
                .values(InsertRow::to_intermediate(self, user_id))
                .get_result(connection)?,
        )
    }
}
