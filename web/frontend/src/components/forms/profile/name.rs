//! Component for the form requiring user name and surname.

use crate::components::forms::basic_form::FormMethod;
use serde::{Deserialize, Serialize};
use validator::Validate;
use yew::prelude::*;

use super::super::inputs::{BasicInput, DynamicLabel, NonEmptyTextField};
use super::super::Form;

#[derive(PartialEq, Clone, Debug, Properties, Validate, Serialize, Deserialize)]
pub struct Name {
    pub first_name: String,
    pub last_name: String,
}

impl Form for Name {
    fn action(&self) -> String {
        "/api/user".to_string()
    }

    fn method(&self) -> FormMethod {
        FormMethod::update()
    }

    fn title(&self) -> String {
        "Name".to_string()
    }

    fn inputs(&self) -> Html {
        let first_name_input = NonEmptyTextField::from(self.first_name.clone());
        let last_name_input = NonEmptyTextField::from(self.last_name.clone());

        html! {
            <>
                <BasicInput<DynamicLabel<NonEmptyTextField>> label="First name" input={first_name_input} />
                <BasicInput<DynamicLabel<NonEmptyTextField>> label="Last name" input={last_name_input} />
            </>
        }
    }
}
