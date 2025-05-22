//! Submodule providing the `RootMediaType` enumeration.

mod display;
mod from_str;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, strum_macros::EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enum representing the root/top-level media types as defined by the MIME
/// standard (IANA Media Types).
///
/// Each variant corresponds to a top-level type used in MIME type strings, such
/// as in HTTP Content-Type headers. These types are used to broadly classify
/// the nature and format of a file or data stream.
///
/// See also: [IANA Media Types Registry](https://www.iana.org/assignments/media-types/media-types.xhtml)
pub enum RootMediaType {
    /// Matches any media type (`*`).
    #[cfg_attr(feature = "serde", serde(rename = "*"))]
    Star,
    /// Application-specific data (e.g., `application/json`, `application/pdf`).
    #[cfg_attr(feature = "serde", serde(rename = "application"))]
    Application,
    /// Audio data (e.g., `audio/mpeg`, `audio/ogg`).
    #[cfg_attr(feature = "serde", serde(rename = "audio"))]
    Audio,
    /// Example media types for documentation or testing (e.g.,
    /// `example/example`).
    #[cfg_attr(feature = "serde", serde(rename = "example"))]
    Example,
    /// Font files (e.g., `font/woff`, `font/ttf`).
    #[cfg_attr(feature = "serde", serde(rename = "font"))]
    Font,
    /// Haptics data (e.g., `haptics/metadata`).
    #[cfg_attr(feature = "serde", serde(rename = "haptics"))]
    Haptics,
    /// Image data (e.g., `image/png`, `image/jpeg`).
    #[cfg_attr(feature = "serde", serde(rename = "image"))]
    Image,
    /// Message containers (e.g., `message/http`, `message/global`).
    #[cfg_attr(feature = "serde", serde(rename = "message"))]
    Message,
    /// 3D models or similar data (e.g., `model/gltf+json`).
    #[cfg_attr(feature = "serde", serde(rename = "model"))]
    Model,
    /// Multipart containers (e.g., `multipart/form-data`, `multipart/mixed`).
    #[cfg_attr(feature = "serde", serde(rename = "multipart"))]
    Multipart,
    /// Textual data (e.g., `text/plain`, `text/html`).
    #[cfg_attr(feature = "serde", serde(rename = "text"))]
    Text,
    /// Video data (e.g., `video/mp4`, `video/webm`).
    #[cfg_attr(feature = "serde", serde(rename = "video"))]
    Video,
}
