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
#[cfg_attr(feature = "diesel", derive(diesel_pgrx::DieselPGRX))]
#[cfg_attr(feature = "diesel", derive(diesel::AsExpression, diesel::FromSqlRow))]
#[
    cfg_attr(feature = "diesel", diesel(sql_type = crate::diesel_impls::CAS))]
#[cfg_attr(
    feature = "pgrx",
    derive(pgrx::PostgresType, pgrx::PostgresEq, pgrx::PostgresOrd, pgrx::PostgresHash)
)]
#[cfg_attr(feature = "pgrx", pg_binary_protocol)]
/// A Chemical Abstracts Service (CAS) Registry Number.
///
/// CAS Registry Numbers are unique identifiers for chemical substances in the
/// format `NNNNNNN-NN-N` where the last digit is a check digit.
///
/// This struct stores the CAS number as a single `u32` for efficiency, where
/// the value represents the concatenation of all digits (e.g., "7732-18-5"
/// becomes 7732185).
///
/// ## Examples
///
/// ```rust
/// use cas_codes::CAS;
///
/// // Parse from string
/// let cas = CAS::try_from("7732-18-5")?;
///
/// // Access components
/// assert_eq!(cas.first(), 7732);
/// assert_eq!(cas.second(), 18);
/// assert_eq!(cas.check_digit(), 5);
///
/// // Display
/// assert_eq!(format!("{}", cas), "7732-18-5");
/// # Ok::<(), cas_codes::errors::Error>(())
/// ```
///
/// ## Validation
///
/// CAS numbers are automatically validated during parsing:
/// - Format must be `N+-N{2}-N{1}` (using hyphens or en-dashes)
/// - Check digit must be correct according to CAS algorithm
///
/// ```rust
/// use cas_codes::CAS;
///
/// // This will fail due to incorrect check digit
/// assert!(CAS::try_from("7732-18-6").is_err());
/// ```
pub struct CAS(u32);

impl CAS {
    /// Creates a new CAS number from its three components.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use cas_codes::CAS;
    ///
    /// let cas = CAS::new(7732, 18, 5)?;
    /// assert_eq!(cas.to_string(), "7732-18-5");
    /// # Ok::<(), cas_codes::errors::Error>(())
    /// ```
    ///
    /// ## Errors
    ///
    /// Returns an error if the check digit doesn't match the calculated
    /// checksum.
    pub fn new(first: u32, second: u8, third: u8) -> Result<Self, crate::errors::Error> {
        Self::try_from((first, second, third))
    }

    /// Returns the check digit (rightmost digit) of the CAS number.
    ///
    /// The check digit is calculated using a specific algorithm to ensure data
    /// integrity.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use cas_codes::CAS;
    ///
    /// let cas = CAS::try_from("7732-18-5")?;
    /// assert_eq!(cas.check_digit(), 5);
    /// # Ok::<(), cas_codes::errors::Error>(())
    /// ```
    #[must_use]
    pub fn check_digit(self) -> u8 {
        u8::try_from(self.0 % 10).unwrap()
    }

    /// Returns the first part of the CAS number (the leftmost digits).
    ///
    /// This is typically the largest component and can range from 2-7 digits.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use cas_codes::CAS;
    ///
    /// let cas = CAS::try_from("7732-18-5")?;
    /// assert_eq!(cas.first(), 7732);
    /// # Ok::<(), cas_codes::errors::Error>(())
    /// ```
    #[must_use]
    pub fn first(self) -> u32 {
        self.0 / 1000
    }

    /// Returns the second part of the CAS number (the middle two digits).
    ///
    /// This part is always exactly 2 digits (padded with leading zeros if
    /// necessary).
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use cas_codes::CAS;
    ///
    /// let cas = CAS::try_from("7732-18-5")?;
    /// assert_eq!(cas.second(), 18);
    ///
    /// let cas2 = CAS::try_from("50-00-0")?; // Formaldehyde
    /// assert_eq!(cas2.second(), 0); // Note: returns 0, not 00
    /// //
    /// # Ok::<(), cas_codes::errors::Error>(())
    /// ```
    #[must_use]
    pub fn second(self) -> u8 {
        u8::try_from((self.0 / 10) % 100).unwrap()
    }

    /// Returns an iterator over the digits of the CAS number.
    ///
    /// The digits are returned from the rightmost digit to the leftmost digit
    /// (least significant to most significant).
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use cas_codes::CAS;
    ///
    /// let cas = CAS::try_from("7732-18-5")?;
    /// let digits: Vec<u8> = cas.digits().collect();
    /// assert_eq!(digits, vec![5, 8, 1, 2, 3, 7, 7]);
    /// # Ok::<(), cas_codes::errors::Error>(())
    /// ```
    #[must_use]
    pub fn digits(self) -> Digits {
        Digits::from(self)
    }

    /// Returns the CAS number as a `u32`.
    ///
    /// This represents the concatenated digits of the CAS number.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use cas_codes::CAS;
    ///
    /// let cas = CAS::try_from("7732-18-5")?;
    /// assert_eq!(cas.as_u32(), 7732185);
    /// # Ok::<(), cas_codes::errors::Error>(())
    /// ```
    #[must_use]
    pub fn as_u32(self) -> u32 {
        self.0
    }

    /// Validates a CAS number string without parsing it into a `CAS` struct.
    ///
    /// This is useful when you only need to check if a string represents a
    /// valid CAS number without creating the struct.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use cas_codes::CAS;
    ///
    /// assert!(CAS::is_valid("7732-18-5"));
    /// assert!(!CAS::is_valid("7732-18-6")); // Wrong check digit
    /// assert!(!CAS::is_valid("invalid")); // Invalid format
    /// ```
    #[must_use]
    pub fn is_valid(s: &str) -> bool {
        Self::try_from(s).is_ok()
    }
}
