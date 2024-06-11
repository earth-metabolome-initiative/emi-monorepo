mod jpeg;
pub(crate) use jpeg::JPEG;
mod point;

pub(crate) trait Convert<Destination> {
    /// Convert the type to the destination type.
    fn convert(self) -> Destination;
}
