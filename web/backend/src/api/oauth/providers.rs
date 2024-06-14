//! This module contains the API for the OAuth2 providers.
use crate::database::*;
use actix_web::{get, web, HttpResponse, Responder};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use std::env;
use web_common::api::oauth::providers::*;
use web_common::api::ApiError;

#[get("/providers")]
/// Returns a list of available OAuth2 providers.
async fn get_providers(pool: web::Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let providers = NestedLoginProvider::all_viewable(None, None, None, &mut conn);

    if providers.is_err() {
        return HttpResponse::InternalServerError().json(ApiError::internal_server_error());
    }

    let providers = providers.unwrap();
    let mut oauth_providers: Vec<OAuth2LoginProvider> = Vec::new();

    for provider in providers {
        let client_id = env::var(provider.inner.client_id_var_name);
        let redirect_uri = env::var(provider.inner.redirect_uri_var_name);

        if client_id.is_err() || redirect_uri.is_err() {
            return HttpResponse::InternalServerError().json(ApiError::internal_server_error());
        }

        oauth_providers.push(OAuth2LoginProvider {
            id: provider.inner.id,
            name: provider.inner.name,
            icon: provider.icon.name,
            client_id: client_id.unwrap(),
            redirect_uri: redirect_uri.unwrap(),
            oauth_url: provider.inner.oauth_url,
            scope: provider.inner.scope,
        });
    }

    HttpResponse::Ok().json(oauth_providers)
}
