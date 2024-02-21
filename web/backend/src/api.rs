use actix_web::web;

mod healthchecker;
mod oauth;

use healthchecker::health_checker_handler;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(oauth::logout::logout_handler)
        .service(oauth::github::github_oauth_handler);
    // .service(feedback_list_handler)
    // .service(create_feedback_handler)
    // .service(get_feedback_handler)
    // .service(edit_feedback_handler)
    // .service(delete_feedback_handler);

    conf.service(scope);
}
