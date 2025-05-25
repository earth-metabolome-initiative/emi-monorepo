//!  Submodule provding the implementation of the `Display` trait for the
//! [`ContainerCategory`] enum.

impl core::fmt::Display for crate::ContainerCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            crate::ContainerCategory::Bottle => write!(f, "Bottle"),
            crate::ContainerCategory::SampleContainerRack => write!(f, "SampleContainerRack"),
            crate::ContainerCategory::ContainerBox => write!(f, "ContainerBox"),
        }
    }
}
