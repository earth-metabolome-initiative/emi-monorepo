//! Implementations of [`HasOid`] for PostgreSQL catalog types.
//!
//! This module implements the [`HasOid`] trait for various model structs that
//! have PostgreSQL Object Identifiers (OIDs). OIDs are unique identifiers for
//! database objects in the system catalogs.
//!
//! ## Implemented Types
//!
//! The trait is implemented for:
//! - [`PgIndex`](crate::models::PgIndex): Returns `indexrelid`
//! - [`PgConstraint`](crate::models::PgConstraint): Returns `oid`
//! - [`PgExtension`](crate::models::PgExtension): Returns `oid`
//! - [`PgOperator`](crate::models::PgOperator): Returns `oid`
//! - [`PgType`](crate::models::PgType): Returns `oid`
//! - [`PgProc`](crate::models::PgProc): Returns `oid`
//! - [`PgDescription`](crate::models::PgDescription): Returns `objoid` (the OID
//!   of the described object)
//!
//! These implementations are used by caching infrastructure to efficiently key
//! cached values.

use crate::traits::HasOid;

impl HasOid for crate::models::PgIndex {
    fn oid(&self) -> u32 {
        self.indexrelid
    }
}

impl HasOid for crate::models::PgConstraint {
    fn oid(&self) -> u32 {
        self.oid
    }
}

impl HasOid for crate::models::PgExtension {
    fn oid(&self) -> u32 {
        self.oid
    }
}

impl HasOid for crate::models::PgOperator {
    fn oid(&self) -> u32 {
        self.oid
    }
}

impl HasOid for crate::models::PgType {
    fn oid(&self) -> u32 {
        self.oid
    }
}

impl HasOid for crate::models::PgProc {
    fn oid(&self) -> u32 {
        self.oid
    }
}

impl HasOid for crate::models::PgDescription {
    fn oid(&self) -> u32 {
        self.objoid
    }
}
