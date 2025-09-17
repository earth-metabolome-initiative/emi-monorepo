#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enum representing insert errors which may occur when inserting an entry from
/// the `asset_models` table DAG.
pub enum AssetModelInsertErrorDAG {
    ///Insert error associated with the `asset_models` table.
    AssetModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute,
        >,
    ),
    ///Insert error associated with the `ball_mill_machine_models` table.
    BallMillMachineModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::BallMillMachineModelAttribute,
        >,
    ),
    ///Insert error associated with the `bead_models` table.
    BeadModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::BeadModelAttribute,
        >,
    ),
    ///Insert error associated with the `camera_models` table.
    CameraModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CameraModelAttribute,
        >,
    ),
    ///Insert error associated with the `cap_models` table.
    CapModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CapModelAttribute,
        >,
    ),
    ///Insert error associated with the `centrifuge_models` table.
    CentrifugeModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CentrifugeModelAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_ball_mill_machine_lots` table.
    CommercialBallMillMachineLot(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialBallMillMachineLotAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_ball_mill_machine_models` table.
    CommercialBallMillMachineModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialBallMillMachineModelAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_bead_lots` table.
    CommercialBeadLot(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialBeadLotAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_bead_models` table.
    CommercialBeadModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialBeadModelAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_camera_lots` table.
    CommercialCameraLot(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialCameraLotAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_camera_models` table.
    CommercialCameraModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialCameraModelAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_cap_lots` table.
    CommercialCapLot(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialCapLotAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_cap_models` table.
    CommercialCapModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialCapModelAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_centrifuge_lots` table.
    CommercialCentrifugeLot(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeLotAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_centrifuge_models` table.
    CommercialCentrifugeModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeModelAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_freeze_dryer_lots` table.
    CommercialFreezeDryerLot(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerLotAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_freeze_dryer_models` table.
    CommercialFreezeDryerModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerModelAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_freezer_lots` table.
    CommercialFreezerLot(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialFreezerLotAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_freezer_models` table.
    CommercialFreezerModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialFreezerModelAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_packaging_lots` table.
    CommercialPackagingLot(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPackagingLotAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_packaging_models` table.
    CommercialPackagingModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPackagingModelAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_pipette_lots` table.
    CommercialPipetteLot(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPipetteLotAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_pipette_models` table.
    CommercialPipetteModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPipetteModelAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_pipette_tip_lots` table.
    CommercialPipetteTipLot(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPipetteTipLotAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_pipette_tip_models` table.
    CommercialPipetteTipModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPipetteTipModelAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_positioning_device_lots` table.
    CommercialPositioningDeviceLot(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPositioningDeviceLotAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_positioning_device_models` table.
    CommercialPositioningDeviceModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPositioningDeviceModelAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_product_lots` table.
    CommercialProductLot(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_products` table.
    CommercialProduct(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_volume_measuring_device_lots` table.
    CommercialVolumeMeasuringDeviceLot(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceLotAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_volume_measuring_device_models` table.
    CommercialVolumeMeasuringDeviceModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceModelAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_weighing_device_lots` table.
    CommercialWeighingDeviceLot(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceLotAttribute,
        >,
    ),
    ///Insert error associated with the `commercial_weighing_device_models` table.
    CommercialWeighingDeviceModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceModelAttribute,
        >,
    ),
    ///Insert error associated with the `container_models` table.
    ContainerModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::ContainerModelAttribute,
        >,
    ),
    ///Insert error associated with the `digital_asset_models` table.
    DigitalAssetModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::DigitalAssetModelAttribute,
        >,
    ),
    ///Insert error associated with the `freeze_dryer_models` table.
    FreezeDryerModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezeDryerModelAttribute,
        >,
    ),
    ///Insert error associated with the `freezer_models` table.
    FreezerModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezerModelAttribute,
        >,
    ),
    ///Insert error associated with the `organism_models` table.
    OrganismModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::OrganismModelAttribute,
        >,
    ),
    ///Insert error associated with the `packaging_models` table.
    PackagingModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PackagingModelAttribute,
        >,
    ),
    ///Insert error associated with the `phone_models` table.
    PhoneModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute,
        >,
    ),
    ///Insert error associated with the `physical_asset_models` table.
    PhysicalAssetModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelAttribute,
        >,
    ),
    ///Insert error associated with the `pipette_models` table.
    PipetteModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PipetteModelAttribute,
        >,
    ),
    ///Insert error associated with the `pipette_tip_models` table.
    PipetteTipModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PipetteTipModelAttribute,
        >,
    ),
    ///Insert error associated with the `positioning_device_models` table.
    PositioningDeviceModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelAttribute,
        >,
    ),
    ///Insert error associated with the `reagent_models` table.
    ReagentModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::ReagentModelAttribute,
        >,
    ),
    ///Insert error associated with the `sample_models` table.
    SampleModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SampleModelAttribute,
        >,
    ),
    ///Insert error associated with the `sample_source_models` table.
    SampleSourceModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SampleSourceModelAttribute,
        >,
    ),
    ///Insert error associated with the `soil_models` table.
    SoilModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SoilModelAttribute,
        >,
    ),
    ///Insert error associated with the `volume_measuring_device_models` table.
    VolumeMeasuringDeviceModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceModelAttribute,
        >,
    ),
    ///Insert error associated with the `volumetric_container_models` table.
    VolumetricContainerModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::VolumetricContainerModelAttribute,
        >,
    ),
    ///Insert error associated with the `weighing_device_models` table.
    WeighingDeviceModel(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::WeighingDeviceModelAttribute,
        >,
    ),
}
impl std::fmt::Display for AssetModelInsertErrorDAG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AssetModel(e) => write!(f, "{e}"),
            Self::BallMillMachineModel(e) => write!(f, "{e}"),
            Self::BeadModel(e) => write!(f, "{e}"),
            Self::CameraModel(e) => write!(f, "{e}"),
            Self::CapModel(e) => write!(f, "{e}"),
            Self::CentrifugeModel(e) => write!(f, "{e}"),
            Self::CommercialBallMillMachineLot(e) => write!(f, "{e}"),
            Self::CommercialBallMillMachineModel(e) => write!(f, "{e}"),
            Self::CommercialBeadLot(e) => write!(f, "{e}"),
            Self::CommercialBeadModel(e) => write!(f, "{e}"),
            Self::CommercialCameraLot(e) => write!(f, "{e}"),
            Self::CommercialCameraModel(e) => write!(f, "{e}"),
            Self::CommercialCapLot(e) => write!(f, "{e}"),
            Self::CommercialCapModel(e) => write!(f, "{e}"),
            Self::CommercialCentrifugeLot(e) => write!(f, "{e}"),
            Self::CommercialCentrifugeModel(e) => write!(f, "{e}"),
            Self::CommercialFreezeDryerLot(e) => write!(f, "{e}"),
            Self::CommercialFreezeDryerModel(e) => write!(f, "{e}"),
            Self::CommercialFreezerLot(e) => write!(f, "{e}"),
            Self::CommercialFreezerModel(e) => write!(f, "{e}"),
            Self::CommercialPackagingLot(e) => write!(f, "{e}"),
            Self::CommercialPackagingModel(e) => write!(f, "{e}"),
            Self::CommercialPipetteLot(e) => write!(f, "{e}"),
            Self::CommercialPipetteModel(e) => write!(f, "{e}"),
            Self::CommercialPipetteTipLot(e) => write!(f, "{e}"),
            Self::CommercialPipetteTipModel(e) => write!(f, "{e}"),
            Self::CommercialPositioningDeviceLot(e) => write!(f, "{e}"),
            Self::CommercialPositioningDeviceModel(e) => write!(f, "{e}"),
            Self::CommercialProductLot(e) => write!(f, "{e}"),
            Self::CommercialProduct(e) => write!(f, "{e}"),
            Self::CommercialVolumeMeasuringDeviceLot(e) => write!(f, "{e}"),
            Self::CommercialVolumeMeasuringDeviceModel(e) => write!(f, "{e}"),
            Self::CommercialWeighingDeviceLot(e) => write!(f, "{e}"),
            Self::CommercialWeighingDeviceModel(e) => write!(f, "{e}"),
            Self::ContainerModel(e) => write!(f, "{e}"),
            Self::DigitalAssetModel(e) => write!(f, "{e}"),
            Self::FreezeDryerModel(e) => write!(f, "{e}"),
            Self::FreezerModel(e) => write!(f, "{e}"),
            Self::OrganismModel(e) => write!(f, "{e}"),
            Self::PackagingModel(e) => write!(f, "{e}"),
            Self::PhoneModel(e) => write!(f, "{e}"),
            Self::PhysicalAssetModel(e) => write!(f, "{e}"),
            Self::PipetteModel(e) => write!(f, "{e}"),
            Self::PipetteTipModel(e) => write!(f, "{e}"),
            Self::PositioningDeviceModel(e) => write!(f, "{e}"),
            Self::ReagentModel(e) => write!(f, "{e}"),
            Self::SampleModel(e) => write!(f, "{e}"),
            Self::SampleSourceModel(e) => write!(f, "{e}"),
            Self::SoilModel(e) => write!(f, "{e}"),
            Self::VolumeMeasuringDeviceModel(e) => write!(f, "{e}"),
            Self::VolumetricContainerModel(e) => write!(f, "{e}"),
            Self::WeighingDeviceModel(e) => write!(f, "{e}"),
        }
    }
}
impl std::error::Error for AssetModelInsertErrorDAG {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::AssetModel(e) => Some(e),
            Self::BallMillMachineModel(e) => Some(e),
            Self::BeadModel(e) => Some(e),
            Self::CameraModel(e) => Some(e),
            Self::CapModel(e) => Some(e),
            Self::CentrifugeModel(e) => Some(e),
            Self::CommercialBallMillMachineLot(e) => Some(e),
            Self::CommercialBallMillMachineModel(e) => Some(e),
            Self::CommercialBeadLot(e) => Some(e),
            Self::CommercialBeadModel(e) => Some(e),
            Self::CommercialCameraLot(e) => Some(e),
            Self::CommercialCameraModel(e) => Some(e),
            Self::CommercialCapLot(e) => Some(e),
            Self::CommercialCapModel(e) => Some(e),
            Self::CommercialCentrifugeLot(e) => Some(e),
            Self::CommercialCentrifugeModel(e) => Some(e),
            Self::CommercialFreezeDryerLot(e) => Some(e),
            Self::CommercialFreezeDryerModel(e) => Some(e),
            Self::CommercialFreezerLot(e) => Some(e),
            Self::CommercialFreezerModel(e) => Some(e),
            Self::CommercialPackagingLot(e) => Some(e),
            Self::CommercialPackagingModel(e) => Some(e),
            Self::CommercialPipetteLot(e) => Some(e),
            Self::CommercialPipetteModel(e) => Some(e),
            Self::CommercialPipetteTipLot(e) => Some(e),
            Self::CommercialPipetteTipModel(e) => Some(e),
            Self::CommercialPositioningDeviceLot(e) => Some(e),
            Self::CommercialPositioningDeviceModel(e) => Some(e),
            Self::CommercialProductLot(e) => Some(e),
            Self::CommercialProduct(e) => Some(e),
            Self::CommercialVolumeMeasuringDeviceLot(e) => Some(e),
            Self::CommercialVolumeMeasuringDeviceModel(e) => Some(e),
            Self::CommercialWeighingDeviceLot(e) => Some(e),
            Self::CommercialWeighingDeviceModel(e) => Some(e),
            Self::ContainerModel(e) => Some(e),
            Self::DigitalAssetModel(e) => Some(e),
            Self::FreezeDryerModel(e) => Some(e),
            Self::FreezerModel(e) => Some(e),
            Self::OrganismModel(e) => Some(e),
            Self::PackagingModel(e) => Some(e),
            Self::PhoneModel(e) => Some(e),
            Self::PhysicalAssetModel(e) => Some(e),
            Self::PipetteModel(e) => Some(e),
            Self::PipetteTipModel(e) => Some(e),
            Self::PositioningDeviceModel(e) => Some(e),
            Self::ReagentModel(e) => Some(e),
            Self::SampleModel(e) => Some(e),
            Self::SampleSourceModel(e) => Some(e),
            Self::SoilModel(e) => Some(e),
            Self::VolumeMeasuringDeviceModel(e) => Some(e),
            Self::VolumetricContainerModel(e) => Some(e),
            Self::WeighingDeviceModel(e) => Some(e),
        }
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::AssetModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::BallMillMachineModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::BallMillMachineModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::BallMillMachineModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::BeadModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::BeadModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::BeadModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CameraModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CameraModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CameraModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CapModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CapModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CapModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CentrifugeModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CentrifugeModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CentrifugeModel(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialBallMillMachineLotAttribute,
    >,
> for AssetModelInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialBallMillMachineLotAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialBallMillMachineLot(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialBallMillMachineModelAttribute,
    >,
> for AssetModelInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialBallMillMachineModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialBallMillMachineModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialBeadLotAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialBeadLotAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialBeadLot(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialBeadModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialBeadModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialBeadModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialCameraLotAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialCameraLotAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialCameraLot(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialCameraModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialCameraModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialCameraModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialCapLotAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialCapLotAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialCapLot(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialCapModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialCapModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialCapModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeLotAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeLotAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialCentrifugeLot(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeModelAttribute,
    >,
> for AssetModelInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialCentrifugeModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerLotAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerLotAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialFreezeDryerLot(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerModelAttribute,
    >,
> for AssetModelInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialFreezeDryerModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialFreezerLotAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialFreezerLotAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialFreezerLot(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialFreezerModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialFreezerModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialFreezerModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPackagingLotAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPackagingLotAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialPackagingLot(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPackagingModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPackagingModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialPackagingModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPipetteLotAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPipetteLotAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialPipetteLot(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPipetteModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPipetteModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialPipetteModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPipetteTipLotAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPipetteTipLotAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialPipetteTipLot(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialPipetteTipModelAttribute,
    >,
> for AssetModelInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPipetteTipModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialPipetteTipModel(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialPositioningDeviceLotAttribute,
    >,
> for AssetModelInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPositioningDeviceLotAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialPositioningDeviceLot(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialPositioningDeviceModelAttribute,
    >,
> for AssetModelInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPositioningDeviceModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialPositioningDeviceModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialProductLot(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialProduct(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceLotAttribute,
    >,
> for AssetModelInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceLotAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialVolumeMeasuringDeviceLot(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceModelAttribute,
    >,
> for AssetModelInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialVolumeMeasuringDeviceModel(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceLotAttribute,
    >,
> for AssetModelInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceLotAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialWeighingDeviceLot(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceModelAttribute,
    >,
> for AssetModelInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::CommercialWeighingDeviceModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::ContainerModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::ContainerModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::ContainerModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::DigitalAssetModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::DigitalAssetModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::DigitalAssetModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezeDryerModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezeDryerModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::FreezeDryerModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezerModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezerModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::FreezerModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::OrganismModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::OrganismModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::OrganismModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PackagingModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PackagingModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::PackagingModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::PhoneModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::PhysicalAssetModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PipetteModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PipetteModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::PipetteModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PipetteTipModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PipetteTipModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::PipetteTipModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::PositioningDeviceModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::ReagentModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::ReagentModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::ReagentModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SampleModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SampleModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::SampleModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SampleSourceModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SampleSourceModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::SampleSourceModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SoilModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SoilModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::SoilModel(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceModelAttribute,
    >,
> for AssetModelInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::VolumeMeasuringDeviceModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::VolumetricContainerModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::VolumetricContainerModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::VolumetricContainerModel(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::WeighingDeviceModelAttribute,
        >,
    > for AssetModelInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::WeighingDeviceModelAttribute,
        >,
    ) -> Self {
        AssetModelInsertErrorDAG::WeighingDeviceModel(error)
    }
}
