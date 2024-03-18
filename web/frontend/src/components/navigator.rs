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
use crate::utils::is_online;
use gloo::timers::callback::Interval;
use wasm_bindgen::UnwrapThrowExt;
use web_common::api::auth::users::User;
use web_common::api::oauth::jwt_cookies::RefreshError::Unauthorized;
use web_common::api::ws::messages::*;
use web_common::api::ApiError;
use yew::prelude::*;
use yew_agent::scope_ext::AgentScopeExt;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::components::hamburger::Hamburger;
use crate::components::search_bar::SearchBar;
use crate::components::sidebar::Sidebar;
use crate::stores::user_state::refresh_access_token;
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
    connectivity_checked: Option<Interval>,
}

impl Navigator {
    fn sidebar_open(&self) -> bool {
        self.app_state.sidebar_open()
    }

    fn user(&self) -> Option<&User> {
        self.user_state.user()
    }

    fn has_access_token(&self) -> bool {
        self.user_state.has_access_token()
    }

    fn has_complete_profile(&self) -> bool {
        self.user_state.has_complete_profile()
    }
}

pub enum NavigatorMessage {
    Backend(BackendMessage),
    UserState(Rc<UserState>),
    AppState(Rc<AppState>),
    ToggleSidebar,
    ResumeTasks,
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
            connectivity_checked: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NavigatorMessage::UserState(user_state) => {
                self.user_state = user_state;
                if let Some(access_token) = self.user_state.access_token() {
                    self.websocket
                        .send(FrontendMessage::Authentication(access_token.clone()));
                } else {
                    refresh_access_token(self.user_dispatch.clone(), ctx.props().navigator.clone());
                }
                true
            }
            NavigatorMessage::AppState(app_state) => {
                self.app_state = app_state;

                ctx.link().send_message(NavigatorMessage::ResumeTasks);

                if let Some(interval) = self.connectivity_checked.take() {
                    interval.cancel();
                }
                // We define an interval that check the internet connection every second and
                // updates the app state accordingly when the connection status changes.
                self.connectivity_checked = Some({
                    let app_dispatch = self.app_dispatch.clone();
                    let current_connection_status = self.app_state.connect_to_internet();
                    Interval::new(1000, move || {
                        let new_connection_status = is_online();
                        if current_connection_status != new_connection_status {
                            log::info!("Connection status changed to {}", new_connection_status);
                            app_dispatch.reduce_mut(|state| {
                                state.set_connect_to_internet(new_connection_status);
                            });
                        }
                    })
                });

                true
            }
            NavigatorMessage::ResumeTasks => {
                if self.app_state.connect_to_internet() {
                    let tasks = self.app_state.tasks();
                    for (task_id, form_action, form_data) in tasks {
                        info!("Resuming task {}", task_id);
                        self.websocket.send(FrontendMessage::task(
                            *task_id,
                            form_action.clone(),
                            form_data.clone(),
                        ));
                    }
                    !tasks.is_empty()
                } else {
                    false
                }
            }
            NavigatorMessage::Backend(BackendMessage::User(_operation, user)) => {
                info!("User updated: {:?}", user);
                self.user_dispatch.reduce_mut(|state| {
                    state.set_user(user);
                });
                true
            }
            NavigatorMessage::Backend(BackendMessage::ExpiredToken) => {
                refresh_access_token(self.user_dispatch.clone(), ctx.props().navigator.clone());
                true
            }
            NavigatorMessage::Backend(BackendMessage::TaskResult(task_id, _, result)) => {
                match result {
                    Ok(()) => {
                        info!("Task {} completed successfully", task_id);
                        // TODO: trigger a popup upon task completion
                        self.app_dispatch.reduce_mut(|state| {
                            state.remove_task(task_id);
                        });
                    }
                    Err(api_error) => {
                        match api_error {
                            ApiError::Unauthorized | ApiError::ExpiredAuthorization => {
                                refresh_access_token(
                                    self.user_dispatch.clone(),
                                    ctx.props().navigator.clone(),
                                );
                            }
                            ApiError::InternalServerError | ApiError::BadGateway => {}
                            ApiError::BadRequest(_errors) => {
                                self.app_dispatch.reduce_mut(|state| {
                                    state.remove_task(task_id);
                                });
                            }
                        }
                    }
                }
                true
            }
            NavigatorMessage::Backend(BackendMessage::Authenticated) => {
                ctx.link().send_message(NavigatorMessage::ResumeTasks);
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
                    if self.has_access_token() {
                        if self.has_complete_profile() {
                            if let Some(user) = self.user() {
                                <div class="user">
                                    <img src={format!("/api/user/{}/avatar", user.id())} alt={format!("{}'s avatar", user.last_name().unwrap())} />
                                    <span>{user.full_name().unwrap_throw()}</span>
                                    // {if store.is_offline() {
                                    //     html! {
                                    //         <span class="badge offline">{"Offline"}</span>
                                    //     }
                                    // } else {
                                    //     html! {}
                                    // }}
                                </div>
                            }
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
