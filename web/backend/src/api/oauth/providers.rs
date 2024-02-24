//! This module contains the API for the OAuth2 providers.

#[get("/providers")]
/// Returns a list of available OAuth2 providers.
async fn get_providers(pool: web::Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    // We retrieve the system variables using dotenvy.
    dotenvy::dotenv().ok();

    let providers = LoginProvider::get_all(&pool);

    if providers.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let providers = providers.unwrap();
    let mut oauth_providers: Vec<OAuth2LoginProvider> = Vec::new();

    for provider in providers {
        let client_id = env::var(provider.client_id_var_name);
        let redirect_uri = env::var(provider.redirect_uri_var_name);

        if client_id.is_err() || redirect_uri.is_err() {
            return HttpResponse::InternalServerError().finish();
        }

        oauth_providers.push(OAuth2LoginProvider {
            id: provider.id,
            name: provider.name,
            font_awesome_icon: provider.font_awesome_icon,
            client_id: client_id.unwrap(),
            redirect_uri: redirect_uri.unwrap(),
            oauth_url: provider.oauth_url,
            scope: provider.scope,
        });
    }

    HttpResponse::Ok().json(oauth_providers)
}