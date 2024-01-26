use crate::sirius_config::SiriusConfig;
use crate::versions::Version;
use std::path::Path;
use std::process::Command;
use dotenvy::dotenv;
use std::env;

pub struct Sirius<V: Version> {
    config: SiriusConfig<V>,
}

impl<V: Version> From<SiriusConfig<V>> for Sirius<V> {
    fn from(config: SiriusConfig<V>) -> Self {
        Sirius { config }
    }
}

// impl<V: Version> Sirius<V> {
//     pub fn run(&self, input_file_path: &Path, output_file_path: &Path) -> Result<(), String> {
//         Command::new("sirius")
//             .args(self.config.args())
//             .spawn()
//             .expect("Sirius failed to start");
//         Ok(())
//     }
// }


impl<V: Version> Sirius<V> {
    pub fn run(&self, input_file_path: &Path, output_file_path: &Path) -> Result<(), String> {
        // Load environment variables from .env file
        dotenv().ok();

        // Fetch the path of the sirius command from environment variables
        let sirius_path = env::var("SIRIUS_PATH")
            .expect("SIRIUS_PATH environment variable not found");

        // Prepare the command
        let mut command = Command::new(sirius_path);
        
        
        // Start building the argument list
        let mut args = Vec::new();

        // Add input and output file paths with their respective flags
        args.push("-i".to_string());
        args.push(input_file_path.to_str().unwrap().to_string());
        args.push("--output".to_string());
        args.push(output_file_path.to_str().unwrap().to_string());

        // Assuming --maxmz=1000 is part of the config and should come before 'config'
        let config_args = self.config.args();
        // We print the config arguments for debugging
        println!("Config arguments: {:?}", config_args);
        if let Some(pos) = config_args.iter().position(|arg| arg == "--maxmz=1000") {
            args.push(config_args[pos].clone());
        }

        // Add the 'config' keyword
        args.push("config".to_string());

        // Add the remaining config arguments after 'config'
        for arg in config_args.into_iter().filter(|arg| arg != "--maxmz=1000") {
            args.push(arg);
        }

        // Add specific command arguments
        args.extend(vec!["formula", "zodiac", "fingerprint", "structure", "canopus"].iter().map(|&s| s.to_string()));


        // Print the command and its arguments for debugging
        println!("Running command: sirius {:?}", args);

        // Add arguments and spawn the command
        command.args(&args).spawn()
            .expect("Sirius failed to start");

        Ok(())
    }
}
