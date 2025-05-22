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
        .name("Absolute Ethanol, >= 95%")?
        .description("Absolute Ethanol, >= 95%, with 5% isopropanol")?
        .purity(95.0)?
        .cas_code("64-17-5")?
        .molecular_formula("CH3CH2OH")?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let _methanol = Reagent::new()
        .name("Methanol, >= 99.8%")?
        .description("Methanol, >= 99.8%, HPLC grade")?
        .purity(99.8)?
        .cas_code("67-56-1")?
        .molecular_formula("CH3OH")?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let _formic_acid = Reagent::new()
        .name("Formic acid, 98+%")?
        .description("Formic acid, 98+%, pure")?
        .purity(98.0)?
        .cas_code("64-18-6")?
        .molecular_formula("HCOOH")?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let _distilled_water = Reagent::new()
        .name("Distilled water")?
        .description("Distilled water, pure")?
        .purity(100.0)?
        .cas_code("7732-18-5")?
        .molecular_formula("H2O")?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    Ok(())
}
