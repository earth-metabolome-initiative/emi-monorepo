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

use crate::router::AppRoute;
use crate::stores::app_state::AppState;
use crate::stores::user_state::UserState;
use crate::workers::ws_worker::WebsocketMessage;

use web_common::database::User;
use yew::prelude::*;
use yew_agent::scope_ext::AgentScopeExt;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::components::hamburger::Hamburger;
use crate::components::search_bar::SearchBar;
use crate::components::sidebar::Sidebar;
use crate::workers::WebsocketWorker;
use std::rc::Rc;
use yew_agent::prelude::WorkerBridgeHandle;

pub struct Navigator {
    websocket: WorkerBridgeHandle<WebsocketWorker>,
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
    Backend(WebsocketMessage),
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
                move |message: WebsocketMessage| {
                    link.send_message(NavigatorMessage::Backend(message));
                }
            })),
            user_state,
            user_dispatch,
            app_state,
            app_dispatch,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NavigatorMessage::UserState(user_state) => {
                if self.user_state == user_state {
                    return false;
                }
                self.user_state = user_state;
                true
            }
            NavigatorMessage::AppState(app_state) => {
                if self.app_state == app_state {
                    return false;
                }

                self.app_state = app_state;

                true
            }
            NavigatorMessage::Backend(WebsocketMessage::RefreshUser(user)) => {
                log::info!("Received new access token");
                self.user_dispatch.reduce_mut(|state| {
                    state.set_user(user);
                });
                false
            }
            NavigatorMessage::Backend(WebsocketMessage::Notification(notification)) => {
                log::info!("Received notification: {:?}", notification);
                false
            }
            NavigatorMessage::Backend(_) => false,
            NavigatorMessage::ToggleSidebar => {
                self.app_dispatch.reduce_mut(|state| {
                    state.toggle_sidebar();
                });
                true
            }
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
                        // if user.has_complete_profile() {
                        //     <div class="user">
                        //         if let Some(thumbnail) = &user.thumbnail {
                        //             <img src={thumbnail.inner.path.clone()} alt={format!("{}'s avatar", user.inner.full_name())} />
                        //         }
                        //         <span>{user.inner.full_name()}</span>
                        //         // {if store.is_offline() {
                        //         //     html! {
                        //         //         <span class="badge offline">{"Offline"}</span>
                        //         //     }
                        //         // } else {
                        //         //     html! {}
                        //         // }}
                        //     </div>
                        // } else {
                            <Link<AppRoute> classes="right_nav_button" to={AppRoute::Profile}>{"Complete profile"}</Link<AppRoute>>
                        // }
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
