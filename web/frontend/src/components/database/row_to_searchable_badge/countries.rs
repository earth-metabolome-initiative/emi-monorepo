use super::RowToSearchableBadge;
use crate::components::basic_page::PageLike;
use crate::router::AppRoute;
use crate::traits::format_match::FormatMatch;
use web_common::{database::Country, traits::CapitalizeString};
use yew::prelude::*;
use yew_router::prelude::Link;

impl RowToSearchableBadge for Country {
    fn to_searchable_badge(&self, query: Option<&str>) -> Html {
        html! {
            <div class="badge primary">
                <Link<AppRoute> to={self.view_path()}>
                <p>
                        <i>{self.emoji.clone()}</i>
                        {'\u{00A0}'}
                        <strong>{self.name.capitalize().maybe_format_match(query)}</strong>
                </p>
                </Link<AppRoute>>
            </div>
        }
    }

    fn similarity_score(&self, query: &str) -> isize {
        self.name.similarity_score(query)
    }
    fn primary_color_class(&self) -> &str {
        "primary"
    }
}
