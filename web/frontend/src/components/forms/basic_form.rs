//! Module providing a yew component that handles a basic form.

use crate::components::forms::InputErrors;
use crate::router::AppRoute;
use crate::stores::app_state::AppState;
use crate::stores::user_state::UserState;
use crate::workers::WebsocketWorker;
use gloo::timers::callback::Timeout;
use serde::Serialize;
use std::collections::HashSet;
use std::rc::Rc;
use web_common::api::form_traits::FormMethod;
use web_common::api::ws::messages::*;
use web_common::api::ApiError;
use web_common::database::Task;
use yew::prelude::*;
use yew_agent::prelude::WorkerBridgeHandle;
use yew_agent::scope_ext::AgentScopeExt;
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

/// Trait defining something that can be used to build something else by a form.
pub trait FormBuilder: Clone + Store + PartialEq + Serialize {
    type Data: FormBuildable<Builder = Self>;
    type Actions: Reducer<Self>;

    /// Returns whether the form is buildable.
    fn buildable(&self) -> Result<(), ApiError>;

    /// Returns the data built by the form.
    /// 
    /// # Implementation details
    /// The reason this method receives a reference to self
    /// instead than consuming self is because the form builder
    /// is generally provided as a property to the form component.
    /// Properties are provided as reference, and thus the caller
    /// would need to clone the form builder to call the build method.
    /// This is not ideal, and by providing the reference to self
    /// possibly the cloned things may be a subset of the form builder.
    fn build(&self) -> Self::Data;
}

/// Trait defining something that can be built by a form.
pub trait FormBuildable: Clone + PartialEq + Serialize + 'static + Into<Task> {
    type Builder: FormBuilder<Data = Self>;

    const METHOD: FormMethod;

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

    /// Returns the description to use for the Form.
    fn description() -> &'static str;

    fn requires_authentication() -> bool;
}

#[derive(Clone, PartialEq, Properties)]
pub struct BasicFormProp<Data>
where
    Data: FormBuildable,
{
    pub builder: Data::Builder,
    pub children: Html,
    #[prop_or_default]
    pub navigator: Option<yew_router::navigator::Navigator>,
}

pub struct InnerBasicForm<Data> {
    websocket: WorkerBridgeHandle<WebsocketWorker<FrontendMessage, BackendMessage>>,
    app_state: Rc<AppState>,
    app_dispatch: Dispatch<AppState>,
    user_state: Rc<UserState>,
    _user_dispatch: Dispatch<UserState>,
    errors: HashSet<String>,
    waiting_for_reply: bool,
    uuid: Option<uuid::Uuid>,
    validate_timeout: Option<Timeout>,
    _phantom: std::marker::PhantomData<Data>,
}

pub enum FormMessage {
    Submit,
    Errors(Vec<String>),
    Backend(BackendMessage),
    RemoveError(String),
    AppState(Rc<AppState>),
    UserState(Rc<UserState>),
}

impl<Data> Component for InnerBasicForm<Data>
where
    Data: FormBuildable,
{
    type Message = FormMessage;
    type Properties = BasicFormProp<Data>;

    fn create(ctx: &Context<Self>) -> Self {
        let app_dispatch =
            Dispatch::<AppState>::global().subscribe(ctx.link().callback(FormMessage::AppState));
        let app_state = app_dispatch.get();
        let user_dispatch =
            Dispatch::<UserState>::global().subscribe(ctx.link().callback(FormMessage::UserState));
        let user_state = user_dispatch.get();
        Self {
            websocket: ctx.link().bridge_worker(Callback::from({
                let link = ctx.link().clone();
                move |message: BackendMessage| {
                    link.send_message(FormMessage::Backend(message));
                }
            })),
            errors: HashSet::new(),
            app_state,
            app_dispatch,
            user_state,
            _user_dispatch: user_dispatch,
            waiting_for_reply: false,
            validate_timeout: None,
            uuid: None,
            _phantom: std::marker::PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FormMessage::Backend(BackendMessage::TaskResult(uuid, outcome)) => {
                log::info!("Received task result");
                if self.uuid == Some(uuid) {
                    self.waiting_for_reply = false;
                    match outcome {
                        Ok(()) => {
                            log::info!("Form submitted successfully");
                            true
                        }
                        Err(errors) => {
                            ctx.link().send_message(FormMessage::Errors(errors.into()));
                            true
                        }
                    }
                } else {
                    false
                }
            }
            FormMessage::Errors(errors) => {
                self.errors = errors.into_iter().collect();
                true
            }
            FormMessage::AppState(app_state) => {
                self.app_state = app_state;
                true
            }
            FormMessage::UserState(user_state) => {
                self.user_state = user_state;
                true
            }
            FormMessage::Backend(_) => false,
            FormMessage::Submit => {
                let mut change = false;
                if !self.errors.is_empty() {
                    self.errors.clear();
                    change = true;
                }

                let data = ctx.props().builder.build();

                let task = data.into();
                self.uuid = Some(task.id());

                // Then, we send the data to the backend
                self.app_dispatch.reduce_mut(move |state| {
                    state.add_task(task);
                });

                self.waiting_for_reply = true;

                change
            }
            FormMessage::RemoveError(error) => {
                self.errors.remove(&error);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if Data::requires_authentication() {
            if let Some(navigator) = ctx.props().navigator.as_ref() {
                if self.user_state.has_no_access_token() {
                    log::info!("No access token found, redirecting to login page.");
                    navigator.push(&AppRoute::Login);
                }
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

        let classes = format!(
            "standard-form{}",
            if self.errors.is_empty() { " error" } else { "" }
        );

        let submit_button_disabled = ctx.props().builder.buildable().is_err();

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

        let on_delete = {
            let link = ctx.link().clone();
            Callback::from(move |error: String| {
                link.send_message(FormMessage::RemoveError(error));
            })
        };

        html! {
            <form enctype={ "multipart/form-data" } disabled={self.waiting_for_reply} class={classes} onsubmit={on_submit} method={Data::METHOD.to_string()}>
                <h4>{ Data::title() }</h4>
                <p class="instructions">{Data::description()}</p>
                { ctx.props().children.clone() }
                <InputErrors errors={self.errors.clone()} on_delete={on_delete} />
                <button type="submit" title={title_message} class={button_classes} disabled={submit_button_disabled || self.waiting_for_reply}>
                    {if self.waiting_for_reply {
                        html! { <i class="fas fa-spinner fa-spin"></i> }
                    } else {
                        html! { <i class={Data::METHOD.font_awesome_icon()}></i> }
                    }}
                    <span>{format!("{} {}", Data::METHOD.to_crud(), Data::task_target())}</span>
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
        <InnerBasicForm<Data> navigator={navigator} builder={props.builder.clone()}>
            { props.children.clone() }
        </InnerBasicForm<Data>>
    }
}
