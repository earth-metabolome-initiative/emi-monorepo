//! Struct defining an OAuth2 login provider supported by the backend.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OAuth2LoginProvider {
    pub id: i16,
    pub name: String,
    pub font_awesome_icon: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub oauth_url: String,
    pub scope: String,
}
