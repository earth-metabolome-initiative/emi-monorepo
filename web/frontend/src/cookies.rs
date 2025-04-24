//! Utilities regarding cookies that can be used in the frontend.
//!
//! # Implementation details
//! There exist a type of cookie called `HttpOnly` cookies, which are not
//! accessible from JavaScript. This is a security feature, and is used for
//! cookies that are not meant to be accessed from JavaScript. As such, we will
//! not be able to access the `HttpOnly` cookies from Yew, and these utilities
//! solely exist to access cookies that are not `HttpOnly`.
use api_path::api::oauth::jwt_cookies::USER_ONLINE_COOKIE_NAME;
use wasm_bindgen::JsCast;
use web_sys::window;

/// Returns whether a cookie with the given name exists.
///
/// # Arguments
/// * `cookie_name` - The name of the cookie to check for.
fn check_cookie(cookie_name: &str) -> Result<bool, String> {
    let document = window()
        .ok_or("Failed to get the window object in order to check for cookies")?
        .document()
        .ok_or("Failed to get the document object in order to check for cookies")?
        .dyn_into::<web_sys::HtmlDocument>()
        .map_err(|_| "Failed to convert the document object into an HtmlDocument")?;

    if let Ok(cookie_str) = document.cookie() {
        let cookies: Vec<&str> = cookie_str.split(';').map(str::trim).collect();
        for cookie in cookies {
            let parts: Vec<&str> = cookie.split('=').map(str::trim).collect();
            if parts.len() == 2 && parts[0] == cookie_name {
                // Check if the cookie is http-only
                if let Some(_cookie_element) = document
                    .cookie()
                    .expect("Failed to get cookies")
                    .split(';')
                    .find(|cookie| cookie.trim().starts_with(cookie_name))
                {
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}

/// Returns whether the user is logged in.
///
/// # Implementation details
/// This function checks whether the `USER_ONLINE_COOKIE_NAME` cookie exists.
pub(crate) fn is_logged_in() -> bool {
    check_cookie(USER_ONLINE_COOKIE_NAME).unwrap_or(false)
}
