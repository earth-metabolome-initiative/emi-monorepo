//! Test whether the COL taxonomy fetcher works as expected.

use strum::IntoEnumIterator;
use taxonomy_fetcher::impls::CatalogOfLifeTaxonomyBuilder;
use taxonomy_fetcher::impls::CatalogOfLifeVersion;
use taxonomy_fetcher::TaxonomyBuilder;

#[tokio::test]
async fn test_all_col() {
    // for version in CatalogOfLifeVersion::iter() {
    //     // We create a new COL taxonomy fetcher.
    //     let fetcher = CatalogOfLifeTaxonomyBuilder::default().version(version);

    //     // We fetch the taxonomy.
    //     let taxonomy = fetcher.build().await.unwrap();
    // }
    // We create a new COL taxonomy fetcher.
    let fetcher = CatalogOfLifeTaxonomyBuilder::latest();

    // We fetch the taxonomy.
    let taxonomy = fetcher.build().await.unwrap();
}
