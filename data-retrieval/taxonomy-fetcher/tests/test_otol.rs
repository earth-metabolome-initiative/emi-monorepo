//! Test whether the OTOL taxonomy fetcher works as expected.

use strum::IntoEnumIterator;
use taxonomy_fetcher::impls::open_tree_of_life::OpenTreeOfLifeTaxonomyBuilder;
use taxonomy_fetcher::impls::open_tree_of_life::OpenTreeOfLifeVersion;
use taxonomy_fetcher::TaxonomyBuilder;

#[tokio::test]
async fn test_all_otol() {
    for version in OpenTreeOfLifeVersion::iter() {
        // We create a new OTOL taxonomy fetcher.
        let fetcher = OpenTreeOfLifeTaxonomyBuilder::default().version(version);

        // We fetch the taxonomy.
        let taxonomy = fetcher.build().await.unwrap();
    }
}
