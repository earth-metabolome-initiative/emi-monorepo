#![doc = include_str!("../README.md")]

#[cfg(feature = "pgrx")]
::pgrx::pg_module_magic!();

mod digits;
mod display;
mod from;
mod from_str;
mod serde;
mod try_from;
pub use digits::Digits;
pub mod errors;
pub mod utils;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "diesel_pgrx", derive(diesel_pgrx::DieselPgrx))]
#[cfg_attr(feature = "diesel", derive(diesel::AsExpression, diesel::FromSqlRow))]
#[
    cfg_attr(feature = "diesel", diesel(sql_type = crate::diesel_impls::CAS))]
#[cfg_attr(
    feature = "pgrx",
    derive(pgrx::PostgresType, pgrx::PostgresEq, pgrx::PostgresOrd, pgrx::PostgresHash)
)]
#[cfg_attr(feature = "pgrx", pg_binary_protocol)]
/// Representation for a Chemical Abstracts Service (CAS) number.
pub struct CAS(u32);

impl CAS {
    #[must_use]
    /// Returns the check digit (rightmost digit) of the CAS number.
    pub fn check_digit(self) -> u8 {
        u8::try_from(self.0 % 10).unwrap()
    }

    #[must_use]
    /// Returns the first part of the CAS number (the leftmost digits).
    pub fn first(self) -> u32 {
        self.0 / 1000
    }

    #[must_use]
    /// Returns the second part of the CAS number (the middle two digits).
    pub fn second(self) -> u8 {
        u8::try_from((self.0 / 10) % 100).unwrap()
    }

    #[must_use]
    /// Returns an iterator over the digits of the CAS number.
    ///
    /// # Implementation details
    ///
    /// The digits are returned from the rightmost digit to the leftmost digit,
    /// i.e. from the least significant digit to the most significant digit.
    pub fn digits(self) -> Digits {
        Digits::from(self)
    }
}
