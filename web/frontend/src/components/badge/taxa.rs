use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::NestedRank;
use web_common::{database::NestedTaxon, traits::CapitalizeString};
use yew::prelude::*;

impl RowToBadge for NestedTaxon {
    fn badge_title(&self) -> String {
        self.inner.name.capitalize()
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        self.icon.font_awesome_icon()
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }

    fn children(&self, props: &super::BadgeProps<Self>) -> Option<yew::prelude::Html> {
        Some(html! {
            <Badge<NestedRank> badge={self.ott_rank.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
        })
    }
}
