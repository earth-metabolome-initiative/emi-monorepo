//! Submodule providing the FormatMatch trait and its blanked implementation
//! for all types that implement ToString.
use sublime_fuzzy::{best_match, format_simple};

/// Trait formatting string for a given query using sublime_fuzzy.
pub trait FormatMatch {
    /// Format the string for a given query using sublime_fuzzy.
    ///
    /// # Arguments
    /// * `query` - The query to match against.
    fn format_match<S: AsRef<str>>(&self, query: S) -> yew::Html;

    /// Returns the similarity score of the implementing type with respect to the query.
    ///
    /// # Arguments
    /// * `query` - The query to compare the implementing type with.
    fn similarity_score<S: AsRef<str>>(&self, query: S) -> isize;
}

impl<T: ToString> FormatMatch for T {
    fn format_match<S: AsRef<str>>(&self, query: S) -> yew::Html {
        let current = self.to_string();
        yew::Html::from_html_unchecked(yew::AttrValue::from(
            best_match(query.as_ref(), &current).map_or_else(
                || current.clone(),
                |match_value| format_simple(&match_value, &current, "<strong>", "</strong>"),
            ),
        ))
    }

    fn similarity_score<S: AsRef<str>>(&self, query: S) -> isize {
        let current = self.to_string();
        best_match(query.as_ref(), &current).map_or(0, |match_value| match_value.score())
    }
}
