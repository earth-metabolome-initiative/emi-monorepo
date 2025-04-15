use std::rc::Rc;

use core_structures::User;
use gloo::{timers::callback::Timeout, utils::window};
use yew::prelude::*;
use yew_agent::{prelude::WorkerBridgeHandle, scope_ext::AgentScopeExt};
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::{Badge, badge::BadgeSize, hamburger::Hamburger, sidebar::Sidebar},
    router::AppRoute,
    stores::{app_state::AppState, user_state::UserState},
    workers::{
        WebsocketWorker,
        ws_worker::{ComponentMessage, WebsocketMessage},
    },
};

pub struct Navigator {
    websocket: WorkerBridgeHandle<WebsocketWorker>,
    app_state: Rc<AppState>,
    app_dispatch: Dispatch<AppState>,
    toggle_timeout: Option<Timeout>,
}

impl Navigator {
    fn sidebar_open(&self) -> bool {
        self.app_state.sidebar_open()
    }
}

pub enum NavigatorMessage {
    Backend(WebsocketMessage),
    AppState(Rc<AppState>),
    ToggleSidebar(bool),
    SetSidebarVisibility(bool),
}

#[derive(Clone, Properties, PartialEq)]
pub struct NavigatorProps {}

impl Component for Navigator {
    type Message = NavigatorMessage;
    type Properties = NavigatorProps;

    fn create(ctx: &Context<Self>) -> Self {
        let app_dispatch = Dispatch::<AppState>::global()
            .subscribe(ctx.link().callback(NavigatorMessage::AppState));
        let app_state = app_dispatch.get();

        let websocket = ctx.link().bridge_worker(Callback::from({
            let link = ctx.link().clone();
            move |message: WebsocketMessage| {
                link.send_message(NavigatorMessage::Backend(message));
            }
        }));

        Self { websocket, app_state, app_dispatch, toggle_timeout: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NavigatorMessage::AppState(app_state) => {
                if self.app_state == app_state {
                    return false;
                }

                self.app_state = app_state;

                true
            }
            NavigatorMessage::Backend(_) => false,
            NavigatorMessage::SetSidebarVisibility(visibility) => {
                self.app_dispatch.reduce_mut(|state| {
                    state.set_sidebar_visibility(visibility);
                });
                true
            }
            NavigatorMessage::ToggleSidebar(visibility) => {
                if let Some(timeout) = self.toggle_timeout.take() {
                    timeout.cancel();
                }
                let link = ctx.link().clone();
                self.toggle_timeout = Some(Timeout::new(100, move || {
                    link.send_message(NavigatorMessage::SetSidebarVisibility(visibility));
                }));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // On click, we send a message to the store to toggle the sidebar.
        let toggle = {
            let link = ctx.link().clone();
            Callback::from(move |visibility: bool| {
                link.send_message(NavigatorMessage::ToggleSidebar(visibility));
            })
        };

        html! {
            <>
                <nav>
                    <Hamburger is_active = {self.sidebar_open()} onclick = {toggle.clone()}/>
                    <h1>
                        <Link<AppRoute> classes="logo" to={AppRoute::Home}>
                            {"EMI"}
                        </Link<AppRoute>>
                    </h1>
                    // <SearchBar />
                    // if let Some(user) = self.user() {
                    //     if user.inner.has_complete_profile() {
                    //         <Badge<User> size={BadgeSize::Small} badge={user.clone()}/>
                    //     } else {
                    //         <Link<AppRoute> classes="right_nav_button fill-profile" to={AppRoute::UsersUpdate { id: user.inner.id }}>
                    //             <i class="fas fa-clipboard-check"></i>
                    //             {'\u{00a0}'}
                    //             <span>{"Fill profile"}</span>
                    //         </Link<AppRoute>>
                    //     }
                    // } else {
                    //     <Link<AppRoute> classes="right_nav_button login" to={AppRoute::Login}>
                    //         <i class="fas fa-right-to-bracket"></i>
                    //         {'\u{00a0}'}
                    //         <span>{"Login"}</span>
                    //     </Link<AppRoute>>
                    // }
                </nav>
                <Sidebar visible={self.sidebar_open()} onclose={toggle} />
            </>
        }
    }
}
