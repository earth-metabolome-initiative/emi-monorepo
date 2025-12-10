pub enum Error {
    /// Error indicating that an unknown element was encountered.
    Element(elements_rs::errors::Error),
    InvalidNumber,
}
