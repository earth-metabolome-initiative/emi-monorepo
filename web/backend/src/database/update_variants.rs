//! This module contains the update variants of the database models.
//!
//! This module is automatically generated. Do not write anything here.

use crate::database::*;
use diesel::prelude::*;
/// Trait providing the update method for the update variants.
pub(super) trait UpdateRow {
    /// The intermediate representation of the row.
    type Intermediate;

    /// The flat variant of the update variant.
    type Flat;

    /// Convert the update variant into the intermediate representation.
    fn to_intermediate(self, user_id: i32) -> Self::Intermediate;

    /// Update the row in the database.
    fn update(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::prelude::PgConnection>,
        >,
    ) -> Result<Self::Flat, diesel::result::Error>;
}

/// Intermediate representation of the update variant UpdateDerivedSample.
#[derive(Identifiable, AsChangeset)]
#[diesel(table_name = crate::database::schema::derived_samples)]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(parent_sample_id, child_sample_id))]
pub(crate) struct IntermediateUpdateDerivedSample {
    updated_by: i32,
    parent_sample_id: uuid::Uuid,
    child_sample_id: uuid::Uuid,
    quantity: f64,
    unit_id: i32,
}

impl UpdateRow for web_common::database::UpdateDerivedSample {
    type Intermediate = IntermediateUpdateDerivedSample;
    type Flat = crate::database::flat_variants::DerivedSample;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateUpdateDerivedSample {
            updated_by: user_id,
            parent_sample_id: self.parent_sample_id,
            child_sample_id: self.child_sample_id,
            quantity: self.quantity,
            unit_id: self.unit_id,
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, diesel::result::Error> {
        UpdateRow::to_intermediate(self, user_id).save_changes(connection)
    }
}

/// Intermediate representation of the update variant UpdateNameplate.
#[derive(Identifiable, AsChangeset)]
#[diesel(table_name = crate::database::schema::nameplates)]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(id))]
pub(crate) struct IntermediateUpdateNameplate {
    updated_by: i32,
    id: i32,
    barcode: String,
    project_id: i32,
    category_id: i32,
    geolocation: postgis_diesel::types::Point,
}

impl UpdateRow for web_common::database::UpdateNameplate {
    type Intermediate = IntermediateUpdateNameplate;
    type Flat = crate::database::flat_variants::Nameplate;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateUpdateNameplate {
            updated_by: user_id,
            id: self.id,
            barcode: self.barcode,
            project_id: self.project_id,
            category_id: self.category_id,
            geolocation: self.geolocation.convert(),
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, diesel::result::Error> {
        UpdateRow::to_intermediate(self, user_id).save_changes(connection)
    }
}

/// Intermediate representation of the update variant UpdateProject.
#[derive(Identifiable, AsChangeset)]
#[diesel(table_name = crate::database::schema::projects)]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(id))]
pub(crate) struct IntermediateUpdateProject {
    updated_by: i32,
    id: i32,
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
}

impl UpdateRow for web_common::database::UpdateProject {
    type Intermediate = IntermediateUpdateProject;
    type Flat = crate::database::flat_variants::Project;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateUpdateProject {
            updated_by: user_id,
            id: self.id,
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
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, diesel::result::Error> {
        UpdateRow::to_intermediate(self, user_id).save_changes(connection)
    }
}

/// Intermediate representation of the update variant UpdateSampleContainer.
#[derive(Identifiable, AsChangeset)]
#[diesel(table_name = crate::database::schema::sample_containers)]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(id))]
pub(crate) struct IntermediateUpdateSampleContainer {
    updated_by: i32,
    id: i32,
    barcode: String,
    project_id: i32,
    category_id: i32,
}

impl UpdateRow for web_common::database::UpdateSampleContainer {
    type Intermediate = IntermediateUpdateSampleContainer;
    type Flat = crate::database::flat_variants::SampleContainer;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateUpdateSampleContainer {
            updated_by: user_id,
            id: self.id,
            barcode: self.barcode,
            project_id: self.project_id,
            category_id: self.category_id,
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, diesel::result::Error> {
        UpdateRow::to_intermediate(self, user_id).save_changes(connection)
    }
}

/// Intermediate representation of the update variant UpdateSpectraCollection.
#[derive(Identifiable, AsChangeset)]
#[diesel(table_name = crate::database::schema::spectra_collections)]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(id))]
pub(crate) struct IntermediateUpdateSpectraCollection {
    updated_by: i32,
    id: i32,
    notes: Option<String>,
    sample_id: uuid::Uuid,
}

impl UpdateRow for web_common::database::UpdateSpectraCollection {
    type Intermediate = IntermediateUpdateSpectraCollection;
    type Flat = crate::database::flat_variants::SpectraCollection;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateUpdateSpectraCollection {
            updated_by: user_id,
            id: self.id,
            notes: self.notes,
            sample_id: self.sample_id,
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, diesel::result::Error> {
        UpdateRow::to_intermediate(self, user_id).save_changes(connection)
    }
}

/// Intermediate representation of the update variant UpdateTeam.
#[derive(Identifiable, AsChangeset)]
#[diesel(table_name = crate::database::schema::teams)]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(id))]
pub(crate) struct IntermediateUpdateTeam {
    updated_by: i32,
    id: i32,
    name: String,
    description: String,
    icon_id: i32,
    color_id: i32,
    state_id: i32,
    parent_team_id: Option<i32>,
}

impl UpdateRow for web_common::database::UpdateTeam {
    type Intermediate = IntermediateUpdateTeam;
    type Flat = crate::database::flat_variants::Team;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateUpdateTeam {
            updated_by: user_id,
            id: self.id,
            name: self.name,
            description: self.description,
            icon_id: self.icon_id,
            color_id: self.color_id,
            state_id: self.state_id,
            parent_team_id: self.parent_team_id,
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, diesel::result::Error> {
        UpdateRow::to_intermediate(self, user_id).save_changes(connection)
    }
}

/// Intermediate representation of the update variant UpdateUser.
#[derive(Identifiable, AsChangeset)]
#[diesel(table_name = crate::database::schema::users)]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(id))]
pub(crate) struct IntermediateUpdateUser {
    id: i32,
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    description: Option<String>,
    organization_id: Option<i32>,
    picture: crate::database::diesel_types::JPEG,
}

impl UpdateRow for web_common::database::UpdateUser {
    type Intermediate = IntermediateUpdateUser;
    type Flat = crate::database::flat_variants::User;

