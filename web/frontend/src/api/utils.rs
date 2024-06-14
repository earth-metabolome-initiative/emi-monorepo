//! Authentication API utilities.
use reqwasm::http::Request;
use web_common::api::oauth::jwt_cookies::AccessToken;

/// Adds the bearer token to the request.
///
/// # Arguments
/// * `request` - The request to add the bearer token to.
/// * `access_token` - The access token to add to the request.
pub(crate) fn add_bearer(request: Request, access_token: &AccessToken) -> Request {
    let header = access_token.header();
    request.header(header.0, &header.1)
}

/// Adds the bearer token to the request if provided.
///
/// # Arguments
/// * `request` - The request to add the bearer token to.
/// * `access_token` - The optional access token to add to the request.
pub(crate) fn add_optional_bearer(request: Request, access_token: Option<&AccessToken>) -> Request {
    if let Some(access_token) = access_token {
        add_bearer(request, access_token)
    } else {
        request
    }
}
