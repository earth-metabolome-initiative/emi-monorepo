//! Module providing a yew component that handles a basic form.

use core::fmt::Display;
use serde::Deserialize;
use validator::{Validate};
use web_sys::FormData;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum FormMethod {
    GET,    // Use GET to execute a query
    POST,   // Use POST to CREATE a new resource
    PUT, // Use PUT instead of POST when you want to UPDATE or PATCH a resource and you have an ID
    DELETE, // Use DELETE to remove a resource
}

impl FormMethod {
    pub fn to_crud(&self) -> &'static str{
        match self {
            FormMethod::GET => "Retrieve",
            FormMethod::POST => "Create",
            FormMethod::PUT => "Update",
            FormMethod::DELETE => "Delete",
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
        }
    }
}

pub trait Form: Properties + Validate + Clone + PartialEq {
    fn action(&self) -> String;
    fn method(&self) -> FormMethod;
    fn inputs(&self) -> Html;
    fn crud(&self) -> &'static str {
        self.method().to_crud()
    }
}

// fn form_submit<F: Form>(form: F) -> Callback<FocusEvent> {
//     Callback::from(move |event: FocusEvent| {
//         event.prevent_default();
//         let form = event
//             .target()
//             .unwrap()
//             .dyn_ref::<web_sys::HtmlFormElement>()
//             .unwrap();
//         let data = FormData::new_with_form(form).unwrap();
//         let request = Request::new(format!("/api/{}", form.action()), form.method().to_string());
//         request.send_with_form(&data).unwrap();
//     })
// }

#[function_component(BasicForm)]
pub fn basic_form<F: Form>(form: &F) -> Html {
    // TODO: handle cross page syncronization!
    // TODO: handle error messages
    // TODO: handle submit button

    let form_button_classes = format!("btn btn-primary {}", form.method());

    html! {
        <form class={format!("standard-form {}", form.method())} action={form.action()} method={form.method().to_string()}>
            { form.inputs()}
            // { if let Some(error) = error {
            //     html! { <div class="invalid-feedback">{ error.message }</div> }
            // } else {
            //     html! {}
            // } }
            <button type="submit" class={form_button_classes}>{form.crud()}</button>
        </form>
    }
}
