use super::RowToSearchableBadge;
use crate::traits::format_match::FormatMatch;
use web_common::database::User;
use yew::prelude::*;

impl RowToSearchableBadge for User {
    fn to_datalist_badge(&self, query: &str) -> Html {
        html! {
            <div>
                <p>
                    <span>{self.first_name.format_match(query)}</span>
                </p>
            </div>
        }
    }

    fn to_selected_datalist_badge(&self) -> Html {
        html! {
            <p>
                <span>{&self.first_name}</span>
            </p>
        }
    }

    fn matches(&self, query: &str) -> bool {
        self.first_name == query
    }

    fn similarity_score(&self, query: &str) -> isize {
        self.first_name.similarity_score(query)
    }

    fn primary_color_class(&self) -> &str {
        "grey"
    }

    fn description(&self) -> &str {
        "The user's full name."
    }
}
