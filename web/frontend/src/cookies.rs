//! Utilities regarding cookies that can be used in the frontend.
//!
//! # Implementative details
//! There exist a type of cookie called `HttpOnly` cookies, which are not accessible from JavaScript.
//! This is a security feature, and is used for cookies that are not meant to be accessed from JavaScript.
//! As such, we will not be able to access the `HttpOnly` cookies from Yew, and these utilities solely
//! exist to access cookies that are not `HttpOnly`.
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_common::api::oauth::jwt_cookies::USER_ONLINE_COOKIE_NAME;
use web_sys::window;

/// Returns whether a cookie with the given name exists.
///
/// # Arguments
/// * `cookie_name` - The name of the cookie to check for.
fn check_cookie(cookie_name: &str) -> bool {
    let document = window()
        .unwrap_throw()
        .document()
        .unwrap_throw()
        .dyn_into::<web_sys::HtmlDocument>()
        .unwrap_throw();

    if let Ok(cookie_str) = document.cookie() {
        let cookies: Vec<&str> = cookie_str.split(';').map(|c| c.trim()).collect();
        for cookie in cookies {
            let parts: Vec<&str> = cookie.split('=').map(|c| c.trim()).collect();
            if parts.len() == 2 && parts[0] == cookie_name {
                // Check if the cookie is http-only
                if let Some(_cookie_element) = document
                    .cookie()
                    .expect("Failed to get cookies")
                    .split(';')
                    .find(|cookie| cookie.trim().starts_with(cookie_name))
                {
                    return true;
                }
            }
        }
    }

    false
}

/// Returns whether the user is logged in.
///
/// # Implementation details
/// This function checks whether the USER_ONLINE_COOKIE_NAME cookie exists.
pub(crate) fn is_logged_in() -> bool {
    check_cookie(USER_ONLINE_COOKIE_NAME)
}
