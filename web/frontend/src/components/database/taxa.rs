/// impl row to badge for associated taxa
/// add function components
use crate::{components::database::row_to_badge::RowToBadge, traits::FormatMatch};
use web_common::database::Taxa;
use yew::prelude::*;

impl RowToBadge for Taxa {
    fn to_datalist_badge(&self, query: &str) -> Html {
        html! {
            <div>
                <p>
                    // <i class={format!("{} {}", self.font_awesome_icon, self.icon_color)}></i>
                    <span>{self.taxon_display().format_match(query)}</span>
                </p>
            </div>
        }
    }

    fn to_selected_datalist_badge(&self) -> Html {
        html! {
            <p>
                // <i class={format!("{} {}", self.font_awesome_icon, self.icon_color)}></i>
                <span>{&self.taxon_display()}</span>
            </p>
        }
    }
    

    fn matches(&self, query: &str) -> bool {
        self.taxon_display() == query
    }

    fn similarity_score(&self, query: &str) -> isize {
        self.taxon_display().similarity_score(query)
    }

    fn primary_color_class(&self) -> &str {
        "grey"
    }

    fn description(&self) -> &str {
        "The taxon name."
    }
}
