//! Submodule with the implementations of the [`HasOid`] trait for various
//! structs.

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
