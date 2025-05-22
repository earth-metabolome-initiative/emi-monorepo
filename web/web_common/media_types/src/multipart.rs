//! Submodule providing the `multipart` media type and its subtypes. This module is generated from the IANA registry at `https://www.iana.org/assignments/media-types/multipart.csv`.
/// The `alternative` subtype of the `multipart` media type.
pub const ALTERNATIVE: &str = "alternative";
/// The `appledouble` subtype of the `multipart` media type.
pub const APPLEDOUBLE: &str = "appledouble";
/// The `byteranges` subtype of the `multipart` media type.
pub const BYTERANGES: &str = "byteranges";
/// The `digest` subtype of the `multipart` media type.
pub const DIGEST: &str = "digest";
/// The `encrypted` subtype of the `multipart` media type.
pub const ENCRYPTED: &str = "encrypted";
/// The `example` subtype of the `multipart` media type.
pub const EXAMPLE: &str = "example";
/// The `form-data` subtype of the `multipart` media type.
pub const FORM_DATA: &str = "form-data";
/// The `header-set` subtype of the `multipart` media type.
pub const HEADER_SET: &str = "header-set";
/// The `mixed` subtype of the `multipart` media type.
pub const MIXED: &str = "mixed";
/// The `multilingual` subtype of the `multipart` media type.
pub const MULTILINGUAL: &str = "multilingual";
/// The `parallel` subtype of the `multipart` media type.
pub const PARALLEL: &str = "parallel";
/// The `related` subtype of the `multipart` media type.
pub const RELATED: &str = "related";
/// The `report` subtype of the `multipart` media type.
pub const REPORT: &str = "report";
/// The `signed` subtype of the `multipart` media type.
pub const SIGNED: &str = "signed";
/// The `vnd.bint.med-plus` subtype of the `multipart` media type.
pub const VND_BINT_MED_PLUS: &str = "vnd.bint.med-plus";
/// The `voice-message` subtype of the `multipart` media type.
pub const VOICE_MESSAGE: &str = "voice-message";
/// The `x-mixed-replace` subtype of the `multipart` media type.
pub const X_MIXED_REPLACE: &str = "x-mixed-replace";
/// Set of all subtypes of the `multipart` media type. This set is generated from the IANA registry at `https://www.iana.org/assignments/media-types/multipart.csv`.
pub static SUBTYPES: ::phf::Set<&'static str> = ::phf::phf_set! { "alternative" , "appledouble" , "byteranges" , "digest" , "encrypted" , "example" , "form-data" , "header-set" , "mixed" , "multilingual" , "parallel" , "related" , "report" , "signed" , "vnd.bint.med-plus" , "voice-message" , "x-mixed-replace" };
