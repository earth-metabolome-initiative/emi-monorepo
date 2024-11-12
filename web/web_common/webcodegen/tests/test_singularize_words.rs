use pluralizer;
use regex::Regex;
#[test]
fn singluarize() -> () {
    pluralizer::add_singular_rule(
        Regex::new("(?i)(spectr|append)a$").unwrap(),
        "$1um".to_string(),
    );
    pluralizer::add_singular_rule(
        Regex::new("(?i)(matr|append)ices$").unwrap(),
        "$1ix".to_string(),
    );
    pluralizer::add_singular_rule(
        Regex::new("(?i)(tax|append)a$").unwrap(),
        "$1on".to_string(),
    );

    let words = vec![
        "countries",
        "Taxa",
        "elephants",
        "taxal",
        "matrices",
        "tables",
        "houses",
        "columns",
        "spectra",
        "is_nullable",
        "is_self_referencing",
        "is_identity",
        "identity_generation",
        "identity_start",
        "identity_increment",
    ];

    let expected = vec![
        "country",
        "Taxon",
        "elephant",
        "taxal",
        "matrix",
        "table",
        "house",
        "column",
        "spectrum",
        "is_nullable",
        "is_self_referencing",
        "is_identity",
        "identity_generation",
        "identity_start",
        "identity_increment",
    ];

    let singularized = words
        .iter()
        .map(|word| pluralizer::pluralize(word, 1, false))
        .collect::<Vec<String>>();

    assert_eq!(singularized, expected);
}
