//! Submodule providing the LinkButton yew component, which is a button that
//! navigates to a new page when clicked.

use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::AppRoute;

pub struct LinkButton {}

#[derive(Clone, PartialEq, Properties)]
pub struct LinkButtonProps {
    pub route: AppRoute,
}

impl Component for LinkButton {
    type Message = ();
    type Properties = LinkButtonProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <Link<AppRoute> classes={"button-like update"} to={ctx.props().route.clone()}>
                { "Click me!" }
            </Link<AppRoute>>
        }
    }
}
