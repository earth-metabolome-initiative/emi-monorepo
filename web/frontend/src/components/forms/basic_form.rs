//! Module providing a yew component that handles a basic form.

use crate::components::forms::InputErrors;
use crate::router::AppRoute;
use crate::stores::user_state::UserState;
use crate::workers::ws_worker::{ComponentMessage, WebsocketMessage};
use crate::workers::WebsocketWorker;
use gloo::timers::callback::Timeout;
use serde::Serialize;
use std::fmt::Debug;
use std::rc::Rc;
use web_common::api::form_traits::FormMethod;
use web_common::api::ApiError;
use web_common::database::Table;
use yew::prelude::*;
use yew_agent::prelude::WorkerBridgeHandle;
use yew_agent::scope_ext::AgentScopeExt;
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

/// Trait defining something that can be used to build something else by a form.
pub(super) trait FormBuilder: Clone + Store + PartialEq + Serialize + Debug {
    type Actions: Reducer<Self>;

    /// Returns whether the form contains errors.
    fn has_errors(&self) -> bool;

    /// Returns whether the can currently be submitted.
    fn can_submit(&self) -> bool;
}

/// Trait defining something that can be built by a form.
pub trait FormBuildable:
    Clone + PartialEq + Serialize + 'static + From<<Self as FormBuildable>::Builder>
{
    type Builder: FormBuilder;
    const TABLE: Table;

    /// Returns the title to use for the Form.
    fn title() -> &'static str;

    /// Returns the name of the task target.
    ///
    /// # Examples
    /// The task target is what is being inserted/deleted/updated.
    /// If you are creating a form to insert a new Taxon, the task target is "Taxon".
    /// If you are creating a form to update a Taxon, the task target is "Taxon".
    /// If you are creating a form to delete a Taxon, the task target is "Taxon".
    fn task_target() -> &'static str;

    /// Returns whether this form requires authentication.
    fn requires_authentication() -> bool;

    /// Returns whether the form can be submitted while offline.
    fn can_operate_offline() -> bool;
}

#[derive(Clone, PartialEq, Properties)]
pub struct BasicFormProp<Data>
where
    Data: FormBuildable,
{
    pub builder: Data::Builder,
    pub children: Html,
    pub method: FormMethod,
    #[prop_or_default]
    pub navigator: Option<yew_router::navigator::Navigator>,
}

pub struct InnerBasicForm<Data> {
    websocket: WorkerBridgeHandle<WebsocketWorker>,
    waiting_for_reply: bool,
    validate_timeout: Option<Timeout>,
    errors: Vec<ApiError>,
    user_state: Rc<UserState>,
    _dispatcher: Dispatch<UserState>,
    _phantom: std::marker::PhantomData<Data>,
}

pub enum FormMessage {
    Submit,
    Backend(WebsocketMessage),
    UserState(Rc<UserState>),
}

impl<Data> Component for InnerBasicForm<Data>
where
    Data: FormBuildable,
{
    type Message = FormMessage;
    type Properties = BasicFormProp<Data>;

    fn create(ctx: &Context<Self>) -> Self {
        let user_dispatch =
            Dispatch::<UserState>::global().subscribe(ctx.link().callback(FormMessage::UserState));
        let user_state = user_dispatch.get();

        Self {
            websocket: ctx.link().bridge_worker(Callback::from({
                let link = ctx.link().clone();
                move |message: WebsocketMessage| {
                    link.send_message(FormMessage::Backend(message));
                }
            })),
            waiting_for_reply: false,
            validate_timeout: None,
            errors: Vec::new(),
            user_state,
            _dispatcher: user_dispatch,
            _phantom: std::marker::PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FormMessage::Backend(WebsocketMessage::Error(error)) => {
                self.errors = vec![error];
                self.waiting_for_reply = false;
                true
            }
            FormMessage::Backend(WebsocketMessage::Completed) => {
                self.waiting_for_reply = false;
                self.errors.clear();
                true
            }
            FormMessage::Backend(_) => false,
            FormMessage::UserState(user_state) => {
                if self.user_state == user_state {
                    return false;
                }
                self.user_state = user_state;
                true
            }
            FormMessage::Submit => {
                let data: Data = ctx.props().builder.clone().into();

                // Then, we send the data to the backend
                self.websocket.send(match ctx.props().method {
                    FormMethod::POST => ComponentMessage::insert(&data),
                    FormMethod::GET => {
                        unreachable!("GET is not supported for forms")
                    }
                    FormMethod::PUT => {
                        todo!("PUT is not yet implemented for forms")
                    }
                    FormMethod::DELETE => {
                        todo!("DELETE is not yet implemented for forms")
                    }
                });

                self.waiting_for_reply = true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if Data::requires_authentication() && !self.user_state.has_user() {
            if let Some(navigator) = ctx.props().navigator.as_ref() {
                log::info!("No access token found, redirecting to login page.");
                navigator.push(&AppRoute::Login);
            }
        }

        let on_submit = {
            let link = ctx.link().clone();
            let waiting_for_reply = self.waiting_for_reply;
            Callback::from(move |event: SubmitEvent| {
                event.prevent_default();
                if waiting_for_reply {
                    return;
                }
                link.send_message(FormMessage::Submit);
            })
        };

        let submit_button_disabled = !ctx.props().builder.can_submit();

        let classes = format!(
            "standard-form{}",
            if self.errors.is_empty() { " error" } else { "" }
        );

        let button_classes = if submit_button_disabled {
            ""
        } else if self.waiting_for_reply {
            "waiting"
        } else {
            "enabled"
        };

        let title_message = if self.waiting_for_reply {
            "You have already submitted the form, please wait for the reply."
        } else if submit_button_disabled {
            "You cannot submit the form until all the fields are valid"
        } else {
            "Submit the form"
        };

        html! {
            <form enctype={ "multipart/form-data" } disabled={self.waiting_for_reply} class={classes} onsubmit={on_submit} method={ctx.props().method.to_string()}>
                <h4>{ Data::title() }</h4>
                <p class="instructions">{format!("{} {}", ctx.props().method.to_crud(), Data::task_target())}</p>
                { ctx.props().children.clone() }
                <InputErrors errors={self.errors.clone()} />
                <button type="submit" title={title_message} class={button_classes} disabled={submit_button_disabled || self.waiting_for_reply}>
                    {if self.waiting_for_reply {
                        html! { <i class="fas fa-spinner fa-spin"></i> }
                    } else {
                        html! { <i class={ctx.props().method.font_awesome_icon()}></i> }
                    }}
                    <span>{format!("{} {}", ctx.props().method.to_crud(), Data::task_target())}</span>
                </button>
                <div class="clear"></div>
            </form>
        }
    }
}

#[function_component(BasicForm)]
pub fn basic_form<Data>(props: &BasicFormProp<Data>) -> Html
where
    Data: FormBuildable,
{
    let navigator = use_navigator().unwrap();
    html! {
        <InnerBasicForm<Data> navigator={navigator} method={props.method.clone()} builder={props.builder.clone()}>
            { props.children.clone() }
        </InnerBasicForm<Data>>
    }
}
