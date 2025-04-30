#![allow(dead_code)]
//! Utility functions for testing.

use std::{
    hash::{DefaultHasher, Hash, Hasher},
    io::Write,
    path::Path,
};

use diesel::{Connection, PgConnection};
use diesel_migrations_utils::prelude::MigrationDirectory;
use quote::quote;
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt, TestcontainersError,
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
};

const DATABASE_PASSWORD: &str = "password";
const DATABASE_USER: &str = "user";

/// Finds the first file matching the requested extension under the provided
/// directory, recursively.
///
/// # Arguments
///
/// * `directory` - The directory to search in.
/// * `extension` - The extension to search for.
///
/// # Returns
///
/// * The path to the file.
///
/// # Errors
///
/// * If the file cannot be found.
/// * If the directory cannot be read.
fn find_file(directory: &str, extension: &str) -> Result<String, std::io::Error> {
    let mut stack = vec![std::path::PathBuf::from(directory)];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if let Some(ext) = path.extension() {
                if ext == extension {
                    return Ok(path.to_string_lossy().to_string());
                }
            }
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!("No file with extension {extension} found in {directory}"),
    ))
}

/// Add a `fn main() {}` to the end of a file.
///
/// # Arguments
///
/// * `file_path` - The path to the file.
pub fn add_main_to_file(file_path: &str) {
    let mut file = std::fs::OpenOptions::new().append(true).open(file_path).unwrap();

    let main = quote! {
        fn main() {}
    };

    file.write_all(main.to_string().as_bytes()).unwrap();
    file.flush().unwrap();
    file.sync_all().unwrap();
}

/// Establish a connection to a postgres database.
///
/// # Arguments
///
/// * `database_port` - The port of the database.
/// * `database_name` - The name of the database.
///
/// # Errors
///
/// * If the connection cannot be established.
pub fn establish_connection_to_postgres(
    database_port: u16,
    database_name: &str,
) -> Result<PgConnection, diesel::ConnectionError> {
    let database_url = format!(
        "postgres://{DATABASE_USER}:{DATABASE_PASSWORD}@localhost:{database_port}/{database_name}",
    );

    let mut number_of_attempts = 0;

    while let Err(e) = PgConnection::establish(&database_url) {
        eprintln!("Failed to establish connection: {e:?}");
        std::thread::sleep(std::time::Duration::from_secs(1));
        if number_of_attempts > 10 {
            eprintln!("Failed to establish connection after 10 attempts");
            std::process::exit(1);
        }
        number_of_attempts += 1;
    }

    PgConnection::establish(&database_url)
}

/// Setup a docker container with a postgres database.
///
/// # Arguments
///
/// * `database_port` - The port of the database.
/// * `database_name` - The name of the database.
///
/// # Panics
///
/// * If the container cannot be started.
async fn setup_docker(
    database_port: u16,
    database_name: &str,
) -> Result<ContainerAsync<GenericImage>, TestcontainersError> {
    let extension_directory = "../pgrx_validation/extension".to_string();

    // We check whether the extension directory exists, or we raise an adequate
    // error warning the reader that they most likely need to build the
    // extension.

    assert!(
        std::path::Path::new(&extension_directory).exists(),
        "The extension directory `{extension_directory}` does not exist. Most likely you forgot to build the extension. Refer to the `pgrx_validation` README for more information."
    );

    GenericImage::new("postgres", "17-bookworm")
        .with_wait_for(WaitFor::message_on_stderr("database system is ready to accept connections"))
        .with_network("bridge")
        .with_env_var("DEBUG", "1")
        .with_env_var("POSTGRES_USER", DATABASE_USER)
        .with_env_var("POSTGRES_PASSWORD", DATABASE_PASSWORD)
        .with_env_var("POSTGRES_DB", database_name)
        .with_mapped_port(database_port, 5432_u16.tcp())
        .with_copy_to(
            "/usr/share/postgresql/17/extension/pgrx_validation.control",
            Path::new(
                &find_file(&extension_directory, "control").expect("Failed to find extension `.control` file - Most likely you forgot to build the extension"),
            ),
        )
        .with_copy_to(
            "/usr/share/postgresql/17/extension/pgrx_validation--0.0.0.sql",
            Path::new(&find_file(&extension_directory, "sql").expect("Failed to find extension `.sql` file - Most likely you forgot to build the extension"))
        )
        .with_copy_to(
            "/usr/lib/postgresql/17/lib/pgrx_validation.so",
            Path::new(&find_file(&extension_directory, "so").expect("Failed to find extension `.so` file - Most likely you forgot to build the extension"))
        )
        .start()
        .await
}

/// Setup a database with the default migrations.
///
/// # Arguments
///
/// * `test_name` - The name of the test.
///
/// # Errors
///
/// * If the connection cannot be established.
///
/// # Panics
///
/// * If the container cannot be started.
pub async fn setup_database_with_default_migrations(
    test_name: &str,
) -> Result<(ContainerAsync<GenericImage>, PgConnection, String), diesel::ConnectionError> {
    setup_database_with_migrations(
        test_name,
        MigrationDirectory::try_from("./test_migrations").unwrap(),
    )
    .await
}

/// Setup a database with a custom migration dir.
///
/// # Arguments
///
/// * `test_name` - The name of the test.
///
/// # Errors
///
/// * If the connection cannot be established.
///
/// # Panics
///
/// * If the container cannot be started.
pub async fn setup_database_with_migrations(
    test_name: &str,
    migrations: MigrationDirectory,
) -> Result<(ContainerAsync<GenericImage>, PgConnection, String), diesel::ConnectionError> {
    let port = random_port(test_name);
    let database_name = format!("{test_name}_db");
    let docker = setup_docker(port, &database_name).await.expect("Failed to start container");
    let mut conn = establish_connection_to_postgres(port, &database_name)?;
    migrations.execute_ups::<diesel::PgConnection>(&mut conn).unwrap();
    Ok((docker, conn, database_name))
}

/// Generate a random port based on the test name.
///
/// # Arguments
///
/// * `test_name` - The name of the test.
#[must_use]
pub fn random_port(test_name: &str) -> u16 {
    let mut hasher = DefaultHasher::default();
    test_name.hash(&mut hasher);
    let test_name_hash: u64 = hasher.finish();

    (test_name_hash % 30000) as u16 + 10000
}

/// Generate the code for a test and run it.
///
/// # Arguments
///
/// * `directory_name` - The name of the directory.
///
/// # Panics
///
/// * If writing to the file fails.
pub fn codegen_test(directory_name: &str) {
    let builder = trybuild::TestCases::new();

    // We create a main document to test the generated code.
    let file_content = quote::quote! {
        pub mod codegen;
    }
    .to_string();

    std::fs::write(format!("tests/{directory_name}/main.rs"), file_content).unwrap();
    add_main_to_file(&format!("tests/{directory_name}/main.rs"));
    builder.pass(format!("tests/{directory_name}/main.rs"));
}
