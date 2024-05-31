use super::RowToSearchableBadge;
use crate::traits::format_match::FormatMatch;
use web_common::database::User;
use yew::prelude::*;

impl RowToSearchableBadge for User {
    fn to_searchable_badge(&self, query: Option<&str>) -> Html {
        html! {
            <div>
                <p>
                    <span>{self.first_name.maybe_format_match(query)}</span>
                </p>
            </div>
        }
    }

    fn similarity_score(&self, query: &str) -> isize {
        self.first_name.similarity_score(query)
    }

    fn primary_color_class(&self) -> &str {
        "grey"
    }
}
