use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::*;
use yew::prelude::*;

impl RowToBadge for NestedDerivedSample {
    fn badge_title(&self) -> String {
        "Sample Derivation Link".to_string()
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        self.child_sample.font_awesome_icon()
    }

    fn children(&self, props: &super::BadgeProps<Self>) -> Option<yew::prelude::Html> {
        Some(html! {
            <>
                <Badge<NestedUser> badge={self.updated_by.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
                <Badge<NestedSample> badge={self.parent_sample.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
                <Badge<NestedSample> badge={self.child_sample.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
            </>
        })
    }
}
