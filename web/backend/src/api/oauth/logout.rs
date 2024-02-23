//! Logout handler
use crate::api::oauth::AuthenticationGuard;
use actix_web::cookie::time::Duration as ActixWebDuration;
use actix_web::{cookie::Cookie, get, HttpResponse, Responder};
use actix_web::http::header::LOCATION;

#[get("/logout")]
async fn logout_handler() -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::ZERO)
        .http_only(true)
        .finish();

    // We redirect the user to the login page.
    // Redirect to login page if unauthenticated
    HttpResponse::TemporaryRedirect()
        .insert_header((LOCATION, "/login"))
        .cookie(cookie)
        .finish()
}
