use super::RowToBadge;
use crate::{components::PageLike, router::AppRoute};
use web_common::database::NestedObservation;
use yew_router::prelude::*;

impl RowToBadge for NestedObservation {
    fn to_badge(&self) -> yew::Html {
        let title_action = format!("View the {} project", self.title());
        yew::html! {
            <div class={format!("badge {}", self.project.color.name)} title={title_action}>
                <Link<AppRoute> to={AppRoute::ObservationsView { id: self.inner.id }}>
                    <p>
                        <i class={"fas fa-tower-observation"}></i>
                        {'\u{00a0}'}
                        <span>{self.title()}</span>
                    </p>
                    </Link<AppRoute>>

                    // Print out the sample state, procedure, sampled by
                    <div class="sample-info">
                        <span title="Created by">{self.created_by.to_tiny_badge()}</span>
                        <span title="Part of project">{self.project.to_tiny_badge()}</span>
                    </div>
            </div>
        }
    }

    fn to_tiny_badge(&self) -> yew::Html {
        yew::html! {
            <div class={format!("tiny-badge {}", self.project.color.name)} >
                <Link<AppRoute> to={AppRoute::ObservationsView { id: self.inner.id }}>
                    <p>
                        <i class={"fas fa-tower-observation"}></i>
                        {'\u{00a0}'}
                        <span>{self.title()}</span>
                    </p>
                    </Link<AppRoute>>
            </div>
        }
    }
}