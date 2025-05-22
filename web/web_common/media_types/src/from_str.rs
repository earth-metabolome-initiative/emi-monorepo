//! Submodule implementing the `FromStr` trait for the `MediaType` struct.

use std::str::FromStr;

use crate::{MediaType, SubMediaType};

impl FromStr for MediaType {
    type Err = crate::errors::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // We split at the first `;` to separate the media type from the parameters.
        let mut parts = s.splitn(2, ';');
        let unparsed_types =
            parts.next().ok_or_else(|| crate::errors::Error::UnknownMediaType(s.to_string()))?;
        let unparsed_params = parts.next().unwrap_or("");

        // Next, we split the media type into its root and subtype.
        let mut types = unparsed_types.splitn(2, '/');
        let root_type =
            types.next().ok_or_else(|| crate::errors::Error::UnknownMediaType(s.to_string()))?;
        let root_media_type = crate::RootMediaType::from_str(root_type)?;

        let subtype =
            types.next().ok_or_else(|| crate::errors::Error::UnknownMediaType(s.to_string()))?;
        let subtype = SubMediaType::from_str_with_root(root_media_type, subtype)?;

        // Finally, we proceed to parse the parameters.

        let mut params = std::collections::HashMap::new();

        for param in unparsed_params.split(';') {
            let mut kv = param.splitn(2, '=');
            let key =
                kv.next().ok_or_else(|| crate::errors::Error::UnknownMediaType(s.to_owned()))?;
            let value = kv.next().unwrap_or("");

            // We trim the key and value to remove any leading or trailing whitespace.
            let key = key.trim().to_lowercase();
            let value = value.trim();

            // We insert the key-value pair into the parameters map.
            params.insert(key, value.to_owned());
        }

        // We return the constructed MediaType instance.
        Ok(MediaType { root: root_media_type, subtype, params: params.into_iter().collect() })
    }
}
