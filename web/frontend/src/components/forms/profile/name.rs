//! Component for the form requiring user name and surname.

use crate::components::forms::*;
use crate::stores::user_state::UserState;
use wasm_bindgen::UnwrapThrowExt;
use web_common::api::{auth::users::name::Name, ws::messages::FormAction, auth::users::name::ValidatedNameField};
use web_sys::FormData;
use yew::prelude::*;
use yewdux::prelude::*;

impl TryFrom<FormData> for FormWrapper<Name> {
    type Error = Vec<String>;

    /// Convert a set of data collected from a form into a `Name` object.
    ///
    /// The names of the fields are expected to be based on the provided labels,
    /// converted to snake case (e.g. "First name" -> "first_name").
    fn try_from(data: FormData) -> Result<Self, Self::Error> {
        let first_name = data.get("first_name").as_string().unwrap_throw();
        let middle_name = data
            .get("middle_name")
            .as_string()
            .map(|string| {
                if string.is_empty() {
                    None
                } else {
                    Some(string)
                }
            })
            .flatten();
        let last_name = data.get("last_name").as_string().unwrap_throw();

        let name: Name = Name::new(first_name, middle_name, last_name)?;

        Ok(FormWrapper::from(name))
    }
}

#[function_component(NameForm)]
pub fn name_form() -> Html {
    let (user_state, _dispatch) = use_store::<UserState>();

    if user_state.has_no_access_token() {
        unreachable!("This component should only be rendered when the user is logged in.");
    }

    if let Some(user) = user_state.user() {
        let name = user
            .name().unwrap_or_else(|_| Default::default());

        let (first_name, middle_name, last_name) = name.clone().scompose();

        let middle_name = middle_name.unwrap_or_default();

        html! {
            <BasicForm<Name> action={FormAction::UpdateName} current={name}>
                <FileInput label="Profile picture" />
                <BasicInput<ValidatedNameField> label="First name" value={first_name} input_type="text" />
                <BasicInput<ValidatedNameField> label="Middle name" value={middle_name} optional={true} input_type="text" />
                <BasicInput<ValidatedNameField> label="Last name" value={last_name} input_type="text" />
            </BasicForm<Name>>
        }
    } else {
        html! {
            <div>{"Loading..."}</div>
        }
    }
}
