//! Module providing a yew component that handles a basic form.

use crate::components::forms::InputErrors;
use crate::router::AppRoute;
use crate::stores::app_state::AppState;
use crate::stores::user_state::UserState;
use crate::workers::WebsocketWorker;
use gloo::timers::callback::Timeout;
use serde::Serialize;
use std::collections::HashSet;
use std::fmt::Debug;
use std::rc::Rc;
use validator::Validate;
use wasm_bindgen::JsCast;
use web_common::api::form_traits::FormResult;
use web_common::api::form_traits::TryFromCallback;
use web_common::api::ws::messages::*;
use web_common::custom_validators::validation_errors::ValidationErrorToString;
use web_sys::FormData;
use web_sys::HtmlFormElement;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::prelude::WorkerBridgeHandle;
use yew_agent::scope_ext::AgentScopeExt;
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct FormWrapper<Data> {
    data: Data,
}

impl<Data> FormWrapper<Data> {
    pub fn data(self) -> Data {
        self.data
    }
}

impl<Data> From<Data> for FormWrapper<Data> {
    fn from(data: Data) -> Self {
        FormWrapper { data }
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct BasicFormProp<Data>
where
    Data: PartialEq + Clone + 'static + Validate + Debug + Serialize,
{
    pub children: Html,
    #[prop_or_default]
    pub navigator: Option<yew_router::navigator::Navigator>,
    #[prop_or_default]
    pub current: Option<Data>,
}

pub struct InnerBasicForm<Data> {
    websocket: WorkerBridgeHandle<WebsocketWorker<FrontendMessage, BackendMessage>>,
    app_state: Rc<AppState>,
    app_dispatch: Dispatch<AppState>,
    user_state: Rc<UserState>,
    _user_dispatch: Dispatch<UserState>,
    errors: HashSet<String>,
    waiting_for_reply: bool,
    submit_button_disabled: bool,
    uuid: Option<uuid::Uuid>,
    validate_timeout: Option<Timeout>,
    _phantom: std::marker::PhantomData<Data>,
}

impl<Data> InnerBasicForm<Data>
where
    FormWrapper<Data>: TryFromCallback<FormData>,
{
    fn extract_data_from_input_event<E, C>(event: E, callback: C) -> Result<(), Vec<String>>
    where
        E: AsRef<Event>,
        C: FnOnce(Result<Data, Vec<String>>) + 'static,
    {
        let form = event
            .as_ref()
            .target()
            .ok_or(vec!["No target found in the event".to_string()])?
            .dyn_into::<HtmlInputElement>()
            .map_err(|e| vec![format!("The target of the event is not a form: {:?}", e)])?
            .closest("form")
            .map_err(|e| vec![format!("Error getting the closest form: {:?}", e)])?
            .ok_or(vec!["No form found".to_string()])?
            .dyn_into::<HtmlFormElement>()
            .map_err(|e| vec![format!("The target of the event is not a form: {:?}", e)])?;
        Self::extract_data_from_form(form, callback)
    }

    fn extract_data_from_form_event<E, C>(event: E, callback: C) -> Result<(), Vec<String>>
    where
        E: AsRef<Event>,
        C: FnOnce(Result<Data, Vec<String>>) + 'static,
    {
        let form = event
            .as_ref()
            .target()
            .ok_or(vec!["No target found in the event".to_string()])?
            .dyn_into::<HtmlFormElement>()
            .map_err(|e| vec![format!("The target of the event is not a form: {:?}", e)])?;
        Self::extract_data_from_form(form, callback)
    }

    fn extract_data_from_form<C>(form: HtmlFormElement, callback: C) -> Result<(), Vec<String>>
    where
        C: FnOnce(Result<Data, Vec<String>>) + 'static,
    {
        FormWrapper::<Data>::try_from_callback(
            FormData::new_with_form(&form)
                .map_err(|e| vec![format!("Error creating form data: {:?}", e)])?,
            move |result| {
                callback(result.map(|wrapper| wrapper.data()));
            },
        )
    }
}

pub enum FormMessage<Data> {
    Submit(Data),
    Errors(Vec<String>),
    Backend(BackendMessage),
    RemoveError(String),
    SetSubmitButtonDisabled(bool),
    StartSetSubmitButtonDisabledTimeout(bool),
    AppState(Rc<AppState>),
    UserState(Rc<UserState>),
}

impl<Data> Component for InnerBasicForm<Data>
where
    Data: 'static + Validate + Debug + Serialize + FormResult,
    FormWrapper<Data>: TryFromCallback<FormData>,
{
    type Message = FormMessage<Data>;
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
            submit_button_disabled: true,
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
            FormMessage::Submit(data) => {
                let mut change = false;
                if !self.errors.is_empty() {
                    self.errors.clear();
                    change = true;
                }

                // First, we validate the data
                if let Err(errors) = data.validate() {
                    self.errors = errors.convert_to_string().into_iter().collect();
                    return true;
                }

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
            FormMessage::SetSubmitButtonDisabled(disabled) => {
                if let Some(timeout) = self.validate_timeout.take() {
                    timeout.cancel();
                }
                if self.submit_button_disabled != disabled {
                    self.submit_button_disabled = disabled;
                    true
                } else {
                    false
                }
            }
            FormMessage::StartSetSubmitButtonDisabledTimeout(disabled) => {
                if let Some(timeout) = self.validate_timeout.take() {
                    timeout.cancel();
                }
                if self.submit_button_disabled == disabled {
                    return false;
                }
                let link = ctx.link().clone();
                self.validate_timeout = Some(Timeout::new(300, move || {
                    link.send_message(FormMessage::SetSubmitButtonDisabled(disabled));
                }));
                false
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
                let link2 = link.clone();
                if let Err(errors) =
                    Self::extract_data_from_form_event(event, move |result| match result {
                        Ok(data) => link2.send_message(FormMessage::Submit(data)),
                        Err(errors) => {
                            link2.send_message(FormMessage::Errors(errors));
                        }
                    })
                {
                    link.send_message(FormMessage::Errors(errors));
                }
            })
        };

        // Every time there is any change in any of the input fields, we validate the form
        // so to know whether the submit button should be enabled or not
        let on_input = {
            let link = ctx.link().clone();
            Callback::from(move |event: InputEvent| {
                let link2 = link.clone();
                if Self::extract_data_from_input_event(event, move |result| {
                    link2.send_message(FormMessage::StartSetSubmitButtonDisabledTimeout(
                        result.is_err(),
                    ))
                })
                .is_err()
                {
                    link.send_message(FormMessage::StartSetSubmitButtonDisabledTimeout(true));
                }
            })
        };

        let classes = format!(
            "standard-form{}",
            if self.errors.is_empty() { " error" } else { "" }
        );

        let button_classes = if self.submit_button_disabled {
            ""
        } else if self.waiting_for_reply {
            "waiting"
        } else {
            "enabled"
        };

        let title_message = if self.waiting_for_reply {
            "You have already submitted the form, please wait for the reply."
        } else if self.submit_button_disabled {
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
            <form enctype={ "multipart/form-data" } disabled={self.waiting_for_reply} class={classes} oninput={on_input} onsubmit={on_submit} method={Data::METHOD.to_string()}>
                <h4>{ Data::title() }</h4>
                <p class="instructions">{Data::description()}</p>
                { ctx.props().children.clone() }
                <InputErrors errors={self.errors.clone()} on_delete={on_delete} />
                <button type="submit" title={title_message} class={button_classes} disabled={self.submit_button_disabled}>
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
    Data: PartialEq + Clone + 'static + Validate + Debug + Serialize + FormResult,
    FormWrapper<Data>: TryFromCallback<FormData>,
{
    let navigator = use_navigator().unwrap();
    html! {
        <InnerBasicForm<Data> navigator={navigator} current={props.current.clone()}>
            { props.children.clone() }
        </InnerBasicForm<Data>>
    }
}
