use super::RowToBadge;
use crate::{components::PageLike, router::AppRoute};
use web_common::database::NestedSample;
use yew_router::prelude::*;

impl RowToBadge for NestedSample {
    fn to_badge(&self) -> yew::Html {
        yew::html! {
            <div class="badge">
                // Print out sample id
                <div class="sample-info">
                    <span>{"Sample ID: "}</span>
                    <Link<AppRoute> to={AppRoute::SamplesView{ id: self.inner.id }}>{self.title()}</Link<AppRoute>>
                </div>

                // Print out the sample state, procedure, sampled by
                <div class="sample-info">
                    <span>{"Sampled by: "}</span>
                    <Link<AppRoute> to={AppRoute::UsersView{ id: self.sampled_by.id}}>{self.sampled_by.title()}</Link<AppRoute>>
                </div>

                // Print the sampling procedure
                <div class="sample-info">
                    <span>{"Sampling procedure: "}</span>
                    <Link<AppRoute> to={AppRoute::SamplingProceduresView{ id: self.procedure.id().try_into().unwrap()}}>{self.procedure.title()}</Link<AppRoute>>
                </div>

                <div class="sample-info">
                    <span>{"Sample state: "}</span>
                    <span>{self.state.inner.name.clone()}</span>
                </div>
            </div>
        }
    }
}
