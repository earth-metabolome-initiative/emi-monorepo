//! Submodule providing implementations of the `TryFrom` trait for the
//! [`NameplateCategory`] enumeration.

impl TryFrom<&str> for crate::NameplateCategory {
    type Error = crate::errors::UnknownNameplateCategory;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "Permanent" => Self::Permanent,
            "SemiPermanent" => Self::SemiPermanent,
            _ => {
                return Err(crate::errors::UnknownNameplateCategory::UnknownString(
                    value.to_string(),
                ));
            }
        })
    }
}

impl TryFrom<String> for crate::NameplateCategory {
    type Error = crate::errors::UnknownNameplateCategory;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&String> for crate::NameplateCategory {
    type Error = crate::errors::UnknownNameplateCategory;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
