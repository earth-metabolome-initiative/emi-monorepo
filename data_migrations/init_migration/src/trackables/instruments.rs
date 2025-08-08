//! Submodule to initialize the `instruments` in the database.

use core_structures::{
    BallMillMachineModel, CameraModel, CentrifugeModel, CompatibilityRule, ContainerModel,
    FreezeDrierModel, FreezerModel, PipetteModel, PositioningDeviceModel, Trackable, User,
    VolumetricContainerModel, WeighingInstrumentModel, traits::CompatibleWith,
};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::trackables::containers::{POLYSTYRENE_BOX, SAFELOCK_TUBE_2ML};
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
pub const SAFELOCK_CENTRIFUGE: &str = "Safelock Tubes Centrifuge";
const PIPETTES: &str = "Pipettes";
const PIPETTE_TIPS: &str = "Pipette Tips";
pub const PIPETTES_1000: &str = "Pipettes 1000μl";
pub const PIPETTE_TIPS_1000: &str = "Pipette Tips 1000μl";
pub const PIPETTES_200: &str = "Pipettes 200μl";
pub const PIPETTE_TIPS_200: &str = "Pipette Tips 200μl";
pub const GEOLOCATION_INSTRUMENT: &str = "Geolocation Instrument";
pub const CAMERA: &str = "Camera";

pub(crate) fn init_instruments(user: &User, conn: &mut PgConnection) {
    let instrument = core_structures::Trackable::new()
        .name(Some("Instrument".to_owned()))
        .unwrap()
        .description(Some("Instruments used in laboratory procedures".to_owned()))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let freezer = FreezerModel::new()
        .name(Some(FREEZER.to_owned()))
        .unwrap()
        .description(Some("-80°C Freezer".to_owned()))
        .unwrap()
        .parent(Some(instrument.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let polystyrene_box = ContainerModel::from_name(POLYSTYRENE_BOX, conn)
        .unwrap()
        .expect("Polystyrene Box should exist");

    let _rule = CompatibilityRule::new()
        .left_trackable(freezer.id)
        .unwrap()
        .right_trackable(polystyrene_box.id)
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let _freeze_dryer = FreezeDrierModel::new()
        .name(Some(FREEZE_DRYER.to_owned()))
        .unwrap()
        .description(Some("Freeze dryer".to_owned()))
        .unwrap()
        .parent(Some(instrument.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let _weighing_scale = WeighingInstrumentModel::new()
        .name(Some(WEIGHING_SCALE.to_owned()))
        .unwrap()
        .description(Some("Weighing scale for weighing samples".to_owned()))
        .unwrap()
        .parent(Some(instrument.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let ball_mill_machine = BallMillMachineModel::new()
        .name(Some(BALL_MILL_MACHINE.to_owned()))
        .unwrap()
        .description(Some("Ball mill machine for grinding samples".to_owned()))
        .unwrap()
        .parent(Some(instrument.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let safelock_2ml =
        VolumetricContainerModel::from_name(SAFELOCK_TUBE_2ML, conn).unwrap().unwrap();

    ball_mill_machine.compatible_with(&safelock_2ml, user, conn).unwrap();

    let safelock_centrifuge = CentrifugeModel::new()
        .name(Some(SAFELOCK_CENTRIFUGE.to_owned()))
        .unwrap()
        .description(Some("Centrifuge for separating samples".to_owned()))
        .unwrap()
        .parent(Some(instrument.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    safelock_centrifuge.compatible_with(&safelock_2ml, user, conn).unwrap();

    let _geolocation_instrument = PositioningDeviceModel::new()
        .name(Some(GEOLOCATION_INSTRUMENT.to_owned()))
        .unwrap()
        .description(Some("Geolocation instrument for tracking positions".to_owned()))
        .unwrap()
        .parent(Some(instrument.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let _camera_instrument = CameraModel::new()
        .name(Some(CAMERA.to_owned()))
        .unwrap()
        .description(Some("Camera for capturing images".to_owned()))
        .unwrap()
        .parent(Some(instrument.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let pipettes = Trackable::new()
        .name(Some(PIPETTES.to_owned()))
        .unwrap()
        .description(Some("Pipettes for liquid handling".to_owned()))
        .unwrap()
        .parent(Some(instrument.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let pipette_tips = Trackable::new()
        .name(Some(PIPETTE_TIPS.to_owned()))
        .unwrap()
        .description(Some("Pipette tips for liquid handling".to_owned()))
        .unwrap()
        .parent(Some(instrument.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let _pipettes_1000 = VolumetricContainerModel::new()
        .name(Some(PIPETTES_1000.to_owned()))
        .unwrap()
        .description(Some("Pipettes for liquid handling".to_owned()))
        .unwrap()
        .parent(Some(pipettes.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .liters(0.001)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let _pipette_tips_1000 = Trackable::new()
        .name(Some(PIPETTE_TIPS_1000.to_owned()))
        .unwrap()
        .description(Some("Pipette tips for liquid handling".to_owned()))
        .unwrap()
        .parent(Some(pipette_tips.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let _pipettes_200 = PipetteModel::new()
        .name(Some(PIPETTES_200.to_owned()))
        .unwrap()
        .description(Some("Pipettes for liquid handling".to_owned()))
        .unwrap()
        .parent(Some(pipettes.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let _pipette_tips_200 = Trackable::new()
        .name(Some(PIPETTE_TIPS_200.to_owned()))
        .unwrap()
        .description(Some("Pipette tips for liquid handling".to_owned()))
        .unwrap()
        .parent(Some(pipette_tips.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();
}
