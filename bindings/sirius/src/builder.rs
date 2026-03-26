//! A builder is a type of struct that will collect configurations and once build, prodiuces a complete struct.
//!
use crate::prelude::*;
use crate::sirius_config::SiriusConfig;

mod version5;
mod version6;

/// The SiriusBuilder is used to set the parameters of the SiriusConfig.
#[derive(Default)]
pub struct SiriusBuilder<V: Version> {
    config: SiriusConfig<V>,
}

impl<V: Version> SiriusBuilder<V> {
    /// Build the Sirius instance from the configuration.
    /// # Example
    /// ```
    /// use sirius::prelude::*;
    /// let sirius = SiriusBuilder::<Version6>::default().build();
    /// ```
    pub fn build(self) -> Sirius<V> {
        Sirius::from(self.config)
    }
}
