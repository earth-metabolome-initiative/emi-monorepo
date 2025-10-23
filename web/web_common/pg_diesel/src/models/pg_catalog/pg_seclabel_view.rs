//! Submodule providing the `PgSeclabelView` struct representing a row of the
//! `pg_seclabels` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_seclabels` view.
///
/// The `pg_seclabels` view provides a user-friendly representation of security
/// labels stored in `pg_seclabel`, showing object names and types rather than
/// just OIDs.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/view-pg-seclabels.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_seclabels::pg_seclabels)]
pub struct PgSeclabelView {
    /// OID of the object.
    pub objoid: Option<u32>,
    /// OID of the system catalog.
    pub classoid: Option<u32>,
    /// Sub-object ID.
    pub objsubid: Option<i32>,
    /// Object type.
    pub objtype: Option<String>,
    /// OID of the namespace.
    pub objnamespace: Option<u32>,
    /// Object name.
    pub objname: Option<String>,
    /// Provider name.
    pub provider: Option<String>,
    /// Security label.
    pub label: Option<String>,
}
