//! Module providing a yew component that handles a basic form.

use crate::components::forms::InputErrors;
use crate::cookies::is_logged_in;
use crate::router::AppRoute;
use crate::stores::user_state::UserState;
use crate::workers::ws_worker::{ComponentMessage, WebsocketMessage};
use crate::workers::WebsocketWorker;
use gloo::timers::callback::Timeout;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::rc::Rc;
use web_common::api::form_traits::FormMethod;
use web_common::api::ApiError;
use web_common::database::*;
use yew::prelude::*;
use yew_agent::prelude::WorkerBridgeHandle;
use yew_agent::scope_ext::AgentScopeExt;
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

/// Trait defining something that can be built from a named get operation.
pub(super) trait FromOperation {
    /// Creates a new instance of the implementing type from the provided operation name and row.
    fn from_operation<S: AsRef<str>>(operation_name: S, row: Vec<u8>) -> Self;
}

/// Trait defining something that can be used to build something else by a form.
pub(super) trait FormBuilder:
    Clone + Store + PartialEq + Serialize + Debug + Default
{
    type Actions: Reducer<Self> + FromOperation;
    type RichVariant: DeserializeOwned + Debug;

    /// Returns whether the form contains errors.
    fn has_errors(&self) -> bool;

    /// Returns whether the can currently be submitted.
    fn can_submit(&self) -> bool;

    /// Returns whether the form is currently equal to the default state.
    fn is_default(&self) -> bool {
        self == &Self::default()
    }

    /// Updates the state of the form builder with the provided data and returns the operations
    /// necessary to complete the update.
    ///
    /// # Arguments
    /// * `rich_variant` - The data to use to update the form builder.
    fn update(
        dispatcher: &Dispatch<Self>,
        rich_variant: Self::RichVariant,
    ) -> Vec<ComponentMessage>;

    /// Returns the (optional) id of the object being built.
    ///
    /// # Implementative details
    /// The ID is optional because it is only present when the form is being used to update an existing object.
    /// If the form is being used to insert a new object, the ID is not present, i.e. it is `None`.
    fn id(&self) -> Option<PrimaryKey>;
}

