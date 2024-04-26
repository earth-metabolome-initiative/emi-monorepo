/// impl row to badge for associated taxa
/// add function components
use crate::{components::database::row_to_badge::RowToBadge, traits::FormatMatch};
use web_common::database::NestedBioOttTaxonItem;
use yew::prelude::*;

impl RowToBadge for NestedBioOttTaxonItem {
    fn to_datalist_badge(&self, query: &str) -> Html {
        html! {
            <div>
                <p>
                    <i class={format!("{} {}", self.font_awesome_icon.name, self.color.name)}></i>
                    <span>{self.name.format_match(query)}</span>
                    <span>{}</span>
                </p>
            </div>
        }
    }

    fn to_selected_datalist_badge(&self) -> Html {
        html! {
            <p>
            <i class={format!("{} {}", self.font_awesome_icon.name, self.color.name)}></i>
                <span>{&self.name}</span>
            </p>
        }
    }

    fn matches(&self, query: &str) -> bool {
        self.name == query
    }

    fn similarity_score(&self, query: &str) -> isize {
        self.name.similarity_score(query)
    }

    fn primary_color_class(&self) -> &str {
        &self.color.name
    }

    fn description(&self) -> &str {
        "The taxon name."
    }
}
