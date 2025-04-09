//! # Sirius
use std::{env, path::Path, process::Command};

use dotenvy::dotenv;
use is_executable::IsExecutable;

use crate::{sirius_config::SiriusConfig, versions::Version};

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
    #[allow(clippy::too_many_lines)]
    /// Run the sirius command with the given input and output file paths.
    ///
    /// The sirius executable is expected to be available in the environment
    /// variable `SIRIUS_PATH`. The username and password for the sirius
    /// account are expected to be available in the environment variables
    /// `SIRIUS_USERNAME` and `SIRIUS_PASSWORD`.
    ///
    /// This function gets the parameters that where set in the `SiriusBuilder`
    /// struct and runs the sirius command with the given input and output file
    /// paths.
    ///
    /// # Arguments
    /// * `input_file_path` - The path to the input file
    /// * `output_file_path` - The path to the output file
    ///
    /// # Errors
    /// Returns an error if:
    /// - The `SIRIUS_PATH` environment variable is not set
    /// - The `SIRIUS_USERNAME` environment variable is not set
    /// - The `SIRIUS_PASSWORD` environment variable is not set
    /// - The sirius path doesn't exist
    /// - The sirius path is not a file
    /// - The sirius executable is not executable
    /// - The sirius username or password is empty
    /// - The login command fails
    /// - The input file doesn't exist
    /// - The input file is not a file
    /// - The input file doesn't have the .mgf extension
    /// - The sirius command fails to execute
    pub fn run(&self, input_file_path: &Path, output_file_path: &Path) -> Result<(), String> {
        // Load environment variables from .env file
        dotenv().ok();

        // Fetch the path of the sirius command from environment variables
        let sirius_path = env::var("SIRIUS_PATH").map_err(|_| {
            concat!(
                "The environment variable `SIRIUS_PATH` is not set. ",
                "We expected there to exist a .env file in the current directory ",
                "with the `SIRIUS_PATH` variable set to the path of the sirius executable. ",
                "The variable may also be set in the environment directly, for instance ",
                "in the .bashrc file."
            )
            .to_string()
        })?;

        // We need to verify that the `SIRIUS_PATH` is a valid path to a file, and not a
        // directory.

        let sirius_path = Path::new(&sirius_path);

        if !sirius_path.exists() {
            return Err(format!("The sirius path {} does not exist", sirius_path.display()));
        }

        if !sirius_path.is_file() {
            return Err(format!("The sirius path {} is not a file", sirius_path.display()));
        }

        // We also need to check whether the file is executable, but this will be
        // different depending on the operating system. Fortunately, the
        // complexity of this is hidden behind the is_executable crate.
        if !sirius_path.is_executable() {
            return Err(format!(
                "The sirius executable at {} is not executable",
                sirius_path.display()
            ));
        }

        // Fetch the `SIRIUS_USERNAME` and the `SIRIUS_PASSWORD` from environment
        // variables in order to login before launching the sirius command

        let sirius_username = env::var("SIRIUS_USERNAME").map_err(|_| {
            concat!(
                "The environment variable `SIRIUS_USERNAME` is not set. ",
                "We expected there to exist a .env file in the current directory ",
                "with the `SIRIUS_USERNAME` variable set to the username of the sirius account. ",
                "The variable may also be set in the environment directly, for instance ",
                "in the .bashrc file."
            )
            .to_string()
        })?;

        let sirius_password = env::var("SIRIUS_PASSWORD").map_err(|_| {
            concat!(
                "The environment variable `SIRIUS_PASSWORD` is not set. ",
                "We expected there to exist a .env file in the current directory ",
                "with the `SIRIUS_PASSWORD` variable set to the password of the sirius account. ",
                "The variable may also be set in the environment directly, for instance ",
                "in the .bashrc file."
            )
            .to_string()
        })?;

        // We check that the provided sirius username and password are not empty
        if sirius_username.is_empty() {
            return Err(concat!(
                "The sirius username provided in the environment variable `SIRIUS_USERNAME` is empty. ",
                "We expected there to exist a .env file in the current directory ",
                "with the `SIRIUS_USERNAME` variable set to the username of the sirius account. ",
                "The variable may also be set in the environment directly, for instance ",
                "in the .bashrc file."
            ).to_string());
        }

        if sirius_password.is_empty() {
            return Err(concat!(
                "The sirius password provided in the environment variable `SIRIUS_PASSWORD` is empty. ",
                "We expected there to exist a .env file in the current directory ",
                "with the `SIRIUS_PASSWORD` variable set to the password of the sirius account. ",
                "The variable may also be set in the environment directly, for instance ",
                "in the .bashrc file."
            ).to_string());
        }

        // Prepare and execute the login command
        let mut binding = Command::new(sirius_path);

        // Set both `SIRIUS_PASSWORD` and `SIRIUS_USERNAME` environment variables
        binding.env("SIRIUS_USERNAME", &sirius_username).env("SIRIUS_PASSWORD", &sirius_password);

        let login_command_status = binding
            .args(["login", "--user-env", "SIRIUS_USERNAME", "--password-env", "SIRIUS_PASSWORD"])
            .status()
            .map_err(|e| format!("Failed to execute Sirius login command: {e}"))?;

        // We make sure to print the login command status for debugging

        if !login_command_status.success() {
            return Err("Sirius login command failed".to_string());
        }

        // We now check that the input file exists and is a file and not a directory
        if !input_file_path.exists() {
            return Err(format!("The input file {} does not exist", input_file_path.display()));
        }

        if !input_file_path.is_file() {
            return Err(format!("The input file {} is not a file", input_file_path.display()));
        }

        // We check that the extension of the input file is MGF, in either upper or
        // lower case
        let input_file_extension = input_file_path.extension().ok_or_else(|| {
            format!(
                "The input file {} does not have an extension. We expected the input file to have the extension .mgf",
                input_file_path.display()
            )
        })?;

        if input_file_extension.to_string_lossy().to_lowercase() != "mgf" {
            return Err(format!(
                "The input file {} does not have the extension .mgf",
                input_file_path.display()
            ));
        }

        // Prepare the command
        let mut command = Command::new(sirius_path);

        // Start building the argument list
        // Add input and output file paths with their respective flags
        let mut args = vec![
            "-i".to_string(),
            input_file_path.to_string_lossy().to_string(),
            "--output".to_string(),
            output_file_path.to_string_lossy().to_string(),
        ];

        // Add arguments from config directly
        args.extend(self.config.args().iter().cloned());

        // Add arguments and spawn the command
        let Ok(status) = command.args(&args).status() else {
            return Err("Sirius failed to start".to_string());
        };

        if !status.success() {
            return Err("Sirius failed".to_string());
        }

        Ok(())
    }
}
