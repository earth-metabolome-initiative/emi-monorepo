//! Submodule defining a trait to be implemented by types that can be converted to a badge.

pub mod bio_ott_ranks;
pub mod bio_ott_taxon_items;
pub mod colors;
pub mod document_formats;
pub mod font_awesome_icons;
pub mod observations;
pub mod organizations;
pub mod project_states;
pub mod projects;
pub mod sample_states;
pub mod spectra_collections;
pub mod teams;
pub mod units;
pub mod users;

/// Trait for types that can be converted to a badge.
pub trait RowToSearchableBadge {
    /// Convert the implementing type to a badge.
    ///
    /// # Arguments
    /// * `query` - The optional query whose values are to be highlighted
    ///              using the sublime_fuzzy best_match and format_simple methods.
    fn to_datalist_badge(&self, query: &str) -> yew::Html;

    /// Converts the implementing type to a selected badge.
    fn to_selected_datalist_badge(&self) -> yew::Html;

    /// Returns the similarity score of the implementing type with respect to the query.
    ///
    /// # Arguments
    /// * `query` - The query to compare the implementing type with.
    fn similarity_score(&self, query: &str) -> isize;

    /// Returns the class of the primary color of this badge.
    fn primary_color_class(&self) -> &str;

    /// Returns the description of this badge.
    fn description(&self) -> &str;
}
pub mod nameplate_categories;
pub mod nameplates;
pub mod organisms;
pub mod roles;
pub mod sample_container_categories;
pub mod sample_containers;
pub mod samples;
pub mod team_states;
