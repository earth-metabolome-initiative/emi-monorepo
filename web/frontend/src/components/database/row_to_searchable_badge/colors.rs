use super::RowToSearchableBadge;
use crate::traits::format_match::FormatMatch;
use web_common::database::Color;
use web_common::traits::CapitalizeString;
use yew::prelude::*;

impl RowToSearchableBadge for Color {
    fn to_datalist_badge(&self, query: &str) -> Html {
        html! {
            <div>
                <p>
                    <i class={format!("fas fa-paint-roller {}", self.name)}></i>
                    <span>{self.name.capitalize().format_match(query)}</span>
                </p>
                <p>{self.description.format_match(query)}</p>
            </div>
        }
    }

    fn to_selected_datalist_badge(&self) -> Html {
        html! {
            <div>
                <p>
                    <i class={format!("fas fa-paint-roller {}", self.name)}></i>
                    <span>{self.name.capitalize()}</span>
                </p>
            </div>
        }
    }

    fn similarity_score(&self, query: &str) -> isize {
        self.name.similarity_score(query) + self.description.similarity_score(query)
    }

    fn primary_color_class(&self) -> &str {
        &self.name
    }
    
    fn description(&self) -> &str {
        &self.description
    }
}
