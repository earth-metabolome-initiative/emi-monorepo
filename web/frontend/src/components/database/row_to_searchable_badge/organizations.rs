use super::RowToSearchableBadge;
use crate::components::basic_page::PageLike;
use crate::router::AppRoute;
use crate::traits::format_match::FormatMatch;
use web_common::{database::NestedOrganization, traits::CapitalizeString};
use yew::prelude::*;
use yew_router::prelude::Link;

impl RowToSearchableBadge for NestedOrganization {
    fn to_searchable_badge(&self, query: Option<&str>) -> Html {
        html! {
            <div class="badge primary">
                <Link<AppRoute> to={self.view_path()}>
                    <p>
                        <i class={format!("fas fa-{}", NestedOrganization::icon())}></i>
                        {'\u{00A0}'}
                        <strong>{self.inner.name.capitalize().maybe_format_match(query)}</strong>
                    </p>
                </Link<AppRoute>>
                <ul class="badges-list">
                    <li>{self.country.to_searchable_small_badge(query)}</li>
                </ul>
            </div>
        }
    }

    fn to_searchable_small_badge(&self, query: Option<&str>) -> Html {
        html! {
            <div class="badge primary">
                <Link<AppRoute> to={self.view_path()}>
                    <p>
                        <i class={format!("fas fa-{}", NestedOrganization::icon())}></i>
                        {'\u{00A0}'}
                        <span>{self.inner.name.maybe_format_match(query)}</span>
                    </p>
                </Link<AppRoute>>
            </div>
        }
    }

    fn similarity_score(&self, query: &str) -> isize {
        self.inner.name.similarity_score(query) + self.country.similarity_score(query)
    }
    fn primary_color_class(&self) -> &str {
        "primary"
    }
}
