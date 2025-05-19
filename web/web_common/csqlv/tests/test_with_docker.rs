//! Submodule for testing CSV schema creation and deletion with Docker.
use std::path::PathBuf;

use csqlv::{CSVSchemaBuilder, CSVSchemaError, SQLGenerationOptions};
use diesel_async::AsyncPgConnection;
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt,
    core::{IntoContainerPort, Mount, WaitFor},
    runners::AsyncRunner,
};

const DATABASE_NAME: &str = "test_db";
const DATABASE_PASSWORD: &str = "password";
const DATABASE_USER: &str = "user";
const DATABASE_PORT: u16 = 33676;

const DATABASE_URL: &str = const_format::formatcp!(
    "postgres://{DATABASE_USER}:{DATABASE_PASSWORD}@localhost:{DATABASE_PORT}/{DATABASE_NAME}",
);

async fn setup_docker() -> ContainerAsync<GenericImage> {
    // We check whether `docker` is installed and running.
    if std::process::Command::new("docker").output().is_err() {
        eprintln!("Docker is not installed or not running.");
        std::process::exit(1);
    }

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
        eprintln!("Failed to start container: {e:?}");
        std::process::exit(1);
    }

    container.unwrap()
}

async fn test_independent_csvs() -> Result<(), CSVSchemaError> {
    let schema = CSVSchemaBuilder::default()
        .container_directory(PathBuf::from("/app/csvs/independent_csvs"))
        .singularize()
        .from_dir("./tests/independent_csvs")
        .unwrap();

    let sql_generation_options = SQLGenerationOptions::default().include_population();
    schema.connect_and_create::<AsyncPgConnection>(DATABASE_URL, &sql_generation_options).await?;
    schema.connect_and_delete::<AsyncPgConnection>(DATABASE_URL).await?;

    Ok(())
}

async fn test_tree_dependent_csvs() -> Result<(), CSVSchemaError> {
    let schema = CSVSchemaBuilder::default()
        .container_directory(PathBuf::from("/app/csvs/tree_dependent_csvs"))
        .singularize()
        .from_dir("./tests/tree_dependent_csvs")
        .unwrap();

    let sql_generation_options = SQLGenerationOptions::default().include_population();
    schema.connect_and_create::<AsyncPgConnection>(DATABASE_URL, &sql_generation_options).await?;
    schema.connect_and_delete::<AsyncPgConnection>(DATABASE_URL).await?;

    Ok(())
}

async fn test_dag_dependent_csvs() -> Result<(), CSVSchemaError> {
    let schema = CSVSchemaBuilder::default()
        .container_directory(PathBuf::from("/app/csvs/dag_dependent_csvs"))
        .include_gz()
        .singularize()
        .from_dir("./tests/dag_dependent_csvs")
        .unwrap();

    let sql_generation_options = SQLGenerationOptions::default().include_population();
    schema.connect_and_create::<AsyncPgConnection>(DATABASE_URL, &sql_generation_options).await?;
    schema.connect_and_delete::<AsyncPgConnection>(DATABASE_URL).await?;

    Ok(())
}

async fn test_bands_csvs() -> Result<(), CSVSchemaError> {
    let schema = CSVSchemaBuilder::default()
        .include_gz()
        .singularize()
        .container_directory(PathBuf::from("/app/csvs/bands"))
        .from_dir("./tests/bands")
        .unwrap();

    let sql_generation_options = SQLGenerationOptions::default().include_population();
    schema.connect_and_create::<AsyncPgConnection>(DATABASE_URL, &sql_generation_options).await?;
    schema.connect_and_delete::<AsyncPgConnection>(DATABASE_URL).await?;

    Ok(())
}

#[tokio::test]
async fn test_with_docker() {
    let container = setup_docker().await;

    if let Err(err) = test_independent_csvs().await {
        container.stop().await.expect("Failed to stop container.");
        panic!("Failed to test independent CSVs: {err:?}");
    }
    if let Err(err) = test_tree_dependent_csvs().await {
        container.stop().await.expect("Failed to stop container.");
        panic!("Failed to test tree dependent CSVs: {err:?}");
    }
    if let Err(err) = test_dag_dependent_csvs().await {
        container.stop().await.expect("Failed to stop container.");
        panic!("Failed to test DAG dependent CSVs: {err:?}");
    }
    if let Err(err) = test_bands_csvs().await {
        container.stop().await.expect("Failed to stop container.");
        panic!("Failed to test bands CSVs: {err:?}");
    }

    container.stop().await.expect("Failed to stop container.");
}
