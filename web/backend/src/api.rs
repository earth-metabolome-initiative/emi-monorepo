use actix_web::web;

mod healthchecker;
mod oauth;
mod logged_user_info;

use healthchecker::health_checker_handler;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(oauth::get_providers)
        .service(oauth::logout::logout_handler)
        .service(logged_user_info::logged_user_info)
        .service(oauth::github::github_oauth_handler);
    // .service(feedback_list_handler)
    // .service(create_feedback_handler)
    // .service(get_feedback_handler)
    // .service(edit_feedback_handler)
    // .service(delete_feedback_handler);

    conf.service(scope);
}
