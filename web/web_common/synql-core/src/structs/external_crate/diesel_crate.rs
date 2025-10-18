//! Submodule implementing the method `diesel` for the [`ExternalCrate`] struct
//! which initializes a `ExternalCrate` instance describing the `diesel` crate.

use common_traits::builder::Builder;
use lazy_static::lazy_static;

use crate::structs::{ExternalCrate, ExternalMacro};

lazy_static! {
    pub static ref DIESEL_CRATE: ExternalCrate = ExternalCrate::new()
        .name("diesel".to_string())
        .unwrap()
        .version("2.3.1")
        .add_macros([
            ExternalMacro::new().name("table".to_string()).unwrap().build().unwrap(),
            ExternalMacro::new()
                .name("allow_tables_to_appear_in_same_query".to_string())
                .unwrap()
                .build()
                .unwrap(),
            ExternalMacro::new().name("joinable".to_string()).unwrap().build().unwrap(),
        ])
        .unwrap()
        .build()
        .unwrap();
}

impl ExternalCrate {
    /// Initializes a `ExternalCrate` instance describing the `diesel` crate.
    pub fn diesel() -> &'static Self {
        &DIESEL_CRATE
    }
}
