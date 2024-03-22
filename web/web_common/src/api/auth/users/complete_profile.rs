//! Submodule providing the version of the User Profile to be used with the CompleteProfile form.
use crate::{api::form_traits::FormResult, custom_validators::*};
use serde::{Deserialize, Serialize};
use validator::Validate;

pub type ProfileImage = Squarish<ContainsOneFace<Image>>;

#[derive(PartialEq, Clone, Debug, Validate, Serialize, Deserialize, Eq)]
pub struct CompleteProfile {
    #[validate]
    name: super::Name,
    #[validate]
    profile_picture: ProfileImage
}

impl CompleteProfile {
    pub fn new(name: super::Name, profile_picture: Image) -> Result<Self, Vec<String>> {
        Ok(Self { name, profile_picture: profile_picture.try_into()? })
    }
}

impl FormResult for CompleteProfile {
    const METHOD: crate::api::form_traits::FormMethod = crate::api::form_traits::FormMethod::PUT;

    fn title() -> &'static str {
        "Complete Profile"
    }

    fn task_target() -> &'static str {
        "Profile"
    }

    fn description() -> &'static str {
        concat!(
            "Hello and welcome to the Earth Metabolome Initiative! ",
            "As a new user, we need you to complete your profile. ",
            "Please provide your given name and profile piture."
        )
    }

    fn requires_authentication() -> bool {
        true
    }
}
