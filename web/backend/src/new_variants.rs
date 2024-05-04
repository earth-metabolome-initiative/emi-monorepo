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

/// Intermediate representation of the new variant NewContainerHorizontalRule.
#[derive(Insertable)]
#[diesel(table_name = container_horizontal_rules)]
pub(super) struct IntermediateContainerHorizontalRule {
    created_by: i32,
    name: String,
    item_type_id: i32,
    other_item_type_id: i32,
    minimum_temperature: Option<i32>,
    maximum_temperature: Option<i32>,
    minimum_humidity: Option<i32>,
    maximum_humidity: Option<i32>,
    minimum_pressure: Option<i32>,
    maximum_pressure: Option<i32>,
}

impl InsertRow for web_common::database::NewContainerHorizontalRule {
    type Intermediate = IntermediateContainerHorizontalRule;
    type Flat = ContainerHorizontalRule;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateContainerHorizontalRule {
            created_by: user_id,
            name: self.name,
            item_type_id: self.item_type_id,
            other_item_type_id: self.other_item_type_id,
            minimum_temperature: self.minimum_temperature,
            maximum_temperature: self.maximum_temperature,
            minimum_humidity: self.minimum_humidity,
            maximum_humidity: self.maximum_humidity,
            minimum_pressure: self.minimum_pressure,
            maximum_pressure: self.maximum_pressure,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::container_horizontal_rules;
        diesel::insert_into(container_horizontal_rules::dsl::container_horizontal_rules)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewContainerVerticalRule.
#[derive(Insertable)]
#[diesel(table_name = container_vertical_rules)]
pub(super) struct IntermediateContainerVerticalRule {
    created_by: i32,
    name: String,
    container_item_type_id: i32,
    contained_item_type_id: i32,
    minimum_temperature: Option<i32>,
    maximum_temperature: Option<i32>,
    minimum_humidity: Option<i32>,
    maximum_humidity: Option<i32>,
    minimum_pressure: Option<i32>,
    maximum_pressure: Option<i32>,
}

impl InsertRow for web_common::database::NewContainerVerticalRule {
    type Intermediate = IntermediateContainerVerticalRule;
    type Flat = ContainerVerticalRule;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateContainerVerticalRule {
            created_by: user_id,
            name: self.name,
            container_item_type_id: self.container_item_type_id,
            contained_item_type_id: self.contained_item_type_id,
            minimum_temperature: self.minimum_temperature,
            maximum_temperature: self.maximum_temperature,
            minimum_humidity: self.minimum_humidity,
            maximum_humidity: self.maximum_humidity,
            minimum_pressure: self.minimum_pressure,
            maximum_pressure: self.maximum_pressure,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::container_vertical_rules;
        diesel::insert_into(container_vertical_rules::dsl::container_vertical_rules)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewItemCategory.
#[derive(Insertable)]
#[diesel(table_name = item_categories)]
pub(super) struct IntermediateItemCategory {
    created_by: i32,
    name: String,
    description: String,
}

impl InsertRow for web_common::database::NewItemCategory {
    type Intermediate = IntermediateItemCategory;
    type Flat = ItemCategory;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateItemCategory {
            created_by: user_id,
            name: self.name,
            description: self.description,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::item_categories;
        diesel::insert_into(item_categories::dsl::item_categories)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewProcedure.
#[derive(Insertable)]
#[diesel(table_name = procedures)]
pub(super) struct IntermediateProcedure {
    created_by: i32,
    name: String,
    description: Option<String>,
}

impl InsertRow for web_common::database::NewProcedure {
    type Intermediate = IntermediateProcedure;
    type Flat = Procedure;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateProcedure {
            created_by: user_id,
            name: self.name,
            description: self.description,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::procedures;
        diesel::insert_into(procedures::dsl::procedures)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewProjectRequirement.
#[derive(Insertable)]
#[diesel(table_name = project_requirements)]
pub(super) struct IntermediateProjectRequirement {
    created_by: i32,
    project_id: i32,
    item_category_id: i32,
    quantity: i32,
    unit_id: Option<i32>,
}

impl InsertRow for web_common::database::NewProjectRequirement {
    type Intermediate = IntermediateProjectRequirement;
    type Flat = ProjectRequirement;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateProjectRequirement {
            created_by: user_id,
            project_id: self.project_id,
            item_category_id: self.item_category_id,
            quantity: self.quantity,
            unit_id: self.unit_id,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::project_requirements;
        diesel::insert_into(project_requirements::dsl::project_requirements)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewProject.
#[derive(Insertable)]
#[diesel(table_name = projects)]
pub(super) struct IntermediateProject {
    created_by: i32,
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

impl InsertRow for web_common::database::NewProject {
    type Intermediate = IntermediateProject;
    type Flat = Project;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateProject {
            created_by: user_id,
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

/// Intermediate representation of the new variant NewSample.
#[derive(Insertable)]
#[diesel(table_name = samples)]
pub(super) struct IntermediateSample {
    inserted_by: i32,
    id: Uuid,
    sampled_by: i32,
    procedure_id: Uuid,
    state: i32,
}

impl InsertRow for web_common::database::NewSample {
    type Intermediate = IntermediateSample;
    type Flat = Sample;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateSample {
            inserted_by: user_id,
            id: self.id,
            sampled_by: self.sampled_by,
            procedure_id: self.procedure_id,
            state: self.state,
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

/// Intermediate representation of the new variant NewSamplingProcedure.
#[derive(Insertable)]
#[diesel(table_name = sampling_procedures)]
pub(super) struct IntermediateSamplingProcedure {
    created_by: i32,
    id: Uuid,
    name: String,
    description: Option<String>,
}

impl InsertRow for web_common::database::NewSamplingProcedure {
    type Intermediate = IntermediateSamplingProcedure;
    type Flat = SamplingProcedure;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateSamplingProcedure {
            created_by: user_id,
            id: self.id,
            name: self.name,
            description: self.description,
        }
    }

    fn insert(
        self,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::sampling_procedures;
        diesel::insert_into(sampling_procedures::dsl::sampling_procedures)
            .values(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the new variant NewTeam.
#[derive(Insertable)]
#[diesel(table_name = teams)]
pub(super) struct IntermediateTeam {
    created_by: i32,
    name: String,
    description: String,
    parent_team_id: Option<i32>,
}

impl InsertRow for web_common::database::NewTeam {
    type Intermediate = IntermediateTeam;
    type Flat = Team;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateTeam {
            created_by: user_id,
            name: self.name,
            description: self.description,
            parent_team_id: self.parent_team_id,
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

