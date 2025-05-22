#![doc = include_str!("../README.md")]

#[cfg(feature = "pgrx")]
::pgrx::pg_module_magic!();

mod display;
pub mod errors;
mod from_str;
pub mod root_media_type;
mod serde;
mod try_from;
pub use root_media_type::RootMediaType;
pub mod sub_media_type;
pub use sub_media_type::SubMediaType;

pub mod application;
pub mod audio;
pub mod font;
pub mod haptics;
pub mod image;
pub mod message;
pub mod model;
pub mod multipart;
pub mod text;
pub mod video;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "diesel_pgrx", derive(diesel_pgrx::DieselPGRX))]
#[cfg_attr(
    feature = "pgrx",
    derive(pgrx::PostgresType, pgrx::PostgresEq, pgrx::PostgresOrd, pgrx::PostgresHash)
)]
#[cfg_attr(feature = "diesel_pgrx", derive(diesel::FromSqlRow, diesel::AsExpression))]
#[
    cfg_attr(feature = "diesel_pgrx", diesel(sql_type = crate::diesel_impls::MediaType))]
#[cfg_attr(feature = "pgrx", pg_binary_protocol)]
/// Struct representing a media type, which consists of a root type, a subtype,
/// and optional parameters.
pub struct MediaType {
    /// The root type of the media type (e.g., `application`, `image`).
    root: RootMediaType,
    /// The subtype of the media type (e.g., `json`, `png`).
    subtype: SubMediaType,
    /// Optional parameters associated with the media type (e.g.,
    /// `charset=utf-8`).
    params: Vec<(String, String)>,
}

impl MediaType {
    /// Returns the root type of the media type.
    pub fn root(&self) -> RootMediaType {
        self.root
    }

    /// Returns the subtype of the media type.
    pub fn subtype(&self) -> &SubMediaType {
        &self.subtype
    }

    /// Returns the parameters associated with the media type.
    pub fn params(&self) -> &[(String, String)] {
        &self.params
    }
}
