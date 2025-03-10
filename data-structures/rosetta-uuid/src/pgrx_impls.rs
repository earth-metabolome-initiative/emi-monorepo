//! Implementation of the traits to allow for [`PGRX`](https://docs.rs/pgrx/latest/pgrx/)

use core::fmt::Write;
use std::ops::{Deref, DerefMut};

use pgrx::{FromDatum, IntoDatum, PgMemoryContexts, pg_sys};
use pgrx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};

impl From<pgrx::Uuid> for crate::Uuid {
    fn from(uuid: pgrx::Uuid) -> Self {
        Self::from(uuid.as_bytes())
    }
}

impl From<crate::Uuid> for pgrx::Uuid {
    fn from(uuid: crate::Uuid) -> Self {
        pgrx::Uuid::from_bytes(uuid.into())
    }
}

impl IntoDatum for crate::Uuid {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let pgrx_uuid: pgrx::Uuid = self.into();
        pgrx_uuid.into_datum()
    }

    #[inline]
    fn type_oid() -> pg_sys::Oid {
        pgrx::Uuid::type_oid()
    }
}

impl FromDatum for crate::Uuid {
    #[inline]
    #[allow(unsafe_code)]
    /// # Safety
    ///
    /// Refer to the trait documentation for [`FromDatum`](pgrx::FromDatum)
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: pg_sys::Oid,
    ) -> Option<Self> {
        unsafe { pgrx::Uuid::from_polymorphic_datum(datum, is_null, typoid).map(Into::into) }
    }
}

// impl std::fmt::Display for Uuid {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{self:-x}")
//     }
// }

// impl<'a> std::fmt::LowerHex for Uuid {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
// {         self.format(f, UuidFormatCase::Lowercase)
//     }
// }

// impl<'a> std::fmt::UpperHex for Uuid {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
// {         self.format(f, UuidFormatCase::Uppercase)
//     }
// }

unsafe impl SqlTranslatable for crate::Uuid {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
		pgrx::Uuid::argument_sql()
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
		pgrx::Uuid::return_sql()
    }
}
