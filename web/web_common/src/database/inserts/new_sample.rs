use crate::{custom_validators::*, database::SampleState};
use crate::database::{Insert, NestedPublicUser, SamplingProcedure, Taxa, User};
use serde::{Deserialize, Serialize};
use validator::Validate;

pub type NewSampleName = NoSpecialCharacters<MustBeCapitalized<NotEmpty>>;

#[derive(Debug, Clone, PartialEq, Eq, Validate, Serialize, Deserialize)]
pub struct NewSample {
    #[validate]
    pub name: NewSampleName,
    #[validate]
    pub description: NotEmpty,
    pub public: bool,
    pub sample_state: SampleState,
    pub collector: NestedPublicUser,
    pub sampling_procedure: SamplingProcedure,
    pub taxa: Vec<Taxa>,
    // pub parent_sample_id: Option<Uuid>,
}

impl NewSample {
    pub fn new(
        name: String,
        description: String,
        public: bool,
        sample_state: SampleState,
        collector: NestedPublicUser,
        sampling_procedure: SamplingProcedure,
        taxa: Vec<Taxa>
    ) -> Result<Self, Vec<String>> {
        let new_sample = Self {
            name: NewSampleName::try_from(name)?,
            description: NotEmpty::try_from(description)?,
            public,
            sample_state,
            collector,
            sampling_procedure,
            taxa
        };

        Ok(new_sample)
    }
}

impl From<NewSample> for Insert {
    fn from(sample: NewSample) -> Self {
        Insert::Sample(sample)
    }
}

impl From<NewSample> for crate::database::Operation {
    fn from(sample: NewSample) -> Self {
        Insert::from(sample).into()
    }
}

impl From<NewSample> for crate::database::Task {
    fn from(sample: NewSample) -> Self {
        crate::database::Operation::from(sample).into()
    }
}

impl From<NewSample> for crate::api::ws::messages::FrontendMessage {
    fn from(sample: NewSample) -> Self {
        crate::api::ws::messages::FrontendMessage::Task(sample.into())
    }
}
