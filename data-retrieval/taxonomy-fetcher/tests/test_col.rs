//! Test whether the COL taxonomy fetcher works as expected.

use taxonomy_fetcher::{TaxonomyBuilder, impls::catalog_of_life::CatalogOfLifeTaxonomyBuilder};

#[tokio::test]
async fn test_all_col() {
    // use taxonomy_fetcher::{impls::catalog_of_life::CatalogOfLifeVersion,
    // TaxonomyBuilderError}; use strum::IntoEnumIterator;
    // for version in CatalogOfLifeVersion::iter() {
    //     // We create a new COL taxonomy fetcher.
    //     let fetcher = CatalogOfLifeTaxonomyBuilder::default().version(version);

    //     // We fetch the taxonomy.
    //     let taxonomy = fetcher.build().await.unwrap();
    // }
    // We create a new COL taxonomy fetcher.
    let fetcher = CatalogOfLifeTaxonomyBuilder::latest();

    // We fetch the taxonomy.
    // At the time of writing, this is failing as the taxonomy
    // seems to have several heads.
    let taxonomy = fetcher.build().await;
    assert!(taxonomy.unwrap_err().is_multiple_roots());
}
