//! Page for the collection of a sample.

use crate::components::badge::BadgeSize;
use crate::router::AppRoute;
use crate::stores::app_state::AppState;
use crate::stores::user_state::UserState;
use crate::workers::ws_worker::{ComponentMessage, WebsocketMessage};

use crate::components::Badge;
use gloo::timers::callback::Timeout;
use gloo::utils::window;
use web_common::database::NestedUser;
use yew::prelude::*;
use yew_agent::scope_ext::AgentScopeExt;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::workers::WebsocketWorker;
use std::rc::Rc;
use yew_agent::prelude::WorkerBridgeHandle;

pub struct Collect {
    user_state: Rc<UserState>,
    user_dispatch: Dispatch<UserState>,
    websocket: WorkerBridgeHandle<WebsocketWorker>,
}

pub enum CollectMessage {
    Backend(WebsocketMessage),
    UserState(Rc<UserState>),
}

#[derive(Clone, Properties, PartialEq)]
pub struct CollectProps {}

impl Component for Collect {
    type Message = CollectMessage;
    type Properties = CollectProps;

    fn create(ctx: &Context<Self>) -> Self {
        let user_dispatch = Dispatch::<UserState>::global()
            .subscribe(ctx.link().callback(CollectMessage::UserState));
        let user_state = user_dispatch.get();

        let websocket = ctx.link().bridge_worker(Callback::from({
            let link = ctx.link().clone();
            move |message: WebsocketMessage| {
                link.send_message(CollectMessage::Backend(message));
            }
        }));

        websocket.send(ComponentMessage::UserState(user_state.user()));
        websocket.send(ComponentMessage::Connect(
            window().location().hostname().unwrap(),
        ));

        Self {
            websocket,
            user_state,
            user_dispatch,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CollectMessage::UserState(user_state) => {
                if self.user_state == user_state {
                    return false;
                }
                self.user_state = user_state;
                true
            }
            CollectMessage::Backend(WebsocketMessage::Notification(notification)) => {
                log::info!("Received notification: {:?}", notification);
                false
            }
            CollectMessage::Backend(_) => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // If the user has not logged in, redirect to the login page.
        if !self.user_state.has_user() {
            ctx.link().navigator().unwrap().push(&AppRoute::Login);
        }

        // If the user is logged in, but has not yet completed their profile, redirect to the profile page.
        if self.user_state.has_user() && !self.user_state.has_complete_profile() {
            ctx.link()
                .navigator()
                .unwrap()
                .push(&AppRoute::UsersUpdate {
                    id: self.user_state.id().unwrap(),
                });
        }

        // If the user is logged in, but has not yet selected which project to work on, redirect to the project selection page.
        if self.user_state.has_user()
            && self.user_state.has_complete_profile()
            && !self.user_state.has_project()
        {
            ctx.link()
                .navigator()
                .unwrap()
                .push(&AppRoute::ProjectSelection {
                    source_page: AppRoute::Collect.to_path(),
                });
        }

        Html::default()
    }
}
