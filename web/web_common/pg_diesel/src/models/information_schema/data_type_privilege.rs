//! Struct representing a row in the `information_schema.data_type_privileges`
//! table.

use diesel::{Queryable, QueryableByName, Selectable};

/// Struct defining the `information_schema.data_type_privileges` table.
///
/// The `data_type_privileges` view contains one row for each data type
/// privilege. This provides information about data type access permissions.
#[derive(
    Queryable, QueryableByName, Selectable, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::data_type_privileges::data_type_privileges)]
pub struct DataTypePrivilege {
    /// Catalog (database) containing the object.
    pub object_catalog: Option<String>,
    /// Schema containing the object.
    pub object_schema: Option<String>,
    /// Name of the object.
    pub object_name: Option<String>,
    /// Type of the object.
    pub object_type: Option<String>,
    /// DTD identifier for the data type.
    pub dtd_identifier: Option<String>,
}
