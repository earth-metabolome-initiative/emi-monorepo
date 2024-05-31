use crate::components::basic_page::PageLike;
use crate::components::database::row_to_searchable_badge::RowToSearchableBadge;
use crate::router::AppRoute;
use crate::traits::format_match::FormatMatch;
use web_common::database::NestedBioOttTaxonItem;
use web_common::{database::NestedBioOttRank, traits::CapitalizeString};
use yew::prelude::*;
use yew_router::hooks::use_route;
use yew_router::prelude::Link;

impl RowToSearchableBadge for NestedBioOttTaxonItem {
    fn to_searchable_badge(&self, query: Option<&str>) -> Html {
        html! {
            <div class={format!("badge {}", self.color.name)}>
                <Link<AppRoute> to={self.view_path()}>
                <p>
                        <i class={format!("fas fa-{} {}", self.icon.name, self.color.name)}></i>
                        {'\u{00A0}'}
                        <strong>{self.inner.name.capitalize().maybe_format_match(query)}</strong>
                </p>
                </Link<AppRoute>>
                <ul class="badges-list">
                    <li>{self.ott_rank.to_searchable_small_badge(query)}</li>
                </ul>
            </div>
        }
    }

    fn to_searchable_small_badge(&self, query: Option<&str>) -> Html {
        html! {
            <div class={format!("badge small {}", self.color.name)}>
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
