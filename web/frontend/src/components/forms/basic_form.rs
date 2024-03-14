//! Module providing a yew component that handles a basic form.

use crate::api::utils::add_optional_bearer;
use crate::api::FrontendApiError;
use crate::components::forms::InputError;
use crate::stores::user_state;
use crate::stores::user_state::UserState;
use crate::workers::WebsocketWorker;
use core::fmt::Display;
use reqwasm::http::Method;
use reqwasm::http::Request;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::ops::Deref;
use validator::{Validate, ValidationError, ValidationErrors, ValidationErrorsKind};
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_common::api::auth::users::User;
use web_common::api::oauth::jwt_cookies::AccessToken;
use web_common::api::ws::messages::*;
use web_common::api::ApiError;
use web_sys::EventTarget;
use web_sys::FormData;
use web_sys::HtmlFormElement;
use yew::html::IntoPropValue;
use yew::prelude::*;
use yew_agent::prelude::WorkerBridgeHandle;
use yew_agent::scope_ext::AgentScopeExt;
use yewdux::prelude::*;

use super::BasicInput;

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
    _phantom: std::marker::PhantomData<Data>,
}

#[derive(Debug, Clone)]
pub enum FormMessage<Data> {
    Submit(Data),
    Backend(BackendMessage),
    RemoveError(usize),
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
            _phantom: std::marker::PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FormMessage::Backend(_bm) => false,
            FormMessage::Submit(data) => {
                // First, we validate the data
                if let Err(errors) = data.validate() {
                    return true;
                }

                // Then, we send the data to the backend
                self.websocket
                    .send(FrontendMessage::submit(ctx.props().action(), data));

                self.waiting_for_reply = true;

                true
            }
            FormMessage::RemoveError(error_number) => {
                self.errors.remove(error_number);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let on_submit = {
            let link = ctx.link().clone();
            Callback::from(move |event: SubmitEvent| {
                event.prevent_default();
                let form = event
                    .target()
                    .unwrap_throw()
                    .dyn_into::<HtmlFormElement>()
                    .unwrap_throw();
                let data =
                    FormWrapper::<Data>::try_from(FormData::new_with_form(&form).unwrap_throw())
                        .unwrap_throw()
                        .data();
                link.send_message(FormMessage::Submit(data));
            })
        };

        let classes = if self.errors.is_empty() {
            "standard-form"
        } else {
            "standard-form error"
        };

        html! {
            <form enctype={ "multipart/form-data" } class={classes} onsubmit={on_submit} method={props.method().to_string()}>
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
                <button type="submit" class="btn btn-primary">{format!("{} {}", props.crud(), props.title())}</button>
            </form>
        }
    }
}
