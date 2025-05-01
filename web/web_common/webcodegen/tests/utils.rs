#![allow(dead_code)]
//! Utility functions for testing.

use std::{
    hash::{DefaultHasher, Hash, Hasher},
    io::Write,
};

use diesel_async::AsyncPgConnection;
use diesel_migrations_utils::prelude::MigrationDirectory;
use quote::quote;
use reference_docker::reference_docker_with_connection;
use testcontainers::{ContainerAsync, GenericImage};

const DATABASE_PASSWORD: &str = "password";
const DATABASE_USER: &str = "user";

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
) -> Result<(ContainerAsync<GenericImage>, AsyncPgConnection, String), diesel::ConnectionError> {
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
) -> Result<(ContainerAsync<GenericImage>, AsyncPgConnection, String), diesel::ConnectionError> {
    let port = random_port(test_name);
    let database_name = format!("{test_name}_db");
    let (docker, mut conn) = reference_docker_with_connection(&database_name, port)
        .await
        .expect("Failed to start container");
    migrations.execute_ups::<diesel_async::AsyncPgConnection>(&mut conn).await.unwrap();
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
