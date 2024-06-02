use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::{NestedNameplate, NestedNameplateCategory, NestedProject, User};
use yew::prelude::*;

impl RowToBadge for NestedNameplate {
    fn badge_title(&self) -> String {
        format!("#{}", self.inner.barcode)
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        self.category.font_awesome_icon()
    }

    fn children(&self, props: &super::BadgeProps<Self>) -> Option<yew::prelude::Html> {
        Some(html! {
            <>
                <Badge<User> badge={self.updated_by.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
                <Badge<NestedProject> badge={self.project.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
                <Badge<NestedNameplateCategory> badge={self.category.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
            </>
        })
    }
}
