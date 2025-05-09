//! Submodule defining the init migrations for the commercial products.

use core_structures::{CommercialProduct, User};
use diesel_async::AsyncPgConnection;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

use crate::brands::{acros_organics, fisher_scientific};

pub(crate) async fn init_commercial_products(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let fisher_scientific_brand = fisher_scientific(user, portal_conn).await?;
    let acros_organics_brand = acros_organics(user, portal_conn).await?;

    let ethanol = CommercialProduct::new()
        .name("Ethanol, 99.8%")?
        .description("Ethanol, 99.8%, Analytical reagent grade")?
        .created_by(user.id)?
        .brand_id(fisher_scientific_brand.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let methanol = CommercialProduct::new()
        .name("Methanol, >= 99.8%")?
        .description("Methanol, >= 99.8%, HPLC grade")?
        .created_by(user.id)?
        .brand_id(fisher_scientific_brand.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let formic_acid = CommercialProduct::new()
        .name("Formic acid, 98+%")?
        .description("Formic acid, 98+%, pure")?
        .created_by(user.id)?
        .brand_id(acros_organics_brand.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    Ok(())
}
