use super::RowToSearchableBadge;
use crate::traits::format_match::FormatMatch;
use web_common::database::NestedSample;
use yew::prelude::*;

impl RowToSearchableBadge for NestedSample {
    fn to_searchable_badge(&self, query: Option<&str>) -> Html {
        html! {
            <div>
                <p>
                <i class="fas fa-question grey"></i>
                    <span>{self.inner.id.maybe_format_match(query)}</span>
                </p>
            </div>
        }
    }

    fn similarity_score(&self, query: &str) -> isize {
        self.inner.id.similarity_score(query)
    }
    fn primary_color_class(&self) -> &str {
        "grey"
    }
}
