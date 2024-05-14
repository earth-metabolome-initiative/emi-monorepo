use super::*;

impl User {
    pub fn full_name(&self) -> String {
        if let Some(middle_name) = &self.middle_name {
            format!("{} {} {}", self.first_name, middle_name, self.last_name)
        } else {
            format!("{} {}", self.first_name, self.last_name)
        }
    }
}