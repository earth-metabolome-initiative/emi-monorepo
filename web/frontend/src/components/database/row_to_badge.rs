//! Submodule defining a trait to be implemented by types that can be converted to a badge.

/// Trait for types that can be converted to a badge.
pub trait RowToBadge {
    /// Convert the implementing type to a badge.
    /// 
    /// # Arguments
    /// * `target` - The optional target whose values are to be highlighted
    ///              using the sublime_fuzzy best_match and format_simple methods.
    fn to_badge(&self, target: Option<&str>) -> yew::Html;
}