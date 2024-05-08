//! This module contains the update variants of the database models.
//!
//! Some of the update variants would be identical to the new variants, //! and as such we do not generate them. You will find here the update variants //! only for the tables that have a primary key that is not a UUID.
//! This module is automatically generated. Do not write anything here.

use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct UpdateContainerHorizontalRule {
    pub id: i32,
    pub name: String,
    pub item_type_id: i32,
    pub other_item_type_id: i32,
    pub minimum_temperature: Option<i32>,
    pub maximum_temperature: Option<i32>,
    pub minimum_humidity: Option<i32>,
    pub maximum_humidity: Option<i32>,
    pub minimum_pressure: Option<i32>,
    pub maximum_pressure: Option<i32>,
}

#[cfg(feature = "frontend")]
impl UpdateContainerHorizontalRule {
    pub fn into_row(self, updated_by: i32) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(updated_by),
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::num(self.item_type_id),
            gluesql::core::ast_builder::num(self.other_item_type_id),
            match self.minimum_temperature {
                Some(minimum_temperature) => gluesql::core::ast_builder::num(minimum_temperature),
                None => gluesql::core::ast_builder::null(),
            },
            match self.maximum_temperature {
                Some(maximum_temperature) => gluesql::core::ast_builder::num(maximum_temperature),
                None => gluesql::core::ast_builder::null(),
            },
            match self.minimum_humidity {
                Some(minimum_humidity) => gluesql::core::ast_builder::num(minimum_humidity),
                None => gluesql::core::ast_builder::null(),
            },
            match self.maximum_humidity {
                Some(maximum_humidity) => gluesql::core::ast_builder::num(maximum_humidity),
                None => gluesql::core::ast_builder::null(),
            },
            match self.minimum_pressure {
                Some(minimum_pressure) => gluesql::core::ast_builder::num(minimum_pressure),
                None => gluesql::core::ast_builder::null(),
            },
            match self.maximum_pressure {
                Some(maximum_pressure) => gluesql::core::ast_builder::num(maximum_pressure),
                None => gluesql::core::ast_builder::null(),
            },
        ]
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
        let mut update_row = table("container_horizontal_rules")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("item_type_id", gluesql::core::ast_builder::num(self.item_type_id))        
