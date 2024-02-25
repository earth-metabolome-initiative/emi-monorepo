use serde::{Deserialize, Serialize};
pub mod jwt_cookies;
pub mod providers;

use crate::combine_path;

pub const ENDPOINT: &str = "/oauth";
pub const FULL_ENDPOINT: &str = combine_path!(super::FULL_ENDPOINT, ENDPOINT);

#[derive(Debug, Deserialize, Serialize)]
pub enum OauthErrors {
    Refresh(jwt_cookies::RefreshError),
}
