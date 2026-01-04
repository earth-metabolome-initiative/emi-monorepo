//! Implementations of the `TryFrom` trait for the `Isotope` enumeration.

use super::HydrogenIsotope;

impl TryFrom<char> for crate::Isotope {
    type Error = crate::errors::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'P' => HydrogenIsotope::H1.into(),
            'D' => HydrogenIsotope::D2.into(),
            'T' => HydrogenIsotope::T3.into(),
            _ => {
                return Err(crate::errors::Error::CharacterIsotope(value));
            }
        })
    }
}
