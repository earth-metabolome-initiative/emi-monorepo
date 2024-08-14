//! Page for the observation action sequence.

use crate::components::badge::BadgeSize;
use crate::router::AppRoute;
use crate::stores::app_state::AppState;
use crate::stores::user_state::UserState;
use crate::workers::ws_worker::{ComponentMessage, WebsocketMessage};
use crate::components::forms::MultiFileInput;
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

pub struct Observe {
    user_state: Rc<UserState>,
    user_dispatch: Dispatch<UserState>,
    websocket: WorkerBridgeHandle<WebsocketWorker>,
    pictures: Vec<Rc<web_common::types::JPEG>>,
}

pub enum ObserveMessage {
    Backend(WebsocketMessage),
    UserState(Rc<UserState>),
    AddPicture(Rc<web_common::types::JPEG>),
    RemovePicture(usize),
}

#[derive(Clone, Properties, PartialEq)]
pub struct ObserveProps {}

impl Component for Observe {
    type Message = ObserveMessage;
    type Properties = ObserveProps;

    fn create(ctx: &Context<Self>) -> Self {
        let user_dispatch = Dispatch::<UserState>::global()
            .subscribe(ctx.link().callback(ObserveMessage::UserState));
        let user_state = user_dispatch.get();

        let websocket = ctx.link().bridge_worker(Callback::from({
            let link = ctx.link().clone();
            move |message: WebsocketMessage| {
                link.send_message(ObserveMessage::Backend(message));
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
            pictures: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ObserveMessage::UserState(user_state) => {
                if self.user_state == user_state {
                    return false;
                }
                self.user_state = user_state;
                true
            }
            ObserveMessage::Backend(WebsocketMessage::Notification(notification)) => {
                log::info!("Received notification: {:?}", notification);
                false
            }
            ObserveMessage::Backend(_) => false,
            ObserveMessage::AddPicture(picture) => {
                self.pictures.push(picture);
                true
            }
            ObserveMessage::RemovePicture(index) => {
                self.pictures.remove(index);
                true
            }
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
                    source_page: AppRoute::Observe.to_path(),
                });
        }

        // If all of the above conditions are met, we display the input field for multiple pictures.

        // We create a callback for when a picture is added.
        let add_picture = ctx.link().callback(ObserveMessage::AddPicture);

        // We create a callback for when a picture is removed.
        let remove_picture = ctx.link().callback(ObserveMessage::RemovePicture);

        // We prepare the classes for the next button, which brings the user to the next step.
        let next_button_classes = format!(
            "next-button{}",
            if !self.pictures.is_empty() { " enabled" } else { "" }
        );

        html!{
            <div class="fullscreen_center_app">
                <div class="observe">
                    <h2>{"New observation"}</h2>
                    <p>{"Add one or more pictures capturing the environment."}</p>
                    <yew_agent::oneshot::OneshotProvider<crate::workers::FileProcessor<web_common::types::JPEG>> path="/jpeg_file_processor.js">
                        <MultiFileInput<web_common::types::JPEG> label="Environment Pictures" optional={false} append_file={add_picture} remove_file={remove_picture} maximum_number_of_expected_files={10} errors={vec![]} files={Rc::new(self.pictures.clone())} />
                    </yew_agent::oneshot::OneshotProvider<crate::workers::FileProcessor<web_common::types::JPEG>>>
                    <button title={"Next step"} class={next_button_classes} disabled={self.pictures.is_empty()}>
                        <i class="fas fa-arrow-right"></i>
                        {'\u{00a0}'}
                        <span>{"Next"}</span>
                    </button>
                </div>
            </div>
        }
    }
}
