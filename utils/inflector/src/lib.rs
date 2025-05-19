#![doc = include_str!("../README.md")]

use std::sync::LazyLock;

use regex::Regex;

static SPECTRUM_PLURAL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("(?i)(spectr|append)um$").unwrap());
static SPECTRUM_SINGULAR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("(?i)(spectr|append)a$").unwrap());
static MATRIX_PLURAL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("(?i)(matr|append)ix$").unwrap());
static MATRIX_SINGULAR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("(?i)(matr|append)ices$").unwrap());
static TAXON_PLURAL: LazyLock<Regex> = LazyLock::new(|| Regex::new("(?i)(tax|append)on$").unwrap());
static TAXON_SINGULAR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("(?i)(tax|append)a$").unwrap());

#[derive(Debug, Clone, Copy)]
/// A struct that provides methods to pluralize and singularize words.
pub struct Inflector {
    _private: (),
}

impl Default for Inflector {
    fn default() -> Self {
        pluralizer::add_plural_rule(SPECTRUM_PLURAL.clone(), "$1a".to_string());
        pluralizer::add_singular_rule(SPECTRUM_SINGULAR.clone(), "$1um".to_string());
        pluralizer::add_plural_rule(MATRIX_PLURAL.clone(), "$1ices".to_string());
        pluralizer::add_singular_rule(MATRIX_SINGULAR.clone(), "$1ix".to_string());
        pluralizer::add_plural_rule(TAXON_PLURAL.clone(), "$1a".to_string());
        pluralizer::add_singular_rule(TAXON_SINGULAR.clone(), "$1on".to_string());

        Self { _private: () }
    }
}

impl Inflector {
    #[must_use]
    /// Returns the plural form of a word.
    pub fn pluralize(&self, word: &str) -> String {
        pluralizer::pluralize(word, 5, false)
    }

    #[must_use]
    /// Returns the singular form of a word.
    pub fn singularize(&self, word: &str) -> String {
        pluralizer::pluralize(word, 1, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pluralize() {
        assert_eq!(Inflector::default().pluralize("country"), "countries");
        assert_eq!(Inflector::default().pluralize("taxon"), "taxa");
        assert_eq!(Inflector::default().pluralize("Taxon"), "Taxa");
        assert_eq!(Inflector::default().pluralize("elephant"), "elephants");
        assert_eq!(Inflector::default().pluralize("matrix"), "matrices");
        assert_eq!(Inflector::default().pluralize("table"), "tables");
        assert_eq!(Inflector::default().pluralize("house"), "houses");
        assert_eq!(Inflector::default().pluralize("column"), "columns");
        assert_eq!(Inflector::default().pluralize("spectrum"), "spectra");
        assert_eq!(Inflector::default().pluralize("is_nullable"), "is_nullables");
        assert_eq!(Inflector::default().pluralize("is_self_referencing"), "is_self_referencings");
        assert_eq!(Inflector::default().pluralize("is_identity"), "is_identities");
        assert_eq!(Inflector::default().pluralize("identity_generation"), "identity_generations");
        assert_eq!(Inflector::default().pluralize("identity_start"), "identity_starts");
        assert_eq!(Inflector::default().pluralize("identity_increment"), "identity_increments");
    }

    #[test]
    fn test_singularize() {
        assert_eq!(Inflector::default().singularize("countries"), "country");
        assert_eq!(Inflector::default().singularize("Taxa"), "Taxon");
        assert_eq!(Inflector::default().singularize("taxa"), "taxon");
        assert_eq!(Inflector::default().singularize("elephants"), "elephant");
        assert_eq!(Inflector::default().singularize("taxal"), "taxal");
        assert_eq!(Inflector::default().singularize("matrices"), "matrix");
        assert_eq!(Inflector::default().singularize("tables"), "table");
        assert_eq!(Inflector::default().singularize("houses"), "house");
        assert_eq!(Inflector::default().singularize("columns"), "column");
        assert_eq!(Inflector::default().singularize("spectra"), "spectrum");
        assert_eq!(Inflector::default().singularize("is_nullables"), "is_nullable");
        assert_eq!(Inflector::default().singularize("is_self_referencings"), "is_self_referencing");
        assert_eq!(Inflector::default().singularize("is_identities"), "is_identity");
        assert_eq!(Inflector::default().singularize("identity_generations"), "identity_generation");
        assert_eq!(Inflector::default().singularize("identity_starts"), "identity_start");
        assert_eq!(Inflector::default().singularize("identity_increments"), "identity_increment");
    }
}
