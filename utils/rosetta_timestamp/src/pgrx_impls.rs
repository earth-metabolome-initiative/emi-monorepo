#![cfg(feature = "pgrx")]
//! Submodule proving the implementations of the pgrx-related traits.

use pgrx::{FromDatum, IntoDatum, pg_sys};
use pgrx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};

impl From<pgrx::pg_sys::TimestampTz> for crate::TimestampUTC {
    fn from(timestamp: pgrx::pg_sys::TimestampTz) -> Self {
        use chrono::TimeZone;
        // Postgres epoch: 2000-01-01 00:00:00 UTC
        // Unix epoch: 1970-01-01 00:00:00 UTC
        // Difference in seconds:
        const POSTGRES_EPOCH_UNIX: i64 = 946684800; // seconds between 1970-01-01 and 2000-01-01
        // TimestampTz is microseconds since 2000-01-01
        let micros_since_pg_epoch = timestamp as i64;
        let unix_timestamp = POSTGRES_EPOCH_UNIX + (micros_since_pg_epoch / 1_000_000);
        let nanos = (micros_since_pg_epoch % 1_000_000) * 1000;
        match chrono::Utc.timestamp_opt(unix_timestamp, nanos as u32) {
            chrono::LocalResult::Single(dt) => crate::TimestampUTC::from(dt),
            chrono::LocalResult::None => {
                // If the timestamp is invalid, we return a default value.
                unimplemented!(
                    "Invalid timestamp conversion from pgrx::pg_sys::TimestampTz `{}` to crate::TimestampUTC",
                    timestamp
                );
            }
            chrono::LocalResult::Ambiguous(_, _) => {
                // If the timestamp is ambiguous, we return a default value.
                unimplemented!(
                    "Ambiguous timestamp conversion from pgrx::pg_sys::TimestampTz `{}` to crate::TimestampUTC",
                    timestamp
                )
            }
        }
    }
}

impl From<crate::TimestampUTC> for pgrx::pg_sys::TimestampTz {
    fn from(timestamp: crate::TimestampUTC) -> Self {
        // Convert chrono::DateTime<Utc> to microseconds since 2000-01-01
        let dt: chrono::DateTime<chrono::Utc> = timestamp.into();
        const POSTGRES_EPOCH_UNIX: i64 = 946684800;
        let unix_ts = dt.timestamp();
        let micros =
            (unix_ts - POSTGRES_EPOCH_UNIX) * 1_000_000 + (dt.timestamp_subsec_micros() as i64);
        micros as pgrx::pg_sys::TimestampTz
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
        1184.into()
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
        Ok(SqlMapping::literal("timestamp with time zone"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("timestamp with time zone")))
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
