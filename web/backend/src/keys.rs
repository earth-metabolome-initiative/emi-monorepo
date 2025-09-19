//! Submodule handling the creation or loading of private and public keys.

use std::{fs, path::Path};

use openssl::rsa::Rsa;

#[derive(Debug, Clone)]
/// Struct representing a pair of RSA keys (private and public).
pub struct KeyPair {
    /// PEM-encoded private key (PKCS#1/PEM)
    private_pem: Vec<u8>,
    /// PEM-encoded public key (PKCS#1/PEM)
    public_pem: Vec<u8>,
}

impl KeyPair {
    /// Get the private key in PEM format.
    pub fn private_pem(&self) -> &[u8] {
        &self.private_pem
    }

    /// Get the public key in PEM format.
    pub fn public_pem(&self) -> &[u8] {
        &self.public_pem
    }
}

impl KeyPair {
    fn load_from_files(path: &Path, key_name: &str) -> Result<Self, std::io::Error> {
        let private_pem_path = path.join(format!("{key_name}_private.pem"));
        let public_pem_path = path.join(format!("{key_name}_public.pem"));

        let private_pem = fs::read(&private_pem_path)?;
        let public_pem = fs::read(&public_pem_path)?;

        Ok(KeyPair { private_pem, public_pem })
    }

    fn save_to_files(&self, path: &Path, key_name: &str) -> Result<(), std::io::Error> {
        // We create the directory if it doesn't exist
        fs::create_dir_all(path)?;

        let private_pem_path = path.join(format!("{key_name}_private.pem"));
        let public_pem_path = path.join(format!("{key_name}_public.pem"));

        fs::write(private_pem_path, &self.private_pem)?;
        fs::write(public_pem_path, &self.public_pem)?;

        Ok(())
    }

    /// Load or generate a pair of RSA keys (private and public) and save them
    /// to the specified path if they do not already exist.
    ///
    /// # Arguments
    ///
    /// * `path` - The path where the keys should be saved.
    /// * `key_name` - The base name for the key files (without extension).
    ///
    /// # Errors
    ///
    /// * If there is an error reading or writing the key files, an
    ///   `std::io::Error` is returned.
    pub fn load_or_generate_keys(path: &Path, key_name: &str) -> Result<Self, std::io::Error> {
        if let Ok(key_pair) = Self::load_from_files(path, key_name) {
            return Ok(key_pair);
        }

        // Generate a 4096-bit RSA keypair
        let rsa = Rsa::generate(4096)?;

        // PEM-encoded private & public keys (PKCS#1/PEM)
        let private_pem = rsa.private_key_to_pem()?; // Vec<u8>
        let public_pem = rsa.public_key_to_pem()?; // Vec<u8>

        let key_pair = Self { private_pem, public_pem };

        // save to files
        key_pair.save_to_files(path, key_name)?;

        Ok(key_pair)
    }
}
