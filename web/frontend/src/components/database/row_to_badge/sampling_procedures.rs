use super::RowToBadge;
use crate::{components::PageLike, router::AppRoute};
use web_common::database::NestedSamplingProcedure;
use yew_router::prelude::*;

impl RowToBadge for NestedSamplingProcedure {
    fn to_badge(&self) -> yew::Html {
        yew::html! {
            <div class="badge">
                <div class="sample-info">
                    <span>{"Sampling Procedure: "}</span>
                    <Link<AppRoute> to={AppRoute::SamplingProceduresView{ id: self.inner.id }}>{self.title()}</Link<AppRoute>>
                </div>

                <div class="sample-info">
                    <span>{"Description: "}</span>
                    <span>{self.inner.description.clone()}</span>
                </div>

            </div>


        }
    }
}
