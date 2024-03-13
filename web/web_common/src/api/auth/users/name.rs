use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::combine_path;

pub const ENDPOINT: &str = "/name";
pub const FULL_ENDPOINT: &str = combine_path!(super::FULL_ENDPOINT, ENDPOINT);

#[derive(PartialEq, Clone, Debug, Default, Validate, Serialize, Deserialize)]
pub struct Name {
    #[validate(length(min = 1, message = "First name is required."))]
    first_name: String,
    #[validate(length(min = 1, message = "Middle name is required."))]
    middle_name: Option<String>,
    #[validate(length(min = 1, message = "Last name is required."))]
    last_name: String,
}

impl Name {
    pub fn new(first_name: String, middle_name: Option<String>, last_name: String) -> Name {
        Name {
            first_name,
            middle_name,
            last_name,
        }
    }

    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }

    pub fn last_name(&self) -> String {
        self.last_name.clone()
    }

    pub fn middle_name(&self) -> Option<String> {
        self.middle_name.clone()
    }

    pub(crate) fn is_complete(&self) -> bool {
        !self.first_name.is_empty() && !self.last_name.is_empty()
    }

    pub(crate) fn full_name(&self) -> Result<String, String> {
        if self.is_complete() {
            if let Some(middle_name) = &self.middle_name {
                Ok(format!(
                    "{} {} {}",
                    self.first_name, middle_name, self.last_name
                ))
            } else {
                Ok(format!("{} {}", self.first_name, self.last_name))
            }
        } else {
            Err("Name is not complete.".to_string())
        }
    }
}
