use super::RowToSearchableBadge;
use crate::traits::format_match::FormatMatch;
use web_common::database::NestedSampleContainerCategory;
use yew::prelude::*;

impl RowToSearchableBadge for NestedSampleContainerCategory {
    fn to_datalist_badge(&self, query: &str) -> Html {
        html! {
            <div>
                <p>
                    <i class={format!("fas fa-{} {}", self.icon.name, self.color.name)}></i>
                    <span>{self.inner.name.format_match(query)}</span>
                </p>
                <p class="description">{self.inner.description.format_match(query)}</p>
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
        self.inner.name.similarity_score(query) + self.inner.description.similarity_score(query)
    }

    fn primary_color_class(&self) -> &str {
        &self.color.name
    }

    fn description(&self) -> &str {
        &self.inner.description
    }
}
