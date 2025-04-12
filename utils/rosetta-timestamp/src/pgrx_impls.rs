//! Submodule proving the implementations of the pgrx-related traits.

use pgrx::{FromDatum, IntoDatum, pg_sys};
use pgrx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};

impl From<pgrx::pg_sys::TimestampTz> for crate::TimestampUTC {
    fn from(timestamp: pgrx::pg_sys::TimestampTz) -> Self {
        use chrono::TimeZone;
        Self(chrono::Utc.timestamp_opt(timestamp, 0).unwrap())
    }
}

impl From<crate::TimestampUTC> for pgrx::pg_sys::TimestampTz {
    fn from(timestamp: crate::TimestampUTC) -> Self {
        let timestamp: chrono::DateTime<chrono::Utc> = timestamp.into();
        timestamp.timestamp() as pgrx::pg_sys::TimestampTz
    }
}

impl IntoDatum for crate::TimestampUTC {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let pgrx_timestamp: pgrx::pg_sys::TimestampTz = self.into();
        pgrx_timestamp.into_datum()
    }

    #[inline]
    fn type_oid() -> pg_sys::Oid {
        pgrx::pg_sys::TimestampTz::type_oid()
    }
}

impl FromDatum for crate::TimestampUTC {
    #[inline]
    #[allow(unsafe_code)]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: pg_sys::Oid,
    ) -> Option<Self> {
        unsafe {
            pgrx::pg_sys::TimestampTz::from_polymorphic_datum(datum, is_null, typoid)
                .map(Into::into)
        }
    }
}

unsafe impl SqlTranslatable for crate::TimestampUTC {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        pgrx::pg_sys::TimestampTz::argument_sql()
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        pgrx::pg_sys::TimestampTz::return_sql()
    }
}

unsafe impl<'fcx> pgrx::callconv::ArgAbi<'fcx> for crate::TimestampUTC {
    unsafe fn unbox_arg_unchecked(arg: pgrx::callconv::Arg<'_, 'fcx>) -> Self {
        unsafe {
            <pgrx::pg_sys::TimestampTz as pgrx::callconv::ArgAbi<'fcx>>::unbox_arg_unchecked(arg)
                .into()
        }
    }

    unsafe fn unbox_nullable_arg(
        arg: pgrx::callconv::Arg<'_, 'fcx>,
    ) -> pgrx::nullable::Nullable<Self> {
        unsafe {
            <pgrx::pg_sys::TimestampTz as pgrx::callconv::ArgAbi<'fcx>>::unbox_nullable_arg(arg)
                .map(Into::into)
        }
    }
}

unsafe impl pgrx::callconv::BoxRet for crate::TimestampUTC {
    unsafe fn box_into<'fcx>(
        self,
        fcinfo: &mut pgrx::callconv::FcInfo<'fcx>,
    ) -> pgrx::datum::Datum<'fcx> {
        unsafe {
            <pgrx::pg_sys::TimestampTz as pgrx::callconv::BoxRet>::box_into(self.into(), fcinfo)
        }
    }
}
