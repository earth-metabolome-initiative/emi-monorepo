//! Module providing a yew component that handles a basic form.

use crate::components::forms::InputError;
use crate::router::AppRoute;
use crate::stores::app_state::AppState;
use crate::stores::user_state::UserState;
use gloo::timers::callback::Timeout;
use serde::Serialize;
use std::fmt::Debug;
use std::rc::Rc;
use validator::Validate;
use wasm_bindgen::JsCast;
use web_common::api::ws::messages::*;
use web_common::custom_validators::validation_errors::ValidationErrorToString;
use web_sys::FormData;
use web_sys::HtmlFormElement;
use web_sys::HtmlInputElement;
use yew::prelude::*;
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
    pub action: FormAction,
    pub children: Html,
    #[prop_or_default]
    pub navigator: Option<yew_router::navigator::Navigator>,
    #[prop_or_default]
    pub current: Option<Data>,
}

impl<Data> BasicFormProp<Data>
where
    Data: PartialEq + Clone + 'static + Validate + Debug + Serialize,
{
    pub fn crud(&self) -> &'static str {
        self.method().to_crud()
    }

    pub fn ongoing_crud(&self) -> &'static str {
        self.method().ongoing_crud()
    }

    pub fn method(&self) -> FormMethod {
        self.action.method()
    }

    pub fn action(&self) -> FormAction {
        self.action.clone()
    }

    pub fn title(&self) -> String {
        self.action.title()
    }
}

pub struct InnerBasicForm<Data> {
    app_state: Rc<AppState>,
    app_dispatch: Dispatch<AppState>,
    user_state: Rc<UserState>,
    _user_dispatch: Dispatch<UserState>,
    errors: Vec<String>,
    waiting_for_reply: bool,
    submit_button_disabled: bool,
    uuid: Option<uuid::Uuid>,
    validate_timeout: Option<Timeout>,
    _phantom: std::marker::PhantomData<Data>,
}

impl<Data> InnerBasicForm<Data>
where
    FormWrapper<Data>: TryFrom<FormData, Error = Vec<String>>,
{
    fn extract_data_from_input_event<E>(event: E) -> Result<Data, String>
    where
        E: AsRef<Event>,
    {
        let form = event
            .as_ref()
            .target()
            .ok_or("No target found in the event")?
            .dyn_into::<HtmlInputElement>()
            .map_err(|e| format!("The target of the event is not a form: {:?}", e))?
            .closest("form")
            .map_err(|e| format!("Error getting the closest form: {:?}", e))?
            .ok_or("No form found")?
            .dyn_into::<HtmlFormElement>()
            .map_err(|e| format!("The target of the event is not a form: {:?}", e))?;
        Self::extract_data_from_form(form)
    }

    fn extract_data_from_form_event<E>(event: E) -> Result<Data, String>
    where
        E: AsRef<Event>,
    {
        let form = event
            .as_ref()
            .target()
            .ok_or("No target found in the event")?
            .dyn_into::<HtmlFormElement>()
            .map_err(|e| format!("The target of the event is not a form: {:?}", e))?;
        Self::extract_data_from_form(form)
    }

    fn extract_data_from_form(form: HtmlFormElement) -> Result<Data, String> {
        let data = FormWrapper::<Data>::try_from(
            FormData::new_with_form(&form)
                .map_err(|e| format!("Error creating form data: {:?}", e))?,
        )
        .map_err(|e| e.join("\n"))?
        .data();
        Ok(data)
    }
}

pub enum FormMessage<Data> {
    Submit(Data),
    Backend(BackendMessage),
    RemoveError(usize),
    SetSubmitButtonDisabled(bool),
    StartSetSubmitButtonDisabledTimeout(bool),
    AppState(Rc<AppState>),
    UserState(Rc<UserState>),
}

