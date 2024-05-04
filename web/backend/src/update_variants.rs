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
    id: i32,
    name: String,
    description: String,
    public: bool,
    state_id: i32,
    parent_project_id: Option<i32>,
    budget: Option<i64>,
    expenses: Option<i64>,
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
            .set(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

