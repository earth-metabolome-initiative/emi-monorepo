use super::RowToSearchableBadge;
use crate::traits::format_match::FormatMatch;
use web_common::database::NestedObservationSubject;
use yew::prelude::*;
use crate::router::AppRoute;
use crate::components::basic_page::PageLike;
use web_common::traits::CapitalizeString;
use yew_router::prelude::Link;

impl RowToSearchableBadge for NestedObservationSubject {
    fn to_searchable_badge(&self, query: Option<&str>) -> Html {
        html! {
            <div class={format!("badge {}", self.color.name)}>
                <Link<AppRoute> to={self.view_path()}>
                <p>
                        <i class={format!("fas fa-{} {}", self.icon.name, self.color.name)}></i>
                        {'\u{00A0}'}
                        <strong>{self.inner.name.capitalize().maybe_format_match(query)}</strong>
                </p>
                <p class="description">{self.inner.description.maybe_format_match(query)}</p>
                </Link<AppRoute>>
            </div>
        }
    }

    fn to_searchable_small_badge(&self, query: Option<&str>) -> Html {
        html! {
            <div class={format!("badge small {}", self.color.name)}>
                <Link<AppRoute> to={self.view_path()}>
                    <p>
                        <i class={format!("fas fa-{} {}", self.icon.name, self.color.name)}></i>
                        {'\u{00A0}'}
                        <span>{self.inner.name.maybe_format_match(query)}</span>
                    </p>
                </Link<AppRoute>>
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