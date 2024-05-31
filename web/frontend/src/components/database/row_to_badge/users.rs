use super::RowToBadge;
use crate::router::AppRoute;
use web_common::database::User;
use yew_router::prelude::*;

impl RowToBadge for User {
    fn to_badge(&self) -> yew::Html {
        let title_action = format!("View {}'s profile", self.full_name());
        yew::html! {
            <div class="badge primary" title={title_action}>
                <Link<AppRoute> to={AppRoute::UsersView { id: self.id }}>
                    <p><img src={self.get_profile_picture_as_url()} alt={self.full_name()} /><span>{self.full_name()}</span></p>
                </Link<AppRoute>>
            </div>
        }
    }

    fn to_small_badge(&self) -> yew::Html {
        yew::html! {
            <div class="tiny-badge primary">
                <Link<AppRoute> to={AppRoute::UsersView { id: self.id }}>
                    <p><img src={self.get_profile_picture_as_url()} alt={self.full_name()} /><span>{self.full_name()}</span></p>
                </Link<AppRoute>>
            </div>
        }
    }
}
