use super::RowToBadge;
use crate::router::AppRoute;
use web_common::database::User;
use yew_router::prelude::*;

impl RowToBadge for User {
    fn to_badge(&self) -> yew::Html {
        yew::html! {
            <div class="badge"> <Link<AppRoute> to={AppRoute::UsersView { id: self.id }}>{self.full_name()}</Link<AppRoute>> </div>
        }
    }
}
