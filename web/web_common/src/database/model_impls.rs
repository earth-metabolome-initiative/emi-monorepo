use super::*;

impl User {
    /// Returns whether the user has a complete profile.
    pub fn has_complete_profile(&self) -> bool {
        !self.first_name.is_empty()
            && !self.last_name.is_empty()
    }

    /// Get the full name of the user.
    pub fn full_name(&self) -> String {
        if let Some(middle_name) = &self.middle_name {
            format!("{} {} {}", self.first_name, middle_name, self.last_name)
        } else {
            format!("{} {}", self.first_name, self.last_name)
        }
    }
}
