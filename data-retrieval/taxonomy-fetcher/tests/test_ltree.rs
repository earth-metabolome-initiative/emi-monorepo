//! Test whether the NCBI taxonomy fetcher works as expected.

use indicatif::ProgressBar;
use indicatif::ProgressIterator;
use strum::IntoEnumIterator;
use taxonomy_fetcher::impls::ncbi::NCBITaxonomyBuilder;
use taxonomy_fetcher::impls::ncbi::NCBIVersion;
use taxonomy_fetcher::Taxon;
use taxonomy_fetcher::Taxonomy;
use taxonomy_fetcher::TaxonomyBuilder;

#[tokio::test]
async fn test_ltree_path() {
    // We create a new OTOL taxonomy fetcher.
    let fetcher = NCBITaxonomyBuilder::latest();

    // We fetch the taxonomy.
    let taxonomy = fetcher.build().await.unwrap();

    let taxon = taxonomy.taxon_by_id(&2).unwrap();

    assert_eq!("root.cellular organisms.Bacteria", taxon.ltree_path());
}
