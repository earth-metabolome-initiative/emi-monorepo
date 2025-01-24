//! Build the core structures.
use csqlv::{CSVSchema, CSVSchemaBuilder};
use taxonomy_fetcher::impls::ncbi::{NCBIRank, NCBITaxonomy, NCBITaxonomyBuilder};
use taxonomy_fetcher::{Rank, Taxonomy, TaxonomyBuilder};

#[tokio::main]
pub async fn main() {
    NCBIRank::to_csv("csvs/ranks.csv").unwrap();

    let taxonomy: NCBITaxonomy = NCBITaxonomyBuilder::latest().build().await.unwrap();

    taxonomy.to_csv("csvs/taxa.csv").unwrap();

    let schema: CSVSchema = CSVSchemaBuilder::default()
        // To show a loading bar while processing the CSVs
        .verbose()
        // To include compressed files such as .gz
        .include_gz()
        // For supporting running the tests within
        // containers such as Docker
        .singularize()
        .container_directory("/app/bands")
        .from_dir("./csvs")
        .unwrap();

    let sql: String = schema.to_postgres().unwrap();

    // let mut conn = establish_connection_to_postgres();

    // We write to the target directory the generated structs

    // Get the output directory
    // let out_dir = env::var("OUT_DIR").unwrap();

    // Path to the file to create
    // let path = Path::new(&out_dir).join("directus_structs.rs");

    // let curation_data = Table::load(&mut conn, "Curation_Data", None, "directus").unwrap();

    // Codegen::default()
    //     .set_output_path(path.as_ref())
    //     .add_table_to_deny_list(&curation_data)
    //     .generate(&mut conn, "directus", None)
    //     .unwrap();
}
