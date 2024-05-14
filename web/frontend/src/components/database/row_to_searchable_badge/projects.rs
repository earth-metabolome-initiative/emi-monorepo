/// impl row to badge for project state
/// add function components
use crate::{components::database::row_to_searchable_badge::RowToSearchableBadge, traits::FormatMatch};
use web_common::database::NestedProject;
use yew::prelude::*;

impl RowToSearchableBadge for NestedProject {
    fn to_datalist_badge(&self, query: &str) -> Html {
        html! {
            <div>
                <p>
                    // <i class={format!("{} {}", self.font_awesome_icon, self.icon_color)}></i>
                    <span>{self.inner.name.format_match(query)}</span>
                </p>
                <p class="description">{self.inner.description.format_match(query)}</p>
            </div>
        }
    }

    fn to_selected_datalist_badge(&self) -> Html {
        html! {
            <p>
                // <i class={format!("{} {}", self.font_awesome_icon, self.icon_color)}></i>
                <span>{&self.inner.name}</span>
            </p>
        }
    }
    

    fn matches(&self, query: &str) -> bool {
        self.inner.name == query
    }

    fn similarity_score(&self, query: &str) -> isize {
        self.inner.name.similarity_score(query) + self.inner.description.similarity_score(query)
    }

    fn primary_color_class(&self) -> &str {
        "grey"
    }

    fn description(&self) -> &str {
        &self.inner.description
    }
}
