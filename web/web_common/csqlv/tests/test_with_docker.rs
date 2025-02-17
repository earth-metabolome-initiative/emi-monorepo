use csqlv::{CSVSchemaBuilder, CSVSchemaError};
use diesel::{connection::SimpleConnection, pg::PgConnection, Connection};
use testcontainers::{
    core::{IntoContainerPort, Mount, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage, ImageExt,
};

const DATABASE_NAME: &str = "test_db";
const DATABASE_PASSWORD: &str = "password";
const DATABASE_USER: &str = "user";
const DATABASE_PORT: u16 = 33676;

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
    let local_absolute_path = std::env::current_dir().unwrap();
    let local_absolute_path = local_absolute_path.to_str().unwrap();

    let container = GenericImage::new("postgres", "17-alpine")
        .with_wait_for(WaitFor::message_on_stderr("database system is ready to accept connections"))
        .with_network("bridge")
        .with_env_var("DEBUG", "1")
        .with_env_var("POSTGRES_USER", DATABASE_USER)
        .with_env_var("POSTGRES_PASSWORD", DATABASE_PASSWORD)
        .with_env_var("POSTGRES_DB", DATABASE_NAME)
        .with_mapped_port(DATABASE_PORT, 5432.tcp())
        .with_mount(Mount::bind_mount(format!("{local_absolute_path}/tests"), "/app/csvs"))
        .start()
        .await;

    if let Err(e) = container {
        eprintln!("Failed to start container: {:?}", e);
        std::process::exit(1);
    }

    container.unwrap()
}

async fn test_independent_csvs(conn: &mut PgConnection) -> Result<(), CSVSchemaError> {
    let schema = CSVSchemaBuilder::default()
        .container_directory("/app/csvs/independent_csvs")
        .singularize()
        .from_dir("./tests/independent_csvs")
        .unwrap();

    let sql = schema.to_postgres()?;
    conn.batch_execute(&sql).unwrap();

    let delete_sql = schema.to_postgres_delete();
    conn.batch_execute(&delete_sql).unwrap();

    Ok(())
}

async fn test_tree_dependent_csvs(conn: &mut PgConnection) -> Result<(), CSVSchemaError> {
    let schema = CSVSchemaBuilder::default()
        .container_directory("/app/csvs/tree_dependent_csvs")
        .singularize()
        .from_dir("./tests/tree_dependent_csvs")
        .unwrap();

    let sql = schema.to_postgres()?;
    conn.batch_execute(&sql).unwrap();

    let delete_sql = schema.to_postgres_delete();
    conn.batch_execute(&delete_sql).unwrap();

    Ok(())
}

async fn test_dag_dependent_csvs(conn: &mut PgConnection) -> Result<(), CSVSchemaError> {
    let schema = CSVSchemaBuilder::default()
        .container_directory("/app/csvs/dag_dependent_csvs")
        .include_gz()
        .singularize()
        .from_dir("./tests/dag_dependent_csvs")
        .unwrap();

    let sql = schema.to_postgres()?;
    conn.batch_execute(&sql).unwrap();

    let delete_sql = schema.to_postgres_delete();
    conn.batch_execute(&delete_sql).unwrap();

    Ok(())
}

async fn test_bands_csvs(conn: &mut PgConnection) -> Result<(), CSVSchemaError> {
    let schema = CSVSchemaBuilder::default()
        .include_gz()
        .singularize()
        .container_directory("/app/csvs/bands")
        .from_dir("./tests/bands")
        .unwrap();

    let sql = schema.to_postgres()?;

    println!("{}", sql);

    conn.batch_execute(&sql).unwrap();

    let delete_sql = schema.to_postgres_delete();
    conn.batch_execute(&delete_sql).unwrap();

    Ok(())
}

#[tokio::test]
async fn test_user_table() {
    let container = setup_postgres().await;

    let mut conn = establish_connection_to_postgres();
    test_independent_csvs(&mut conn).await.unwrap();
    test_tree_dependent_csvs(&mut conn).await.unwrap();
    test_dag_dependent_csvs(&mut conn).await.unwrap();
    test_bands_csvs(&mut conn).await.unwrap();

    container.stop().await.unwrap();
}
