use super::RowToSearchableBadge;
use crate::traits::format_match::FormatMatch;
use web_common::database::NestedObservationSubject;
use yew::prelude::*;

impl RowToSearchableBadge for NestedObservationSubject {
    fn to_searchable_badge(&self, query: Option<&str>) -> Html {
        html! {
            <div>
                <p>
                    <i class={format!("fas fa-{} {}", self.icon.name, self.color.name)}></i>
                    {'\u{00A0}'}
                    <span>{self.inner.name.maybe_format_match(query)}</span>
                </p>
                <p>{self.inner.description.maybe_format_match(query)}</p>
            </div>
        }
    }

    fn to_searchable_small_badge(&self, query: Option<&str>) -> Html {
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
        self.inner.name.similarity_score(query) + self.inner.description.similarity_score(query)
    }
    fn primary_color_class(&self) -> &str {
        &self.color.name
    }
}
