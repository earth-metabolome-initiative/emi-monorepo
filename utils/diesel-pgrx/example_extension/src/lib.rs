#![doc = include_str!("../README.md")]

#[cfg(feature = "pgrx")]
::pgrx::pg_module_magic!();

#[derive(diesel_pgrx::DieselPGRX)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresType))]
#[derive(Debug, PartialEq, Eq, diesel::FromSqlRow, diesel::AsExpression)]
#[diesel(sql_type = diesel_impls::PositiveU32)]
pub struct PositiveU32 {
    pub field: i32,
}

#[cfg(feature = "pgrx")]
#[pgrx::pg_extern]
/// Validation function that checks that the provided `PositiveU32` has a
/// `field` value that is greater than 0.
pub fn validate_positive_u32(value: PositiveU32) -> bool {
    if value.field > 0 {
        true
    } else {
        pgrx::error!("Validation failed: field must be greater than 0");
    }
}
