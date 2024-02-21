//! Logout handler
use crate::api::oauth::AuthenticationGuard;
use actix_web::cookie::time::Duration as ActixWebDuration;
use actix_web::{cookie::Cookie, get, HttpResponse, Responder};

#[get("/oauth/logout")]
async fn logout_handler(_: AuthenticationGuard) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({"status": "success"}))
}
