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

/// Intermediate representation of the update variant UpdateProject.
#[derive(AsChangeset)]
#[diesel(table_name = projects)]
pub(super) struct IntermediateUpdateProject {
    updated_by: i32,
    name: String,
    description: String,
    public: bool,
    state_id: i32,
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
            name: self.name,
            description: self.description,
            public: self.public,
            state_id: self.state_id,
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
        use crate::schema::projects;
        diesel::update(projects::dsl::projects)
            .filter(
                projects::dsl::id.eq(self.id)
            )
            .set(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the update variant UpdateTeam.
#[derive(AsChangeset)]
#[diesel(table_name = teams)]
pub(super) struct IntermediateUpdateTeam {
    updated_by: i32,
    name: String,
    description: String,
    parent_team_id: Option<i32>,
}

impl UpdateRow for web_common::database::UpdateTeam {
    type Intermediate = IntermediateUpdateTeam;
    type Flat = Team;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateUpdateTeam {
            updated_by: user_id,
            name: self.name,
            description: self.description,
            parent_team_id: self.parent_team_id,
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::teams;
        diesel::update(teams::dsl::teams)
            .filter(
                teams::dsl::id.eq(self.id)
            )
            .set(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the update variant UpdateUser.
#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub(super) struct IntermediateUpdateUser {
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    profile_picture: Vec<u8>,
}

impl UpdateRow for web_common::database::UpdateUser {
    type Intermediate = IntermediateUpdateUser;
    type Flat = User;

    fn to_intermediate(self, _user_id: i32) -> Self::Intermediate {
        IntermediateUpdateUser {
            first_name: self.first_name,
            middle_name: self.middle_name,
            last_name: self.last_name,
            profile_picture: self.profile_picture,
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::users;
        diesel::update(users::dsl::users)
            .filter(
                users::dsl::id.eq(self.id)
            )
            .set(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the update variant NewSampledIndividual.
#[derive(AsChangeset)]
#[diesel(table_name = sampled_individuals)]
pub(super) struct IntermediateNewSampledIndividual {
    updated_by: i32,
    tagged: bool,
}

impl UpdateRow for web_common::database::NewSampledIndividual {
    type Intermediate = IntermediateNewSampledIndividual;
    type Flat = SampledIndividual;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSampledIndividual {
            updated_by: user_id,
            tagged: self.tagged,
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::sampled_individuals;
        diesel::update(sampled_individuals::dsl::sampled_individuals)
            .filter(
                sampled_individuals::dsl::id.eq(self.id)
            )
            .set(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the update variant NewSample.
#[derive(AsChangeset)]
#[diesel(table_name = samples)]
pub(super) struct IntermediateNewSample {
    updated_by: i32,
    sampled_by: i32,
    state: i32,
}

impl UpdateRow for web_common::database::NewSample {
    type Intermediate = IntermediateNewSample;
    type Flat = Sample;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSample {
            updated_by: user_id,
            sampled_by: self.sampled_by,
            state: self.state,
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::samples;
        diesel::update(samples::dsl::samples)
            .filter(
                samples::dsl::id.eq(self.id)
            )
            .set(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

