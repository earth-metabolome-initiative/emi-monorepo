//! Utility functions for working with CAS numbers.

/// Computes the checksum for a given CAS number according to the official CAS
/// algorithm.
///
/// The check digit is calculated by:
/// 1. Taking each digit from right to left (excluding the check digit itself)
/// 2. Multiplying the first digit by 1, second by 2, third by 3, etc.
/// 3. Summing all these products
/// 4. Taking the sum modulo 10
///
/// ## Examples
///
/// ```rust
/// use cas_codes::{CAS, utils::checksum};
///
/// // Water: 7732-18-5
/// // Calculation: (8×1 + 1×2 + 2×3 + 3×4 + 7×5 + 7×6) = 105; 105 mod 10 = 5
/// let cas = CAS::try_from("7732-18-5")?;
/// assert_eq!(checksum(cas), 5);
///
/// // Ethanol: 64-17-5
/// // Calculation: (7×1 + 1×2 + 4×3 + 6×4) = 35; 35 mod 10 = 5
/// let cas = CAS::try_from("64-17-5")?;
/// assert_eq!(checksum(cas), 5);
/// # Ok::<(), cas_codes::errors::Error>(())
/// ```
///
/// ## Algorithm Details
///
/// This implements the official Chemical Abstracts Service checksum algorithm.
/// The calculation starts from the rightmost digit (excluding check digit) and
/// works leftward, with each position having an incrementally higher
/// multiplier.
#[must_use]
pub fn checksum(cas: crate::CAS) -> u8 {
    let mut factor: u8 = 0;

    let sum: u32 = cas
        .digits()
        // We skip the first digit (the check digit) and start with the second
        .skip(1)
        .map(|digit| {
            factor += 1;
            u32::from(digit * factor)
        })
        .sum();

    u8::try_from(sum % 10).unwrap()
}
