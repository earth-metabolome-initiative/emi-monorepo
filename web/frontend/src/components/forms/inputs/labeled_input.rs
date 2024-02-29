use super::{Input, InputRef, Labeled};
use serde::{Deserialize, Serialize};
use validator::Validate;
use yew::prelude::*;

#[derive(Clone, Properties, Validate, PartialEq, Debug, Serialize, Deserialize)]
pub struct DynamicLabel<I>
where
    I: Validate + Clone + PartialEq + Properties
{
    pub label: String,
    #[validate]
    pub input: I,
}

impl<I> InputRef for DynamicLabel<I>
where
    I: Validate + Clone + PartialEq + Input,
{
    type InputType = I;

    fn input_ref(&self) -> &Self::InputType {
        &self.input
    }

    fn input_mut(&mut self) -> &mut Self::InputType {
        &mut self.input
    }
}

impl<I> Labeled for DynamicLabel<I>
where
    I: Validate + Clone + PartialEq + Input,
{
    fn label(&self) -> String {
        self.label.clone()
    }
}
