//! Submodule providing the version of the User Profile to be used with the CompleteProfile form.
use super::Update;
use crate::api::ApiError;
use crate::{
    api::{
        database::{operations::Operation, Id},
        form_traits::FormResult,
    },
    custom_validators::*,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

pub type ValidatedNameField = NoSpecialCharacters<MustBeCapitalized<NoDigits<NotEmpty>>>;
pub type ProfileImage = MinShape<Squarish<ContainsOneFace<Image>>, STANDARD_IMAGE_SIZE, STANDARD_IMAGE_SIZE>;

#[derive(PartialEq, Clone, Debug, Validate, Serialize, Deserialize, Eq)]
pub struct CompleteProfile {
    #[validate]
    pub first_name: ValidatedNameField,
    #[validate]
    pub middle_name: Option<ValidatedNameField>,
    #[validate]
    pub last_name: ValidatedNameField,
    #[validate]
    pub picture: ProfileImage,
}

impl CompleteProfile {
    pub fn new(
        first_name: String,
        middle_name: Option<String>,
        last_name: String,
        picture: Image,
    ) -> Result<Self, Vec<String>> {
        let profile = CompleteProfile {
            first_name: first_name.try_into()?,
            middle_name: middle_name
                .map(|middle_name| middle_name.try_into())
                .transpose()?,
            last_name: last_name.try_into()?,
            picture: picture.try_into()?,
        };

        Ok(profile)
    }

    pub fn id(&self) -> Id {
        Id::LoggedInUser
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

impl From<CompleteProfile> for Operation {
    fn from(update_profile: CompleteProfile) -> Self {
        Operation::Update(Update::Profile(update_profile))
    }
}
