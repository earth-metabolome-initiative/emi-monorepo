use super::RowToBadge;
use crate::{
    components::{database::row_to_searchable_badge::RowToSearchableBadge, PageLike},
    router::AppRoute,
};
use web_common::database::NestedProject;
use yew_router::prelude::*;

impl RowToBadge for NestedProject {
    fn to_badge(&self) -> yew::Html {
        yew::html! {
            <div class="badge">
                // Print out sample id
                <div class="sample-info">
                    <Link<AppRoute> to={AppRoute::ProjectsView{ id: self.inner.id }}>{self.title()}</Link<AppRoute>>
                </div>

                // Print out the sample state, procedure, sampled by
                <div class="sample-info">
                    {self.state.to_selected_datalist_badge()}
                    <span title="Created by">{self.created_by.to_selected_datalist_badge()}</span>
                </div>
            </div>
        }
    }
}
