//! Module providing error handling for the backend.

use actix_web::cookie::{time::Duration, Cookie};
use actix_web::{dev::ServiceResponse, http::header, middleware::ErrorHandlerResponse, Result};
use log::debug;

pub(crate) fn handle_unauthorized_request<B>(
    res: ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>> {
    // split service response into request and response components
    let (req, mut res) = res.into_parts();

    // We printout the headers of the request to help with debugging
    debug!("Response: {:?}", res.headers());

    res.headers_mut()
        .insert(header::LOCATION, "/login".parse().unwrap());

    let mut res = res.set_body("An error occurred.");

    // We delete the token cookie so that if the current cookie is invalid, the user is logged out.
    let cookie = Cookie::build("token", "")
        .path("/")
        .http_only(true)
        .max_age(Duration::ZERO)
        .finish();

    // We set the response to include the cookie
    res.add_cookie(&cookie);

    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res))
}

/// Error handler for 500 internal server errors.
pub(crate) fn handle_internal_server_error<B>(
    res: ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>> {
    // split service response into request and response components
    let (req, mut res) = res.into_parts();

    res.headers_mut()
        .insert(header::LOCATION, "/error".parse().unwrap());

    let mut res = res.set_body("An error occurred.");

    // We delete the token cookie so that if the current cookie is invalid, the user is logged out.
    let cookie = Cookie::build("token", "")
        .path("/")
        .http_only(true)
        .max_age(Duration::ZERO)
        .finish();

    // We set the response to include the cookie
    res.add_cookie(&cookie);

    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res))
}
