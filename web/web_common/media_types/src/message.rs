//! Submodule providing the `message` media type and its subtypes. This module is generated from the IANA registry at `https://www.iana.org/assignments/media-types/message.csv`.
/// The `bhttp` subtype of the `message` media type.
pub const BHTTP: &str = "bhttp";
/// The `CPIM` subtype of the `message` media type.
pub const CPIM: &str = "CPIM";
/// The `delivery-status` subtype of the `message` media type.
pub const DELIVERY_STATUS: &str = "delivery-status";
/// The `disposition-notification` subtype of the `message` media type.
pub const DISPOSITION_NOTIFICATION: &str = "disposition-notification";
/// The `example` subtype of the `message` media type.
pub const EXAMPLE: &str = "example";
/// The `external-body` subtype of the `message` media type.
pub const EXTERNAL_BODY: &str = "external-body";
/// The `feedback-report` subtype of the `message` media type.
pub const FEEDBACK_REPORT: &str = "feedback-report";
/// The `global` subtype of the `message` media type.
pub const GLOBAL: &str = "global";
/// The `global-delivery-status` subtype of the `message` media type.
pub const GLOBAL_DELIVERY_STATUS: &str = "global-delivery-status";
/// The `global-disposition-notification` subtype of the `message` media type.
pub const GLOBAL_DISPOSITION_NOTIFICATION: &str = "global-disposition-notification";
/// The `global-headers` subtype of the `message` media type.
pub const GLOBAL_HEADERS: &str = "global-headers";
/// The `http` subtype of the `message` media type.
pub const HTTP: &str = "http";
/// The `imdn+xml` subtype of the `message` media type.
pub const IMDN_XML: &str = "imdn+xml";
/// The `mls` subtype of the `message` media type.
pub const MLS: &str = "mls";
/// The `news` subtype of the `message` media type.
#[deprecated(since = "0.1.0", note = "This subtype is deprecated and should not be used.")]
pub const NEWS: &str = "news";
/// The `ohttp-req` subtype of the `message` media type.
pub const OHTTP_REQ: &str = "ohttp-req";
/// The `ohttp-res` subtype of the `message` media type.
pub const OHTTP_RES: &str = "ohttp-res";
/// The `partial` subtype of the `message` media type.
pub const PARTIAL: &str = "partial";
/// The `rfc822` subtype of the `message` media type.
pub const RFC822: &str = "rfc822";
/// The `s-http` subtype of the `message` media type.
#[deprecated(since = "0.1.0", note = "This subtype is deprecated and should not be used.")]
pub const S_HTTP: &str = "s-http";
/// The `sip` subtype of the `message` media type.
pub const SIP: &str = "sip";
/// The `sipfrag` subtype of the `message` media type.
pub const SIPFRAG: &str = "sipfrag";
/// The `tracking-status` subtype of the `message` media type.
pub const TRACKING_STATUS: &str = "tracking-status";
/// The `vnd.si.simp` subtype of the `message` media type.
#[deprecated(since = "0.1.0", note = "This subtype is deprecated and should not be used.")]
pub const VND_SI_SIMP: &str = "vnd.si.simp";
/// The `vnd.wfa.wsc` subtype of the `message` media type.
pub const VND_WFA_WSC: &str = "vnd.wfa.wsc";
/// Set of all subtypes of the `message` media type. This set is generated from the IANA registry at `https://www.iana.org/assignments/media-types/message.csv`.
pub static SUBTYPES: ::phf::Set<&'static str> = ::phf::phf_set! { "bhttp" , "CPIM" , "delivery-status" , "disposition-notification" , "example" , "external-body" , "feedback-report" , "global" , "global-delivery-status" , "global-disposition-notification" , "global-headers" , "http" , "imdn+xml" , "mls" , "news" , "ohttp-req" , "ohttp-res" , "partial" , "rfc822" , "s-http" , "sip" , "sipfrag" , "tracking-status" , "vnd.si.simp" , "vnd.wfa.wsc" };
