//! Submodule implementing the `FromStr` trait for the `RootMediaType` struct.

use std::str::FromStr;

use super::RootMediaType;

impl FromStr for RootMediaType {
    type Err = crate::errors::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Self::Star),
            "application" => Ok(Self::Application),
            "audio" => Ok(Self::Audio),
            "example" => Ok(Self::Example),
            "font" => Ok(Self::Font),
            "haptics" => Ok(Self::Haptics),
            "image" => Ok(Self::Image),
            "message" => Ok(Self::Message),
            "model" => Ok(Self::Model),
            "multipart" => Ok(Self::Multipart),
            "text" => Ok(Self::Text),
            "video" => Ok(Self::Video),
            _ => Err(crate::errors::Error::UnknownRootMediaType(s.to_string())),
        }
    }
}
