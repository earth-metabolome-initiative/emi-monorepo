//! Contains enum of the different templates available in Directus

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub(crate) enum DirectusTemplates {
    DBGI,
    VineshPartOfOrganism,
    VineshWholeOrganism,
}
