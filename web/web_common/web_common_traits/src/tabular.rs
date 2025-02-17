//! Submodule providing the `Tabular` trait which represents metadata
//! associated to structs that are stored in a tabular format.

use crate::prelude::Connection;

/// Trait representing metadata associated to structs that are stored in a
/// tabular format.
pub trait Tabular {
    /// Type of the connection associated to the struct.
    type Connection: Connection;
    #[cfg(feature = "backend")]
    /// Type of the table associated to the struct.
    type Table: diesel::Table + Send + Sync;
    #[cfg(feature = "backend")]
    /// Table associated to the struct.
    const TABLE: Self::Table;
    /// Struct representing a row in the table.
    type Row: Send + Sync;
    /// Name of the table.
    const TABLE_NAME: &'static str;
}

/// Trait representing an object that can be inserted into a table.
pub trait Insertable: Tabular {
    /// Values to be inserted.
    fn insert(self) -> Result<Self::Row, <Self::Connection as Connection>::Error>;
}
