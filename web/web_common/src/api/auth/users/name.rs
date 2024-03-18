use crate::custom_validators::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::combine_path;

pub const ENDPOINT: &str = "/name";
pub const FULL_ENDPOINT: &str = combine_path!(super::FULL_ENDPOINT, ENDPOINT);

pub type ValidatedNameField = NoSlurs<NoSpecialCharacters<
    MustBeCapitalized<
        NoDigits<NoLeadingSpaces<NoTrailingSpaces<NoDoubleSpaces<NotEmpty<String>>>>>,
    >,
>>;

#[derive(PartialEq, Clone, Debug, Default, Validate, Serialize, Deserialize)]
pub struct Name {
    #[validate]
    first_name: ValidatedNameField,
    #[validate]
    middle_name: Option<ValidatedNameField>,
    #[validate]
    last_name: ValidatedNameField,
}

impl Name {
    pub fn new(
        first_name: String,
        middle_name: Option<String>,
        last_name: String,
    ) -> Result<Name, Vec<String>> {
        Ok(Name {
            first_name: first_name.try_into()?,
            middle_name: middle_name
                .map(|middle_name| middle_name.try_into())
                .transpose()?,
            last_name: last_name.try_into()?,
        })
    }

    pub fn scompose(
        self,
    ) -> (
        ValidatedNameField,
        Option<ValidatedNameField>,
        ValidatedNameField,
    ) {
        (self.first_name, self.middle_name, self.last_name)
    }

    pub fn first_name(&self) -> ValidatedNameField {
        self.first_name.clone()
    }

    pub fn last_name(&self) -> ValidatedNameField {
        self.last_name.clone()
    }

    pub fn middle_name(&self) -> Option<ValidatedNameField> {
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
