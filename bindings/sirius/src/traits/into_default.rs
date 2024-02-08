/// A trait to convert a type into its default value, as defined
/// by the authors of the current crate.
pub trait IntoDefault {
    fn into_default(self) -> Self;
}
