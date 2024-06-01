use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::{NestedSpectra, NestedSpectraCollection, User};
use yew::prelude::*;

impl RowToBadge for NestedSpectra {
    fn badge_title(&self) -> String {
        "Spectra".to_string()
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        self.spectra_collection.font_awesome_icon()
    }

    fn children(&self, props: &super::BadgeProps<Self>) -> Option<yew::prelude::Html> {
        Some(html! {
            <>
                <Badge<NestedSpectraCollection> badge={self.spectra_collection.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
                <Badge<User> badge={self.updated_by.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
            </>
        })
    }
}
