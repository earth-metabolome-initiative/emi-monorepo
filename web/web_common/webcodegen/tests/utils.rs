//! Utility functions for testing.

use std::{
    hash::{DefaultHasher, Hash, Hasher},
    io::Write,
};

use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use quote::quote;
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage, ImageExt,
};

const DEFAULT_MIGRATIONS: EmbeddedMigrations = embed_migrations!("./test_migrations");
const DATABASE_PASSWORD: &str = "password";
const DATABASE_USER: &str = "user";

/// Add a `fn main() {}` to the end of a file.
pub fn add_main_to_file(file_path: &str) {
    let mut file = std::fs::OpenOptions::new().append(true).open(file_path).unwrap();

    let main = quote! {
        fn main() {}
    };

    file.write_all(main.to_string().as_bytes()).unwrap();
    file.flush().unwrap();
    file.sync_all().unwrap();
}

pub fn establish_connection_to_postgres(
    database_port: u16,
    database_name: &str,
) -> Result<PgConnection, diesel::ConnectionError> {
    let database_url = format!(
        "postgres://{DATABASE_USER}:{DATABASE_PASSWORD}@localhost:{database_port}/{database_name}",
    );

    let mut number_of_attempts = 0;

    while let Err(e) = PgConnection::establish(&database_url) {
        eprintln!("Failed to establish connection: {:?}", e);
        std::thread::sleep(std::time::Duration::from_secs(1));
        if number_of_attempts > 10 {
            eprintln!("Failed to establish connection after 10 attempts");
            std::process::exit(1);
        }
        number_of_attempts += 1;
    }

    PgConnection::establish(&database_url)
}

pub async fn setup_docker(database_port: u16, database_name: &str) -> ContainerAsync<GenericImage> {
    let container = GenericImage::new("postgres", "17-alpine")
        .with_wait_for(WaitFor::message_on_stderr("database system is ready to accept connections"))
        .with_network("bridge")
        .with_env_var("DEBUG", "1")
        .with_env_var("POSTGRES_USER", DATABASE_USER)
        .with_env_var("POSTGRES_PASSWORD", DATABASE_PASSWORD)
        .with_env_var("POSTGRES_DB", database_name)
        .with_mapped_port(database_port, 5432_u16.tcp())
        .start()
        .await;

    if let Err(e) = container {
        eprintln!("Failed to start container: {:?}", e);
        std::process::exit(1);
    }

    container.unwrap()
}

pub async fn setup_database_with_default_migrations(
    test_name: &str,
) -> Result<(ContainerAsync<GenericImage>, PgConnection, String), diesel::ConnectionError> {
    let port = random_port(test_name);
    let database_name = format!("{}_db", test_name);
    let docker = setup_docker(port, &database_name).await;
    let mut conn = establish_connection_to_postgres(port, &database_name)?;
    conn.run_pending_migrations(DEFAULT_MIGRATIONS).unwrap();
    Ok((docker, conn, database_name))
}

pub fn random_port(test_name: &str) -> u16 {
    let mut hasher = DefaultHasher::default();
    test_name.hash(&mut hasher);
    let test_name_hash: u64 = hasher.finish();
    let port = (test_name_hash % 30000) as u16 + 10000;
    port
}

pub fn codegen_test(directory_name: &str) {
    let builder = trybuild::TestCases::new();

    // We create a main document to test the generated code.
    let file_content = quote::quote! {
        pub mod codegen;
    }
    .to_string();

    std::fs::write(&format!("tests/{directory_name}/main.rs"), file_content).unwrap();
    add_main_to_file(&format!("tests/{directory_name}/main.rs"));
    builder.pass(&format!("tests/{directory_name}/main.rs"));
}
