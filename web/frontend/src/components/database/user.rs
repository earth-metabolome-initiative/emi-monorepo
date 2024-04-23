/// impl row to badge for project state
/// add function components
use crate::{components::database::row_to_badge::RowToBadge, traits::FormatMatch};
use web_common::database::User;
use yew::prelude::*;

impl RowToBadge for User {
    fn to_datalist_badge(&self, query: &str) -> Html {
        html! {
            <div>
                <p>
                    // <i class={format!("{} {}", self.font_awesome_icon, self.icon_color)}></i>
                    <span>{self.full_name().format_match(query)}</span>
                </p>
            </div>
        }
    }

    fn to_selected_datalist_badge(&self) -> Html {
        html! {
            <p>
                // <i class={format!("{} {}", self.font_awesome_icon, self.icon_color)}></i>
                <span>{&self.full_name()}</span>
            </p>
        }
    }
    

    fn matches(&self, query: &str) -> bool {
        self.full_name() == query
    }

    fn similarity_score(&self, query: &str) -> isize {
        self.full_name().similarity_score(query)
    }

    fn primary_color_class(&self) -> &str {
        "grey"
    }

    fn description(&self) -> &str {
        "The user's full name."
    }
}
