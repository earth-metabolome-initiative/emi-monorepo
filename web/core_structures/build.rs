//! Build the core structures.
use csqlv::{CSVSchema, CSVSchemaBuilder};
use diesel::connection::SimpleConnection;
use diesel::pg::PgConnection;
use diesel::Connection;
use std::env;
use std::path::Path;
use taxonomy_fetcher::impls::ncbi::{NCBIRank, NCBITaxonomy, NCBITaxonomyBuilder};
use taxonomy_fetcher::{Rank, Taxonomy, TaxonomyBuilder};
use testcontainers::core::Mount;
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage, ImageExt,
};
use webcodegen::Codegen;

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
        .with_wait_for(WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        ))
        .with_network("bridge")
        .with_env_var("DEBUG", "1")
        .with_env_var("POSTGRES_USER", DATABASE_USER)
        .with_env_var("POSTGRES_PASSWORD", DATABASE_PASSWORD)
        .with_env_var("POSTGRES_DB", DATABASE_NAME)
        .with_mapped_port(DATABASE_PORT, 5432.tcp())
        .with_mount(Mount::bind_mount(
            format!("{local_absolute_path}/"),
            "/app/",
        ))
        .start()
        .await;

    if let Err(e) = container {
        eprintln!("Failed to start container: {:?}", e);
        std::process::exit(1);
    }

    container.unwrap()
}

#[tokio::main]
pub async fn main() {
    // First, we create the CSV for the font-awesome icons
    font_awesome::Icon::to_csv("csvs/icons.csv").unwrap();

    // Next, we create the CSV for the taxonomical ranks
    NCBIRank::to_csv("csvs/ranks.csv").unwrap();

    // We retrieve and build the latest version of the NCBI taxonomy
    let taxonomy: NCBITaxonomy = NCBITaxonomyBuilder::latest().build().await.unwrap();
    taxonomy.to_csv("csvs/taxa.csv").unwrap();

    // Next, we build the SQL associated with the CSVs present in the 'csvs' directory
    let schema: CSVSchema = CSVSchemaBuilder::default()
        // To show a loading bar while processing the CSVs
        .verbose()
        // To include compressed files such as .gz
        .include_gz()
        // For supporting running the tests within
        // containers such as Docker
        .singularize()
        .container_directory("/app/csvs")
        .from_dir("./csvs")
        .unwrap();

    // We create the SQL to create the tables and populate them
    let sql: String = schema.to_postgres().unwrap();

    // We initialize the database
    let container = setup_postgres().await;

    // And obtain a connection to the database
    let mut conn = establish_connection_to_postgres();

    // We execute the SQL
    conn.batch_execute(&sql).unwrap();

    // We write to the target directory the generated structs

    // Get the output directory
    let out_dir = env::var("OUT_DIR").unwrap();

    // Path to the file to create
    let path = Path::new(&out_dir).join("core_structs.rs");

    // Generate the code associated with the database
    Codegen::default()
        .set_output_path(path.as_ref())
        .generate(&mut conn, DATABASE_NAME, None)
        .unwrap();

    // And we stop the container
    container.stop().await.unwrap();
}
