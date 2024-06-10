use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::*;
use yew::prelude::*;

impl RowToBadge for NestedUser {
    fn badge_title(&self) -> String {
        self.inner.full_name()
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }

    fn primary_image_url(&self) -> Option<String> {
        Some(self.inner.get_picture_as_url())
    }

    fn children(&self, props: &super::BadgeProps<Self>) -> Option<yew::prelude::Html> {
        Some(html! {
            if let Some(organization) = self.organization.as_ref() {
                <Badge<NestedOrganization> badge={organization.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
            }
        })
    }
}
