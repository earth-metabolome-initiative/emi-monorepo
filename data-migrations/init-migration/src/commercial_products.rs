//! Submodule defining the init migrations for the commercial products.

use core_structures::{CommercialProduct, User};
use diesel_async::AsyncPgConnection;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

pub(crate) async fn init_commercial_products(
    darwin: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let ethanol = CommercialProduct::new()
        .name("Ethanol, 99.8%")?
        .description("Ethanol, 99.8%, Analytical reagent grade")?
        .created_by(darwin.id)?
        // .brand_id(brand_id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    Ok(())
}
