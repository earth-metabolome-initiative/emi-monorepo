/// impl row to badge for team state
/// add function components
use crate::{components::database::row_to_badge::RowToBadge, traits::FormatMatch};
use web_common::database::TeamState;
use yew::prelude::*;

impl RowToBadge for TeamState {
    fn to_datalist_badge(&self, query: &str) -> Html {
        html! {
            <div>
                <p>
                    <i class={format!("{} {}", self.font_awesome_icon, self.icon_color)}></i>
                    <span>{self.name.format_match(query)}</span>
                </p>
                <p class="description">{self.description.format_match(query)}</p>
            </div>
        }
    }

    fn to_selected_datalist_badge(&self) -> Html {
        html! {
            <p>
                <i class={format!("{} {}", self.font_awesome_icon, self.icon_color)}></i>
                <span>{&self.name}</span>
            </p>
        }
    }
    

    fn matches(&self, query: &str) -> bool {
        self.name == query
    }

    fn similarity_score(&self, query: &str) -> isize {
        self.name.similarity_score(query) + self.description.similarity_score(query)
    }

    fn primary_color_class(&self) -> &str {
        &self.icon_color
    }

    fn description(&self) -> &str {
        &self.description
    }
}
