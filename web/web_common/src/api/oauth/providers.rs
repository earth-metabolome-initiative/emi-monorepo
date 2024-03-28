//! Struct defining an OAuth2 login provider supported by the backend.
use crate::combine_path;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const ENDPOINT: &str = "/providers";
pub const FULL_ENDPOINT: &str = combine_path!(super::FULL_ENDPOINT, ENDPOINT);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OAuth2LoginProvider {
    pub id: Uuid,
    pub name: String,
    pub font_awesome_icon: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub oauth_url: String,
    pub scope: String,
}
