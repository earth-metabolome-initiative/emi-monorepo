//!  Submodule provding implementations of the `TryFrom` trait for the
//! [`ContainerCategory`] enum.

use std::str::FromStr;

impl FromStr for crate::ContainerCategory {
    type Err = crate::errors::UnknownContainerCategory;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "Bottle" => Self::Bottle,
            "SampleContainerRack" => Self::SampleContainerRack,
            "ContainerBox" => Self::ContainerBox,
            _ => {
                return Err(crate::errors::UnknownContainerCategory::UnknownString(
                    value.to_string(),
                ));
            }
        })
    }
}

impl TryFrom<&str> for crate::ContainerCategory {
    type Error = crate::errors::UnknownContainerCategory;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for crate::ContainerCategory {
    type Error = crate::errors::UnknownContainerCategory;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
