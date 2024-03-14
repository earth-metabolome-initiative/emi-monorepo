//! The navigator Yew 0.21 component is responsible for rendering the navigation bar at the top of the page.
//!
//! Depending on whether we are on a large page, a middle page (such as table)
//! or a small page (such as a mobile device), the navigation bar will render differently.
//!
//! In the case of a large page, a side bar component is always shown on the left side of the page.
//! In the case of a middle page, the side bar is hidden by default and can be toggled by clicking the hamburger icon,
//! which is located on the left side of the navigation bar.
//! In the case of a small page, the side bar is hidden by default and can be toggled by clicking the hamburger icon,
//! which is located on the left side of the navigation bar.
//!
//! The navigator component, besides the occasional hamburger icon, also contains the logo of the application,
//! and in both the large and medium page cases, in the center shows a search bar. On the right side of the navigation bar,
//! it is shown in the large and medium page cases, the user name and the user avatar, and in the small page case, solely the user avatar.
//! On the right of the user name, in the large and medium page cases.
//!
//! The overall web application needs to function also offline, as the user may want to use it while
//! they do not have an internet connection. Therefore, the navigator will also display a message to the
//! user if they are offline by putting a badge with the text "Offline" on their avatar. Upon returning online,
//! the badge will disappear.
//!

use crate::router::AppRoute;
use crate::stores::app_state::AppState;
use crate::stores::user_state::UserState;
use web_common::api::auth::users::User;
use wasm_bindgen::UnwrapThrowExt;
use web_common::api::ws::messages::*;
use yew::prelude::*;
use yew_agent::scope_ext::AgentScopeExt;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::components::hamburger::Hamburger;
use crate::components::search_bar::SearchBar;
use crate::components::sidebar::Sidebar;
use crate::stores::user_state::refresh_access_token;
use crate::stores::user_state::retrieve_user_informations;
use crate::workers::WebsocketWorker;
use log::info;
use std::rc::Rc;
use yew_agent::prelude::WorkerBridgeHandle;

pub struct Navigator {
    websocket: WorkerBridgeHandle<WebsocketWorker<FrontendMessage, BackendMessage>>,
    user_state: Rc<UserState>,
    user_dispatch: Dispatch<UserState>,
    app_state: Rc<AppState>,
    app_dispatch: Dispatch<AppState>,
}

impl Navigator {
    fn sidebar_open(&self) -> bool {
        self.app_state.sidebar_open()
    }

    fn user(&self) -> Option<&User> {
        self.user_state.user()
    }
}

pub enum NavigatorMessage {
    Backend(BackendMessage),
    UserState(Rc<UserState>),
    AppState(Rc<AppState>),
    ToggleSidebar,
}

#[derive(Clone, Properties, PartialEq)]
pub struct NavigatorProps {
    pub navigator: yew_router::navigator::Navigator,
}

impl Component for Navigator {
    type Message = NavigatorMessage;
    type Properties = NavigatorProps;

    fn create(ctx: &Context<Self>) -> Self {
        let user_dispatch = Dispatch::<UserState>::global()
            .subscribe(ctx.link().callback(NavigatorMessage::UserState));
        let user_state = user_dispatch.get();
        let app_dispatch = Dispatch::<AppState>::global()
            .subscribe(ctx.link().callback(NavigatorMessage::AppState));
        let app_state = app_dispatch.get();

        Self {
            websocket: ctx.link().bridge_worker(Callback::from({
                let link = ctx.link().clone();
                move |message: BackendMessage| {
                    link.send_message(NavigatorMessage::Backend(message));
                }
            })),
            user_state,
            user_dispatch,
            app_state,
            app_dispatch,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NavigatorMessage::UserState(user_state) => {
                self.user_state = user_state;
                if let Some(access_token) = self.user_state.access_token() {
                    log::info!("Access token found, recovering user info.");
                    self.websocket.send(FrontendMessage::Authentication(access_token.clone()));
                } else {
                    refresh_access_token(self.user_dispatch.clone(), ctx.props().navigator.clone());
                }
                true
            }
            NavigatorMessage::AppState(app_state) => {
                self.app_state = app_state;
                true
            }
            NavigatorMessage::Backend(BackendMessage::Authenticated(user)) => {
                info!("User authenticated: {:?}", user);
                self.user_dispatch.reduce_mut(|state| {
                    state.set_user(user);
                });
                true
            }
            NavigatorMessage::ToggleSidebar => {
                self.app_dispatch.reduce_mut(|state| {
                    state.toggle_sidebar();
                });
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // On click, we send a message to the store to toggle the sidebar.
        let onclick = {
            let link = ctx.link().clone();
            Callback::from(move |_| {
                link.send_message(NavigatorMessage::ToggleSidebar);
            })
        };

        html! {
            <>
                <nav>
                    <Hamburger is_active = {self.sidebar_open()} onclick = {onclick}/>
                    <h1>
                        <Link<AppRoute> classes="logo" to={AppRoute::Home}>
                            {"EMI"}
                        </Link<AppRoute>>
                    </h1>
                    <SearchBar />
                    if let Some(user) = self.user() {
                        if user.has_complete_profile() {
                            <div class="user">
                                <img src={format!("/api/user/{}/avatar", user.id())} alt={format!("{}'s avatar", user.last_name())} />
                                <span>{user.full_name().unwrap_throw()}</span>
                                // {if store.is_offline() {
                                //     html! {
                                //         <span class="badge offline">{"Offline"}</span>
                                //     }
                                // } else {
                                //     html! {}
                                // }}
                            </div>
                        } else {
                            <Link<AppRoute> classes="right_nav_button" to={AppRoute::Profile}>{"Complete profile"}</Link<AppRoute>>
                        }
                    } else {
                        <Link<AppRoute> classes="right_nav_button" to={AppRoute::Login}>{"Login"}</Link<AppRoute>>
                    }
                </nav>
                <Sidebar visible={self.sidebar_open()} />
            </>
        }
    }
}

#[function_component(NavigatorWrapper)]
pub fn navigator_wrapper() -> Html {
    let navigator = use_navigator().unwrap();
    html! {
        <Navigator navigator={navigator} />
    }
}