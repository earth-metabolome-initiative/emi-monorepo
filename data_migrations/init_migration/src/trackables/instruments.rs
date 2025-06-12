//! Submodule to initialize the `instruments` in the database.

use core_structures::{ContainerModel, Trackable, User};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

pub mod ball_mill_instrument;
pub mod centrifuge_instrument;
pub mod pipette_1000;
pub(crate) use pipette_1000::{init_gilson_pipette_1000, init_sarstedt_pipette_tip_1000};
pub mod pipette_200;
pub(crate) use pipette_200::{init_gilson_pipette_200, init_sarstedt_pipette_tip_200};

pub const FREEZER: &str = "Freezer";
pub const FREEZE_DRYER: &str = "Freeze Dryer";
pub const WEIGHING_SCALE: &str = "Weighing Scale";
pub const BALL_MILL_MACHINE: &str = "Ball Mill Machine";
pub const CENTRIFUGE: &str = "Centrifuge";
const PIPETTES: &str = "Pipettes";
const PIPETTE_TIPS: &str = "Pipette Tips";
pub const PIPETTES_1000: &str = "Pipettes 1000μl";
pub const PIPETTE_TIPS_1000: &str = "Pipette Tips 1000μl";
pub const PIPETTES_200: &str = "Pipettes 200μl";
pub const PIPETTE_TIPS_200: &str = "Pipette Tips 200μl";

pub(crate) fn init_instruments(user: &User, portal_conn: &mut PgConnection) {
    let instrument = core_structures::Trackable::new()
        .name(Some("Instrument".to_owned()))
        .unwrap()
        .description(Some("Instruments used in laboratory procedures".to_owned()))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    let _freezer = Trackable::new()
        .name(Some(FREEZER.to_owned()))
        .unwrap()
        .description(Some("-80°C Freezer".to_owned()))
        .unwrap()
        .parent_id(Some(instrument.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    let _freeze_dryer = Trackable::new()
        .name(Some(FREEZE_DRYER.to_owned()))
        .unwrap()
        .description(Some("Freeze dryer".to_owned()))
        .unwrap()
        .parent_id(Some(instrument.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    let _weighing_scale = Trackable::new()
        .name(Some(WEIGHING_SCALE.to_owned()))
        .unwrap()
        .description(Some("Weighing scale for weighing samples".to_owned()))
        .unwrap()
        .parent_id(Some(instrument.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    let _ball_mill_machine = Trackable::new()
        .name(Some(BALL_MILL_MACHINE.to_owned()))
        .unwrap()
        .description(Some("Ball mill machine for grinding samples".to_owned()))
        .unwrap()
        .parent_id(Some(instrument.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    let _centrifuge = Trackable::new()
        .name(Some(CENTRIFUGE.to_owned()))
        .unwrap()
        .description(Some("Centrifuge for separating samples".to_owned()))
        .unwrap()
        .parent_id(Some(instrument.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    let pipettes = Trackable::new()
        .name(Some(PIPETTES.to_owned()))
        .unwrap()
        .description(Some("Pipettes for liquid handling".to_owned()))
        .unwrap()
        .parent_id(Some(instrument.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    let pipette_tips = Trackable::new()
        .name(Some(PIPETTE_TIPS.to_owned()))
        .unwrap()
        .description(Some("Pipette tips for liquid handling".to_owned()))
        .unwrap()
        .parent_id(Some(instrument.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    let _pipettes_1000 = ContainerModel::new()
        .name(Some(PIPETTES_1000.to_owned()))
        .unwrap()
        .description(Some("Pipettes for liquid handling".to_owned()))
        .unwrap()
        .parent_id(Some(pipettes.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .liters(0.001)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    let _pipette_tips_1000 = Trackable::new()
        .name(Some(PIPETTE_TIPS_1000.to_owned()))
        .unwrap()
        .description(Some("Pipette tips for liquid handling".to_owned()))
        .unwrap()
        .parent_id(Some(pipette_tips.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    let _pipettes_200 = ContainerModel::new()
        .name(Some(PIPETTES_200.to_owned()))
        .unwrap()
        .description(Some("Pipettes for liquid handling".to_owned()))
        .unwrap()
        .parent_id(Some(pipettes.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .liters(0.0002)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    let _pipette_tips_200 = Trackable::new()
        .name(Some(PIPETTE_TIPS_200.to_owned()))
        .unwrap()
        .description(Some("Pipette tips for liquid handling".to_owned()))
        .unwrap()
        .parent_id(Some(pipette_tips.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();
}
