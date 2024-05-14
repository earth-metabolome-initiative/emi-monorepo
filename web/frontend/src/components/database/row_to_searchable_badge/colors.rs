use super::RowToSearchableBadge;
use crate::traits::format_match::FormatMatch;
use web_common::database::Color;
use yew::prelude::*;

impl RowToSearchableBadge for Color {
    fn to_datalist_badge(&self, query: &str) -> Html {
        html! {
            <div>
                <p>
                    <i class={format!("fas fa-paint-roller {}", self.name)}></i>
                    <span>{self.name.format_match(query)}</span>
                </p>
            </div>
        }
    }

    fn to_selected_datalist_badge(&self) -> Html {
        html! {
            <div>
                <p>
                    <i class={format!("fas fa-paint-roller {}", self.name)}></i>
                    <span>{self.name.clone()}</span>
                </p>
            </div>
        }
    }
    fn matches(&self, query: &str) -> bool {
        self.name == query
    }
    fn similarity_score(&self, query: &str) -> isize {
        self.name.similarity_score(query)
    }
    fn primary_color_class(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        ""
    }
}
