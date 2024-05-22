//! This module contains the new variants of the database models.
//!
//! This module is automatically generated. Do not write anything here.

use diesel::prelude::*;
use crate::models::*;
use crate::schema::*;
use diesel::r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;
use uuid::Uuid;
use chrono::NaiveDateTime;
/// Trait providing the insert method for the new variants.
pub(super) trait InsertRow {
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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error>;
}

/// Intermediate representation of the new variant NewDerivedSample.
#[derive(Insertable)]
#[diesel(table_name = derived_samples)]
pub(super) struct IntermediateNewDerivedSample {
    created_by: i32,
    parent_sample_id: Uuid,
    child_sample_id: Uuid,
}

impl InsertRow for web_common::database::NewDerivedSample {
    type Intermediate = IntermediateNewDerivedSample;
    type Flat = DerivedSample;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewDerivedSample {
            created_by: user_id,
            parent_sample_id: self.parent_sample_id,
            child_sample_id: self.child_sample_id,
        }
    }

    fn insert(
        self,
       user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::derived_samples;
        diesel::insert_into(derived_samples::dsl::derived_samples)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewObservation.
#[derive(Insertable)]
#[diesel(table_name = observations)]
pub(super) struct IntermediateNewObservation {
    created_by: i32,
    id: Uuid,
    project_id: i32,
    individual_id: Option<Uuid>,
    notes: Option<String>,
    picture: Vec<u8>,
    updated_by: i32,
}

impl InsertRow for web_common::database::NewObservation {
    type Intermediate = IntermediateNewObservation;
    type Flat = Observation;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewObservation {
            created_by: user_id,
            id: self.id,
            project_id: self.project_id,
            individual_id: self.individual_id,
            notes: self.notes,
            picture: self.picture,
            updated_by: user_id,
        }
    }

    fn insert(
        self,
       user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::observations;
        diesel::insert_into(observations::dsl::observations)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewProject.
#[derive(Insertable)]
#[diesel(table_name = projects)]
pub(super) struct IntermediateNewProject {
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
    expected_end_date: Option<NaiveDateTime>,
    end_date: Option<NaiveDateTime>,
    updated_by: i32,
}

impl InsertRow for web_common::database::NewProject {
    type Intermediate = IntermediateNewProject;
    type Flat = Project;

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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::projects;
        diesel::insert_into(projects::dsl::projects)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewProjectsTeamsRoleInvitation.
#[derive(Insertable)]
#[diesel(table_name = projects_teams_role_invitations)]
pub(super) struct IntermediateNewProjectsTeamsRoleInvitation {
    created_by: i32,
    table_id: i32,
    team_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewProjectsTeamsRoleInvitation {
    type Intermediate = IntermediateNewProjectsTeamsRoleInvitation;
    type Flat = ProjectsTeamsRoleInvitation;

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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::projects_teams_role_invitations;
        diesel::insert_into(projects_teams_role_invitations::dsl::projects_teams_role_invitations)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewProjectsTeamsRoleRequest.
#[derive(Insertable)]
#[diesel(table_name = projects_teams_role_requests)]
pub(super) struct IntermediateNewProjectsTeamsRoleRequest {
    created_by: i32,
    table_id: i32,
    team_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewProjectsTeamsRoleRequest {
    type Intermediate = IntermediateNewProjectsTeamsRoleRequest;
    type Flat = ProjectsTeamsRoleRequest;

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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::projects_teams_role_requests;
        diesel::insert_into(projects_teams_role_requests::dsl::projects_teams_role_requests)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewProjectsTeamsRole.
#[derive(Insertable)]
#[diesel(table_name = projects_teams_roles)]
pub(super) struct IntermediateNewProjectsTeamsRole {
    created_by: i32,
    table_id: i32,
    team_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewProjectsTeamsRole {
    type Intermediate = IntermediateNewProjectsTeamsRole;
    type Flat = ProjectsTeamsRole;

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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::projects_teams_roles;
        diesel::insert_into(projects_teams_roles::dsl::projects_teams_roles)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewProjectsUsersRoleInvitation.
#[derive(Insertable)]
#[diesel(table_name = projects_users_role_invitations)]
pub(super) struct IntermediateNewProjectsUsersRoleInvitation {
    created_by: i32,
    table_id: i32,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewProjectsUsersRoleInvitation {
    type Intermediate = IntermediateNewProjectsUsersRoleInvitation;
    type Flat = ProjectsUsersRoleInvitation;

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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::projects_users_role_invitations;
        diesel::insert_into(projects_users_role_invitations::dsl::projects_users_role_invitations)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewProjectsUsersRoleRequest.
#[derive(Insertable)]
#[diesel(table_name = projects_users_role_requests)]
pub(super) struct IntermediateNewProjectsUsersRoleRequest {
    created_by: i32,
    table_id: i32,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewProjectsUsersRoleRequest {
    type Intermediate = IntermediateNewProjectsUsersRoleRequest;
    type Flat = ProjectsUsersRoleRequest;

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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::projects_users_role_requests;
        diesel::insert_into(projects_users_role_requests::dsl::projects_users_role_requests)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewProjectsUsersRole.
#[derive(Insertable)]
#[diesel(table_name = projects_users_roles)]
pub(super) struct IntermediateNewProjectsUsersRole {
    created_by: i32,
    table_id: i32,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewProjectsUsersRole {
    type Intermediate = IntermediateNewProjectsUsersRole;
    type Flat = ProjectsUsersRole;

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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::projects_users_roles;
        diesel::insert_into(projects_users_roles::dsl::projects_users_roles)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewSampleBioOttTaxonItem.
#[derive(Insertable)]
#[diesel(table_name = sample_bio_ott_taxon_items)]
pub(super) struct IntermediateNewSampleBioOttTaxonItem {
    created_by: i32,
    sample_id: Uuid,
    taxon_id: i32,
}

impl InsertRow for web_common::database::NewSampleBioOttTaxonItem {
    type Intermediate = IntermediateNewSampleBioOttTaxonItem;
    type Flat = SampleBioOttTaxonItem;

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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::sample_bio_ott_taxon_items;
        diesel::insert_into(sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewSampleContainer.
#[derive(Insertable)]
#[diesel(table_name = sample_containers)]
pub(super) struct IntermediateNewSampleContainer {
    created_by: i32,
    barcode: String,
    category_id: i32,
}

impl InsertRow for web_common::database::NewSampleContainer {
    type Intermediate = IntermediateNewSampleContainer;
    type Flat = SampleContainer;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSampleContainer {
            created_by: user_id,
            barcode: self.barcode,
            category_id: self.category_id,
        }
    }

    fn insert(
        self,
       user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::sample_containers;
        diesel::insert_into(sample_containers::dsl::sample_containers)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewSampledIndividualBioOttTaxonItem.
#[derive(Insertable)]
#[diesel(table_name = sampled_individual_bio_ott_taxon_items)]
pub(super) struct IntermediateNewSampledIndividualBioOttTaxonItem {
    created_by: i32,
    sampled_individual_id: Uuid,
    taxon_id: i32,
}

impl InsertRow for web_common::database::NewSampledIndividualBioOttTaxonItem {
    type Intermediate = IntermediateNewSampledIndividualBioOttTaxonItem;
    type Flat = SampledIndividualBioOttTaxonItem;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSampledIndividualBioOttTaxonItem {
            created_by: user_id,
            sampled_individual_id: self.sampled_individual_id,
            taxon_id: self.taxon_id,
        }
    }

    fn insert(
        self,
       user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        diesel::insert_into(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewSampledIndividual.
#[derive(Insertable)]
#[diesel(table_name = sampled_individuals)]
pub(super) struct IntermediateNewSampledIndividual {
    created_by: i32,
    id: Uuid,
    notes: Option<String>,
    project_id: i32,
    tagged: bool,
    picture: Vec<u8>,
    updated_by: i32,
}

impl InsertRow for web_common::database::NewSampledIndividual {
    type Intermediate = IntermediateNewSampledIndividual;
    type Flat = SampledIndividual;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSampledIndividual {
            created_by: user_id,
            id: self.id,
            notes: self.notes,
            project_id: self.project_id,
            tagged: self.tagged,
            picture: self.picture,
            updated_by: user_id,
        }
    }

    fn insert(
        self,
       user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::sampled_individuals;
        diesel::insert_into(sampled_individuals::dsl::sampled_individuals)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewSample.
#[derive(Insertable)]
#[diesel(table_name = samples)]
pub(super) struct IntermediateNewSample {
    created_by: i32,
    id: Uuid,
    container_id: i32,
    notes: Option<String>,
    sampled_by: i32,
    state: i32,
    updated_by: i32,
}

impl InsertRow for web_common::database::NewSample {
    type Intermediate = IntermediateNewSample;
    type Flat = Sample;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSample {
            created_by: user_id,
            id: self.id,
            container_id: self.container_id,
            notes: self.notes,
            sampled_by: self.sampled_by,
            state: self.state,
            updated_by: user_id,
        }
    }

    fn insert(
        self,
       user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::samples;
        diesel::insert_into(samples::dsl::samples)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewSamplesTeamsRoleInvitation.
#[derive(Insertable)]
#[diesel(table_name = samples_teams_role_invitations)]
pub(super) struct IntermediateNewSamplesTeamsRoleInvitation {
    created_by: i32,
    table_id: Uuid,
    team_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewSamplesTeamsRoleInvitation {
    type Intermediate = IntermediateNewSamplesTeamsRoleInvitation;
    type Flat = SamplesTeamsRoleInvitation;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSamplesTeamsRoleInvitation {
            created_by: user_id,
            table_id: self.table_id,
            team_id: self.team_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
       user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::samples_teams_role_invitations;
        diesel::insert_into(samples_teams_role_invitations::dsl::samples_teams_role_invitations)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewSamplesTeamsRoleRequest.
#[derive(Insertable)]
#[diesel(table_name = samples_teams_role_requests)]
pub(super) struct IntermediateNewSamplesTeamsRoleRequest {
    created_by: i32,
    table_id: Uuid,
    team_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewSamplesTeamsRoleRequest {
    type Intermediate = IntermediateNewSamplesTeamsRoleRequest;
    type Flat = SamplesTeamsRoleRequest;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSamplesTeamsRoleRequest {
            created_by: user_id,
            table_id: self.table_id,
            team_id: self.team_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
       user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::samples_teams_role_requests;
        diesel::insert_into(samples_teams_role_requests::dsl::samples_teams_role_requests)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewSamplesTeamsRole.
#[derive(Insertable)]
#[diesel(table_name = samples_teams_roles)]
pub(super) struct IntermediateNewSamplesTeamsRole {
    created_by: i32,
    table_id: Uuid,
    team_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewSamplesTeamsRole {
    type Intermediate = IntermediateNewSamplesTeamsRole;
    type Flat = SamplesTeamsRole;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSamplesTeamsRole {
            created_by: user_id,
            table_id: self.table_id,
            team_id: self.team_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
       user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::samples_teams_roles;
        diesel::insert_into(samples_teams_roles::dsl::samples_teams_roles)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewSamplesUsersRoleInvitation.
#[derive(Insertable)]
#[diesel(table_name = samples_users_role_invitations)]
pub(super) struct IntermediateNewSamplesUsersRoleInvitation {
    created_by: i32,
    table_id: Uuid,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewSamplesUsersRoleInvitation {
    type Intermediate = IntermediateNewSamplesUsersRoleInvitation;
    type Flat = SamplesUsersRoleInvitation;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSamplesUsersRoleInvitation {
            created_by: user_id,
            table_id: self.table_id,
            user_id: self.user_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
       user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::samples_users_role_invitations;
        diesel::insert_into(samples_users_role_invitations::dsl::samples_users_role_invitations)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewSamplesUsersRoleRequest.
#[derive(Insertable)]
#[diesel(table_name = samples_users_role_requests)]
pub(super) struct IntermediateNewSamplesUsersRoleRequest {
    created_by: i32,
    table_id: Uuid,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewSamplesUsersRoleRequest {
    type Intermediate = IntermediateNewSamplesUsersRoleRequest;
    type Flat = SamplesUsersRoleRequest;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSamplesUsersRoleRequest {
            created_by: user_id,
            table_id: self.table_id,
            user_id: self.user_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
       user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::samples_users_role_requests;
        diesel::insert_into(samples_users_role_requests::dsl::samples_users_role_requests)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewSamplesUsersRole.
#[derive(Insertable)]
#[diesel(table_name = samples_users_roles)]
pub(super) struct IntermediateNewSamplesUsersRole {
    created_by: i32,
    table_id: Uuid,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewSamplesUsersRole {
    type Intermediate = IntermediateNewSamplesUsersRole;
    type Flat = SamplesUsersRole;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSamplesUsersRole {
            created_by: user_id,
            table_id: self.table_id,
            user_id: self.user_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
       user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::samples_users_roles;
        diesel::insert_into(samples_users_roles::dsl::samples_users_roles)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewSpectraCollection.
#[derive(Insertable)]
#[diesel(table_name = spectra_collections)]
pub(super) struct IntermediateNewSpectraCollection {
    created_by: i32,
    notes: Option<String>,
    sample_id: Uuid,
    updated_by: i32,
}

impl InsertRow for web_common::database::NewSpectraCollection {
    type Intermediate = IntermediateNewSpectraCollection;
    type Flat = SpectraCollection;

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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::spectra_collections;
        diesel::insert_into(spectra_collections::dsl::spectra_collections)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewSpectraCollectionsTeamsRoleInvitation.
#[derive(Insertable)]
#[diesel(table_name = spectra_collections_teams_role_invitations)]
pub(super) struct IntermediateNewSpectraCollectionsTeamsRoleInvitation {
    created_by: i32,
    table_id: i32,
    team_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewSpectraCollectionsTeamsRoleInvitation {
    type Intermediate = IntermediateNewSpectraCollectionsTeamsRoleInvitation;
    type Flat = SpectraCollectionsTeamsRoleInvitation;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSpectraCollectionsTeamsRoleInvitation {
            created_by: user_id,
            table_id: self.table_id,
            team_id: self.team_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
       user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::spectra_collections_teams_role_invitations;
        diesel::insert_into(spectra_collections_teams_role_invitations::dsl::spectra_collections_teams_role_invitations)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewSpectraCollectionsTeamsRoleRequest.
#[derive(Insertable)]
#[diesel(table_name = spectra_collections_teams_role_requests)]
pub(super) struct IntermediateNewSpectraCollectionsTeamsRoleRequest {
    created_by: i32,
    table_id: i32,
    team_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewSpectraCollectionsTeamsRoleRequest {
    type Intermediate = IntermediateNewSpectraCollectionsTeamsRoleRequest;
    type Flat = SpectraCollectionsTeamsRoleRequest;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSpectraCollectionsTeamsRoleRequest {
            created_by: user_id,
            table_id: self.table_id,
            team_id: self.team_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
       user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::spectra_collections_teams_role_requests;
        diesel::insert_into(spectra_collections_teams_role_requests::dsl::spectra_collections_teams_role_requests)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewSpectraCollectionsTeamsRole.
#[derive(Insertable)]
#[diesel(table_name = spectra_collections_teams_roles)]
pub(super) struct IntermediateNewSpectraCollectionsTeamsRole {
    created_by: i32,
    table_id: i32,
    team_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewSpectraCollectionsTeamsRole {
    type Intermediate = IntermediateNewSpectraCollectionsTeamsRole;
    type Flat = SpectraCollectionsTeamsRole;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSpectraCollectionsTeamsRole {
            created_by: user_id,
            table_id: self.table_id,
            team_id: self.team_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
       user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::spectra_collections_teams_roles;
        diesel::insert_into(spectra_collections_teams_roles::dsl::spectra_collections_teams_roles)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewSpectraCollectionsUsersRoleInvitation.
#[derive(Insertable)]
#[diesel(table_name = spectra_collections_users_role_invitations)]
pub(super) struct IntermediateNewSpectraCollectionsUsersRoleInvitation {
    created_by: i32,
    table_id: i32,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewSpectraCollectionsUsersRoleInvitation {
    type Intermediate = IntermediateNewSpectraCollectionsUsersRoleInvitation;
    type Flat = SpectraCollectionsUsersRoleInvitation;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSpectraCollectionsUsersRoleInvitation {
            created_by: user_id,
            table_id: self.table_id,
            user_id: self.user_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
       user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::spectra_collections_users_role_invitations;
        diesel::insert_into(spectra_collections_users_role_invitations::dsl::spectra_collections_users_role_invitations)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewSpectraCollectionsUsersRoleRequest.
#[derive(Insertable)]
#[diesel(table_name = spectra_collections_users_role_requests)]
pub(super) struct IntermediateNewSpectraCollectionsUsersRoleRequest {
    created_by: i32,
    table_id: i32,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewSpectraCollectionsUsersRoleRequest {
    type Intermediate = IntermediateNewSpectraCollectionsUsersRoleRequest;
    type Flat = SpectraCollectionsUsersRoleRequest;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSpectraCollectionsUsersRoleRequest {
            created_by: user_id,
            table_id: self.table_id,
            user_id: self.user_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
       user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::spectra_collections_users_role_requests;
        diesel::insert_into(spectra_collections_users_role_requests::dsl::spectra_collections_users_role_requests)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewSpectraCollectionsUsersRole.
#[derive(Insertable)]
#[diesel(table_name = spectra_collections_users_roles)]
pub(super) struct IntermediateNewSpectraCollectionsUsersRole {
    created_by: i32,
    table_id: i32,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewSpectraCollectionsUsersRole {
    type Intermediate = IntermediateNewSpectraCollectionsUsersRole;
    type Flat = SpectraCollectionsUsersRole;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSpectraCollectionsUsersRole {
            created_by: user_id,
            table_id: self.table_id,
            user_id: self.user_id,
            role_id: self.role_id,
        }
    }

    fn insert(
        self,
       user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::spectra_collections_users_roles;
        diesel::insert_into(spectra_collections_users_roles::dsl::spectra_collections_users_roles)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewTeam.
#[derive(Insertable)]
#[diesel(table_name = teams)]
pub(super) struct IntermediateNewTeam {
    created_by: i32,
    name: String,
    description: String,
    icon_id: i32,
    color_id: i32,
    parent_team_id: Option<i32>,
    updated_by: i32,
}

impl InsertRow for web_common::database::NewTeam {
    type Intermediate = IntermediateNewTeam;
    type Flat = Team;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewTeam {
            created_by: user_id,
            name: self.name,
            description: self.description,
            icon_id: self.icon_id,
            color_id: self.color_id,
            parent_team_id: self.parent_team_id,
            updated_by: user_id,
        }
    }

    fn insert(
        self,
       user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::teams;
        diesel::insert_into(teams::dsl::teams)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewTeamsTeamsRoleInvitation.
#[derive(Insertable)]
#[diesel(table_name = teams_teams_role_invitations)]
pub(super) struct IntermediateNewTeamsTeamsRoleInvitation {
    created_by: i32,
    table_id: i32,
    team_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewTeamsTeamsRoleInvitation {
    type Intermediate = IntermediateNewTeamsTeamsRoleInvitation;
    type Flat = TeamsTeamsRoleInvitation;

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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::teams_teams_role_invitations;
        diesel::insert_into(teams_teams_role_invitations::dsl::teams_teams_role_invitations)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewTeamsUsersRoleInvitation.
#[derive(Insertable)]
#[diesel(table_name = teams_users_role_invitations)]
pub(super) struct IntermediateNewTeamsUsersRoleInvitation {
    created_by: i32,
    table_id: i32,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewTeamsUsersRoleInvitation {
    type Intermediate = IntermediateNewTeamsUsersRoleInvitation;
    type Flat = TeamsUsersRoleInvitation;

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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::teams_users_role_invitations;
        diesel::insert_into(teams_users_role_invitations::dsl::teams_users_role_invitations)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewTeamsUsersRoleRequest.
#[derive(Insertable)]
#[diesel(table_name = teams_users_role_requests)]
pub(super) struct IntermediateNewTeamsUsersRoleRequest {
    created_by: i32,
    table_id: i32,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewTeamsUsersRoleRequest {
    type Intermediate = IntermediateNewTeamsUsersRoleRequest;
    type Flat = TeamsUsersRoleRequest;

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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::teams_users_role_requests;
        diesel::insert_into(teams_users_role_requests::dsl::teams_users_role_requests)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewTeamsUsersRole.
#[derive(Insertable)]
#[diesel(table_name = teams_users_roles)]
pub(super) struct IntermediateNewTeamsUsersRole {
    created_by: i32,
    table_id: i32,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewTeamsUsersRole {
    type Intermediate = IntermediateNewTeamsUsersRole;
    type Flat = TeamsUsersRole;

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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::teams_users_roles;
        diesel::insert_into(teams_users_roles::dsl::teams_users_roles)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewUserEmail.
#[derive(Insertable)]
#[diesel(table_name = user_emails)]
pub(super) struct IntermediateNewUserEmail {
    created_by: i32,
    email: String,
    login_provider_id: i32,
    primary_email: bool,
}

impl InsertRow for web_common::database::NewUserEmail {
    type Intermediate = IntermediateNewUserEmail;
    type Flat = UserEmail;

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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::user_emails;
        diesel::insert_into(user_emails::dsl::user_emails)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewUser.
#[derive(Insertable)]
#[diesel(table_name = users)]
pub(super) struct IntermediateNewUser {
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    description: Option<String>,
    profile_picture: Vec<u8>,
}

impl InsertRow for web_common::database::NewUser {
    type Intermediate = IntermediateNewUser;
    type Flat = User;

    fn to_intermediate(self, _user_id: i32) -> Self::Intermediate {
        IntermediateNewUser {
            first_name: self.first_name,
            middle_name: self.middle_name,
            last_name: self.last_name,
            description: self.description,
            profile_picture: self.profile_picture,
        }
    }

    fn insert(
        self,
       user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::users;
        assert_eq!(user_id, 0);
        diesel::insert_into(users::dsl::users)
            .values(self.to_intermediate(0))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewUsersUsersRoleInvitation.
#[derive(Insertable)]
#[diesel(table_name = users_users_role_invitations)]
pub(super) struct IntermediateNewUsersUsersRoleInvitation {
    created_by: i32,
    table_id: i32,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewUsersUsersRoleInvitation {
    type Intermediate = IntermediateNewUsersUsersRoleInvitation;
    type Flat = UsersUsersRoleInvitation;

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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::users_users_role_invitations;
        diesel::insert_into(users_users_role_invitations::dsl::users_users_role_invitations)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewUsersUsersRoleRequest.
#[derive(Insertable)]
#[diesel(table_name = users_users_role_requests)]
pub(super) struct IntermediateNewUsersUsersRoleRequest {
    created_by: i32,
    table_id: i32,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewUsersUsersRoleRequest {
    type Intermediate = IntermediateNewUsersUsersRoleRequest;
    type Flat = UsersUsersRoleRequest;

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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::users_users_role_requests;
        diesel::insert_into(users_users_role_requests::dsl::users_users_role_requests)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewUsersUsersRole.
#[derive(Insertable)]
#[diesel(table_name = users_users_roles)]
pub(super) struct IntermediateNewUsersUsersRole {
    created_by: i32,
    table_id: i32,
    user_id: i32,
    role_id: i32,
}

impl InsertRow for web_common::database::NewUsersUsersRole {
    type Intermediate = IntermediateNewUsersUsersRole;
    type Flat = UsersUsersRole;

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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::users_users_roles;
        diesel::insert_into(users_users_roles::dsl::users_users_roles)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

