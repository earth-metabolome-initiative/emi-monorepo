#![doc = include_str!("../README.md")]

#[cfg(feature = "pgrx")]
::pgrx::pg_module_magic!();

#[derive(diesel_pgrx::DieselPGRX, serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "pgrx",
    derive(pgrx::PostgresType, pgrx::PostgresEq, pgrx::PostgresOrd, pgrx::PostgresHash)
)]
#[cfg_attr(feature = "pgrx", pg_binary_protocol)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    diesel::FromSqlRow,
    diesel::AsExpression,
)]
#[diesel(sql_type = diesel_impls::PositiveU32)]
/// A struct representing a positive 32-bit unsigned integer.
pub struct PositiveU32 {
    /// The field that holds the positive 32-bit unsigned integer value.
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
