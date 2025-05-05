#![doc = include_str!("../README.md")]

use std::{fmt::Debug, path::PathBuf};

use diesel_async::AsyncConnection;
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt,
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
};

pub mod errors;

/// The default database user.
pub const DATABASE_USER: &str = "user";
/// The default database user password.
pub const DATABASE_PASSWORD: &str = "password";

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
fn find_file<P>(directory: P, extension: &str) -> Result<PathBuf, std::io::Error>
where
    P: AsRef<std::path::Path> + Debug,
{
    let mut stack = vec![directory.as_ref().to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if let Some(ext) = path.extension() {
                if ext == extension {
                    return Ok(path);
                }
            }
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!("No file with extension {extension} found in {directory:?}"),
    ))
}

/// Returns whether the provided directory contains a pgrx extension.
fn is_extension_crate<P>(directory: P) -> Result<bool, std::io::Error>
where
    P: AsRef<std::path::Path>,
{
    let path = directory.as_ref().join("Cargo.toml");
    if !path.exists() {
        return Ok(false);
    }
    let content = std::fs::read_to_string(path)?;
    if content.contains("[lib]")
        && content.contains("crate-type = [\"cdylib\", \"lib\"]")
        && content.contains("pgrx")
    {
        return Ok(true);
    }
    Ok(false)
}

/// Returns all the pgrx_extensions in the given directory.
fn find_pgrx_extensions<P>(directory: P) -> Result<Vec<PathBuf>, std::io::Error>
where
    P: AsRef<std::path::Path> + Debug,
{
    let mut pgrx_extensions = Vec::new();
    for entry in std::fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if is_extension_crate(&path).unwrap_or(false) {
                pgrx_extensions.push(path);
            }
        }
    }
    Ok(pgrx_extensions)
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
) -> Result<ContainerAsync<GenericImage>, crate::errors::Error> {
    let pgrx_extensions =
        find_pgrx_extensions(&PathBuf::from(format!("{}/../", env!("CARGO_MANIFEST_DIR"))))
            .unwrap_or_default();

    // We check whether the extension directory exists, or we raise an adequate
    // error warning the reader that they most likely need to build the
    // extension.
    for extension in &pgrx_extensions {
        assert!(
            extension.join("extension").exists(),
            "The extension `{extension:?}` was not built. Most likely you forgot to build the extension. Refer to the README for more information."
        );
    }

    let mut container_builder = GenericImage::new("mycustom/postgres-postgis", "17.4")
        .with_wait_for(WaitFor::message_on_stderr("database system is ready to accept connections"))
        .with_network("bridge")
        .with_env_var("DEBUG", "1")
        .with_env_var("POSTGRES_USER", DATABASE_USER)
        .with_env_var("POSTGRES_PASSWORD", DATABASE_PASSWORD)
        .with_env_var("POSTGRES_DB", database_name)
        .with_mapped_port(database_port, 5432_u16.tcp());

    let share_postgres_extension = PathBuf::from("/usr/share/postgresql/17/extension");
    let lib_postgres_extension = PathBuf::from("/usr/lib/postgresql/17/lib");

    for extension in &pgrx_extensions {
        // We get the name of the extension by the last part of the path
        let extension = extension.join("extension");
        let control_path = find_file(&extension, "control").expect("Failed to find extension `.control` file - Most likely you forgot to build the extension");
        let sql_path = find_file(&extension, "sql").expect(
            "Failed to find extension `.sql` file - Most likely you forgot to build the extension",
        );
        let so_path = find_file(&extension, "so").expect(
            "Failed to find extension `.so` file - Most likely you forgot to build the extension",
        );

        container_builder = container_builder
            .with_copy_to(
                share_postgres_extension.join(control_path.file_name().unwrap()).to_str().unwrap(),
                control_path,
            )
            .with_copy_to(
                share_postgres_extension.join(sql_path.file_name().unwrap()).to_str().unwrap(),
                sql_path,
            )
            .with_copy_to(
                lib_postgres_extension.join(so_path.file_name().unwrap()).to_str().unwrap(),
                so_path,
            );
    }

    Ok(container_builder.start().await?)
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
async fn establish_connection_to_postgres<C: AsyncConnection>(
    database_port: u16,
    database_name: &str,
) -> Result<C, diesel::ConnectionError> {
    let database_url = format!(
        "postgres://{DATABASE_USER}:{DATABASE_PASSWORD}@localhost:{database_port}/{database_name}",
    );

    let mut number_of_attempts = 0;

    while let Err(e) = C::establish(&database_url).await {
        eprintln!("Failed to establish connection: {e:?}");
        std::thread::sleep(std::time::Duration::from_secs(1));
        if number_of_attempts > 10 {
            return Err(e);
        }
        number_of_attempts += 1;
    }

    C::establish(&database_url).await
}

/// Setup a database with a custom migration dir.
///
/// # Arguments
///
/// * `database_name` - The name of the database.
/// * `port` - The port of the database.
///
/// # Errors
///
/// * If the connection cannot be established.
/// * If the container cannot be started.
///
/// # Example
///
/// ```rust
/// use diesel_async::AsyncPgConnection;
/// use reference_docker::reference_docker_with_connection;
///
/// #[tokio::main]
/// async fn main() {
///     let (docker, conn) =
///         reference_docker_with_connection::<AsyncPgConnection>("test_db", 6437).await.unwrap();
/// }
/// ```
pub async fn reference_docker_with_connection<C: AsyncConnection>(
    database_name: &str,
    port: u16,
) -> Result<(ContainerAsync<GenericImage>, C), crate::errors::Error> {
    let docker = reference_docker(port, &database_name).await?;
    let conn = establish_connection_to_postgres(port, &database_name).await?;
    Ok((docker, conn))
}
