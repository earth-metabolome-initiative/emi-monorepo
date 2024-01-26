use std::fmt::Debug;

use crate::traits::IntoDefault;

pub trait Version: Default {
    const VERSION: usize;
    type Core: ToString + Debug + IntoDefault;
    type Config: ToString + Debug + IntoDefault;
}
#[derive(Default)]
pub struct Version5;

impl Version for Version5 {
    const VERSION: usize = 5;
    type Core = crate::parameters::core::CoreV5;
    type Config = crate::parameters::config::ConfigV5;
}
