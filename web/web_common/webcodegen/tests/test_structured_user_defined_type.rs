mod utils;

use diesel::pg::PgConnection;
use diesel::Connection;
use diesel_migrations::{
    EmbeddedMigration, EmbeddedMigrations, EmbeddedName, MigrationHarness,
    TomlMetadataWrapper,
};
use std::io::Write;
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage, ImageExt,
};

use utils::add_main_to_file;
use webcodegen::*;

const MIGRATIONS: EmbeddedMigrations = EmbeddedMigrations::new(&[EmbeddedMigration::new(
    "CREATE TYPE Point2d AS (x DOUBLE PRECISION,y DOUBLE PRECISION);",
    None,
    EmbeddedName::new("2021-10-10-10-10-10-000000_point2d"),
    TomlMetadataWrapper::new(false),
)]);
const DATABASE_NAME: &str = "test_db";
const DATABASE_PASSWORD: &str = "password";
const DATABASE_USER: &str = "user";
const DATABASE_PORT: u16 = 35676;

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
        .start()
        .await;

    if let Err(e) = container {
        eprintln!("Failed to start container: {:?}", e);
        std::process::exit(1);
    }

    container.unwrap()
}

#[tokio::test]
async fn test_structured_user_defined_type() {
    let container = setup_postgres().await;

    let mut conn = establish_connection_to_postgres();
    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let point2d = PgType::from_name("point2d", &mut conn).unwrap();
    let attributes = point2d.attributes(&mut conn).unwrap();

    assert_eq!(attributes.len(), 2, "Expected 2 attributes, found {}", attributes.len());
    assert_eq!(attributes[0].attname, "x");
    assert_eq!(attributes[1].attname, "y");

    let builder = trybuild::TestCases::new();
    let path = "tests/ui/structured_user_defined_type.rs";

    write!(
        std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .unwrap(),
        "{}",
        point2d.to_syn(&mut conn).unwrap()
    ).unwrap();
    add_main_to_file(path);
    builder.pass(path);

    container.stop().await.unwrap();
}
