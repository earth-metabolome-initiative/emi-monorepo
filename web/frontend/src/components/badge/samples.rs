use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::*;
use yew::prelude::*;

impl RowToBadge for NestedSample {
    fn badge_title(&self) -> String {
        self.container.badge_title()
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        self.container.font_awesome_icon()
    }

    fn children(&self, props: &super::BadgeProps<Self>) -> Option<yew::prelude::Html> {
        Some(html! {
            <>
                <Badge<NestedUser> badge={self.updated_by.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
                <Badge<NestedProject> badge={self.project.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
                <Badge<NestedSampleContainer> badge={self.container.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
                <Badge<NestedSampleState> badge={self.state.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
            </>
        })
    }
}
