use std::fmt::Debug;

pub trait Version: Default {
    const VERSION: usize;
    type Parameters: ToString + Debug;
}
#[derive(Default)]
pub struct Version5;

impl Version for Version5 {
    const VERSION: usize = 5;
    type Parameters = crate::sirius_parameters::SiriusParametersVersion5;
}