    fn to_intermediate(self, _user_id: i32) -> Self::Intermediate {
        IntermediateUpdateUser {
            id: self.id,
            first_name: self.first_name,
            middle_name: self.middle_name,
            last_name: self.last_name,
            description: self.description,
            organization_id: self.organization_id,
            picture: self.picture.convert(),
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, diesel::result::Error> {
        UpdateRow::to_intermediate(self, user_id).save_changes(connection)
    }
}

/// Intermediate representation of the update variant NewObservation.
#[derive(Identifiable, AsChangeset)]
#[diesel(table_name = crate::database::schema::observations)]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(id))]
pub(crate) struct IntermediateNewObservation {
    updated_by: i32,
    id: uuid::Uuid,
    parent_observation_id: Option<uuid::Uuid>,
    project_id: i32,
    organism_id: Option<uuid::Uuid>,
    sample_id: Option<uuid::Uuid>,
    subject_id: i32,
    notes: Option<String>,
    picture: crate::database::diesel_types::JPEG,
}

impl UpdateRow for web_common::database::NewObservation {
    type Intermediate = IntermediateNewObservation;
    type Flat = crate::database::flat_variants::Observation;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewObservation {
            updated_by: user_id,
            id: self.id,
            parent_observation_id: self.parent_observation_id,
            project_id: self.project_id,
            organism_id: self.organism_id,
            sample_id: self.sample_id,
            subject_id: self.subject_id,
            notes: self.notes,
            picture: self.picture.convert(),
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, diesel::result::Error> {
        UpdateRow::to_intermediate(self, user_id).save_changes(connection)
    }
}

/// Intermediate representation of the update variant NewOrganism.
#[derive(Identifiable, AsChangeset)]
#[diesel(table_name = crate::database::schema::organisms)]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(id))]
pub(crate) struct IntermediateNewOrganism {
    updated_by: i32,
    id: uuid::Uuid,
    host_organism_id: Option<uuid::Uuid>,
    sample_id: Option<uuid::Uuid>,
    notes: Option<String>,
    wild: bool,
    nameplate_id: i32,
    project_id: i32,
}

impl UpdateRow for web_common::database::NewOrganism {
    type Intermediate = IntermediateNewOrganism;
    type Flat = crate::database::flat_variants::Organism;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewOrganism {
            updated_by: user_id,
            id: self.id,
            host_organism_id: self.host_organism_id,
            sample_id: self.sample_id,
            notes: self.notes,
            wild: self.wild,
            nameplate_id: self.nameplate_id,
            project_id: self.project_id,
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, diesel::result::Error> {
        UpdateRow::to_intermediate(self, user_id).save_changes(connection)
    }
}

/// Intermediate representation of the update variant NewSample.
#[derive(Identifiable, AsChangeset)]
#[diesel(table_name = crate::database::schema::samples)]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(id))]
pub(crate) struct IntermediateNewSample {
    updated_by: i32,
    id: uuid::Uuid,
    container_id: i32,
    notes: Option<String>,
    project_id: i32,
    sampled_by: i32,
    state_id: i32,
}

impl UpdateRow for web_common::database::NewSample {
    type Intermediate = IntermediateNewSample;
    type Flat = crate::database::flat_variants::Sample;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSample {
            updated_by: user_id,
            id: self.id,
            container_id: self.container_id,
            notes: self.notes,
            project_id: self.project_id,
            sampled_by: self.sampled_by,
            state_id: self.state_id,
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self::Flat, diesel::result::Error> {
        UpdateRow::to_intermediate(self, user_id).save_changes(connection)
    }
}