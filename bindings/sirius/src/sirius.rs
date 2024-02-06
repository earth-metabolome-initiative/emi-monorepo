use crate::sirius_config::SiriusConfig;
use crate::versions::Version;
use dotenvy::dotenv;
use std::env;
use std::path::Path;
use std::process::Command;

/// The main struct for the Sirius bindings
pub struct Sirius<V: Version> {
    config: SiriusConfig<V>,
}

impl<V: Version> From<SiriusConfig<V>> for Sirius<V> {
    fn from(config: SiriusConfig<V>) -> Self {
        Sirius { config }
    }
}

impl<V: Version> Sirius<V> {
    /// Run the sirius command with the given input and output file paths.
    ///
    /// The sirius executable is expected to be available in the environment variable SIRIUS_PATH.
    /// The username and password for the sirius account are expected to be available in the environment variables SIRIUS_USERNAME and SIRIUS_PASSWORD.
    ///
    /// This function gets the parameters that where set in the SiriusBuilder struct and runs the sirius command with the given input and output file paths.
    ///
    /// # Arguments
    /// * `input_file_path` - The path to the input file
    /// * `output_file_path` - The path to the output file
    pub fn run(&self, input_file_path: &Path, output_file_path: &Path) -> Result<(), String> {
        // Load environment variables from .env file
        dotenv().ok();

        // Fetch the path of the sirius command from environment variables
        let sirius_path = env::var("SIRIUS_PATH").map_err(|_| {
            concat!(
                "The environment variable SIRIUS_PATH is not set. ",
                "We expected there to exist a .env file in the current directory ",
                "with the SIRIUS_PATH variable set to the path of the sirius executable. ",
                "The variable may also be set in the environment directly, for instance ",
                "in the .bashrc file."
            )
            .to_string()
        })?;

        // Fetch the SIRIUS_USERNAME and the SIRIUS_PASSWORD from environment variables
        // in order to login before launching the sirius command

        let sirius_username = env::var("SIRIUS_USERNAME").map_err(|_| {
            concat!(
                "The environment variable SIRIUS_USERNAME is not set. ",
                "We expected there to exist a .env file in the current directory ",
                "with the SIRIUS_USERNAME variable set to the username of the sirius account. ",
                "The variable may also be set in the environment directly, for instance ",
                "in the .bashrc file."
            )
            .to_string()
        })?;

        let sirius_password = env::var("SIRIUS_PASSWORD").map_err(|_| {
            concat!(
                "The environment variable SIRIUS_PASSWORD is not set. ",
                "We expected there to exist a .env file in the current directory ",
                "with the SIRIUS_PASSWORD variable set to the password of the sirius account. ",
                "The variable may also be set in the environment directly, for instance ",
                "in the .bashrc file."
            )
            .to_string()
        })?;

        // Prepare and execute the login command
        let mut binding = Command::new(&sirius_path);

        // Set both SIRIUS_PASSWORD and SIRIUS_USERNAME environment variables
        binding
            .env("SIRIUS_USERNAME", &sirius_username)
            .env("SIRIUS_PASSWORD", &sirius_password);

        let login_command_status = binding
            .args([
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

        if !login_command_status.success() {
            return Err("Sirius login command failed".to_string());
        }

        // Prepare the command
        let mut command = Command::new(sirius_path);

        // Start building the argument list
        // Add input and output file paths with their respective flags
        let mut args = vec![
            "-i".to_string(),
            input_file_path.to_str().unwrap().to_string(),
            "--output".to_string(),
            output_file_path.to_str().unwrap().to_string(),
        ];

        // Add arguments from config directly
        args.extend(self.config.args().iter().cloned());

        // Print the command and its arguments for debugging
        println!("Running command: sirius {:?}", args);

        // Add arguments and spawn the command
        let mut child = command.args(&args).spawn().expect("Sirius failed to start");
        let status = child.wait().expect("Failed to wait on child");

        if !status.success() {
            return Err("Sirius failed".to_string());
        }

        Ok(())
    }
}
