//! Module providing a yew component that handles a basic form.

use serde::Deserialize;
use validator::{Validate, ValidationError};
use yew::prelude::*;
use yewdux::prelude::*;

pub trait Form: Properties + Validate {
    
} 

#[function_component(BasicForm)]
pub fn basic_form<F: Form>(form: F) -> Html {

}
