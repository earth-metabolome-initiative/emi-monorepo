//! Submodule testing the generation of cross-target code, including the execution
//! of the frontend, database and backend code generation.

mod utils;

use diesel::pg::PgConnection;
use diesel::Connection;
use diesel_migrations::{
    EmbeddedMigration, EmbeddedMigrations, EmbeddedName, MigrationHarness, TomlMetadataWrapper,
};
use std::io::Write;
use testcontainers::{
    core::{ExecCommand, IntoContainerPort, Mount, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage, ImageExt,
};

use utils::add_main_to_file;
use webcodegen::*;

const MIGRATIONS: EmbeddedMigrations = EmbeddedMigrations::new(&[EmbeddedMigration::new(
    "CREATE EXTENSION IF NOT EXISTS validations;",
    None,
    EmbeddedName::new("2021-10-10-10-10-10-000000_extension_import"),
    TomlMetadataWrapper::new(false),
)]);
const DATABASE_NAME: &str = "test_db";
const DATABASE_PASSWORD: &str = "password";
const DATABASE_USER: &str = "user";
const DATABASE_PORT: u16 = 37676;

fn establish_connection_to_postgres() -> PgConnection {
    let database_url = format!(
        "postgres://{DATABASE_USER}:{DATABASE_PASSWORD}@localhost:{DATABASE_PORT}/{DATABASE_NAME}",
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

    PgConnection::establish(&database_url).unwrap()
}

async fn setup_postgres() -> ContainerAsync<GenericImage> {
    // We retrieve the target directory, which we will use to mount
    // the Postgres extension created using pgrx.
    let target_directory =
        std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "../../../target".to_string());

    // Convert the relative path to an absolute path.
    let target_directory = std::fs::canonicalize(target_directory)
        .unwrap()
        .to_string_lossy()
        .to_string();

    // We check that the target directory exists.
    if !std::path::Path::new(&target_directory).exists() {
        eprintln!("Target directory does not exist: {}", target_directory);
        std::process::exit(1);
    }

    let container = GenericImage::new("postgres", "17-alpine")
        .with_wait_for(WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        ))
        .with_network("bridge")
        .with_env_var("DEBUG", "1")
        .with_env_var("POSTGRES_USER", DATABASE_USER)
        .with_env_var("POSTGRES_PASSWORD", DATABASE_PASSWORD)
        .with_env_var("POSTGRES_DB", DATABASE_NAME)
        .with_mapped_port(DATABASE_PORT, 5432.tcp())
        .with_copy_to(
            "/usr/lib/postgresql/17/lib",
            format!("{target_directory}/release/validations-pg17/usr/lib/postgresql/17/lib").as_str().into(),
        )
        .with_copy_to(
            "/usr/share/postgresql/17/extension",
            format!(
                "{target_directory}/release/validations-pg17/usr/share/postgresql/17/extension"
            ).as_str()
            .into(),
        )
        .start()
        .await;

    if let Err(e) = container {
        eprintln!("Failed to start container: {:?}", e);
        std::process::exit(1);
    }

    container.unwrap()
}

#[tokio::test]
async fn test_pgrx_integration() {
    let container = setup_postgres().await;

    let mut conn = establish_connection_to_postgres();
    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let is_not_empty = SQLFunction::from_name("is_not_empty", &mut conn).unwrap();

    let builder = trybuild::TestCases::new();
    let path = "tests/ui/pgrx_integration.rs";

    write!(
        std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .unwrap(),
        "{}",
        is_not_empty.to_syn()
    )
    .unwrap();
    add_main_to_file(path);
    builder.pass(path);

    container.stop().await.unwrap();
}
