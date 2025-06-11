//! Submodule for handling product trackables in the migration process.

use core_structures::{CommercialProduct, Trackable, User};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

pub mod containers;
