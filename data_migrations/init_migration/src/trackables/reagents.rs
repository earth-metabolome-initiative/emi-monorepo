//! Submodule to initialize the `reagents` in the database.

use core_structures::{Reagent, User};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

pub const ETHANOL_95: &str = "Absolute Ethanol, >= 95%";
pub const METHANOL_HPLC: &str = "Methanol, >= 99.8%";
pub const FORMIC_ACID: &str = "Formic acid, 98+%";
pub const DISTILLED_WATER: &str = "Distilled water";
pub const LIQUID_NITROGEN: &str = "Liquid nitrogen";

pub(crate) fn init_reagents(user: &User, conn: &mut PgConnection) {
    let reagent_category = core_structures::Trackable::new()
        .name(Some("Reagent".to_owned()))
        .unwrap()
        .description(Some("Common solvents used in laboratory procedures".to_owned()))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let _ethanol = Reagent::new()
        .name(Some(ETHANOL_95.to_owned()))
        .unwrap()
        .description(Some("Absolute Ethanol, >= 95%, with 5% isopropanol".to_owned()))
        .unwrap()
        .parent_id(Some(reagent_category.id))
        .unwrap()
        .purity(95.0)
        .unwrap()
        .cas_code("64-17-5")
        .unwrap()
        .molecular_formula("CH3CH2OH")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let _methanol = Reagent::new()
        .name(Some(METHANOL_HPLC.to_owned()))
        .unwrap()
        .description(Some("Methanol, >= 99.8%, HPLC grade".to_owned()))
        .unwrap()
        .parent_id(Some(reagent_category.id))
        .unwrap()
        .purity(99.8)
        .unwrap()
        .cas_code("67-56-1")
        .unwrap()
        .molecular_formula("CH3OH")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let _formic_acid = Reagent::new()
        .name(Some(FORMIC_ACID.to_owned()))
        .unwrap()
        .description(Some("Formic acid, 98+%, pure".to_owned()))
        .unwrap()
        .parent_id(Some(reagent_category.id))
        .unwrap()
        .purity(98.0)
        .unwrap()
        .cas_code("64-18-6")
        .unwrap()
        .molecular_formula("HCOOH")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let _distilled_water = Reagent::new()
        .name(Some(DISTILLED_WATER.to_owned()))
        .unwrap()
        .description(Some("Distilled water, pure".to_owned()))
        .unwrap()
        .parent_id(Some(reagent_category.id))
        .unwrap()
        .purity(100.0)
        .unwrap()
        .cas_code("7732-18-5")
        .unwrap()
        .molecular_formula("H2O")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let _liquid_nitrogen = Reagent::new()
        .name(Some(LIQUID_NITROGEN.to_owned()))
        .unwrap()
        .description(Some("Liquid nitrogen, pure".to_owned()))
        .unwrap()
        .parent_id(Some(reagent_category.id))
        .unwrap()
        .purity(100.0)
        .unwrap()
        .cas_code("7727-37-9")
        .unwrap()
        .molecular_formula("N2")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();
}
