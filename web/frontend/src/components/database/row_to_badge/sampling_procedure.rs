use super::RowToBadge;
use crate::{components::PageLike, router::AppRoute};
use web_common::database::NestedSamplingProcedure;
use yew_router::prelude::*;

impl RowToBadge for NestedSamplingProcedure {
    fn to_badge(&self) -> yew::Html {
        yew::html! {
            <div class="badge"> <Link<AppRoute> to={AppRoute::SamplingProceduresView{ id: self.inner.id }}>{self.title()}</Link<AppRoute>> </div>
        }
    }
}
