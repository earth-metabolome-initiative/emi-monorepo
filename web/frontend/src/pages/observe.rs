//! Page for the observation action sequence.

use std::rc::Rc;

use core_structures::{Taxon, User};
use gloo::{timers::callback::Timeout, utils::window};
use yew::prelude::*;
use yew_agent::{prelude::WorkerBridgeHandle, scope_ext::AgentScopeExt};
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::{
        badge::BadgeSize,
        forms::{Datalist, GPSInput, MultiFileInput},
        Badge,
    },
    router::AppRoute,
    stores::{app_state::AppState, user_state::UserState},
    workers::{
        ws_worker::{ComponentMessage, WebsocketMessage},
        WebsocketWorker,
    },
};

#[derive(Debug, Copy, Eq, PartialEq, Clone)]
enum PageSection {
    Environment,
    Details,
    Location,
    Taxa,
    Summary,
}

pub struct Observe {
    user_state: Rc<UserState>,
    user_dispatch: Dispatch<UserState>,
    section: PageSection,
    websocket: WorkerBridgeHandle<WebsocketWorker>,
    environment_pictures: Vec<Rc<web_common::types::JPEG>>,
    details_pictures: Vec<Rc<web_common::types::JPEG>>,
    location: Option<web_common::types::Point>,
    taxon: Option<Rc<Taxon>>,
}

pub enum ObserveMessage {
    Backend(WebsocketMessage),
    UserState(Rc<UserState>),
    AddEnvironmentPicture(Rc<web_common::types::JPEG>),
    AddDetailsPicture(Rc<web_common::types::JPEG>),
    RemoveEnvironmentPicture(usize),
    RemoveDetailsPicture(usize),
    GeoLocation(Option<web_common::types::Point>),
    SelectedTaxon(Option<Rc<Taxon>>),
    SetSection(PageSection),
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
        websocket.send(ComponentMessage::Connect(window().location().hostname().unwrap()));

