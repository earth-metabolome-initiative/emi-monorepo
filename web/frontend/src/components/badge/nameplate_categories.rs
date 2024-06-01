use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::{NestedMaterial, NestedNameplateCategory, NestedPermanenceCategory};
use web_common::traits::CapitalizeString;
use yew::prelude::*;

impl RowToBadge for NestedNameplateCategory {
    fn badge_title(&self) -> String {
        self.inner.name.capitalize()
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        self.icon.font_awesome_icon()
    }

    fn children(&self, props: &super::BadgeProps<Self>) -> Option<yew::prelude::Html> {
        Some(html! {
            <>
                <Badge<NestedPermanenceCategory> badge={self.permanence.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
                <Badge<NestedMaterial> badge={self.material.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
            </>
        })
    }
}