.set("other_item_type_id", gluesql::core::ast_builder::num(self.other_item_type_id))        
.set("updated_by", gluesql::core::ast_builder::num(user_id));
        if let Some(minimum_temperature) = self.minimum_temperature {
            update_row = update_row.set("minimum_temperature", gluesql::core::ast_builder::num(minimum_temperature));
        }
        if let Some(maximum_temperature) = self.maximum_temperature {
            update_row = update_row.set("maximum_temperature", gluesql::core::ast_builder::num(maximum_temperature));
        }
        if let Some(minimum_humidity) = self.minimum_humidity {
            update_row = update_row.set("minimum_humidity", gluesql::core::ast_builder::num(minimum_humidity));
        }
        if let Some(maximum_humidity) = self.maximum_humidity {
            update_row = update_row.set("maximum_humidity", gluesql::core::ast_builder::num(maximum_humidity));
        }
        if let Some(minimum_pressure) = self.minimum_pressure {
            update_row = update_row.set("minimum_pressure", gluesql::core::ast_builder::num(minimum_pressure));
        }
        if let Some(maximum_pressure) = self.maximum_pressure {
            update_row = update_row.set("maximum_pressure", gluesql::core::ast_builder::num(maximum_pressure));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct UpdateContainerVerticalRule {
    pub id: i32,
    pub name: String,
    pub container_item_type_id: i32,
    pub contained_item_type_id: i32,
    pub minimum_temperature: Option<i32>,
    pub maximum_temperature: Option<i32>,
    pub minimum_humidity: Option<i32>,
    pub maximum_humidity: Option<i32>,
    pub minimum_pressure: Option<i32>,
    pub maximum_pressure: Option<i32>,
}

#[cfg(feature = "frontend")]
impl UpdateContainerVerticalRule {
    pub fn into_row(self, updated_by: i32) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(updated_by),
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::num(self.container_item_type_id),
            gluesql::core::ast_builder::num(self.contained_item_type_id),
            match self.minimum_temperature {
                Some(minimum_temperature) => gluesql::core::ast_builder::num(minimum_temperature),
                None => gluesql::core::ast_builder::null(),
            },
            match self.maximum_temperature {
                Some(maximum_temperature) => gluesql::core::ast_builder::num(maximum_temperature),
                None => gluesql::core::ast_builder::null(),
            },
            match self.minimum_humidity {
                Some(minimum_humidity) => gluesql::core::ast_builder::num(minimum_humidity),
                None => gluesql::core::ast_builder::null(),
            },
            match self.maximum_humidity {
                Some(maximum_humidity) => gluesql::core::ast_builder::num(maximum_humidity),
                None => gluesql::core::ast_builder::null(),
            },
            match self.minimum_pressure {
                Some(minimum_pressure) => gluesql::core::ast_builder::num(minimum_pressure),
                None => gluesql::core::ast_builder::null(),
            },
            match self.maximum_pressure {
                Some(maximum_pressure) => gluesql::core::ast_builder::num(maximum_pressure),
                None => gluesql::core::ast_builder::null(),
            },
        ]
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
        let mut update_row = table("container_vertical_rules")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("container_item_type_id", gluesql::core::ast_builder::num(self.container_item_type_id))        
.set("contained_item_type_id", gluesql::core::ast_builder::num(self.contained_item_type_id))        
.set("updated_by", gluesql::core::ast_builder::num(user_id));
        if let Some(minimum_temperature) = self.minimum_temperature {
            update_row = update_row.set("minimum_temperature", gluesql::core::ast_builder::num(minimum_temperature));
        }
        if let Some(maximum_temperature) = self.maximum_temperature {
            update_row = update_row.set("maximum_temperature", gluesql::core::ast_builder::num(maximum_temperature));
        }
        if let Some(minimum_humidity) = self.minimum_humidity {
            update_row = update_row.set("minimum_humidity", gluesql::core::ast_builder::num(minimum_humidity));
        }
        if let Some(maximum_humidity) = self.maximum_humidity {
            update_row = update_row.set("maximum_humidity", gluesql::core::ast_builder::num(maximum_humidity));
        }
        if let Some(minimum_pressure) = self.minimum_pressure {
            update_row = update_row.set("minimum_pressure", gluesql::core::ast_builder::num(minimum_pressure));
        }
        if let Some(maximum_pressure) = self.maximum_pressure {
            update_row = update_row.set("maximum_pressure", gluesql::core::ast_builder::num(maximum_pressure));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct UpdateItemCategory {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[cfg(feature = "frontend")]
impl UpdateItemCategory {
    pub fn into_row(self, updated_by: i32) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(updated_by),
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
        ]
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
        table("item_categories")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("description", gluesql::core::ast_builder::text(self.description))        
.set("updated_by", gluesql::core::ast_builder::num(user_id))            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct UpdateProcedure {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[cfg(feature = "frontend")]
impl UpdateProcedure {
    pub fn into_row(self, updated_by: i32) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(updated_by),
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            match self.description {
                Some(description) => gluesql::core::ast_builder::text(description),
                None => gluesql::core::ast_builder::null(),
            },
        ]
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
        let mut update_row = table("procedures")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("updated_by", gluesql::core::ast_builder::num(user_id));
        if let Some(description) = self.description {
            update_row = update_row.set("description", gluesql::core::ast_builder::text(description));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct UpdateProjectRequirement {
    pub id: i32,
    pub project_id: i32,
    pub item_category_id: i32,
    pub quantity: i32,
    pub unit_id: Option<i32>,
}

#[cfg(feature = "frontend")]
impl UpdateProjectRequirement {
    pub fn into_row(self, updated_by: i32) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(updated_by),
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::num(self.project_id),
            gluesql::core::ast_builder::num(self.item_category_id),
            gluesql::core::ast_builder::num(self.quantity),
            match self.unit_id {
                Some(unit_id) => gluesql::core::ast_builder::num(unit_id),
                None => gluesql::core::ast_builder::null(),
            },
        ]
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
        let mut update_row = table("project_requirements")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("project_id", gluesql::core::ast_builder::num(self.project_id))        
.set("item_category_id", gluesql::core::ast_builder::num(self.item_category_id))        
.set("quantity", gluesql::core::ast_builder::num(self.quantity))        
.set("updated_by", gluesql::core::ast_builder::num(user_id));
        if let Some(unit_id) = self.unit_id {
            update_row = update_row.set("unit_id", gluesql::core::ast_builder::num(unit_id));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct UpdateProject {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub public: bool,
    pub state_id: i32,
    pub parent_project_id: Option<i32>,
    pub budget: Option<f64>,
    pub expenses: Option<f64>,
    pub expected_end_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
}

#[cfg(feature = "frontend")]
impl UpdateProject {
    pub fn into_row(self, updated_by: i32) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(updated_by),
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            (self.public.into()),
            gluesql::core::ast_builder::num(self.state_id),
            match self.parent_project_id {
                Some(parent_project_id) => gluesql::core::ast_builder::num(parent_project_id),
                None => gluesql::core::ast_builder::null(),
            },
            match self.budget {
                Some(budget) => gluesql::core::ast_builder::num(budget),
                None => gluesql::core::ast_builder::null(),
            },
            match self.expenses {
                Some(expenses) => gluesql::core::ast_builder::num(expenses),
                None => gluesql::core::ast_builder::null(),
            },
            match self.expected_end_date {
                Some(expected_end_date) => gluesql::core::ast_builder::timestamp(expected_end_date.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
            match self.end_date {
                Some(end_date) => gluesql::core::ast_builder::timestamp(end_date.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
        ]
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
        let mut update_row = table("projects")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("description", gluesql::core::ast_builder::text(self.description))        
.set("public", self.public)        
.set("state_id", gluesql::core::ast_builder::num(self.state_id))        
.set("updated_by", gluesql::core::ast_builder::num(user_id));
        if let Some(parent_project_id) = self.parent_project_id {
            update_row = update_row.set("parent_project_id", gluesql::core::ast_builder::num(parent_project_id));
        }
        if let Some(budget) = self.budget {
            update_row = update_row.set("budget", gluesql::core::ast_builder::num(budget));
        }
        if let Some(expenses) = self.expenses {
            update_row = update_row.set("expenses", gluesql::core::ast_builder::num(expenses));
        }
        if let Some(expected_end_date) = self.expected_end_date {
            update_row = update_row.set("expected_end_date", gluesql::core::ast_builder::timestamp(expected_end_date.to_string()));
        }
        if let Some(end_date) = self.end_date {
            update_row = update_row.set("end_date", gluesql::core::ast_builder::timestamp(end_date.to_string()));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

}
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct UpdateTeam {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub parent_team_id: Option<i32>,
}

#[cfg(feature = "frontend")]
impl UpdateTeam {
    pub fn into_row(self, updated_by: i32) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(updated_by),
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            match self.parent_team_id {
                Some(parent_team_id) => gluesql::core::ast_builder::num(parent_team_id),
                None => gluesql::core::ast_builder::null(),
            },
        ]
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
        let mut update_row = table("teams")
            .update()        
.set("id", gluesql::core::ast_builder::num(self.id))        
.set("name", gluesql::core::ast_builder::text(self.name))        
.set("description", gluesql::core::ast_builder::text(self.description))        
.set("updated_by", gluesql::core::ast_builder::num(user_id));
        if let Some(parent_team_id) = self.parent_team_id {
            update_row = update_row.set("parent_team_id", gluesql::core::ast_builder::num(parent_team_id));
        }
            update_row.execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                 _ => unreachable!("Expected Payload::Update")
})
    }

}
