use super::RowToSearchableBadge;
use crate::traits::format_match::FormatMatch;
use web_common::database::NestedProcedure;
use yew::prelude::*;

impl RowToSearchableBadge for NestedProcedure {
    fn to_datalist_badge(&self, query: &str) -> Html {
        html! {
            <div>
                <p>
                    <span>{self.inner.name.format_match(query)}</span>
                </p>
            </div>
        }
    }

    fn to_selected_datalist_badge(&self) -> Html {
        html! {
            <div>
                <p>
                    <span>{self.inner.name.clone()}</span>
                </p>
            </div>
        }
    }
    fn matches(&self, query: &str) -> bool {
        self.inner.name == query
    }
    fn similarity_score(&self, query: &str) -> isize {
        self.inner.name.similarity_score(query)
    }
    fn primary_color_class(&self) -> &str {
        "gray"
    }
    fn description(&self) -> &str {
        ""
    }
}
