//! Test whether the NCBI taxonomy fetcher works as expected.

use indicatif::{ProgressBar, ProgressIterator};
use strum::IntoEnumIterator;
use taxonomy_fetcher::{
    impls::ncbi::{NCBITaxonomyBuilder, NCBIVersion},
    TaxonomyBuilder,
};

#[tokio::test]
async fn test_all_ncbi() {
    let mut pb = ProgressBar::new(NCBIVersion::iter().count() as u64);

    pb = pb.with_style(indicatif::ProgressStyle::default_bar()
	.template("NCBI: {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})").unwrap()
	.progress_chars("#>-"),);

    for version in NCBIVersion::iter().progress_with(pb) {
        // We create a new OTOL taxonomy fetcher.
        let fetcher = NCBITaxonomyBuilder::default().version(version);

        // We fetch the taxonomy.
        let taxonomy = fetcher.build().await.unwrap();
    }
}
