//! Submodule providing the FormatMatch trait and its blanked implementation
//! for all types that implement ToString.
use sublime_fuzzy::{best_match, format_simple};

fn custom_best_matches<S: AsRef<str>, T: AsRef<str>>(
    query: S,
    target: T,
) -> Vec<sublime_fuzzy::Match> {
    query
        .as_ref()
        .split_whitespace()
        .map(|query| best_match(query, target.as_ref()))
        .filter_map(|x| x)
        .collect::<Vec<sublime_fuzzy::Match>>()
}

/// Trait formatting string for a given query using sublime_fuzzy.
pub trait FormatMatch {
    /// Format the string for a given query using sublime_fuzzy.
    ///
    /// # Arguments
    /// * `query` - The query to match against.
    fn format_match<S: AsRef<str>>(&self, query: S) -> yew::Html;

    /// Format the string for a given query using sublime_fuzzy if the query is present.
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
        query
            .map(|query| self.format_match(query))
            .unwrap_or_else(|| yew::Html::from(self.as_ref().to_string()))
    }
}
