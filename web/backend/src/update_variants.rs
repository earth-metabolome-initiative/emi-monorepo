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

/// Intermediate representation of the update variant UpdateContainerHorizontalRule.
#[derive(AsChangeset)]
#[diesel(table_name = container_horizontal_rules)]
pub(super) struct IntermediateUpdateContainerHorizontalRule {
    updated_by: i32,
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

impl UpdateRow for web_common::database::UpdateContainerHorizontalRule {
    type Intermediate = IntermediateUpdateContainerHorizontalRule;
    type Flat = ContainerHorizontalRule;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateUpdateContainerHorizontalRule {
            updated_by: user_id,
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

    fn update(
        self,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::container_horizontal_rules;
        diesel::update(container_horizontal_rules::dsl::container_horizontal_rules)
            .filter(
                container_horizontal_rules::dsl::id.eq(self.id)
            )
            .set(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the update variant UpdateContainerVerticalRule.
#[derive(AsChangeset)]
#[diesel(table_name = container_vertical_rules)]
pub(super) struct IntermediateUpdateContainerVerticalRule {
    updated_by: i32,
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

impl UpdateRow for web_common::database::UpdateContainerVerticalRule {
    type Intermediate = IntermediateUpdateContainerVerticalRule;
    type Flat = ContainerVerticalRule;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateUpdateContainerVerticalRule {
            updated_by: user_id,
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

    fn update(
        self,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::container_vertical_rules;
        diesel::update(container_vertical_rules::dsl::container_vertical_rules)
            .filter(
                container_vertical_rules::dsl::id.eq(self.id)
            )
            .set(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the update variant UpdateItemCategory.
#[derive(AsChangeset)]
#[diesel(table_name = item_categories)]
pub(super) struct IntermediateUpdateItemCategory {
    updated_by: i32,
    name: String,
    description: String,
}

impl UpdateRow for web_common::database::UpdateItemCategory {
    type Intermediate = IntermediateUpdateItemCategory;
    type Flat = ItemCategory;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateUpdateItemCategory {
            updated_by: user_id,
            name: self.name,
            description: self.description,
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::item_categories;
        diesel::update(item_categories::dsl::item_categories)
            .filter(
                item_categories::dsl::id.eq(self.id)
            )
            .set(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the update variant UpdateProcedure.
#[derive(AsChangeset)]
#[diesel(table_name = procedures)]
pub(super) struct IntermediateUpdateProcedure {
    updated_by: i32,
    name: String,
    description: Option<String>,
}

impl UpdateRow for web_common::database::UpdateProcedure {
    type Intermediate = IntermediateUpdateProcedure;
    type Flat = Procedure;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateUpdateProcedure {
            updated_by: user_id,
            name: self.name,
            description: self.description,
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::procedures;
        diesel::update(procedures::dsl::procedures)
            .filter(
                procedures::dsl::id.eq(self.id)
            )
            .set(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

/// Intermediate representation of the update variant UpdateProjectRequirement.
#[derive(AsChangeset)]
#[diesel(table_name = project_requirements)]
pub(super) struct IntermediateUpdateProjectRequirement {
    updated_by: i32,
    project_id: i32,
    item_category_id: i32,
    quantity: i32,
    unit_id: Option<i32>,
}

impl UpdateRow for web_common::database::UpdateProjectRequirement {
    type Intermediate = IntermediateUpdateProjectRequirement;
    type Flat = ProjectRequirement;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateUpdateProjectRequirement {
            updated_by: user_id,
            project_id: self.project_id,
            item_category_id: self.item_category_id,
            quantity: self.quantity,
            unit_id: self.unit_id,
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::project_requirements;
        diesel::update(project_requirements::dsl::project_requirements)
            .filter(
                project_requirements::dsl::id.eq(self.id)
            )
            .set(self.to_intermediate(user_id))
            .get_result(connection)
    }
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

/// Intermediate representation of the update variant NewSampledIndividual.
#[derive(AsChangeset)]
#[diesel(table_name = sampled_individuals)]
pub(super) struct IntermediateNewSampledIndividual {
    updated_by: i32,
    name: Option<String>,
    tagged: bool,
}

impl UpdateRow for web_common::database::NewSampledIndividual {
    type Intermediate = IntermediateNewSampledIndividual;
    type Flat = SampledIndividual;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSampledIndividual {
            updated_by: user_id,
            name: self.name,
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
    procedure_id: Uuid,
    state: i32,
}

impl UpdateRow for web_common::database::NewSample {
    type Intermediate = IntermediateNewSample;
    type Flat = Sample;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSample {
            updated_by: user_id,
            sampled_by: self.sampled_by,
            procedure_id: self.procedure_id,
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

/// Intermediate representation of the update variant NewSamplingProcedure.
#[derive(AsChangeset)]
#[diesel(table_name = sampling_procedures)]
pub(super) struct IntermediateNewSamplingProcedure {
    updated_by: i32,
    name: String,
    description: Option<String>,
}

impl UpdateRow for web_common::database::NewSamplingProcedure {
    type Intermediate = IntermediateNewSamplingProcedure;
    type Flat = SamplingProcedure;

    fn to_intermediate(self, user_id: i32) -> Self::Intermediate {
        IntermediateNewSamplingProcedure {
            updated_by: user_id,
            name: self.name,
            description: self.description,
        }
    }

    fn update(
        self,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self::Flat, diesel::result::Error> {
        use crate::schema::sampling_procedures;
        diesel::update(sampling_procedures::dsl::sampling_procedures)
            .filter(
                sampling_procedures::dsl::id.eq(self.id)
            )
            .set(self.to_intermediate(user_id))
            .get_result(connection)
    }
}

