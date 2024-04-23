//! Method to build the first name, middle name and last name for a User

use crate::database::User;
use std::fmt::Display;

impl User {
    pub fn full_name(&self) -> String {
        let mut full_name = self.first_name.clone();
        if let Some(middle_name) = &self.middle_name {
            full_name.push_str(" ");
            full_name.push_str(middle_name);
        }
        full_name.push_str(" ");
        full_name.push_str(&self.last_name);
        full_name
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_name())
    }
}
