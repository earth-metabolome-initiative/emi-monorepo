//! Submodule to initialize the `instruments` in the database.

use core_structures::{
    traits::CompatibleWith, BallMillMachineModel, CameraModel, CentrifugeModel, CompatibilityRule, ContainerModel, FreezeDrierModel, FreezerModel, PipetteModel, PipetteTipModel, PositioningDeviceModel, Trackable, User, VolumetricContainerModel, WeighingInstrumentModel
};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::trackables::containers::{
    CONICAL_CENTRIFUGAL_TUBE_50ML, POLYSTYRENE_BOX, SAFELOCK_TUBE_2ML,
};
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

pub(crate) fn init_instruments(user: &User, conn: &mut PgConnection) -> anyhow::Result<()> {
    let instrument = core_structures::Trackable::new()
        .name("Instrument".to_owned())?
        .description("Instruments used in laboratory procedures".to_owned())?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let freezer = FreezerModel::new()
        .name(FREEZER.to_owned())?
        .description("-80°C Freezer".to_owned())?
        .parent(Some(instrument.id))?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let polystyrene_box = ContainerModel::from_name(POLYSTYRENE_BOX, conn)?;

    let _rule = CompatibilityRule::new()
        .left_trackable(freezer.id)?
        .right_trackable(polystyrene_box.id)?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let freeze_dryer = FreezeDrierModel::new()
        .name(FREEZE_DRYER.to_owned())?
        .description("Freeze dryer".to_owned())?
        .parent(Some(instrument.id))?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let procedure_conical_tube_builder =
        VolumetricContainerModel::from_name(CONICAL_CENTRIFUGAL_TUBE_50ML, conn)?;
    freeze_dryer.compatible_with(&procedure_conical_tube_builder, user, conn)?;

    let _weighing_scale = WeighingInstrumentModel::new()
        .name(WEIGHING_SCALE.to_owned())?
        .description("Weighing scale for weighing samples".to_owned())?
        .parent(Some(instrument.id))?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let ball_mill_machine = BallMillMachineModel::new()
        .name(BALL_MILL_MACHINE.to_owned())?
        .description("Ball mill machine for grinding samples".to_owned())?
        .parent(Some(instrument.id))?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let safelock_2ml = VolumetricContainerModel::from_name(SAFELOCK_TUBE_2ML, conn)?;

    ball_mill_machine.compatible_with(&safelock_2ml, user, conn)?;

    let safelock_centrifuge = CentrifugeModel::new()
        .name(SAFELOCK_CENTRIFUGE.to_owned())?
        .description("Centrifuge for separating samples".to_owned())?
        .parent(Some(instrument.id))?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    safelock_centrifuge.compatible_with(&safelock_2ml, user, conn)?;

    let _geolocation_instrument = PositioningDeviceModel::new()
        .name(GEOLOCATION_INSTRUMENT.to_owned())?
        .description("Geolocation instrument for tracking positions".to_owned())?
        .parent(Some(instrument.id))?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let _camera_instrument = CameraModel::new()
        .name(CAMERA.to_owned())?
        .description("Camera for capturing images".to_owned())?
        .parent(Some(instrument.id))?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let pipettes = Trackable::new()
        .name(PIPETTES.to_owned())?
        .description("Pipettes for liquid handling".to_owned())?
        .parent(Some(instrument.id))?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let pipette_tips = Trackable::new()
        .name(PIPETTE_TIPS.to_owned())?
        .description("Pipette tips for liquid handling".to_owned())?
        .parent(Some(instrument.id))?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let pipettes_1000 = PipetteModel::new()
        .name(PIPETTES_1000.to_owned())?
        .description("Pipettes for handling 1ml of liquid".to_owned())?
        .parent(Some(pipettes.id))?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let pipette_tips_1000 = PipetteTipModel::new()
        .name(PIPETTE_TIPS_1000.to_owned())?
        .description("Pipette tips for handling 1ml of liquid".to_owned())?
        .parent(Some(pipette_tips.id))?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    pipettes_1000.compatible_with(&pipette_tips_1000, user, conn)?;

    let pipettes_200 = PipetteModel::new()
        .name(PIPETTES_200.to_owned())?
        .description("Pipettes for handling 0.2ml of liquid".to_owned())?
        .parent(Some(pipettes.id))?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let pipette_tips_200 = PipetteTipModel::new()
        .name(PIPETTE_TIPS_200.to_owned())?
        .description("Pipette tips for handling 0.2ml of liquid".to_owned())?
        .parent(Some(pipette_tips.id))?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    pipettes_200.compatible_with(&pipette_tips_200, user, conn)?;

    Ok(())
}
