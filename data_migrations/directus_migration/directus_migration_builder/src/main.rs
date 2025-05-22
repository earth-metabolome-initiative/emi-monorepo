//! Build the core structures.
use std::path::Path;

use diesel_async::{AsyncConnection, AsyncPgConnection};
use webcodegen::{Codegen, Table};

const DATABASE_NAME: &str = "directus";
const DATABASE_PASSWORD: &str = "directus_dbgi";
const DATABASE_USER: &str = "directus";
const DATABASE_PORT: u16 = 5434;
const HOSTNAME: &str = "134.21.20.118";
const DATABASE_URL: &str = const_format::formatcp!(
    "postgres://{DATABASE_USER}:{DATABASE_PASSWORD}@{HOSTNAME}:{DATABASE_PORT}/{DATABASE_NAME}",
);

#[tokio::main]
/// Executable to generate the code for the Directus database.
pub async fn main() {
    // Get the output directory
    let out_dir = Path::new("../src");

    let mut conn = AsyncPgConnection::establish(DATABASE_URL).await.unwrap();

    // We write to the target directory the generated structs
    let curation_data = Table::load(&mut conn, "Curation_Data", None, DATABASE_NAME).await.unwrap();

    // Generate the code associated with the database
    Codegen::default()
        .add_table_to_deny_list(&curation_data)
        .set_output_directory(out_dir.as_ref())
        .enable_foreign_trait()
        .beautify()
        .generate(&mut conn, DATABASE_NAME, None)
        .await
        .unwrap();
}
