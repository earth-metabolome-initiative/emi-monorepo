//! Submodule defining a trait to be implemented by types that can be converted to a badge.

use super::row_to_badge::RowToBadge;

pub mod bio_ott_ranks;
pub mod bio_ott_taxon_items;
pub mod colors;
pub mod countries;
pub mod document_formats;
pub mod font_awesome_icons;
pub mod nameplate_categories;
pub mod nameplates;
pub mod observation_subjects;
pub mod observations;
pub mod organisms;
pub mod organizations;
pub mod project_states;
pub mod projects;
pub mod roles;
pub mod sample_container_categories;
pub mod sample_containers;
pub mod sample_states;
pub mod samples;
pub mod spectra_collections;
pub mod team_states;
pub mod teams;
pub mod units;
pub mod users;

/// Trait for types that can be converted to a badge.
pub trait RowToSearchableBadge: RowToBadge {
    /// Convert the implementing type to a badge.
    ///
    /// # Arguments
    /// * `query` - The optional query whose values are to be highlighted
    ///              using the sublime_fuzzy best_match and format_simple methods.
    fn to_searchable_badge(&self, query: Option<&str>) -> yew::Html;

    /// Convert the implementing type to a small badge.
    ///
    /// # Arguments
    /// * `query` - The optional query whose values are to be highlighted
    ///             using the sublime_fuzzy best_match and format_simple methods.
    fn to_searchable_small_badge(&self, query: Option<&str>) -> yew::Html {
        self.to_searchable_badge(query)
    }

    /// Returns the similarity score of the implementing type with respect to the query.
    ///
    /// # Arguments
    /// * `query` - The query to compare the implementing type with.
    fn similarity_score(&self, query: &str) -> isize;

    /// Returns the class of the primary color of this badge.
    fn primary_color_class(&self) -> &str;
}
