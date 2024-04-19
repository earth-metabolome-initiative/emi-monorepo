//! Submodule defining a trait to be implemented by types that can be converted to a badge.

/// Trait for types that can be converted to a badge.
pub trait RowToBadge {
    /// Convert the implementing type to a badge.
    ///
    /// # Arguments
    /// * `query` - The optional query whose values are to be highlighted
    ///              using the sublime_fuzzy best_match and format_simple methods.
    fn to_datalist_badge(&self, query: &str) -> yew::Html;
    
    /// Converts the implementing type to a selected badge.
    fn to_selected_datalist_badge(&self) -> yew::Html;

    /// Returns whether the implementing type matches completely with the query.
    /// 
    /// # Arguments
    /// * `query` - The query to compare the implementing type with.
    fn matches(&self, query: &str) -> bool;

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
