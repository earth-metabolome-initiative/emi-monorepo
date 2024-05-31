/// impl row to badge for associated taxa
/// add function components
use crate::{
    components::database::row_to_searchable_badge::RowToSearchableBadge, traits::FormatMatch,
};
use web_common::database::NestedBioOttTaxonItem;
use yew::prelude::*;

impl RowToSearchableBadge for NestedBioOttTaxonItem {
    fn to_searchable_badge(&self, query: Option<&str>) -> Html {
        html! {
            <div>
                <p>
                    <i class={format!("fas fa-{} {}", self.icon.name, self.color.name)}></i>
                    {'\u{00A0}'}
                    <span>{self.inner.name.maybe_format_match(query)}</span>
                </p>
            </div>
        }
    }

    fn similarity_score(&self, query: &str) -> isize {
        self.inner.name.similarity_score(query) + self.ott_rank.similarity_score(query)
    }

    fn primary_color_class(&self) -> &str {
        &self.color.name
    }
}
