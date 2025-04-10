//! This module contains the API for the OAuth2 providers.
use std::env;

use actix_web::{get, web, HttpResponse, Responder};
use backend_errors::Error;
use core_structures::LoginProvider;
use web_common_traits::database::Loadable;

#[get("/providers")]
/// Returns a list of available OAuth2 providers.
async fn get_providers(pool: web::Data<crate::DBPool>) -> impl Responder {
    let Ok(mut conn) = pool.get().await else {
        return HttpResponse::InternalServerError().json(Error::DieselError);
    };
    let Ok(providers) = LoginProvider::load_all(&mut conn).await else {
        return HttpResponse::InternalServerError().json(Error::DieselError);
    };

    let mut oauth_providers: Vec<OAuth2LoginProvider> = Vec::new();

    for provider in providers {
        let Ok(icon) = provider.icon(&mut conn).await else {
            return HttpResponse::InternalServerError().json(Error::DieselError);
        };

        let (Ok(client_id), Ok(redirect_uri)) =
            (env::var(provider.client_id_var_name), env::var(provider.redirect_uri_var_name))
        else {
            return HttpResponse::InternalServerError().json(Error::DieselError);
        };

        oauth_providers.push(OAuth2LoginProvider {
            id: provider.id,
            name: provider.name,
            icon: icon.name,
            client_id,
            redirect_uri,
            oauth_url: provider.oauth_url,
            scope: provider.scope,
        });
    }

    HttpResponse::Ok().json(oauth_providers)
}
