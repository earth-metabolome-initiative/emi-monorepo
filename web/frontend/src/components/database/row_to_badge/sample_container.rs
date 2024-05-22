use super::RowToBadge;
use crate::{components::PageLike, router::AppRoute};
use web_common::database::NestedSampleContainer;
use yew_router::prelude::*;

impl RowToBadge for NestedSampleContainer {
    fn to_badge(&self) -> yew::Html {
        let title_action = format!("View the container {}", self.title());
        yew::html! {
            <div class={format!("badge {}", self.category.color.name)} title={title_action}>
                <Link<AppRoute> to={AppRoute::ProjectsView{ id: self.inner.id }}>
                    <p>
                        <i class={format!("fas fa-{}", self.category.icon.name)}></i>
                        {'\u{00a0}'}
                        <span>{self.title()}</span>
                    </p>

                    // Print out the sample state, procedure, sampled by
                    <div class="sample-info">
                        <span title="Created by">{self.created_by.to_tiny_badge()}</span>
                    </div>
                </Link<AppRoute>>
            </div>
        }
    }

    fn to_tiny_badge(&self) -> yew::Html {
        let title_action = format!("View the {} project", self.title());
        yew::html! {
            <div class={format!("badge {}", self.category.color.name)} title={title_action}>
                <Link<AppRoute> to={AppRoute::ProjectsView{ id: self.inner.id }}>
                    <p>
                        <i class={format!("fas fa-{}", self.category.icon.name)}></i>
                        {'\u{00a0}'}
                        <span>{self.title()}</span>
                    </p>
                </Link<AppRoute>>
            </div>
        }
    }
}
