/// Represents a row from the `information_schema.views` table.
///
/// Contains metadata about database views including their definitions and
/// capabilities. Views are virtual tables based on the result of a SQL query,
/// and this table provides information about their properties such as whether
/// they're updatable, insertable, or support trigger operations.
#[derive(diesel::Queryable, diesel::QueryableByName, diesel::Selectable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = crate::schema::information_schema::views::views)]
pub struct Views {
    /// Name of the database (catalog) containing the view.
    pub table_catalog: Option<String>,
    /// Name of the schema containing the view.
    pub table_schema: Option<String>,
    /// Name of the view.
    pub table_name: Option<String>,
    /// Query expression defining the view (SQL text).
    pub view_definition: Option<String>,
    /// Whether the view has a CHECK OPTION defined.
    pub check_option: Option<String>,
    /// Whether the view is updatable (can accept UPDATE statements).
    pub is_updatable: Option<String>,
    /// Whether the view allows INSERT operations.
    pub is_insertable_into: Option<String>,
    /// Whether the view supports trigger-based updates.
    pub is_trigger_updatable: Option<String>,
    /// Whether the view supports trigger-based deletes.
    pub is_trigger_deletable: Option<String>,
    /// Whether the view supports trigger-based inserts.
    pub is_trigger_insertable_into: Option<String>,
}
