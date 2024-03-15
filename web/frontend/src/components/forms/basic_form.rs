//! Module providing a yew component that handles a basic form.

use crate::components::forms::InputError;
use crate::workers::WebsocketWorker;
use core::fmt::Display;
use gloo::timers::callback::Timeout;
use reqwasm::http::Method;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use validator::Validate;
use wasm_bindgen::JsCast;
use web_common::api::ws::messages::*;
use web_common::custom_validators::validation_errors::ValidationErrorToString;
use web_sys::FormData;
use web_sys::HtmlFormElement;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::prelude::WorkerBridgeHandle;
use yew_agent::scope_ext::AgentScopeExt;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum FormMethod {
    GET,    // Use GET to execute a query
    POST,   // Use POST to CREATE a new resource
    PUT, // Use PUT instead of POST when you want to UPDATE or PATCH a resource and you have an ID
    DELETE, // Use DELETE to remove a resource
    TRACE, // Use TRACE to test the connection
    HEAD, // Use HEAD to retrieve the headers of a resource
    PATCH, // Use PATCH to partially update a resource
}

impl FormMethod {
    pub fn to_reqwasm(&self) -> Method {
        match self {
            FormMethod::GET => Method::GET,
            FormMethod::POST => Method::POST,
            FormMethod::PUT => Method::PUT,
            FormMethod::DELETE => Method::DELETE,
            FormMethod::TRACE => Method::TRACE,
            FormMethod::HEAD => Method::HEAD,
            FormMethod::PATCH => Method::PATCH,
        }
    }

    pub fn to_crud(&self) -> &'static str {
        match self {
            FormMethod::GET => "Retrieve",
            FormMethod::POST => "Create",
            FormMethod::PUT => "Update",
            FormMethod::DELETE => "Delete",
            FormMethod::TRACE => "Trace",
            FormMethod::HEAD => "Head",
            FormMethod::PATCH => "Patch",
        }
    }

    pub fn ongoing_crud(&self) -> &'static str {
        match self {
            FormMethod::GET => "Retrieving",
            FormMethod::POST => "Creating",
            FormMethod::PUT => "Updating",
            FormMethod::DELETE => "Deleting",
            FormMethod::TRACE => "Tracing",
            FormMethod::HEAD => "Heading",
            FormMethod::PATCH => "Patching",
        }
    }

    pub fn lower(&self) -> &'static str {
        match self {
            FormMethod::GET => "get",
            FormMethod::POST => "post",
            FormMethod::PUT => "put",
            FormMethod::DELETE => "delete",
            FormMethod::TRACE => "trace",
            FormMethod::HEAD => "head",
            FormMethod::PATCH => "patch",
        }
    }

    pub fn font_awesome_icon(&self) -> &'static str {
        match self {
            FormMethod::GET => "fas fa-search",
            FormMethod::POST => "fas fa-plus",
            FormMethod::PUT => "fas fa-pen",
            FormMethod::DELETE => "fas fa-trash",
            FormMethod::TRACE => "fas fa-search",
            FormMethod::HEAD => "fas fa-search",
            FormMethod::PATCH => "fas fa-pen",
        }
    }

    pub fn get() -> FormMethod {
        FormMethod::GET
    }

    pub fn post() -> FormMethod {
        FormMethod::POST
    }

    pub fn put() -> FormMethod {
        FormMethod::PUT
    }

    pub fn delete() -> FormMethod {
        FormMethod::DELETE
    }

    pub fn read() -> FormMethod {
        Self::get()
    }

    pub fn create() -> FormMethod {
        Self::post()
    }

    pub fn update() -> FormMethod {
        Self::put()
    }
}

impl Display for FormMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormMethod::GET => write!(f, "GET"),
            FormMethod::POST => write!(f, "POST"),
            FormMethod::PUT => write!(f, "PUT"),
            FormMethod::DELETE => write!(f, "DELETE"),
            FormMethod::TRACE => write!(f, "TRACE"),
            FormMethod::HEAD => write!(f, "HEAD"),
            FormMethod::PATCH => write!(f, "PATCH"),
        }
    }
}

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
pub struct BasicFormProp {
    pub title: String,
    pub method: FormMethod,
    pub action: FormAction,
    pub children: Html,
}

impl BasicFormProp {
    pub fn crud(&self) -> &'static str {
        self.method.to_crud()
    }

    pub fn ongoing_crud(&self) -> &'static str {
        self.method.ongoing_crud()
    }

    pub fn method(&self) -> FormMethod {
        self.method
    }

    pub fn action(&self) -> FormAction {
        self.action.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }
}

pub struct BasicForm<Data> {
    websocket: WorkerBridgeHandle<WebsocketWorker<FrontendMessage, BackendMessage>>,
    errors: Vec<String>,
    waiting_for_reply: bool,
    submit_button_disabled: bool,
    validate_timeout: Option<Timeout>,
    _phantom: std::marker::PhantomData<Data>,
}

impl<Data> BasicForm<Data>
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

#[derive(Debug, Clone)]
pub enum FormMessage<Data> {
    Submit(Data),
    Backend(BackendMessage),
    RemoveError(usize),
    SetSubmitButtonDisabled(bool),
    StartSetSubmitButtonDisabledTimeout(bool),
}

impl<Data> Component for BasicForm<Data>
where
    Data: PartialEq + Clone + 'static + Validate + Debug + Serialize,
    FormWrapper<Data>: TryFrom<FormData, Error = Vec<String>>,
{
    type Message = FormMessage<Data>;
    type Properties = BasicFormProp;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            websocket: ctx.link().bridge_worker(Callback::from({
                let link = ctx.link().clone();
                move |message: BackendMessage| {
                    link.send_message(FormMessage::Backend(message));
                }
            })),
            errors: vec![],
            waiting_for_reply: false,
            submit_button_disabled: true,
            validate_timeout: None,
            _phantom: std::marker::PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FormMessage::Backend(_bm) => false,
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

                // Then, we send the data to the backend
                self.websocket
                    .send(FrontendMessage::submit(ctx.props().action(), data));

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
        let props = ctx.props();

        let on_submit = {
            let link = ctx.link().clone();
            Callback::from(move |event: SubmitEvent| {
                event.prevent_default();
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
        let on_input = ctx.link().callback(|event: InputEvent| {
            event.prevent_default();
            FormMessage::StartSetSubmitButtonDisabledTimeout(
                Self::extract_data_from_input_event(event).is_err(),
            )
        });

        let classes = if self.errors.is_empty() {
            "standard-form"
        } else {
            "standard-form error"
        };

        let button_classes = if self.submit_button_disabled {
            ""
        } else {
            "enabled"
        };

        let title_message = if self.submit_button_disabled {
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
                    <i class={props.method().font_awesome_icon()}></i>
                    <span>{format!("{} {}", props.crud(), props.title())}</span>
                </button>
            </form>
        }
    }
}
