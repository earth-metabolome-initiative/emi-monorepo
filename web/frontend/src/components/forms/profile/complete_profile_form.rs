//! Component for the form requiring user name and surname.

use std::rc::Rc;

use crate::components::forms::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use web_common::custom_validators::Image;
use web_common::database::updates::update_profile::{ProfileImage, ValidatedNameField};
use web_common::database::updates::CompleteProfile;
use web_common::file_formats::GenericFileFormat;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Debug, PartialEq, Clone, Default, Store, Serialize, Deserialize)]
#[store(storage = "session")]
pub struct CompleteProfileBuilder {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub picture: Option<Image>,
}

pub enum CompleteProfileBuilderMessage {
    SetFirstName(String),
    SetMiddleName(String),
    SetLastName(String),
    SetPicture(Option<Image>),
}

impl FormBuilder for CompleteProfileBuilder {
    type Data = CompleteProfile;
    type Actions = CompleteProfileBuilderMessage;

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }

    fn buildable(&self) -> Result<(), web_common::api::ApiError> {
        if self.first_name.is_none() {
            return Err(web_common::api::ApiError::BadRequest(vec![
                "First name is required.".to_string(),
            ]));
        }
        if self.last_name.is_none() {
            return Err(web_common::api::ApiError::BadRequest(vec![
                "Last name is required.".to_string(),
            ]));
        }
        if self.picture.is_none() {
            return Err(web_common::api::ApiError::BadRequest(vec![
                "Profile picture is required.".to_string(),
            ]));
        }

        CompleteProfile::new(
            self.first_name.clone().unwrap(),
            self.middle_name.clone(),
            self.last_name.clone().unwrap(),
            self.picture.clone().unwrap(),
        )
        .map(|_| ())
        .map_err(|errors| web_common::api::ApiError::BadRequest(errors))
    }

    fn build(&self) -> Self::Data {
        CompleteProfile::new(
            self.first_name.clone().unwrap(),
            self.middle_name.clone(),
            self.last_name.clone().unwrap(),
            self.picture.clone().unwrap(),
        )
        .unwrap()
    }
}

impl Reducer<CompleteProfileBuilder> for CompleteProfileBuilderMessage {
    fn apply(
        self,
        mut state: std::rc::Rc<CompleteProfileBuilder>,
    ) -> std::rc::Rc<CompleteProfileBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            CompleteProfileBuilderMessage::SetFirstName(first_name) => {
                state_mut.first_name = Some(first_name);
            }
            CompleteProfileBuilderMessage::SetMiddleName(middle_name) => {
                state_mut.middle_name = Some(middle_name);
            }
            CompleteProfileBuilderMessage::SetLastName(last_name) => {
                state_mut.last_name = Some(last_name);
            }
            CompleteProfileBuilderMessage::SetPicture(picture) => {
                state_mut.picture = picture;
            }
        }

        state
    }
}

impl FormBuildable for CompleteProfile {
    type Builder = CompleteProfileBuilder;
    const METHOD: web_common::api::form_traits::FormMethod =
        web_common::api::form_traits::FormMethod::PUT;

    fn title() -> &'static str {
        "Complete Profile"
    }

    fn description() -> &'static str {
        concat!(
            "Hello and welcome to the Earth Metabolome Initiative! ",
            "As a new user, we need you to complete your profile. ",
            "Please provide your given name and profile picture."
        )
    }

    fn requires_authentication() -> bool {
        true
    }

    fn task_target() -> &'static str {
        "Profile"
    }
}

// impl TryFromCallback<FormData> for FormWrapper<CompleteProfile> {
//     fn try_from_callback<C>(data: FormData, callback: C) -> Result<(), Vec<String>>
//     where
//         C: FnOnce(Result<Self, Vec<String>>) + 'static,
//     {
//         let first_name = data
//             .get("first_name")
//             .as_string()
//             .ok_or_else(|| vec!["The first name field is missing or not a string.".to_string()])?;
//         let middle_name = data
//             .get("middle_name")
//             .as_string()
//             .map(|string| {
//                 if string.is_empty() {
//                     None
//                 } else {
//                     Some(string)
//                 }
//             })
//             .flatten();
//         let last_name = data
//             .get("last_name")
//             .as_string()
//             .ok_or_else(|| vec!["The last name field is missing or not a string.".to_string()])?;

//         let files: web_sys::FileList = data
//             .get_all("profile_picture")
//             .unchecked_into::<web_sys::FileList>();

//         let file = match files.length() {
//             0 => Err(vec!["File list is empty.".to_string()]),
//             1 => match files.get(0) {
//                 Some(file) => Ok(file),
//                 None => Err(vec!["Impossible to index zeroth file.".to_string()]),
//             },
//             number => Err(vec![format!("Expected 1 file, but received {}.", number)]),
//         }?;

//         image::Image::try_from_callback(file, move |image| match image {
//             Ok(image) => match CompleteProfile::new(first_name, middle_name, last_name, image) {
//                 Ok(form) => {
//                     callback(Ok(FormWrapper::from(form)));
//                 }
//                 Err(errors) => {
//                     callback(Err(errors));
//                 }
//             },
//             Err(errors) => {
//                 callback(Err(errors));
//             }
//         })?;

//         Ok(())
//     }
// }

#[function_component(CompleteProfileForm)]
pub fn complete_profile_form() -> Html {
    // The use_reducer hook takes an initialization function which will be called only once.
    let (store, dispatch) = use_store::<CompleteProfileBuilder>();

    let set_first_name = dispatch.apply_callback(|first_name: ValidatedNameField| {
        CompleteProfileBuilderMessage::SetFirstName(first_name.to_string())
    });

    let set_middle_name = dispatch.apply_callback(|middle_name: ValidatedNameField| {
        CompleteProfileBuilderMessage::SetMiddleName(middle_name.to_string())
    });

    let set_last_name = dispatch.apply_callback(|last_name: ValidatedNameField| {
        CompleteProfileBuilderMessage::SetLastName(last_name.to_string())
    });

    let set_picture = dispatch.apply_callback(|mut picture: Vec<ProfileImage>| {
        CompleteProfileBuilderMessage::SetPicture(picture.pop().map(|image| image.into()))
    });

    html! {
    <BasicForm<CompleteProfile>  builder={store.deref().clone()}>
        <FileInput<ProfileImage>
            label="Profile picture"
            builder={set_picture}
            maximal_size={5*1024_u64.pow(2)}
            allowed_formats={vec![GenericFileFormat::Image]}
        />
        <ul class="name-wrapper input-group">
            <li><BasicInput<ValidatedNameField> builder={set_first_name} show_label={false} label="First name" input_type={InputType::Text} /></li>
            <li><BasicInput<ValidatedNameField> builder={set_middle_name} show_label={false} label="Middle name" optional={true} input_type={InputType::Text} /></li>
            <li><BasicInput<ValidatedNameField> builder={set_last_name} show_label={false} label="Last name" input_type={InputType::Text} /></li>
        </ul>
    </BasicForm<CompleteProfile>>
    }
}
