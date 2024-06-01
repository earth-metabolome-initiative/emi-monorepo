use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::NestedOrganism;
use web_common::database::{NestedProject, User, NestedNameplate, NestedSample};
use yew::prelude::*;

impl RowToBadge for NestedOrganism {
    fn badge_title(&self) -> String {
        self.nameplate.badge_title()
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }

    fn primary_image_url(&self) -> Option<String> {
        Some(self.inner.get_picture_as_url())
    }

    fn children(&self, props: &super::BadgeProps<Self>) -> Option<yew::prelude::Html> {
        Some(html! {
            <>
                <Badge<User> badge={self.updated_by.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
                <Badge<NestedProject> badge={self.project.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
                <Badge<NestedNameplate> badge={self.nameplate.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
                if let Some(sample) = self.sample.as_ref() {
                    <Badge<NestedSample> badge={sample.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
                }
            </>
        })
    }
}
