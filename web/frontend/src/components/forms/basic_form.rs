//! Module providing a yew component that handles a basic form.

use crate::api::utils::add_optional_bearer;
use crate::api::FrontendApiError;
use crate::stores::user_state;
use crate::stores::user_state::UserState;
use core::fmt::Display;
use reqwasm::http::Method;
use reqwasm::http::Request;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::ops::Deref;
use validator::{Validate, ValidationError, ValidationErrors, ValidationErrorsKind};
use wasm_bindgen::JsCast;
use web_common::api::oauth::jwt_cookies::AccessToken;
use web_sys::EventTarget;
use web_sys::FormData;
use web_common::api::ApiError;
use web_sys::HtmlFormElement;
use yew::prelude::*;
use yewdux::prelude::*;

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
    pub action: String,
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

    pub fn action(&self) -> String {
        self.action.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn children(&self) -> Html {
        self.children.clone()
    }

    async fn submit<Data>(
        &self,
        data: Data,
        access_token: Option<&AccessToken>,
    ) -> Result<(), FrontendApiError>
    where
        Data: Debug + Serialize,
    {
        // We convert the provided data to JSON using SERDE
        // so to be able to set it as the body of the request.
        let body = serde_json::to_string(&data).map_err(FrontendApiError::from)?;

        let response = add_optional_bearer(
            Request::new(&self.action).method(self.method().to_reqwasm()),
            access_token
        )
        .body(body)
        .send()
        .await
        .map_err(FrontendApiError::from)?;

        match response.status() {
            200 => Ok(()),
            _ => {
                let api_error: ApiError = response.json().await.map_err(FrontendApiError::from)?;
                Err(FrontendApiError::from(api_error))
            }
        }
    }
}

#[function_component(BasicForm)]
pub fn basic_form<Data>(props: &BasicFormProp) -> Html
where
    Data: PartialEq + Clone + 'static + Validate + Debug + Serialize,
    FormWrapper<Data>: TryFrom<FormData, Error = ValidationErrors>,
{
    // TODO: handle cross page syncronization!
    // TODO: handle submit button

    // We retrieve the access token of the user if it is present
    // so to add it to the request as an Authorization header token
    // to the Bearer field.
    let (user_state, _dispatch) = use_store::<UserState>();


    let errors = use_state(|| None);

    let form_button_classes = format!("btn btn-primary {}", props.method.lower());

    let on_submit = {
        let props = props.clone();
        let user_state = user_state.clone();
        let errors = errors.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            log::info!("Submitting form");

            let handle_errors = |message: &str| {
                log::info!("{}", message);
                let mut error = ValidationErrors::new();
                error.add("form", ValidationError::new("Failed to create form data"));
                errors.set(Some(error));
            };

            match event.target() {
                Some(target) => match target.dyn_into::<HtmlFormElement>() {
                    Ok(form) => match FormData::new_with_form(&form) {
                        Ok(form_data) => match FormWrapper::<Data>::try_from(form_data) {
                            Ok(wrapper) => {
                                let data = wrapper.data();
                                log::info!("Form data: {:?}", data);
                                match data.validate() {
                                    Ok(_) => {
                                        log::info!("Form data is valid");
                                        errors.set(None);
                                        let props = props.clone();
                                        let user_state = user_state.clone();
                                        let errors = errors.clone();
                                        wasm_bindgen_futures::spawn_local(async move {
                                            let access_token: Option<&AccessToken> = user_state.access_token();
                                            match props.submit(data, access_token).await {
                                                Ok(_) => {
                                                    log::info!("Form submitted successfully");
                                                }
                                                Err(error) => {
                                                    log::info!("Failed to submit form: {:?}", error);
                                                    //errors.set(Some(error));
                                                }
                                            }
                                        });
                                    }
                                    Err(error) => {
                                        log::info!("Form data is invalid: {:?}", error);
                                        errors.set(Some(error));
                                    }
                                }
                            }
                            Err(error) => {
                                log::info!("Failed to create form wrapper: {:?}", error);
                                errors.set(Some(error));
                            }
                        },
                        Err(_error) => {
                            handle_errors("Failed to create form data");
                        }
                    },
                    Err(_error) => {
                        handle_errors("Failed to get form");
                    }
                },
                None => {
                    handle_errors("Failed to get target");
                }
            }
        })
    };

    html! {
        <form enctype={ "multipart/form-data" } class={format!("standard-form {}", props.method)} action={props.action()} onsubmit={on_submit} method={props.method().to_string()}>
            <h4>{ props.title() }</h4>
            { props.children()}
            if let Some(errors) = errors.deref() {
                <div class="form-group alert alert-danger" role="alert">
                    <ul>
                        { for errors.errors().iter().map(|(field, error_kinds): (&&'static str, &ValidationErrorsKind)| {
                            match error_kinds {
                                ValidationErrorsKind::Field(errors) => {
                                    html! {
                                        <li>{ format!("{}: {}", field, errors.iter().filter(|error| error.message.is_some()).map(|error| error.clone().message.unwrap().to_string()).collect::<Vec<String>>().join(", ")) }</li>
                                    }
                                }
                                ValidationErrorsKind::Struct(_errors) => {
                                    unimplemented!("Struct errors are not yet supported")
                                }
                                ValidationErrorsKind::List(_errors) => {
                                    unimplemented!("List errors are not yet supported")
                                }
                            }
                        }) }
                    </ul>
                </div>
            }
            <button type="submit" class={form_button_classes}>{format!("{} {}", props.crud(), props.title())}</button>
        </form>
    }
}
