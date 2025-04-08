pub mod picture;

use crate::combine_path;

pub const ENDPOINT: &str = "/profile";
pub const FULL_ENDPOINT: &str = combine_path!(super::FULL_ENDPOINT, ENDPOINT);
