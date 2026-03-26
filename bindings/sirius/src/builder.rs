//! A builder is a type of struct that will collect configurations and once build, prodiuces a complete struct.
//!
use crate::prelude::*;
use crate::sirius_config::SiriusConfig;

pub mod version5;

/// The SiriusBuilder is used to set the parameters of the SiriusConfig.
#[derive(Default)]
pub struct SiriusBuilder<V: Version> {
    config: SiriusConfig<V>,
}
