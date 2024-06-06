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

    /// Returns the similarity score of the implementing type with respect to the query.
    ///
    /// # Arguments
    /// * `query` - The query to compare the implementing type with.
    fn similarity_score<S: AsRef<str>>(&self, query: S) -> isize;
}

impl<T: AsRef<str>> FormatMatch for T {
    fn format_match<S: AsRef<str>>(&self, query: S) -> yew::Html {
        let mut matches = custom_best_matches(query, self);
        if matches.is_empty() {
            yew::Html::from(self.as_ref().to_string())
        } else {
            // In order to properly format the string, we need to sort the matches
            // by their start index. If it happens that after sorting a match starts
            // before the previous match ends, we keep the largest match.
            matches.sort_by(|a, b| a.matched_indices().min().cmp(&b.matched_indices().min()));

            let start = matches
                .first()
                .unwrap()
                .matched_indices()
                .min()
                .unwrap()
                .clone();
            let mut prev_end = matches
                .first()
                .unwrap()
                .matched_indices()
                .max()
                .unwrap()
                .clone()
                + 1;

            let mut formatted = self.as_ref()[..start].to_string();

            formatted.push_str("<strong>");
            formatted.push_str(&self.as_ref()[start..prev_end]);
            formatted.push_str("</strong>");

            for m in matches.into_iter().skip(1) {
                let start = m.matched_indices().min().unwrap().clone();
                let end = m.matched_indices().max().unwrap().clone() + 1;

                if start > prev_end {
                    formatted.push_str(&self.as_ref()[prev_end..start]);
                    formatted.push_str("<strong>");
                    formatted.push_str(&self.as_ref()[start..end]);
                    formatted.push_str("</strong>");
                } else {
                    continue;
                }

                prev_end = end;
            }

            formatted.push_str(&self.as_ref()[prev_end..]);

            yew::Html::from_html_unchecked(yew::AttrValue::from(formatted))
        }
    }

    fn maybe_format_match<S: AsRef<str>>(&self, query: Option<S>) -> yew::Html {
        query
            .map(|query| self.format_match(query))
            .unwrap_or_else(|| yew::Html::from(self.as_ref().to_string()))
    }

    fn similarity_score<S: AsRef<str>>(&self, query: S) -> isize {
        let matches = custom_best_matches(query, self);
        if matches.is_empty() {
            return 0;
        }
        matches.iter().map(|m| m.score()).sum::<isize>() / matches.len() as isize
    }
}
