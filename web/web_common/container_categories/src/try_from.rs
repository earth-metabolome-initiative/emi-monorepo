//!  Submodule provding implementations of the `TryFrom` trait for the
//! [`ContainerCategory`] enum.

impl TryFrom<&str> for crate::ContainerCategory {
    type Error = crate::errors::UnknownContainerCategory;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
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

impl TryFrom<String> for crate::ContainerCategory {
    type Error = crate::errors::UnknownContainerCategory;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
