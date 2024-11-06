use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage, ImageExt,
};
use std::error::Error;
use webcodegen::Table;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./test_migrations");
const DATABASE_NAME: &str = "test_db";
const DATABASE_PASSWORD: &str = "password";
const DATABASE_USER: &str = "user";
const DATABASE_PORT: u16 = 33676;

fn establish_connection_to_postres() -> PgConnection {
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


fn teardown_test_database(conn: &mut PgConnection) {
    diesel::sql_query(format!("DROP DATABASE IF EXISTS {}", DATABASE_NAME))
        .execute(conn)
        .unwrap();
}

async fn setup_postgres() -> ContainerAsync<GenericImage> {
    let container = GenericImage::new("postgres", "13-alpine")
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
    } else {
        println!("Started container");
    }

    container.unwrap()
}

#[tokio::test]
async fn test_user_table() {
    let container = setup_postgres().await;

    let mut conn = establish_connection_to_postres();
    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let users = Table::load(&mut conn, "users", None, DATABASE_NAME);
    
    let columns = users.unwrap().columns(&mut conn);

    if let Err(e) = columns {
        eprintln!("Failed to load columns: {:?}", e.to_string());
    } else {
        println!("{:?}", columns);
    }
    container.stop().await.unwrap();
}
