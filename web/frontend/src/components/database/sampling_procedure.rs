/// impl row to badge for project state
/// add function components
use crate::{components::database::row_to_badge::RowToBadge, traits::FormatMatch};
use web_common::database::SamplingProcedure;
use yew::prelude::*;

impl RowToBadge for SamplingProcedure {
    fn to_datalist_badge(&self, query: &str) -> Html {
        html! {
            <div>
                <p>
                    // <i class={format!("{} {}", self.font_awesome_icon, self.icon_color)}></i>
                    <span>{self.name.format_match(query)}</span>
                </p>
                <p class="description">{self.description.as_ref().map(|description| description.format_match(query))}</p>
            </div>
        }
    }

    fn to_selected_datalist_badge(&self) -> Html {
        html! {
            <p>
                // <i class={format!("{} {}", self.font_awesome_icon, self.icon_color)}></i>
                <span>{&self.name}</span>
            </p>
        }
    }
    

    fn matches(&self, query: &str) -> bool {
        self.name == query
    }

    fn similarity_score(&self, query: &str) -> isize {
        self.name.similarity_score(query) + self.description.as_ref().map_or(0, |description| description.similarity_score(query))
    }

    fn primary_color_class(&self) -> &str {
        "grey"
    }

    fn description(&self) -> &str {
        &self.description.as_ref().map_or("", |description| description.as_str())
    }
}
