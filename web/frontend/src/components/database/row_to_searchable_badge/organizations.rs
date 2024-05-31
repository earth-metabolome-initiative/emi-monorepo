use super::RowToSearchableBadge;
use crate::traits::format_match::FormatMatch;
use web_common::database::NestedOrganization;
use yew::prelude::*;

impl RowToSearchableBadge for NestedOrganization {
    fn to_searchable_badge(&self, query: Option<&str>) -> Html {
        html! {
            <div>
                <p>
                    <i class="fas fa-building-columns grey"></i>
                    {'\u{00A0}'}
                    <span>{self.inner.name.maybe_format_match(query)}</span>
                </p>
            </div>
        }
    }
    fn similarity_score(&self, query: &str) -> isize {
        self.inner.name.similarity_score(query)
    }
    fn primary_color_class(&self) -> &str {
        "grey"
    }
}
