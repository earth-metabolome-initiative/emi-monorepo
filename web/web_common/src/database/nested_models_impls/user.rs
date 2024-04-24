//! Method to build the first name, middle name and last name for a User

use crate::database::NestedPublicUser;
use std::fmt::Display;

impl NestedPublicUser {
    pub fn full_name(&self) -> String {
        let mut full_name = self.inner.first_name.clone();
        if let Some(middle_name) = &self.inner.middle_name {
            full_name.push_str(" ");
            full_name.push_str(middle_name);
        }
        full_name.push_str(" ");
        full_name.push_str(&self.inner.last_name);
        full_name
    }

    pub fn has_complete_profile(&self) -> bool {
        !self.inner.first_name.is_empty()
            && !self.inner.last_name.is_empty()
            && self.thumbnail.is_some()
            && self.picture.is_some()
    }
}

impl Display for NestedPublicUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_name())
    }
}
