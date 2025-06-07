//! Submodule providing the `SubMediaType` enumeration.

use crate::RootMediaType;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, strum_macros::EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enum representing the root media types.
pub enum SubMediaType {
    /// Wildcard subtype (`*`).
    Star,
    /// Other subtypes which are too many to enumerate.
    Other(String),
}

impl core::fmt::Display for SubMediaType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Star => write!(f, "*"),
            Self::Other(subtype) => write!(f, "{subtype}"),
        }
    }
}

impl SubMediaType {
    /// Returns the subtype as a string slice.
    ///
    /// # Arguments
    ///
    /// * `root` - The root media type.
    /// * `subtype_candidate` - The subtype candidate string.
    ///
    /// # Errors
    ///
    /// * If the subtype candidate is not valid for the given root media type,
    ///   an error is returned.
    ///
    /// # Examples
    ///
    /// Let's start with a few simple working examples:
    ///
    /// ```rust
    /// use media_types::SubMediaType;
    ///
    /// let subtype =
    ///     SubMediaType::from_str_with_root(media_types::RootMediaType::Application, "json").unwrap();
    ///
    /// assert_eq!(subtype.to_string(), "json");
    ///
    /// let subtype =
    ///     SubMediaType::from_str_with_root(media_types::RootMediaType::Image, "png").unwrap();
    ///
    /// assert_eq!(subtype.to_string(), "png");
    ///
    /// let subtype = SubMediaType::from_str_with_root(media_types::RootMediaType::Star, "*").unwrap();
    ///
    /// assert_eq!(subtype.to_string(), "*");
    /// ```
    ///
    /// Now let's consider a few examples that will fail:
    ///
    /// ```rust
    /// use media_types::SubMediaType;
    ///
    /// let subtype =
    ///     SubMediaType::from_str_with_root(media_types::RootMediaType::Image, "mp4").unwrap_err();
    ///
    /// assert_eq!(
    ///     subtype,
    ///     media_types::errors::Error::UnknownSubMediaType(
    ///         media_types::RootMediaType::Image,
    ///         "mp4".to_owned()
    ///     )
    /// );
    /// ```
    ///
    /// Similarly, it is not possible to specify a subtype for the `Star` or
    /// `Example` root media types:
    ///
    /// ```rust
    /// use media_types::SubMediaType;
    ///
    /// let subtype =
    ///     SubMediaType::from_str_with_root(media_types::RootMediaType::Star, "json").unwrap_err();
    ///
    /// assert_eq!(
    ///     subtype,
    ///     media_types::errors::Error::UnknownSubMediaType(
    ///         media_types::RootMediaType::Star,
    ///         "json".to_owned()
    ///     )
    /// );
    /// ```
    pub fn from_str_with_root(
        root: RootMediaType,
        subtype_candidate: &str,
    ) -> Result<Self, crate::errors::Error> {
        if subtype_candidate == "*" {
            return Ok(Self::Star);
        }

        let subtypes = match root {
            RootMediaType::Application => &crate::application::SUBTYPES,
            RootMediaType::Audio => &crate::audio::SUBTYPES,
            RootMediaType::Font => &crate::font::SUBTYPES,
            RootMediaType::Haptics => &crate::haptics::SUBTYPES,
            RootMediaType::Image => &crate::image::SUBTYPES,
            RootMediaType::Message => &crate::message::SUBTYPES,
            RootMediaType::Model => &crate::model::SUBTYPES,
            RootMediaType::Multipart => &crate::multipart::SUBTYPES,
            RootMediaType::Text => &crate::text::SUBTYPES,
            RootMediaType::Video => &crate::video::SUBTYPES,
            RootMediaType::Star | RootMediaType::Example => {
                return Err(crate::errors::Error::UnknownSubMediaType(
                    root,
                    subtype_candidate.to_owned(),
                ));
            }
        };

        if !subtypes.contains(subtype_candidate) {
            return Err(crate::errors::Error::UnknownSubMediaType(
                root,
                subtype_candidate.to_owned(),
            ));
        }

        Ok(Self::Other(subtype_candidate.to_owned()))
    }
}

impl PartialEq<str> for SubMediaType {
    fn eq(&self, other: &str) -> bool {
        match self {
            Self::Star => other == "*",
            Self::Other(subtype) => subtype == other,
        }
    }
}

impl AsRef<str> for SubMediaType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Star => "*",
            Self::Other(subtype) => subtype,
        }
    }
}
