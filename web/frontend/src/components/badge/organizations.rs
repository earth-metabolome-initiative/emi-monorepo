use super::{Badge, BadgeSize, RowToBadge};
use web_common::{
    database::{Country, NestedOrganization},
    traits::CapitalizeString,
};
use yew::prelude::*;

impl RowToBadge for NestedOrganization {
    fn badge_title(&self) -> String {
        self.inner.name.capitalize()
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        Some("building")
    }

    fn children(&self, props: &super::BadgeProps<Self>) -> Option<yew::prelude::Html> {
        Some(html! {
            <Badge<Country> badge={self.country.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
        })
    }
}
