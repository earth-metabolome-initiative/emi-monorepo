//! Test whether the NCBI taxonomy fetcher works as expected.

use taxonomy_fetcher::{TaxonomyBuilder, impls::ncbi::NCBITaxonomyBuilder};

#[tokio::test]
async fn test_all_ncbi() {
    // We create a new OTOL taxonomy fetcher.
    let fetcher = NCBITaxonomyBuilder::latest();

    // We fetch the taxonomy.
    let _taxonomy = fetcher.build().await.unwrap();
}
