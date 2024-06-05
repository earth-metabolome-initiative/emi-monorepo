use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::*;
use yew::prelude::*;

impl RowToBadge for NestedProjectsTeamsRoleRequest {
    fn badge_title(&self) -> String {
        "Organism to taxon link".to_string()
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }

    fn children(&self, props: &super::BadgeProps<Self>) -> Option<yew::prelude::Html> {
        Some(html! {
            <>
                <Badge<NestedUser> badge={self.created_by.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
                <Badge<NestedProject> badge={self.table.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
                <Badge<NestedTeam> badge={self.team.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
                <Badge<NestedRole> badge={self.role.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
            </>
        })
    }
}
