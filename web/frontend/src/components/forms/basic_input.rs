//! Module providing a yew component that handles a basic input, which is meant to be used in combination with BasicForm.

use serde::Deserialize;
use yew::prelude::*;
use validator::{Validate, ValidationError};

pub trait Input: Properties + Validate {
    fn input_id(&self) -> String;
    fn input_type(&self) -> String;
    fn input_placeholder(&self) -> String;
    fn input_label(&self) -> String;
    fn input_value(&self) -> String;
    fn input_oninput(&self) -> Callback<InputData>;
    fn input_error(&self) -> Option<ValidationError>;
}