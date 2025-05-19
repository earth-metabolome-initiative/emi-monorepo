#![doc = include_str!("../README.md")]

#[cfg(feature = "pgrx")]
::pgrx::pg_module_magic!();

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    diesel::FromSqlRow,
    diesel::AsExpression,
    diesel_pgrx::DieselPGRX,
)]
#[
	diesel(sql_type = crate::diesel_impls::CAS)
]
#[cfg_attr(
    feature = "pgrx",
    derive(pgrx::PostgresType, pgrx::PostgresEq, pgrx::PostgresOrd, pgrx::PostgresHash)
)]
#[cfg_attr(feature = "pgrx", pg_binary_protocol)]
/// Representation for a Chemical Abstracts Service (CAS) number.
pub struct CAS(u16, u8, u8);
