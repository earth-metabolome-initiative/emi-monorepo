use std::path::Path;

use csqlv::{CSVSchemaBuilder, SQLGenerationOptions};
use taxonomy_fetcher::{
    Rank, Taxonomy, TaxonomyBuilder,
    impls::ncbi::{NCBIRank, NCBITaxonomy, NCBITaxonomyBuilder},
};

pub async fn retrieve_csvs(csv_directory: &Path) -> Result<(), crate::errors::Error> {
    NCBIRank::to_csv(csv_directory.join("ranks.csv"))?;

    // We retrieve and build the latest version of the NCBI taxonomy
    if !Path::new(&csv_directory.join("taxa.csv")).exists() {
        let taxonomy: NCBITaxonomy = NCBITaxonomyBuilder::latest().build().await?;
        taxonomy.to_csv(csv_directory.join("taxa.csv"))?;
    }

    Ok(())
}

pub(crate) fn init_csvs(
    csv_directory: &Path,
    container_directory: &Path,
    conn: &mut diesel::PgConnection,
) -> Result<(), crate::errors::Error> {
    // Load the CSV directory using `csqlv`.
    let schema = CSVSchemaBuilder::default()
        .include_gz()
        .singularize()
        .container_directory(container_directory.to_path_buf())
        .from_dir(csv_directory)
        .expect("Failed to load CSV directory");

    let sql_generation_options: SQLGenerationOptions =
        SQLGenerationOptions::default().include_extensions();

    schema.create(conn, &sql_generation_options)?;

    Ok(())
}
