//! Submodule implementing the `TryFrom<&str>` trait for
//! [`FAIcon`](crate::FAIcon).

use std::str::FromStr;

use crate::FAIcon;

impl TryFrom<&str> for FAIcon {
    type Error = crate::errors::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        FAIcon::from_str(value)
    }
}
