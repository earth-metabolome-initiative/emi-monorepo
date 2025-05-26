//! Submodule to initialize the `reagents` in the database.

use core_structures::{Reagent, TrackableCategory, User};
use diesel_async::AsyncPgConnection;
use web_common_traits::{
    database::{Insertable, InsertableVariant},
    prelude::Builder,
};

use crate::trackable_categories::{DISTILLED_WATER, ETHANOL_95, FORMIC_ACID, METHANOL_HPLC};

pub(crate) async fn init_reagents(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let ethanol_trackable_category = TrackableCategory::from_name(ETHANOL_95, portal_conn)
        .await?
        .expect("Ethanol 95 trackable category not found");

    let _ethanol = Reagent::new()
        .id(ethanol_trackable_category.id)?
        .purity(95.0)?
        .cas_code("64-17-5")?
        .molecular_formulas("CH3CH2OH")?
        .created_by(user.id)?
        .build()?
        .insert(&user.id, portal_conn)
        .await?;

    let methanol_trackable_category = TrackableCategory::from_name(METHANOL_HPLC, portal_conn)
        .await?
        .expect("Ethanol 95 trackable category not found");

    let _methanol = Reagent::new()
        .id(methanol_trackable_category.id)?
        .purity(99.8)?
        .cas_code("67-56-1")?
        .molecular_formulas("CH3OH")?
        .created_by(user.id)?
        .build()?
        .insert(&user.id, portal_conn)
        .await?;

    let formic_acid_trackable_category = TrackableCategory::from_name(FORMIC_ACID, portal_conn)
        .await?
        .expect("Formic acid trackable category not found");

    let _formic_acid = Reagent::new()
        .id(formic_acid_trackable_category.id)?
        .purity(98.0)?
        .cas_code("64-18-6")?
        .molecular_formulas("HCOOH")?
        .created_by(user.id)?
        .build()?
        .insert(&user.id, portal_conn)
        .await?;

    let distilled_water_trackable_category =
        TrackableCategory::from_name(DISTILLED_WATER, portal_conn)
            .await?
            .expect("Formic acid trackable category not found");

    let _distilled_water = Reagent::new()
        .id(distilled_water_trackable_category.id)?
        .purity(100.0)?
        .cas_code("7732-18-5")?
        .molecular_formulas("H2O")?
        .created_by(user.id)?
        .build()?
        .insert(&user.id, portal_conn)
        .await?;

    Ok(())
}
