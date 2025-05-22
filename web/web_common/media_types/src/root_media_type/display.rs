//! Submodule implementing the `Display` trait for the `RootMediaType` enum.

impl core::fmt::Display for super::RootMediaType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Star => write!(f, "*"),
            Self::Application => write!(f, "application"),
            Self::Audio => write!(f, "audio"),
            Self::Example => write!(f, "example"),
            Self::Font => write!(f, "font"),
            Self::Haptics => write!(f, "haptics"),
            Self::Image => write!(f, "image"),
            Self::Message => write!(f, "message"),
            Self::Model => write!(f, "model"),
            Self::Multipart => write!(f, "multipart"),
            Self::Text => write!(f, "text"),
            Self::Video => write!(f, "video"),
        }
    }
}
