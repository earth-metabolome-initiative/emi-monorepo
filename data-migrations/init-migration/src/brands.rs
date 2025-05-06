//! Submodule defining the init migrations for the brands.

use core_structures::{Brand, User};
use diesel_async::AsyncPgConnection;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

pub(crate) async fn init_brands(
    darwin: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    Brand::new()
        .name("Fisher Scientific")?
        .created_by(darwin.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    Ok(())
}
