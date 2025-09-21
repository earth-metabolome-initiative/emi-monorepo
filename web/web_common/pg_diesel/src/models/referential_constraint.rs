use diesel::{Queryable, QueryableByName, Selectable};

#[derive(Queryable, QueryableByName, Selectable, PartialEq, Eq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::referential_constraints)]
/// A struct representing a referential constraint
pub struct ReferentialConstraint {
    /// The name of the constraint catalog
    pub constraint_catalog: String,
    /// The name of the constraint schema
    pub constraint_schema: String,
    /// The name of the constraint
    pub constraint_name: String,
    /// The name of the table catalog the constraint is associated with
    pub unique_constraint_catalog: Option<String>,
    /// The name of the table schema the constraint is associated with
    pub unique_constraint_schema: Option<String>,
    /// The name of the table the constraint is associated with
    pub unique_constraint_name: Option<String>,
    /// Match options
    pub match_option: String,
    /// Update rule
    pub update_rule: String,
    /// Delete rule
    pub delete_rule: String,
}