        Self {
            websocket,
            section: PageSection::Environment,
            user_state,
            user_dispatch,
            environment_pictures: vec![],
            details_pictures: vec![],
            location: None,
            taxon: None,
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
            ObserveMessage::AddEnvironmentPicture(picture) => {
                self.environment_pictures.push(picture);
                true
            }
            ObserveMessage::AddDetailsPicture(picture) => {
                self.details_pictures.push(picture);
                true
            }
            ObserveMessage::RemoveEnvironmentPicture(index) => {
                self.environment_pictures.remove(index);
                true
            }
            ObserveMessage::RemoveDetailsPicture(index) => {
                self.details_pictures.remove(index);
                true
            }
            ObserveMessage::SelectedTaxon(taxon) => {
                self.taxon = taxon;
                true
            }
            ObserveMessage::GeoLocation(location) => {
                self.location = location;
                true
            }
            ObserveMessage::SetSection(section) => {
                let changed_section = self.section != section;
                self.section = section;
                changed_section
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // If the user has not logged in, redirect to the login page.
        if !self.user_state.has_user() {
            ctx.link().navigator().unwrap().push(&AppRoute::Login);
        }

        // If the user is logged in, but has not yet completed their profile, redirect
        // to the profile page.
        if self.user_state.has_user() && !self.user_state.has_complete_profile() {
            ctx.link()
                .navigator()
                .unwrap()
                .push(&AppRoute::UsersUpdate { id: self.user_state.id().unwrap() });
        }

        // If the user is logged in, but has not yet selected which project to work on,
        // redirect to the project selection page.
        if self.user_state.has_user()
            && self.user_state.has_complete_profile()
            && !self.user_state.has_project()
        {
            ctx.link()
                .navigator()
                .unwrap()
                .push(&AppRoute::ProjectSelection { source_page: AppRoute::Observe.to_path() });
        }

        // If all of the above conditions are met, we display the input field for
        // multiple environment_pictures.

        let section = match self.section {
            PageSection::Environment => {
                // We create a callback for when a picture is added.
                let add_picture = ctx.link().callback(ObserveMessage::AddEnvironmentPicture);

                // We create a callback for when a picture is removed.
                let remove_picture = ctx.link().callback(ObserveMessage::RemoveEnvironmentPicture);

                // We prepare the classes for the next button, which brings the user to the next
                // step.
                let next_button_classes = format!(
                    "next-button{}",
                    if !self.environment_pictures.is_empty() { " enabled" } else { "" }
                );

                let on_click_next =
                    ctx.link().callback(|_| ObserveMessage::SetSection(PageSection::Details));

                html! {
                    <>
                        <p>{"Add one or more pictures capturing the environment."}</p>
                        <yew_agent::oneshot::OneshotProvider<crate::workers::FileProcessor<web_common::types::JPEG>> path="/jpeg_file_processor.js">
                            <MultiFileInput<web_common::types::JPEG> label="Environment Pictures" optional={false} append_file={add_picture} remove_file={remove_picture} maximum_number_of_expected_files={10} errors={vec![]} files={Rc::new(self.environment_pictures.clone())} />
                        </yew_agent::oneshot::OneshotProvider<crate::workers::FileProcessor<web_common::types::JPEG>>>
                        <button title={"Next step"} onclick={on_click_next} class={next_button_classes} disabled={self.environment_pictures.is_empty()}>
                            <i class="fas fa-arrow-right"></i>
                            {'\u{00a0}'}
                            <span>{"Next"}</span>
                        </button>
                    </>
                }
            }
            PageSection::Details => {
                // We create a callback for when a picture is added.
                let add_picture = ctx.link().callback(ObserveMessage::AddDetailsPicture);

                // We create a callback for when a picture is removed.
                let remove_picture = ctx.link().callback(ObserveMessage::RemoveDetailsPicture);

                // We prepare the classes for the next button, which brings the user to the next
                // step.
                let next_button_classes = format!(
                    "next-button{}",
                    if !self.details_pictures.is_empty() { " enabled" } else { "" }
                );

                let on_click_back =
                    ctx.link().callback(|_| ObserveMessage::SetSection(PageSection::Environment));

                let on_click_next =
                    ctx.link().callback(|_| ObserveMessage::SetSection(PageSection::Location));

                html! {
                    <>
                        <p>{"Add one or more pictures capturing details of the observation."}</p>
                        <yew_agent::oneshot::OneshotProvider<crate::workers::FileProcessor<web_common::types::JPEG>> path="/jpeg_file_processor.js">
                            <MultiFileInput<web_common::types::JPEG> label="Detail Pictures" optional={false} append_file={add_picture} remove_file={remove_picture} maximum_number_of_expected_files={10} errors={vec![]} files={Rc::new(self.details_pictures.clone())} />
                        </yew_agent::oneshot::OneshotProvider<crate::workers::FileProcessor<web_common::types::JPEG>>>
                        <button title={"Next step"} onclick={on_click_next} class={next_button_classes} disabled={self.details_pictures.is_empty()}>
                            <i class="fas fa-arrow-right"></i>
                            {'\u{00a0}'}
                            <span>{"Next"}</span>
                        </button>
                        <button title={"Last step"} onclick={on_click_back} class={"next-button"}>
                            <i class="fas fa-arrow-left"></i>
                            {'\u{00a0}'}
                            <span>{"Previous"}</span>
                        </button>
                    </>
                }
            }
            PageSection::Location => {
                let set_geolocation = ctx.link().callback(ObserveMessage::GeoLocation);

                let next_button_classes =
                    format!("next-button{}", if self.location.is_some() { " enabled" } else { "" });

                let on_click_back =
                    ctx.link().callback(|_| ObserveMessage::SetSection(PageSection::Details));

                let on_click_next =
                    ctx.link().callback(|_| ObserveMessage::SetSection(PageSection::Taxa));

                html! {
                    <>
                        <p>{"Add the location of the observation."}</p>
                        <GPSInput label="Geolocation" optional={false} builder={set_geolocation} coordinates={self.location.clone()} />
                        <button title={"Next step"} onclick={on_click_next} class={next_button_classes} disabled={self.location.is_none()}>
                            <i class="fas fa-arrow-right"></i>
                            {'\u{00a0}'}
                            <span>{"Next"}</span>
                        </button>
                        <button title={"Last step"} onclick={on_click_back} class={"next-button"}>
                            <i class="fas fa-arrow-left"></i>
                            {'\u{00a0}'}
                            <span>{"Previous"}</span>
                        </button>
                    </>
                }
            }
            PageSection::Taxa => {
                let selected_taxon =
                    ctx.link().callback(move |project: Option<Rc<Taxon>>| {
                        ObserveMessage::SelectedTaxon(project)
                    });

                let on_click_back =
                    ctx.link().callback(|_| ObserveMessage::SetSection(PageSection::Location));

                let on_click_next =
                    ctx.link().callback(|_| ObserveMessage::SetSection(PageSection::Summary));

                html! {
                    <>
                        <p>{"Select the primary taxon associated with this observation."}</p>
                        <Datalist<Taxon, false> builder={selected_taxon} optional={false} value={self.taxon.clone()} label="Select taxon" scanner={false} />
                        <button title={"Next step"} onclick={on_click_next} class={"next-button enabled"}>
                            <i class="fas fa-arrow-right"></i>
                            {'\u{00a0}'}
                            <span>{"Next"}</span>
                        </button>
                        <button title={"Last step"} onclick={on_click_back} class={"next-button"}>
                            <i class="fas fa-arrow-left"></i>
                            {'\u{00a0}'}
                            <span>{"Previous"}</span>
                        </button>
                    </>
                }
            }
            PageSection::Summary => {
                let on_click_back =
                    ctx.link().callback(|_| ObserveMessage::SetSection(PageSection::Taxa));

                html! {
                    <>
                        <p>{"Review the observation before submitting."}</p>
                        <button title={"Submit observation"} class={"next-button enabled"}>
                            <i class="fas fa-check"></i>
                            {'\u{00a0}'}
                            <span>{"Submit"}</span>
                        </button>
                        <button title={"Last step"} onclick={on_click_back} class={"next-button"}>
                            <i class="fas fa-arrow-left"></i>
                            {'\u{00a0}'}
                            <span>{"Previous"}</span>
                        </button>
                    </>
                }
            }
        };

        html! {
            <div class="fullscreen_center_app">
                <div class="observe">
                    <h2>{"New observation"}</h2>
                    {section}
                </div>
            </div>
        }
    }
}
