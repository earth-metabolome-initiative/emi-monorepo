//! Module providing a yew component that handles a basic form.

use std::{fmt::Debug, rc::Rc};

use gloo::timers::callback::Timeout;
use serde::{de::DeserializeOwned, Serialize};
use web_common::api::{form_traits::FormMethod, ApiError};
use yew::prelude::*;
use yew_agent::{prelude::WorkerBridgeHandle, scope_ext::AgentScopeExt};
use yew_router::scope_ext::RouterScopeExt;
use yewdux::prelude::*;

use crate::{
    components::forms::InputErrors,
    cookies::is_logged_in,
    router::AppRoute,
    stores::user_state::UserState,
    workers::{
        ws_worker::{ComponentMessage, WebsocketMessage},
        WebsocketWorker,
    },
};

/// Trait defining something that can be built from a named get operation.
pub(super) trait FromOperation {
    /// Creates a new instance of the implementing type from the provided
    /// operation name and row.
    fn from_operation<S: AsRef<str>>(operation_name: S, row: Vec<u8>) -> Self;
}

/// Trait defining something that can be used to build something else by a form.
pub(super) trait FormBuilder:
    Clone + Store + PartialEq + Serialize + Debug + Default
{
    type Actions: Reducer<Self> + FromOperation;
    type RichVariant: DeserializeOwned + Debug + Viewable;

    /// Returns whether the form contains errors.
    fn has_errors(&self) -> bool;

    /// Returns whether the can currently be submitted.
    fn can_submit(&self) -> bool;

    /// Returns whether the form is currently equal to the default state.
    fn is_default(&self) -> bool {
        self == &Self::default()
    }

    /// Updates the state of the form builder with the provided data and returns
    /// the operations necessary to complete the update.
    ///
    /// # Arguments
    /// * `richest_variant` - The data to use to update the form builder.
    fn update(
        dispatcher: &Dispatch<Self>,
        richest_variant: Self::RichVariant,
    ) -> Vec<ComponentMessage>;
}

/// Trait defining something that can be built by a form.
pub(super) trait FormBuildable:
    Clone + PartialEq + Serialize + 'static + From<<Self as FormBuildable>::Builder> + Tabular + Debug
{
    type Builder: FormBuilder;

    /// Returns the title to use for the Form.
    fn title() -> &'static str;

    /// Returns the name of the task target.
    ///
    /// # Examples
    /// The task target is what is being inserted/deleted/updated.
    /// If you are creating a form to insert a new Taxon, the task target is
    /// "Taxon". If you are creating a form to update a Taxon, the task
    /// target is "Taxon". If you are creating a form to delete a Taxon, the
    /// task target is "Taxon".
    fn task_target() -> &'static str;

    /// Returns whether this form requires authentication.
    fn requires_authentication() -> bool;

    /// Returns whether the form can be submitted while offline.
    fn can_operate_offline() -> bool;
}

#[derive(Clone, PartialEq, Properties)]
pub(super) struct BasicFormProp<Data>
where
    Data: FormBuildable,
{
    pub builder: Data::Builder,
    pub builder_dispatch: Dispatch<Data::Builder>,
    pub named_requests: Vec<ComponentMessage>,
    pub children: Html,
    pub method: FormMethod,
}

pub(super) struct BasicForm<Data> {
    websocket: WorkerBridgeHandle<WebsocketWorker>,
    waiting_for_reply: bool,
    validate_timeout: Option<Timeout>,
    errors: Vec<ApiError>,
    user_state: Rc<UserState>,
    _dispatcher: Dispatch<UserState>,
    loading_operations: usize,
    _phantom: std::marker::PhantomData<Data>,
}

