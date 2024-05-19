/// impl row to badge for associated taxa
/// add function components
use crate::{components::database::row_to_searchable_badge::RowToSearchableBadge, traits::FormatMatch};
use web_common::database::NestedBioOttTaxonItem;
use yew::prelude::*;

impl RowToSearchableBadge for NestedBioOttTaxonItem {
    fn to_datalist_badge(&self, query: &str) -> Html {
        html! {
            <div>
                <p>
                    <i class={format!("fas fa-{} {}", self.icon.name, self.color.name)}></i>
                    // we add the rank of the taxon to the badge
                    <span>{self.inner.name.format_match(query)}</span>
                </p>
                {self.ott_rank.to_selected_datalist_badge()}
            </div>
        }
    }

    fn to_selected_datalist_badge(&self) -> Html {
        html! {
            <p>
            <i class={format!("fas fa-{} {}", self.icon.name, self.color.name)}></i>
                <span>{&self.inner.name}</span>
            </p>
        }
    }

    fn similarity_score(&self, query: &str) -> isize {
        self.inner.name.similarity_score(query) + self.ott_rank.similarity_score(query)
    }

    fn primary_color_class(&self) -> &str {
        &self.color.name
    }

    fn description(&self) -> &str {
        "The taxon name."
    }
}
