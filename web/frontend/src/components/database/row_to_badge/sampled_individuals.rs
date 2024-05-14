use super::RowToBadge;
use crate::traits::format_match::FormatMatch;
use web_common::database::NestedSampledIndividual;
use yew::prelude::*;

impl RowToBadge for NestedSampledIndividual {
    fn to_datalist_badge(&self, query: &str) -> Html {
        html! {
            <div>
                <p>
                    <span>{self.inner.name.clone().unwrap_or_else(|| "Nested individual".to_string()).format_match(query)}</span>
                </p>
            </div>
        }
    }

    fn to_selected_datalist_badge(&self) -> Html {
        html! {
            <div>
                <p>
                    <span>{self.inner.name.clone().unwrap_or_else(|| "Nested individual".to_string())}</span>
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