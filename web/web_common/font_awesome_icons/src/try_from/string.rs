//! Submodule implementing the `TryFrom<String>` trait for
//! [`FAIcon`](crate::FAIcon).

use std::str::FromStr;

use crate::FAIcon;

impl TryFrom<String> for FAIcon {
    type Error = crate::errors::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        FAIcon::from_str(&value)
    }
}
