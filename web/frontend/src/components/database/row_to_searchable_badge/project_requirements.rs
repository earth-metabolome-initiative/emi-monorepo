use super::RowToSearchableBadge;
use crate::traits::format_match::FormatMatch;
use web_common::database::NestedProjectRequirement;
use yew::prelude::*;

impl RowToSearchableBadge for NestedProjectRequirement {
    fn to_datalist_badge(&self, query: &str) -> Html {
        html! {
            <div>
                <p>
                    <span>{"Project Requirement"}</span>
                </p>
            </div>
        }
    }

    fn to_selected_datalist_badge(&self) -> Html {
        html! {
            <div>
                <p>
                <span>{"Project Requirement"}</span>
                </p>
            </div>
        }
    }
    fn matches(&self, query: &str) -> bool {
        false
    }
    fn similarity_score(&self, query: &str) -> isize {
        0
    }
    fn primary_color_class(&self) -> &str {
        "gray"
    }
    fn description(&self) -> &str {
        ""
    }
}
