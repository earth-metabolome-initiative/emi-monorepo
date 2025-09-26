//! Submodule providing the `DynamicSchema` trait, which defines a struct characterized by
//! being able to represent DB objects subject to changes. It also is built with the assumption
//! that possibly some of the SQL objects encountered during the visit (e.g. foreign tables in `FOREIGN KEY` constraints)
//! may not be defined via the provided SQL statements but some other means (e.g. pre-existing tables in the DB), and
//! thus will refrain from enforcing strict checks on the existence of such objects.

/// Trait for types that define a dynamic SQL schema.
pub trait DynamicSchema {}
