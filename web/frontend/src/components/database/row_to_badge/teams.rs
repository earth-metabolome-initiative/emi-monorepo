use super::RowToBadge;
use crate::{components::PageLike, router::AppRoute};
use web_common::database::NestedTeam;
use yew_router::prelude::*;

impl RowToBadge for NestedTeam {
    fn to_badge(&self) -> yew::Html {
        yew::html! {
            <div class="badge"> <Link<AppRoute> to={AppRoute::TeamsView { id: self.inner.id }}>{self.title()}</Link<AppRoute>> </div>
        }
    }
}
