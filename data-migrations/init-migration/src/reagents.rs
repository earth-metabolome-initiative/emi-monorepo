//! Submodule to initialize the `reagents` in the database.

use core_structures::{Reagent, User};
use diesel_async::AsyncPgConnection;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

pub(crate) async fn init_reagents(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let _ethanol = Reagent::new()
        .name("Ethanol, 99.8%")?
        .description("Ethanol, 99.8%, Analytical reagent grade")?
        .purity(99.8)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let _methanol = Reagent::new()
        .name("Methanol, >= 99.8%")?
        .description("Methanol, >= 99.8%, HPLC grade")?
        .purity(99.8)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let _formic_acid = Reagent::new()
        .name("Formic acid, 98+%")?
        .description("Formic acid, 98+%, pure")?
        .purity(98.0)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let _distilled_water = Reagent::new()
        .name("Distilled water")?
        .description("Distilled water, pure")?
        .purity(100.0)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    Ok(())
}
