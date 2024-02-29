// A trait that the Validate derive will impl
use yew::prelude::*;
use validator::{Validate, ValidationErrors};
use serde::{Deserialize, Serialize};
use super::Input;

#[derive(Validate, Debug, Clone, PartialEq, Properties, Serialize, Deserialize)]
pub struct NonEmptyTextField {
    #[validate(length(min = 1, message="This field cannot be empty"))]
    value: String,
}

impl Input for NonEmptyTextField {
    const TYPE: &'static str = "text";
    type DataType = String;    

    fn value(&self) -> String {
        self.value.clone()
    }

    fn set(&mut self, value: String) -> Result<(), ValidationErrors> {
        self.value = value;
        self.validate()
    }
}

impl From<String> for NonEmptyTextField {
    fn from(value: String) -> Self {
        NonEmptyTextField { value }
    }
}

impl Into<String> for NonEmptyTextField {
    fn into(self) -> String {
        self.value
    }
}