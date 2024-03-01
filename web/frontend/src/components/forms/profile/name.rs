//! Component for the form requiring user name and surname.

use crate::components::forms::*;
use crate::stores::user_state::UserState;
use validator::{Validate, ValidationErrors};
use wasm_bindgen::UnwrapThrowExt;
use web_common::api::auth::users::name::{Name, FULL_ENDPOINT};
use web_sys::FormData;
use yew::prelude::*;
use yewdux::prelude::*;


impl TryFrom<FormData> for FormWrapper<Name> {
    type Error = ValidationErrors;

    /// Convert a set of data collected from a form into a `Name` object.
    ///
    /// The names of the fields are expected to be based on the provided labels,
    /// converted to snake case (e.g. "First name" -> "first_name").
    fn try_from(data: FormData) -> Result<Self, ValidationErrors> {
        let first_name = data.get("first_name").as_string().unwrap_throw();
        let middle_name = data.get("middle_name").as_string().map(|string| if string.is_empty() { None } else { Some(string) }).flatten();
        let last_name = data.get("last_name").as_string().unwrap_throw();

        let name: Name = Name::new(first_name, middle_name, last_name);

        name.validate()?;

        Ok(FormWrapper::from(name))
    }
}

#[function_component(NameForm)]
pub fn name_form() -> Html {
    let (user_state, _dispatch) = use_store::<UserState>();

    if user_state.is_not_logged_in() {
        unreachable!("This component should only be rendered when the user is logged in.");
    }

    let user = user_state.user().unwrap_throw();

    let first_name = user.first_name();
    let middle_name = user.middle_name().unwrap_or_default();
    let last_name = user.last_name();

    html! {
        <BasicForm<Name> title="Name" method={FormMethod::update()} action={FULL_ENDPOINT}>
            <BasicInput label="First name" value={first_name} input_type="text" />
            <BasicInput label="Middle name" value={middle_name} input_type="text" />
            <BasicInput label="Last name" value={last_name} input_type="text" />
        </BasicForm<Name>>
    }
}
