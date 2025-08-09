//! Submodule to initialize the `reagents` in the database.

use core_structures::{Reagent, User};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

pub const ETHANOL_95: &str = "Absolute Ethanol, >= 95%";
pub const METHANOL_HPLC: &str = "Methanol, >= 99.8%";
pub const FORMIC_ACID: &str = "Formic acid, 98+%";
pub const DISTILLED_WATER: &str = "Distilled water";
pub const LIQUID_NITROGEN: &str = "Liquid nitrogen";

pub(crate) fn init_reagents(user: &User, conn: &mut PgConnection) -> anyhow::Result<()> {
    let reagent_category = core_structures::Trackable::new()
        .name("Reagent".to_owned())?
        .description("Common solvents used in laboratory procedures".to_owned())?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let _ethanol = Reagent::new()
        .name(ETHANOL_95.to_owned())?
        .description("Absolute Ethanol, >= 95%, with 5% isopropanol".to_owned())?
        .parent(Some(reagent_category.id))?
        .purity(95.0)?
        .cas_code("64-17-5")?
        .molecular_formula("CH3CH2OH")?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let _methanol = Reagent::new()
        .name(METHANOL_HPLC.to_owned())?
        .description("Methanol, >= 99.8%, HPLC grade".to_owned())?
        .parent(Some(reagent_category.id))?
        .purity(99.8)?
        .cas_code("67-56-1")?
        .molecular_formula("CH3OH")?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let _formic_acid = Reagent::new()
        .name(FORMIC_ACID.to_owned())?
        .description("Formic acid, 98+%, pure".to_owned())?
        .parent(Some(reagent_category.id))?
        .purity(98.0)?
        .cas_code("64-18-6")?
        .molecular_formula("HCOOH")?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let _distilled_water = Reagent::new()
        .name(DISTILLED_WATER.to_owned())?
        .description("Distilled water, pure".to_owned())?
        .parent(Some(reagent_category.id))?
        .purity(100.0)?
        .cas_code("7732-18-5")?
        .molecular_formula("H2O")?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let _liquid_nitrogen = Reagent::new()
        .name(LIQUID_NITROGEN.to_owned())?
        .description("Liquid nitrogen, pure".to_owned())?
        .parent(Some(reagent_category.id))?
        .purity(100.0)?
        .cas_code("7727-37-9")?
        .molecular_formula("N2")?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    Ok(())
}
