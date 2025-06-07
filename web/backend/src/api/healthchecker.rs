use actix_web::{HttpResponse, Responder};
use actix_web_codegen::get;
use serde_json::json;

#[get("/healthchecker")]
/// Health checker endpoint to verify if the server is running.
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build API with Rust, Diesel, Postgres,and Actix Web";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}
