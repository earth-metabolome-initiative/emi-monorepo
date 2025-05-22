//! Implementation of the traits to allow for [`PGRX`](https://docs.rs/pgrx/latest/pgrx/)

use pgrx::{FromDatum, IntoDatum, pg_sys};
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
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: pg_sys::Oid,
    ) -> Option<Self> {
        unsafe { pgrx::Uuid::from_polymorphic_datum(datum, is_null, typoid).map(Into::into) }
    }
}

unsafe impl SqlTranslatable for crate::Uuid {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        pgrx::Uuid::argument_sql()
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        pgrx::Uuid::return_sql()
    }
}

unsafe impl<'fcx> pgrx::callconv::ArgAbi<'fcx> for crate::Uuid {
    unsafe fn unbox_arg_unchecked(arg: pgrx::callconv::Arg<'_, 'fcx>) -> Self {
        unsafe { <pgrx::Uuid as pgrx::callconv::ArgAbi<'fcx>>::unbox_arg_unchecked(arg).into() }
    }

    unsafe fn unbox_nullable_arg(
        arg: pgrx::callconv::Arg<'_, 'fcx>,
    ) -> pgrx::nullable::Nullable<Self> {
        unsafe {
            <pgrx::Uuid as pgrx::callconv::ArgAbi<'fcx>>::unbox_nullable_arg(arg).map(Into::into)
        }
    }
}

unsafe impl pgrx::callconv::BoxRet for crate::Uuid {
    unsafe fn box_into<'fcx>(
        self,
        fcinfo: &mut pgrx::callconv::FcInfo<'fcx>,
    ) -> pgrx::datum::Datum<'fcx> {
        unsafe { <pgrx::Uuid as pgrx::callconv::BoxRet>::box_into(self.into(), fcinfo) }
    }
}
