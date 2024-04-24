//! A trait for serializing a vector of objects to a vector of byte vectors.
use serde::Serialize;
use web_common::api::ApiError;

pub trait BincodeSerialize {
    /// Serialize a vector of objects to a vector of byte vectors.
    fn bincode_serialize(self) -> Result<Vec<Vec<u8>>, ApiError>;
}


impl<T,E> BincodeSerialize for Result<Vec<T>, E>
where
    T: Serialize,
    E: Into<ApiError>,
{
    fn bincode_serialize(self) -> Result<Vec<Vec<u8>>, ApiError> {
        self.map_err(E::into)
        .and_then(|x| {
            x
                .iter()
                .map(|x| {
                    bincode::serialize(x).map_err(
                        ApiError::from,
                    )
                })
                .collect()
    })
    }
}
