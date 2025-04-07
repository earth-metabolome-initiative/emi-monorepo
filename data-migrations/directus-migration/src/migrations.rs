//! Submodule providing the methods for all of the directus migrations.

mod utils;
pub(crate) use utils::get_user;
mod insert_missing_brands;
pub use insert_missing_brands::insert_missing_brands;
mod insert_missing_users;
pub use insert_missing_users::insert_missing_users;
mod ensure_instrument_types_compatibility;
pub(crate) use ensure_instrument_types_compatibility::ensure_instrument_types_compatibility;
mod insert_missing_instrument_models;
pub(crate) use insert_missing_instrument_models::insert_missing_instrument_models;
mod insert_missing_instruments;
pub(crate) use insert_missing_instruments::insert_missing_instruments;