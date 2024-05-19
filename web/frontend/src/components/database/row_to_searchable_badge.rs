//! Submodule defining a trait to be implemented by types that can be converted to a badge.

pub mod project_states;
pub mod sample_states;
pub mod projects;
pub mod bio_ott_taxon_items;
pub mod font_awesome_icons;
pub mod bio_ott_ranks;
pub mod colors;
pub mod users;
pub mod document_formats;
pub mod units;
pub mod organizations;
pub mod teams;

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
pub mod roles;
