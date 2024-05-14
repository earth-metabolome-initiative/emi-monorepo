//! Submodule defining a trait to be implemented by types that can be converted to a badge.


/// Trait for types that can be converted to a badge.
pub trait RowToBadge {
    /// Convert the implementing type to a badge.
    ///
    /// # Arguments
    /// * `query` - The optional query whose values are to be highlighted
    ///              using the sublime_fuzzy best_match and format_simple methods.
    fn to_badge(&self, query: &str) -> yew::Html;
}
