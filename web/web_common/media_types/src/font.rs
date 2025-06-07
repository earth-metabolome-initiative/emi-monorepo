//! Submodule providing the `font` media type and its subtypes. This module is generated from the IANA registry at `https://www.iana.org/assignments/media-types/font.csv`.
/// The `collection` subtype of the `font` media type.
pub const COLLECTION: &str = "collection";
/// The `otf` subtype of the `font` media type.
pub const OTF: &str = "otf";
/// The `sfnt` subtype of the `font` media type.
pub const SFNT: &str = "sfnt";
/// The `ttf` subtype of the `font` media type.
pub const TTF: &str = "ttf";
/// The `woff` subtype of the `font` media type.
pub const WOFF: &str = "woff";
/// The `woff2` subtype of the `font` media type.
pub const WOFF2: &str = "woff2";
/// Set of all subtypes of the `font` media type. This set is generated from the IANA registry at `https://www.iana.org/assignments/media-types/font.csv`.
pub static SUBTYPES: ::phf::Set<&'static str> =
    ::phf::phf_set! { "collection" , "otf" , "sfnt" , "ttf" , "woff" , "woff2" };
