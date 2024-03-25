//! Provide traits needed for executing forms in the frontend and receiving data in the backend.
use serde::Deserialize;
use std::fmt::Display;

use super::database::operations::Operation;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum FormMethod {
    GET,    // Use GET to execute a query
    POST,   // Use POST to CREATE a new resource
    PUT, // Use PUT instead of POST when you want to UPDATE or PATCH a resource and you have an ID
    DELETE, // Use DELETE to remove a resource
}

impl FormMethod {
    #[cfg(feature = "frontend")]
    pub fn to_reqwasm(&self) -> gloo_net::http::Method {
        match self {
            FormMethod::GET => gloo_net::http::Method::GET,
            FormMethod::POST => gloo_net::http::Method::POST,
            FormMethod::PUT => gloo_net::http::Method::PUT,
            FormMethod::DELETE => gloo_net::http::Method::DELETE,
        }
    }

    pub fn to_crud(&self) -> &'static str {
        match self {
            FormMethod::GET => "Retrieve",
            FormMethod::POST => "Create",
            FormMethod::PUT => "Update",
            FormMethod::DELETE => "Delete",
        }
    }

    pub fn ongoing_crud(&self) -> &'static str {
        match self {
            FormMethod::GET => "Retrieving",
            FormMethod::POST => "Creating",
            FormMethod::PUT => "Updating",
            FormMethod::DELETE => "Deleting",
        }
    }

    pub fn lower(&self) -> &'static str {
        match self {
            FormMethod::GET => "get",
            FormMethod::POST => "post",
            FormMethod::PUT => "put",
            FormMethod::DELETE => "delete",
        }
    }

    pub fn font_awesome_icon(&self) -> &'static str {
        match self {
            FormMethod::GET => "fas fa-search",
            FormMethod::POST => "fas fa-plus",
            FormMethod::PUT => "fas fa-pen",
            FormMethod::DELETE => "fas fa-trash",
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

    pub fn is_update(&self) -> bool {
        match self {
            FormMethod::PUT => true,
            _ => false,
        }
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

pub trait TryFromCallback<T>: Sized {
    fn try_from_callback<C>(value: T, callback: C) -> Result<(), Vec<String>>
    where
        C: FnOnce(Result<Self, Vec<String>>) + 'static;
}

pub trait FormResult: Into<Operation> {
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
