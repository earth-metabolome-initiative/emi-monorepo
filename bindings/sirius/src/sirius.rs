use crate::sirius_config::SiriusConfig;
use crate::versions::Version;
use std::path::Path;
use std::process::Command;

pub struct Sirius<V: Version> {
    config: SiriusConfig<V>,
}

impl<V: Version> From<SiriusConfig<V>> for Sirius<V> {
    fn from(config: SiriusConfig<V>) -> Self {
        Sirius { config }
    }
}

impl<V: Version> Sirius<V> {
    pub fn run(&self, input_file_path: &Path, output_file_path: &Path) -> Result<(), String> {
        Command::new("sirius")
            .args(self.config.args())
            .spawn()
            .expect("Sirius failed to start");
        Ok(())
    }
}