pub(super) enum FormMessage {
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

impl<Data> Component for BasicForm<Data>
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
                    ctx.link().navigator().unwrap().push(&AppRoute::Home);
                }
                true
            }
            FormMessage::Backend(WebsocketMessage::GetTable(operation_name, row)) => {
                self.loading_operations -= 1;
                if let Some(operation_name) = operation_name {
                    log::info!("Received a row from the backend for operation: {}", operation_name);
                    ctx.props().builder_dispatch.apply(
                        <<Data as FormBuildable>::Builder as FormBuilder>::Actions::from_operation(
                            operation_name,
                            row,
                        ),
                    )
                } else {
                    log::info!("Received a row from the backend for an unknown operation");
                    let richest_variant: <<Data as FormBuildable>::Builder as FormBuilder>::RichVariant = bincode::deserialize(&row).unwrap();

                    // if ctx.props().method.is_update() {
                    //     if let Some(id) = ctx.props().builder.id() {
                    //         match ctx.props().method {
                    //             FormMethod::POST => {}
                    //             FormMethod::GET => {
                    //                 ctx.link()
                    //                     .send_message(ComponentMessage::can_view::<Data>(id));
                    //             }
                    //             FormMethod::PUT => {
                    //                 ctx.link()
                    //                     .send_message(ComponentMessage::can_update::<Data>(id));
                    //             }
                    //             FormMethod::DELETE => {
                    //                 ctx.link()
                    //                     .send_message(ComponentMessage::can_admin::<Data>(id));
                    //             }
                    //         }
                    //     }
                    // }

                    log::debug!("Updating the form with the received data, {:?}", richest_variant);

                    <<Data as FormBuildable>::Builder as FormBuilder>::update(
                        &ctx.props().builder_dispatch,
                        richest_variant,
                    )
                    .into_iter()
                    .for_each(|message| {
                        ctx.link().send_message(message);
                    });
                }

                true
            }
            FormMessage::Backend(WebsocketMessage::Completed(maybe_row)) => {
                self.waiting_for_reply = false;
                self.errors.clear();
                // If we received a row, it means we have successfully inserted/updated the data
                // and we can redirect to the page associated to the provided row. If we have
                // not received a row and the current operation is NOT a delete, we have an
                // error.
                if let Some(row) = maybe_row {
                    assert!(
                        ctx.props().method.is_post() || ctx.props().method.is_update(),
                        "Received a row from the backend for a non-insert operation"
                    );
                    let entry: <<Data as FormBuildable>::Builder as FormBuilder>::RichVariant =
                        bincode::deserialize(&row).unwrap();
                    ctx.link().navigator().unwrap().push(&entry.view_route());
                } else {
                    if ctx.props().method.is_delete() {
                        log::info!("Successfully deleted the data, redirecting to home page.");
                        ctx.link().navigator().unwrap().push(&AppRoute::Home);
                    } else {
                        unreachable!(
                            "Received a completion message with ROW for a non-insert operation"
                        );
                    }
                }
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
        if first_render {
            ctx.props().builder_dispatch.reduce_mut(|state| {
                *state = Default::default();
            });
            for operation in ctx.props().named_requests.iter() {
                ctx.link().send_message(operation.clone());
            }
        }
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        ctx.props().builder_dispatch.reduce_mut(|state| {
            *state = Default::default();
        });
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if Data::requires_authentication() && (!self.user_state.has_user() || !is_logged_in()) {
            log::info!("No access token found, redirecting to login page.");
            ctx.link().navigator().unwrap().push(&AppRoute::Login);
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

        let classes =
            format!("standard-form{}", if self.errors.is_empty() { " error" } else { "" });

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
                        {'\u{00a0}'}
                        <span>{ctx.props().method.to_crud()}</span>
                    </button>
                    if ctx.props().method.is_post() {
                        <button onclick={clear_form} title={clear_button_title_message} class={clear_button_classes} disabled={ctx.props().builder.is_default() || self.waiting_for_reply}>
                            <i class="fas fa-hand-sparkles"></i>
                            {'\u{00a0}'}
                            <span>{"Clear"}</span>
                        </button>
                    }
                }
                <div class="clear"></div>
            </form>
        }
    }
}
