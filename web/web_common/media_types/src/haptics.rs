//! Submodule providing the `haptics` media type and its subtypes. This module is generated from the IANA registry at `https://www.iana.org/assignments/media-types/haptics.csv`.
/// The `ivs` subtype of the `haptics` media type.
pub const IVS: &str = "ivs";
/// The `hjif` subtype of the `haptics` media type.
pub const HJIF: &str = "hjif";
/// The `hmpg` subtype of the `haptics` media type.
pub const HMPG: &str = "hmpg";
/// Set of all subtypes of the `haptics` media type. This set is generated from the IANA registry at `https://www.iana.org/assignments/media-types/haptics.csv`.
pub static SUBTYPES: ::phf::Set<&'static str> = ::phf::phf_set! { "ivs" , "hjif" , "hmpg" };
