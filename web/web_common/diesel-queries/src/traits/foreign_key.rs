//! Submodule defining a foreign key marker trait for Diesel schemas.

use crate::traits::compatible_type::ForeignKeyCompatibleColumns;

/// This trait can be implemented for types that represent foreign keys in
/// Diesel schemas. It serves as a marker trait without any associated methods
/// or functionality.
pub trait ForeignKey<HostColumns, ReferencedColumns>: diesel::Table
where
    HostColumns: ForeignKeyCompatibleColumns<ReferencedColumns>,
{
}
