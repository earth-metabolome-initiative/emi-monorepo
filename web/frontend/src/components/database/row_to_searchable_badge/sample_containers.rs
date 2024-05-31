use super::RowToSearchableBadge;
use crate::traits::format_match::FormatMatch;
use web_common::database::NestedSampleContainer;
use yew::prelude::*;

impl RowToSearchableBadge for NestedSampleContainer {
    fn to_searchable_badge(&self, query: Option<&str>) -> Html {
        html! {
            <div>
                <p>
                    <i class={format!("fas fa-{} {}", self.category.icon.name, self.category.color.name)}></i>
                    <span>{self.inner.barcode.maybe_format_match(query)}</span>
                </p>
            </div>
        }
    }

    fn similarity_score(&self, query: &str) -> isize {
        self.inner.barcode.similarity_score(query)
    }

    fn primary_color_class(&self) -> &str {
        &self.category.color.name
    }
}
