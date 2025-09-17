pub enum Error {
    SampleID(SampleIDError),
}

pub enum SampleIDError {
    Empty,
    InvalidFormat,
}
