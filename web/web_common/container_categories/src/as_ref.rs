//! Submodule providing implementations of the `AsRef` trait
//! for the `ContainerCategory` enumeration.

impl AsRef<str> for crate::ContainerCategory {
    fn as_ref(&self) -> &str {
        match self {
            Self::Bottle => "Bottle",
            Self::SampleContainerRack => "SampleContainerRack",
            Self::ContainerBox => "ContainerBox",
        }
    }
}
