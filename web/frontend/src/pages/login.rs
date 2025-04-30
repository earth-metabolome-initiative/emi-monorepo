//! Login page of the application.

use yew::prelude::*;

use crate::{
    components::{ReadableList, login_provider::LoginProvider},
    utils::{ConnectorMessage, ConnectorProps},
};

/// Login page component.
pub struct Login;

impl Component for Login {
    type Message = ConnectorMessage;
    type Properties = ConnectorProps;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="fullscreen_center_app">
                <div class="login_box">
                    <h2>{"Login"}</h2>
                    <ReadableList<LoginProvider> dispatch={ctx.props().dispatch.clone()} />
                </div>
            </div>
        }
    }
}
