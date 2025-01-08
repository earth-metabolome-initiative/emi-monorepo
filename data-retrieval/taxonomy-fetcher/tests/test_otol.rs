//! Test whether the OTOL taxonomy fetcher works as expected.

use taxonomy_fetcher::TaxonomyBuilder;
use taxonomy_fetcher::impls::OpenTreeOfLifeTaxonomyBuilder;

#[tokio::test]
async fn test_otol() {
    // We create a new OTOL taxonomy fetcher.
    let fetcher = OpenTreeOfLifeTaxonomyBuilder::latest();

    // We fetch the taxonomy.
    let taxonomy = fetcher.build().await.unwrap();
}
