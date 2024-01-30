use crate::sirius_config::SiriusConfig;
use crate::versions::Version;
use dotenvy::dotenv;
use std::env;
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
        let sirius_path = env::var("SIRIUS_PATH").map_err(|_| {
            format!(concat!(
                "The environment variable SIRIUS_PATH is not set. ",
                "We expected there to exist a .env file in the current directory ",
                "with the SIRIUS_PATH variable set to the path of the sirius executable. ",
                "The variable may also be set in the environment directly, for instance ",
                "in the .bashrc file."
            ))
        })?;

        // Fetch the SIRIUS_USERNAME and the SIRIUS_PASSWORD from environment variables
        // in order to login before launching the sirius command

        let sirius_username = env::var("SIRIUS_USERNAME").map_err(|_| {
            format!(concat!(
                "The environment variable SIRIUS_USERNAME is not set. ",
                "We expected there to exist a .env file in the current directory ",
                "with the SIRIUS_USERNAME variable set to the username of the sirius account. ",
                "The variable may also be set in the environment directly, for instance ",
                "in the .bashrc file."
            ))
        })?;

        let sirius_password = env::var("SIRIUS_PASSWORD").map_err(|_| {
            format!(concat!(
                "The environment variable SIRIUS_PASSWORD is not set. ",
                "We expected there to exist a .env file in the current directory ",
                "with the SIRIUS_PASSWORD variable set to the password of the sirius account. ",
                "The variable may also be set in the environment directly, for instance ",
                "in the .bashrc file."
            ))
        })?;

        // Prepare and execute the login command
        let mut binding = Command::new(&sirius_path);

        // Set both SIRIUS_PASSWORD and SIRIUS_USERNAME environment variables
        binding
            .env("SIRIUS_USERNAME", &sirius_username)
            .env("SIRIUS_PASSWORD", &sirius_password);

        let login_command_status = binding
            .args(&[
                "login",
                "--user-env",
                "SIRIUS_USERNAME",
                "--password-env",
                "SIRIUS_PASSWORD",
            ])
            .status()
            .map_err(|e| format!("Failed to execute Sirius login command: {}", e))?;

        // We make sure to print the login command status for debugging

        println!("Sirius login command status: {:#?}", login_command_status);

        // if !login_command_status.success() {
        //     return Err("Sirius login command failed".to_string());
        // }

        // Prepare the command
        let mut command = Command::new(sirius_path);

        // Start building the argument list
        let mut args = Vec::new();

        // Add input and output file paths with their respective flags
        args.push("-i".to_string());
        args.push(input_file_path.to_str().unwrap().to_string());
        args.push("--output".to_string());
        args.push(output_file_path.to_str().unwrap().to_string());

        // Add arguments from config directly
        args.extend(self.config.args().iter().cloned());

        // Add specific command arguments
        args.extend(vec!["write-summaries"].iter().map(|&s| s.to_string()));

        // Print the command and its arguments for debugging
        println!("Running command: sirius {:?}", args);

        // Add arguments and spawn the command
        command.args(&args).spawn()
            .expect("Sirius failed to start");

        Ok(())
    }
}
