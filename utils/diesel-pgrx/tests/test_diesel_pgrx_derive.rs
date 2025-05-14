//! Test submodule for the `diesel_pgrx_derive` and `diesel_pgrx` crates.

use std::path::PathBuf;

use diesel::{ExpressionMethods, query_dsl::methods::FilterDsl};
use diesel_async::{AsyncConnection, RunQueryDsl, SimpleAsyncConnection};
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt, TestcontainersError,
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
};

const DATABASE_USER: &str = "postgres";
const DATABASE_PASSWORD: &str = "postgres";
const PORT: u16 = 23456;

diesel::table! {
    fields (id) {
        id -> diesel::sql_types::Integer,
        field -> example_extension::diesel_impls::PositiveU32
    }
}

#[derive(Debug, diesel::Queryable)]
#[diesel(table_name = fields)]
struct Field {
    id: i32,
    field: example_extension::PositiveU32,
}

#[derive(Debug, diesel::Insertable)]
#[diesel(table_name = fields)]
struct InsertableField {
    field: example_extension::PositiveU32,
}

async fn init_db(conn: &mut diesel_async::AsyncPgConnection) -> Result<(), diesel::result::Error> {
    // We load the extension which we have copied into the container.
    conn.batch_execute("CREATE EXTENSION example_extension;").await?;
    // We create a table which will include a column with the custom type
    // `PositiveU32`, which also includes the validation function
    // `validate_positive_u32`.
    conn.batch_execute(
        "CREATE TABLE fields (
            id SERIAL PRIMARY KEY,
            field PositiveU32 NOT NULL
        );",
    )
    .await?;
    Ok(())
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
async fn reference_docker(
    database_port: u16,
    database_name: &str,
) -> Result<ContainerAsync<GenericImage>, TestcontainersError> {
    let docker_share_postgres_extension = PathBuf::from("/usr/share/postgresql/17/extension");
    let docker_lib_postgres_extension = PathBuf::from("/usr/lib/postgresql/17/lib");
    let local_share_postgres_extension = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("example_extension")
        .join("extension")
        .join("usr")
        .join("share")
        .join("postgresql")
        .join("17")
        .join("extension");
    let local_lib_postgres_extension = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("example_extension")
        .join("extension")
        .join("usr")
        .join("lib")
        .join("postgresql")
        .join("17")
        .join("lib");
    let control_path = local_share_postgres_extension.join("example_extension.control");
    let sql_path = local_share_postgres_extension.join("example_extension--0.1.0.sql");
    let so_path = local_lib_postgres_extension.join("example_extension.so");

    // We check that all the paths exist, or we panic advising the user to compile
    // the extension with the instructions in the README.
    for path in [&control_path, &sql_path, &so_path] {
        if !path.exists() {
            panic!(
                "Path {path:?} does not exist. Please compile the extension with the instructions in the README."
            );
        }
    }

    let container_builder = GenericImage::new("postgres", "latest")
        .with_wait_for(WaitFor::message_on_stderr("database system is ready to accept connections"))
        .with_network("bridge")
        .with_env_var("DEBUG", "1")
        .with_env_var("POSTGRES_USER", DATABASE_USER)
        .with_env_var("POSTGRES_PASSWORD", DATABASE_PASSWORD)
        .with_env_var("POSTGRES_DB", database_name)
        .with_mapped_port(database_port, 5432_u16.tcp())
        .with_copy_to(
            docker_share_postgres_extension
                .join(control_path.file_name().unwrap())
                .to_str()
                .unwrap(),
            control_path,
        )
        .with_copy_to(
            docker_share_postgres_extension.join(sql_path.file_name().unwrap()).to_str().unwrap(),
            sql_path,
        )
        .with_copy_to(
            docker_lib_postgres_extension.join(so_path.file_name().unwrap()).to_str().unwrap(),
            so_path,
        );

    container_builder.start().await
}

#[tokio::test]
/// Test function for the `diesel_pgrx_derive` and `diesel_pgrx` crates.
pub async fn test_diesel_pgrx_derive() {
    let docker = reference_docker(PORT, "test_db").await.expect("Failed to start container");
    let mut conn = diesel_async::AsyncPgConnection::establish(&format!(
        "postgres://{DATABASE_USER}:{DATABASE_PASSWORD}@localhost:{PORT}/test_db"
    ))
    .await
    .expect("Failed to connect to database");
    if let Err(err) = init_db(&mut conn).await {
        docker.stop().await.expect("Failed to stop container");
        panic!("Failed to initialize database: {err:?}");
    }

    // We attempt to insert a valid value into the table.
    let valid_value = InsertableField { field: example_extension::PositiveU32 { field: 1 } };

    if let Err(err) =
        diesel::insert_into(fields::table).values(&valid_value).execute(&mut conn).await
    {
        docker.stop().await.expect("Failed to stop container");
        panic!("Failed to insert valid value: {err:?}");
    }

    // We attempt to insert an invalid value into the table.
    let invalid_value = InsertableField { field: example_extension::PositiveU32 { field: 0 } };
    match diesel::insert_into(fields::table).values(&invalid_value).execute(&mut conn).await {
        Ok(_) => {
            docker.stop().await.expect("Failed to stop container");
            panic!("Inserted invalid value into table");
        }
        Err(err) => {
            assert_eq!(err.to_string(), "Validation failed: field must be greater than 0");
        }
    }

    // We attempt to query the table to retrieve the valid value.
    match fields::table.filter(fields::field.eq(&valid_value.field)).first::<Field>(&mut conn).await
    {
        Ok(field) => {
            assert_eq!(field.id, 1);
            assert_eq!(field.field, valid_value.field);
        }
        Err(err) => {
            docker.stop().await.expect("Failed to stop container");
            panic!("Failed to query table: {err}");
        }
    }

    docker.stop().await.expect("Failed to stop container");
}
