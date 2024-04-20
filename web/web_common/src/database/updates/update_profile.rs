//! Submodule providing the version of the User Profile to be used with the CompleteProfile form.
use super::Update;
use crate::{custom_validators::*};
use serde::{Deserialize, Serialize};
use validator::Validate;

pub type ValidatedNameField = NoSpecialCharacters<MustBeCapitalized<NoDigits<NotEmpty>>>;
pub type ProfileImage =
    MinShape<Squarish<ContainsOneFace<Image>>, STANDARD_IMAGE_SIZE, STANDARD_IMAGE_SIZE>;

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
}


impl From<CompleteProfile> for Update {
    fn from(update_profile: CompleteProfile) -> Self {
        Update::CompleteProfile(update_profile)
    }
}

impl From<CompleteProfile> for crate::database::Operation {
    fn from(update_profile: CompleteProfile) -> Self {
        Update::from(update_profile).into()
    }
}

impl From<CompleteProfile> for crate::database::Task {
    fn from(update_profile: CompleteProfile) -> Self {
        crate::database::Operation::from(update_profile).into()
    }
}

impl From<CompleteProfile> for crate::api::ws::messages::FrontendMessage {
    fn from(update_profile: CompleteProfile) -> Self {
        crate::api::ws::messages::FrontendMessage::Task(update_profile.into())
    }
}
