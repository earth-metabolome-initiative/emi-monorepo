use crate::sirius_config::SiriusConfig;
use std::path::Path;
use std::process::Command;

pub struct Sirius {
    config: SiriusConfig,
}

impl From<SiriusConfig> for Sirius {
    fn from(config: SiriusConfig) -> Self {
        Sirius { config }
    }
}

impl Sirius {
    pub fn run(&self, input_file_path: &Path, output_file_path: &Path) -> Result<(), String> {
        Command::new("sirius")
            .args(self.config.args())
            .spawn()
            .expect("Sirius failed to start");
        Ok(())
    }
}
