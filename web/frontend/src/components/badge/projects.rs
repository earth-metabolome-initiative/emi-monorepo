use super::{Badge, BadgeSize, RowToBadge};
use crate::traits::format_match::FormatMatch;
use web_common::database::*;
use yew::prelude::*;

impl RowToBadge for NestedProject {
    fn badge_title(&self) -> String {
        self.inner.name.clone()
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        self.icon.font_awesome_icon()
    }

    fn similarity_score<S: AsRef<str>>(&self, query: S) -> isize {
        let query = query.as_ref();
        (self.inner.name.similarity_score(query)
            + self.inner.description.similarity_score(query)
            + self.created_by.similarity_score(query)
            + self.updated_by.similarity_score(query))
            / 4
    }

    fn children(&self, props: &super::BadgeProps<Self>) -> Option<yew::prelude::Html> {
        Some(html! {
            <>
                <Badge<NestedProjectState> badge={self.state.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
                <Badge<NestedUser> badge={self.updated_by.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
            </>
        })
    }
}
