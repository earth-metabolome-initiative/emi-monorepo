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

/// Intermediate representation of the new variant NewSampledIndividual.
#[derive(Insertable)]
#[diesel(table_name = sampled_individuals)]
pub(super) struct IntermediateNewSampledIndividual {
    created_by: i32,
    id: Uuid,
    tagged: bool,
    updated_by: i32,
}

impl InsertRow for web_common::database::NewSampledIndividual {
    type Intermediate = IntermediateNewSampledIndividual;
    type Flat = SampledIndividual;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSampledIndividual {
            created_by: user_id,
            id: self.id,
            tagged: self.tagged,
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