impl<Data> Component for InnerBasicForm<Data>
where
    Data: PartialEq + Clone + 'static + Validate + Debug + Serialize,
    FormWrapper<Data>: TryFrom<FormData, Error = Vec<String>>,
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
            errors: vec![],
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
            FormMessage::Backend(BackendMessage::ExpiredToken) => {
                log::info!("Token expired, redirecting to login page.");
                if let Some(navigator) = ctx.props().navigator.as_ref() {
                    navigator.push(&AppRoute::Login);
                }
                false
            }
            FormMessage::Backend(BackendMessage::Authenticated) => false,
            FormMessage::Backend(BackendMessage::TaskResult(uuid, form_action, outcome)) => {
                if form_action == ctx.props().action() && self.uuid == Some(uuid) {
                    self.waiting_for_reply = false;
                    match outcome {
                        Ok(()) => {
                            log::info!("Form submitted successfully");
                            true
                        }
                        Err(errors) => {
                            self.errors = errors.into();
                            true
                        }
                    }
                } else {
                    false
                }
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
                    self.errors = errors.convert_to_string();
                    return true;
                }

                let uuid = uuid::Uuid::new_v4();
                self.uuid = Some(uuid);

                let action = ctx.props().action().clone();

                // Then, we send the data to the backend
                self.app_dispatch.reduce_mut(move |state| {
                    state.add_task((uuid, action, bincode::serialize(&data).unwrap()));
                });

                self.waiting_for_reply = true;

                change
            }
            FormMessage::RemoveError(error_number) => {
                self.errors.remove(error_number);
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
                let link = ctx.link().clone();
                self.validate_timeout = Some(Timeout::new(300, move || {
                    link.send_message(FormMessage::SetSubmitButtonDisabled(disabled));
                }));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if ctx.props().action().requires_authentication() {
            if let Some(navigator) = ctx.props().navigator.as_ref() {
                if self.user_state.has_no_access_token() {
                    log::info!("No access token found, redirecting to login page.");
                    navigator.push(&AppRoute::Login);
                }
            }
        }

        let props = ctx.props();

        let on_submit = {
            let link = ctx.link().clone();
            let waiting_for_reply = self.waiting_for_reply;
            Callback::from(move |event: SubmitEvent| {
                event.prevent_default();
                if waiting_for_reply {
                    return;
                }
                match Self::extract_data_from_form_event(event) {
                    Ok(data) => link.send_message(FormMessage::Submit(data)),
                    Err(errors) => {
                        log::error!("Error extracting data from form: {:?}", errors);
                    }
                }
            })
        };

        // Every time there is any change in any of the input fields, we validate the form
        // so to know whether the submit button should be enabled or not
        let on_input = {
            let current = ctx.props().current.clone();
            let method = ctx.props().action().method();
            let link = ctx.link().clone();
            Callback::from(move |event: InputEvent| {
                link.send_message(FormMessage::StartSetSubmitButtonDisabledTimeout(
                    match Self::extract_data_from_input_event(event) {
                        Ok(data) => current
                            .as_ref()
                            .map_or(false, |current| *current == data && method.is_update()),
                        Err(_) => true,
                    },
                ));
            })
        };

        let classes = if self.errors.is_empty() {
            "standard-form"
        } else {
            "standard-form error"
        };

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

        html! {
            <form enctype={ "multipart/form-data" } class={classes} oninput={on_input} onsubmit={on_submit} method={props.method().to_string()}>
                <h4>{ props.title() }</h4>
                { ctx.props().children.clone() }
                <ul class="form-errors">
                    { for self.errors.iter().enumerate().map(|(error_number, error)| {
                        let link = ctx.link().clone();
                        let on_delete = Callback::from(move |_| {
                            link.send_message(FormMessage::RemoveError(error_number));
                        });
                        html! {
                            <InputError error={error.clone()} on_delete={on_delete}/>
                        }
                    })
                    }
                </ul>
                <button type="submit" title={title_message} class={button_classes} disabled={self.submit_button_disabled}>
                    {if self.waiting_for_reply {
                        html! { <i class="fas fa-spinner fa-spin"></i> }
                    } else {
                        html! { <i class={props.method().font_awesome_icon()}></i> }
                    }}
                    <span>{format!("{} {}", props.crud(), props.title())}</span>
                </button>
            </form>
        }
    }
}

#[function_component(BasicForm)]
pub fn basic_form<Data>(props: &BasicFormProp<Data>) -> Html
where
    Data: PartialEq + Clone + 'static + Validate + Debug + Serialize,
    FormWrapper<Data>: TryFrom<FormData, Error = Vec<String>>,
{
    let navigator = use_navigator().unwrap();
    html! {
        <InnerBasicForm<Data> navigator={navigator} action={props.action.clone()} current={props.current.clone()}>
            { props.children.clone() }
        </InnerBasicForm<Data>>
    }
}
