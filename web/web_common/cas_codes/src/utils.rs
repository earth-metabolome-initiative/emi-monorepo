//! Submodule implementing utilities for the `CAS` number struct.

#[must_use]
/// Computes the checksum for a given CAS number.
///
/// # Arguments
/// 
/// * `cas` - A `CAS` number instance.
/// 
/// # Implementation details
///
/// The check digit is found by taking the last digit times 1, the preceding
/// digit times 2, the preceding digit times 3 etc., adding all these up and
/// computing the sum modulo 10.
///
/// # Example
///
/// For example, the CAS number of water is 7732-18-5: the checksum 5 is
/// calculated as (8×1 + 1×2 + 2×3 + 3×4 + 7×5 + 7×6) = 105; 105 mod 10 = 5.
/// 
/// ```rust
/// use cas_codes::CAS;
/// use cas_codes::utils::checksum;
/// 
/// let cas = CAS::try_from("7732-18-5").unwrap();
/// let expected_checksum = 5;
/// 
/// assert_eq!(checksum(cas), expected_checksum);
/// ```
/// 
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


