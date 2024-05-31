use super::RowToSearchableBadge;
use crate::traits::format_match::FormatMatch;
use web_common::database::Unit;
use yew::prelude::*;

impl RowToSearchableBadge for Unit {
    fn to_searchable_badge(&self, query: Option<&str>) -> Html {
        html! {
            <div>
                <p>
                <i class="fas fa-question grey"></i>
                    <span>{self.name.maybe_format_match(query)}</span>
                    <span>{self.description.maybe_format_match(query)}</span>
                    <span>{self.symbol.maybe_format_match(query)}</span>
                </p>
            </div>
        }
    }
    fn similarity_score(&self, query: &str) -> isize {
        self.name.similarity_score(query)
            + self.description.similarity_score(query)
            + self.symbol.similarity_score(query)
    }
    fn primary_color_class(&self) -> &str {
        "grey"
    }
}
