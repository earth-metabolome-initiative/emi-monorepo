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

impl ReferentialConstraint {
    /// Returns true if the referential constraint has an ON DELETE CASCADE rule
    pub fn on_delete_cascade(&self) -> bool {
        self.delete_rule.eq_ignore_ascii_case("CASCADE")
    }

    /// Returns the match kind of the referential constraint
    pub fn match_kind(&self) -> sqlparser::ast::MatchKind {
        match self.match_option.to_uppercase().as_str() {
            "FULL" => sqlparser::ast::MatchKind::Full,
            "PARTIAL" => sqlparser::ast::MatchKind::Partial,
            "SIMPLE" => sqlparser::ast::MatchKind::Simple,
            other => unreachable!("Unexpected match option: {other}"),
        }
    }
}
