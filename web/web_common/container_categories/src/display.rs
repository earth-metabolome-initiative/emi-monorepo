//!  Submodule provding the implementation of the `Display` trait for the
//! [`ContainerCategory`] enum.

impl core::fmt::Display for crate::ContainerCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            crate::ContainerCategory::SampleContainer => write!(f, "Sample container"),
            crate::ContainerCategory::Bottle { liters } => {
                write!(f, "Bottle ({liters} L)")
            }
            crate::ContainerCategory::SampleContainerRack => write!(f, "Sample container rack"),
            crate::ContainerCategory::ContainerBox => write!(f, "Container box"),
        }
    }
}