/// Trait defining something that can be built by a form.
pub trait FormBuildable:
    Clone + PartialEq + Serialize + 'static + From<<Self as FormBuildable>::Builder> + Tabular + Debug
{
    type Builder: FormBuilder;

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
pub struct InnerBasicFormProp<Data>
where
    Data: FormBuildable,
{
    pub builder: Data::Builder,
    pub builder_dispatch: Dispatch<Data::Builder>,
    pub named_requests: Vec<ComponentMessage>,
    pub children: Html,
    pub method: FormMethod,
    pub navigator: yew_router::navigator::Navigator,
}

#[derive(Clone, PartialEq, Properties)]
pub struct BasicFormProp<Data>
where
    Data: FormBuildable,
{
    pub builder: Data::Builder,
    pub builder_dispatch: Dispatch<Data::Builder>,
    #[prop_or_default]
    pub named_requests: Vec<ComponentMessage>,
    pub children: Html,
    pub method: FormMethod,
}

pub struct InnerBasicForm<Data> {
    websocket: WorkerBridgeHandle<WebsocketWorker>,
    waiting_for_reply: bool,
    validate_timeout: Option<Timeout>,
    errors: Vec<ApiError>,
    user_state: Rc<UserState>,
    loading_operations: usize,
    _dispatcher: Dispatch<UserState>,
    _phantom: std::marker::PhantomData<Data>,
}

pub enum FormMessage {
    Submit,
    Frontend(ComponentMessage),
    Backend(WebsocketMessage),
    UserState(Rc<UserState>),
}

impl From<ComponentMessage> for FormMessage {
    fn from(message: ComponentMessage) -> Self {
        FormMessage::Frontend(message)
    }
}

impl<Data> Component for InnerBasicForm<Data>
where
    Data: FormBuildable,
{
    type Message = FormMessage;
    type Properties = InnerBasicFormProp<Data>;

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
            loading_operations: 0,
            _dispatcher: user_dispatch,
            _phantom: std::marker::PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FormMessage::Frontend(operation) => {
                log::info!("Sending operation to the backend: {:?}", operation);
                self.websocket.send(operation);
                self.loading_operations += 1;
                true
            }
            FormMessage::Backend(WebsocketMessage::Error(error)) => {
                self.errors = vec![error];
                self.waiting_for_reply = false;
                true
            }
            FormMessage::Backend(WebsocketMessage::CanView(status))
            | FormMessage::Backend(WebsocketMessage::CanUpdate(status))
            | FormMessage::Backend(WebsocketMessage::CanDelete(status)) => {
                self.loading_operations -= 1;
                if !status {
                    log::info!(
                        "User does not have permission to view the form, redirecting to home page."
                    );
                    ctx.props().navigator.push(&AppRoute::Home);
                }
                true
            }
            FormMessage::Backend(WebsocketMessage::GetTable(operation_name, row)) => {
                self.loading_operations -= 1;
                if let Some(operation_name) = operation_name {
                    log::info!(
                        "Received a row from the backend for operation: {}",
                        operation_name
                    );
                    ctx.props().builder_dispatch.apply(
                        <<Data as FormBuildable>::Builder as FormBuilder>::Actions::from_operation(
                            operation_name,
                            row,
                        ),
                    )
                } else {
                    log::info!("Received a row from the backend for an unknown operation");
                    let rich_variant: <<Data as FormBuildable>::Builder as FormBuilder>::RichVariant = bincode::deserialize(&row).unwrap();

                    log::debug!("Updating the form with the received data, {:?}", rich_variant);

                    <<Data as FormBuildable>::Builder as FormBuilder>::update(
                        &ctx.props().builder_dispatch,
                        rich_variant,
                    )
                    .into_iter()
                    .for_each(|message| {
                        ctx.link().send_message(message);
                    });
                }

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
                    FormMethod::PUT => ComponentMessage::update(&data),
                    FormMethod::DELETE => {
                        todo!("DELETE is not yet implemented for forms")
                    }
                });

                self.waiting_for_reply = true;
                true
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        log::info!("Rendering the form for the first time: {}", first_render);
        if first_render {
            // If it is the first time that we are rendering this component,
            // we check whether the provided builder has an ID. If it does,
            // we retrieve from the backend the most up to date version of the
            // data that we are editing.
            if ctx.props().method.is_post() {
                ctx.props().builder_dispatch.reduce_mut(|state| {
                    *state = Default::default();
                });
            }
            for operation in ctx.props().named_requests.iter() {
                ctx.link().send_message(operation.clone());
            }
            if ctx.props().method.is_update() {
                if let Some(id) = ctx.props().builder.id() {
                    match ctx.props().method {
                        FormMethod::POST => {}
                        FormMethod::GET => {
                            ctx.link()
                                .send_message(ComponentMessage::can_view::<Data>(id));
                        }
                        FormMethod::PUT => {
                            ctx.link()
                                .send_message(ComponentMessage::can_update::<Data>(id));
                        }
                        FormMethod::DELETE => {
                            ctx.link()
                                .send_message(ComponentMessage::can_delete::<Data>(id));
                        }
                    }
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if Data::requires_authentication() && (!self.user_state.has_user() || !is_logged_in()) {
            log::info!("No access token found, redirecting to login page.");
            ctx.props().navigator.push(&AppRoute::Login);
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

        let clear_form = {
            let builder_dispatch = ctx.props().builder_dispatch.clone();
            let waiting_for_reply = self.waiting_for_reply;
            Callback::from(move |event: MouseEvent| {
                event.prevent_default();
                if waiting_for_reply {
                    return;
                }
                builder_dispatch.reduce_mut(|state| {
                    *state = Default::default();
                });
            })
        };

        let submit_button_disabled = !ctx.props().builder.can_submit();

        let classes = format!(
            "standard-form{}",
            if self.errors.is_empty() { " error" } else { "" }
        );

        let submit_button_classes = if submit_button_disabled {
            ""
        } else if self.waiting_for_reply {
            "waiting"
        } else {
            "enabled"
        };

        let clear_button_classes = format!(
            "clear-button{}",
            if ctx.props().builder.is_default() {
                ""
            } else if self.waiting_for_reply {
                " waiting"
            } else {
                " enabled"
            }
        );

        let title_message = if self.waiting_for_reply {
            "You have already submitted the form, please wait for the reply."
        } else if submit_button_disabled {
            "You cannot submit the form until all the fields are valid"
        } else {
            "Submit the form"
        };

        let clear_button_title_message = if self.waiting_for_reply {
            "You have already submitted the form, please wait for the reply."
        } else if ctx.props().builder.is_default() {
            "The form is already clear"
        } else {
            "Clear the form"
        };

        html! {
            <form enctype={ "multipart/form-data" } disabled={self.waiting_for_reply} class={classes} onsubmit={on_submit} method={ctx.props().method.to_string()}>
                <h4>{ Data::title() }</h4>
                <p class="instructions">{format!("{} {}", ctx.props().method.to_crud(), Data::task_target())}</p>
                if self.loading_operations > 0 {
                    <p class="instructions">{format!("Loading, remaining {} operations...", self.loading_operations)}</p>
                } else {
                    { ctx.props().children.clone() }
                    <InputErrors errors={self.errors.clone()} />
                    <button type="submit" title={title_message} class={submit_button_classes} disabled={submit_button_disabled || self.waiting_for_reply}>
                        if self.waiting_for_reply {
                            <i class="fas fa-spinner fa-spin"></i>
                        } else {
                            <i class={ctx.props().method.font_awesome_icon()}></i>
                        }
                        <span>{format!("{} new {}", ctx.props().method.to_crud(), Data::task_target())}</span>
                    </button>
                    if ctx.props().method.is_post() {
                        <button onclick={clear_form} title={clear_button_title_message} class={clear_button_classes} disabled={ctx.props().builder.is_default() || self.waiting_for_reply}>
                            <i class="fas fa-hand-sparkles"></i>
                            <span>{format!("Clear {}", Data::task_target())}</span>
                        </button>
                    } else {
                        <></>
                    }
                }
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
        <InnerBasicForm<Data> navigator={navigator} named_requests={props.named_requests.clone()} method={props.method.clone()} builder={props.builder.clone()} builder_dispatch={props.builder_dispatch.clone()}>
            { props.children.clone() }
        </InnerBasicForm<Data>>
    }
}
