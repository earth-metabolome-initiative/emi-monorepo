//! Component for the form requiring user name and surname.

use crate::components::forms::*;
use crate::stores::user_state::UserState;
use wasm_bindgen::JsCast;
use web_common::api::auth::users::{CompleteProfile, ProfileImage};
use web_common::api::form_traits::TryFromCallback;
use web_common::{
    api::auth::users::name::{Name, ValidatedNameField},
    custom_validators::image,
};
use web_common::file_formats::GenericFileFormat;
use web_sys::FormData;
use yew::prelude::*;
use yewdux::prelude::*;

impl TryFromCallback<FormData> for FormWrapper<CompleteProfile> {
    fn try_from_callback<C>(data: FormData, callback: C) -> Result<(), Vec<String>>
    where
        C: FnOnce(Result<Self, Vec<String>>) + 'static,
    {
        let first_name = data
            .get("first_name")
            .as_string()
            .ok_or_else(|| vec!["The first name field is missing or not a string.".to_string()])?;
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
        let last_name = data
            .get("last_name")
            .as_string()
            .ok_or_else(|| vec!["The last name field is missing or not a string.".to_string()])?;

        let name: Name = Name::new(first_name, middle_name, last_name)?;

        let files: web_sys::FileList = data
            .get_all("profile_picture")
            .unchecked_into::<web_sys::FileList>();

        let file = match files.length() {
            0 => Err(vec!["File list is empty.".to_string()]),
            1 => match files.get(0) {
                Some(file) => Ok(file),
                None => Err(vec!["Impossible to index zeroth file.".to_string()]),
            },
            number => Err(vec![format!("Expected 1 file, but received {}.", number)]),
        }?;

        image::Image::try_from_callback(file, move |image| {
            match image {
                Ok(image) => {
                    match CompleteProfile::new(name.clone(), image) {
                        Ok(form) => {
                            callback(Ok(FormWrapper::from(form)));
                        }
                        Err(errors) => {
                            callback(Err(errors));
                        }
                    }
                },
                Err(errors) => {
                    callback(Err(errors));
                }
            }
        })?;

        Ok(())
    }
}

#[function_component(CompleteProfileForm)]
pub fn complete_profile_form() -> Html {
    let (user_state, _dispatch) = use_store::<UserState>();

    if user_state.has_no_access_token() {
        unreachable!("This component should only be rendered when the user is logged in.");
    }

    if let Some(user) = user_state.user() {
        let name = user.name().unwrap_or_else(|_| Default::default());

        let (first_name, middle_name, last_name) = name.clone().scompose();

        let middle_name = middle_name.unwrap_or_default();

        let profile_picture_url = user.profile_picture_url();

        html! {
            <BasicForm<CompleteProfile>>
                <FileInput<ProfileImage>
                    label="Profile picture"
                    maximal_size={5*1024_u64.pow(2)}
                    urls={profile_picture_url.map_or_else(|| vec![], |url| vec![url])}
                    allowed_formats={vec![GenericFileFormat::Image]}
                />
                <ul class="name-wrapper input-group">
                    <li><BasicInput<ValidatedNameField> label="First name" value={first_name} input_type="text" /></li>
                    <li><BasicInput<ValidatedNameField> label="Middle name" value={middle_name} optional={true} input_type="text" /></li>
                    <li><BasicInput<ValidatedNameField> label="Last name" value={last_name} input_type="text" /></li>
                </ul>
            </BasicForm<CompleteProfile>>
        }
    } else {
        html! {
            <div>{"Loading..."}</div>
        }
    }
}
