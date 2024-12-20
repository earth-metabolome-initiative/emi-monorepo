//! Builder which executes the webcodegen on the 'directus' database
//! in order to generate the structs used throught the 'directus_migration' crate.

use diesel::pg::PgConnection;
use diesel::Connection;
use std::{env, path::PathBuf};
use std::path::Path;
use webcodegen::{Codegen, Table};

use config::{Config, File};

enum DirectusBuildError {
    MissingConfiguration,
    MissingConfigurationField(String),
    ConnectionError,
    FailedToEstablishConnection,
    OutDirMissing,
}

impl From<diesel::result::ConnectionError> for DirectusBuildError {
    fn from(_: diesel::result::ConnectionError) -> Self {
        DirectusBuildError::ConnectionError
    }
}

impl std::fmt::Debug for DirectusBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DirectusBuildError::MissingConfiguration => write!(f, "Missing configuration"),
            DirectusBuildError::MissingConfigurationField(field) => {
                write!(f, "Missing configuration field: {}", field)
            }
            DirectusBuildError::ConnectionError => write!(f, "Connection error"),
            DirectusBuildError::FailedToEstablishConnection => {
                write!(f, "Failed to establish connection")
            }
            DirectusBuildError::OutDirMissing => write!(f, "OUT_DIR is missing"),
        }
    }
}

fn establish_connection_to_postgres() -> Result<PgConnection, DirectusBuildError> {
    // Load the configuration
    let settings = Config::builder()
        .add_source(File::with_name("config.ini"))
        .build()
        .map_err(|_| DirectusBuildError::MissingConfiguration)?;

    // Extract the database settings
    let database_name: String = settings
        .get("database.name")
        .map_err(|_| DirectusBuildError::MissingConfigurationField("database.name".to_string()))?;
    let database_password: String = settings
        .get("database.password")
        .map_err(|_| DirectusBuildError::MissingConfigurationField("database.password".to_string()))?;
    let database_user: String = settings
        .get("database.user")
        .map_err(|_| DirectusBuildError::MissingConfigurationField("database.user".to_string()))?;
    let database_port: u16 = settings
        .get("database.port")
        .map_err(|_| DirectusBuildError::MissingConfigurationField("database.port".to_string()))?;
    let database_host: String = settings
        .get("database.host")
        .map_err(|_| DirectusBuildError::MissingConfigurationField("database.host".to_string()))?;

    let database_url = format!(
        "postgres://{database_user}:{database_password}@{database_host}:{database_port}/{database_name}",
    );

    let mut number_of_attempts = 0;

    while let Err(e) = PgConnection::establish(&database_url) {
        eprintln!("Failed to establish connection: {:?}", e);
        std::thread::sleep(std::time::Duration::from_secs(1));
        if number_of_attempts > 10 {
            return Err(DirectusBuildError::FailedToEstablishConnection);
        }
        number_of_attempts += 1;
    }

    Ok(PgConnection::establish(&database_url)?)
}


fn get_path() -> Result<PathBuf, DirectusBuildError> {
    // Get the output directory
    let out_dir = env::var("OUT_DIR").map_err(|_| DirectusBuildError::OutDirMissing)?;

    // Path to the file to create
    let path = Path::new(&out_dir).join("directus_structs.rs");

    Ok(path)
}

/// Main function
pub fn main() {
    let Ok(mut conn) = establish_connection_to_postgres() else {
        eprintln!("Failed to establish connection to the database");
        return;
    };

    // We write to the target directory the generated structs
    let Ok(path) = get_path() else {
        eprintln!("Failed to get the path to the output file");
        return;
    };

    let curation_data = Table::load(&mut conn, "Curation_Data", None, "directus").unwrap();

    Codegen::default()
        .set_output_path(path.as_ref())
        .add_table_to_deny_list(&curation_data)
        .generate(&mut conn, "directus", None)
        .expect("Failed to generate the structs");
}
