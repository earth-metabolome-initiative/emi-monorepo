//! Component for the form requiring user name and surname.

use yew::prelude::*;
use yewdux::prelude::*;
use validator::{Validate, ValidationError};
use serde::Deserialize;
use crate::stores::user_store::UserStore;


#[derive(Validate, PartialEq, Clone, Default, Debug, Deserialize, Properties)]
pub struct Name {
    #[validate(length(min = 1, message = "Please provide your name"))]
    name: String,
    #[validate(length(min = 1, message = "Please provide your surname"))]
    surname: String,
}
