pub mod users;

use crate::combine_path;

pub const ENDPOINT: &str = "/auth";
pub const FULL_ENDPOINT: &str = combine_path!(super::FULL_ENDPOINT, ENDPOINT);
