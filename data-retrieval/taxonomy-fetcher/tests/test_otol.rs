//! Test whether the OTOL taxonomy fetcher works as expected.

use taxonomy_fetcher::{TaxonomyBuilder, impls::open_tree_of_life::OpenTreeOfLifeTaxonomyBuilder};

#[tokio::test]
async fn test_all_otol() {
    // We create a new OTOL taxonomy fetcher.
    let fetcher = OpenTreeOfLifeTaxonomyBuilder::latest();

    // We fetch the taxonomy.
    let _taxonomy = fetcher.build().await.unwrap();
}
