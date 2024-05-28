//! This module contains the update variants of the database models.
//!
//! This module is automatically generated. Do not write anything here.

use diesel::prelude::*;
use crate::models::*;
use crate::schema::*;
use diesel::r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;
use uuid::Uuid;
use chrono::NaiveDateTime;
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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error>;
}

/// Intermediate representation of the update variant UpdateDerivedSample.
#[derive(Identifiable, AsChangeset)]
#[diesel(table_name = derived_samples)]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(parent_sample_id, child_sample_id))]
pub(super) struct IntermediateUpdateDerivedSample {
    updated_by: i32,
    parent_sample_id: Uuid,
    child_sample_id: Uuid,
}

impl UpdateRow for web_common::database::UpdateDerivedSample {
    type Intermediate = IntermediateUpdateDerivedSample;
    type Flat = DerivedSample;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateUpdateDerivedSample {
            updated_by: user_id,
            parent_sample_id: self.parent_sample_id,
            child_sample_id: self.child_sample_id,
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        self.to_intermediate(user_id)
            .save_changes(connection)
    }
}

/// Intermediate representation of the update variant UpdateProject.
#[derive(Identifiable, AsChangeset)]
#[diesel(table_name = projects)]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(id))]
pub(super) struct IntermediateUpdateProject {
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
    expected_end_date: Option<NaiveDateTime>,
    end_date: Option<NaiveDateTime>,
}

impl UpdateRow for web_common::database::UpdateProject {
    type Intermediate = IntermediateUpdateProject;
    type Flat = Project;

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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        self.to_intermediate(user_id)
            .save_changes(connection)
    }
}

/// Intermediate representation of the update variant UpdateSampleContainer.
#[derive(Identifiable, AsChangeset)]
#[diesel(table_name = sample_containers)]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(id))]
pub(super) struct IntermediateUpdateSampleContainer {
    updated_by: i32,
    id: i32,
    barcode: String,
    project_id: i32,
    category_id: i32,
}

impl UpdateRow for web_common::database::UpdateSampleContainer {
    type Intermediate = IntermediateUpdateSampleContainer;
    type Flat = SampleContainer;

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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        self.to_intermediate(user_id)
            .save_changes(connection)
    }
}

/// Intermediate representation of the update variant UpdateSpectra.
#[derive(Identifiable, AsChangeset)]
#[diesel(table_name = spectra)]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(id))]
pub(super) struct IntermediateUpdateSpectra {
    updated_by: i32,
    id: i32,
    notes: Option<String>,
    spectra_collection_id: i32,
}

impl UpdateRow for web_common::database::UpdateSpectra {
    type Intermediate = IntermediateUpdateSpectra;
    type Flat = Spectra;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateUpdateSpectra {
            updated_by: user_id,
            id: self.id,
            notes: self.notes,
            spectra_collection_id: self.spectra_collection_id,
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        self.to_intermediate(user_id)
            .save_changes(connection)
    }
}

/// Intermediate representation of the update variant UpdateSpectraCollection.
#[derive(Identifiable, AsChangeset)]
#[diesel(table_name = spectra_collections)]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(id))]
pub(super) struct IntermediateUpdateSpectraCollection {
    updated_by: i32,
    id: i32,
    notes: Option<String>,
    sample_id: Uuid,
}

impl UpdateRow for web_common::database::UpdateSpectraCollection {
    type Intermediate = IntermediateUpdateSpectraCollection;
    type Flat = SpectraCollection;

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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        self.to_intermediate(user_id)
            .save_changes(connection)
    }
}

/// Intermediate representation of the update variant UpdateTeam.
#[derive(Identifiable, AsChangeset)]
#[diesel(table_name = teams)]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(id))]
pub(super) struct IntermediateUpdateTeam {
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
    type Flat = Team;

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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        self.to_intermediate(user_id)
            .save_changes(connection)
    }
}

/// Intermediate representation of the update variant UpdateUser.
#[derive(Identifiable, AsChangeset)]
#[diesel(table_name = users)]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(id))]
pub(super) struct IntermediateUpdateUser {
    id: i32,
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    description: Option<String>,
    profile_picture: Vec<u8>,
}

impl UpdateRow for web_common::database::UpdateUser {
    type Intermediate = IntermediateUpdateUser;
    type Flat = User;

    fn to_intermediate(self, _user_id: i32) -> Self::Intermediate {
        IntermediateUpdateUser {
            id: self.id,
            first_name: self.first_name,
            middle_name: self.middle_name,
            last_name: self.last_name,
            description: self.description,
            profile_picture: self.profile_picture,
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        self.to_intermediate(user_id)
            .save_changes(connection)
    }
}

/// Intermediate representation of the update variant NewObservation.
#[derive(Identifiable, AsChangeset)]
#[diesel(table_name = observations)]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(id))]
pub(super) struct IntermediateNewObservation {
    updated_by: i32,
    id: Uuid,
    project_id: i32,
    individual_id: Option<Uuid>,
    notes: Option<String>,
    picture: Vec<u8>,
}

impl UpdateRow for web_common::database::NewObservation {
    type Intermediate = IntermediateNewObservation;
    type Flat = Observation;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewObservation {
            updated_by: user_id,
            id: self.id,
            project_id: self.project_id,
            individual_id: self.individual_id,
            notes: self.notes,
            picture: self.picture,
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        self.to_intermediate(user_id)
            .save_changes(connection)
    }
}

/// Intermediate representation of the update variant NewSampledIndividual.
#[derive(Identifiable, AsChangeset)]
#[diesel(table_name = sampled_individuals)]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(id))]
pub(super) struct IntermediateNewSampledIndividual {
    updated_by: i32,
    id: Uuid,
    notes: Option<String>,
    barcode: Option<String>,
    project_id: i32,
    picture: Vec<u8>,
}

impl UpdateRow for web_common::database::NewSampledIndividual {
    type Intermediate = IntermediateNewSampledIndividual;
    type Flat = SampledIndividual;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSampledIndividual {
            updated_by: user_id,
            id: self.id,
            notes: self.notes,
            barcode: self.barcode,
            project_id: self.project_id,
            picture: self.picture,
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        self.to_intermediate(user_id)
            .save_changes(connection)
    }
}

/// Intermediate representation of the update variant NewSample.
#[derive(Identifiable, AsChangeset)]
#[diesel(table_name = samples)]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(id))]
pub(super) struct IntermediateNewSample {
    updated_by: i32,
    id: Uuid,
    container_id: i32,
    notes: Option<String>,
    project_id: i32,
    sampled_by: i32,
    state: i32,
}

impl UpdateRow for web_common::database::NewSample {
    type Intermediate = IntermediateNewSample;
    type Flat = Sample;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSample {
            updated_by: user_id,
            id: self.id,
            container_id: self.container_id,
            notes: self.notes,
            project_id: self.project_id,
            sampled_by: self.sampled_by,
            state: self.state,
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        self.to_intermediate(user_id)
            .save_changes(connection)
    }
}

