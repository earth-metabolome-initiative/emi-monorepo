//! Submodule providing the `FormatMatch` trait and its blanked implementation
//! for all types that implement `ToString`.
use sublime_fuzzy::{best_match, format_simple};

/// Trait formatting string for a given query using `sublime_fuzzy`.
pub trait FormatMatch {
    /// Format the string for a given query using `sublime_fuzzy`.
    ///
    /// # Arguments
    /// * `query` - The query to match against.
    fn format_match<S: AsRef<str>>(&self, query: S) -> yew::Html;

    /// Format the string for a given query using `sublime_fuzzy` if the query
    /// is present.
    ///
    /// # Arguments
    /// * `query` - The optional query to match against.
    fn maybe_format_match<S: AsRef<str>>(&self, query: Option<S>) -> yew::Html;
}

impl<T: AsRef<str>> FormatMatch for T {
    fn format_match<S: AsRef<str>>(&self, query: S) -> yew::Html {
        yew::Html::from_html_unchecked(yew::AttrValue::from(
            best_match(query.as_ref(), self.as_ref()).map_or_else(
                || self.as_ref().to_owned(),
                |match_value| format_simple(&match_value, self.as_ref(), "<strong>", "</strong>"),
            ),
        ))
    }

    fn maybe_format_match<S: AsRef<str>>(&self, query: Option<S>) -> yew::Html {
        query.map_or_else(
            || yew::Html::from(self.as_ref().to_string()),
            |query| self.format_match(query),
        )
    }
}
