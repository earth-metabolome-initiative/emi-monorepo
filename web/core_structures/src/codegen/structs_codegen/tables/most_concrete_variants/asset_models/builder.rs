#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enum representing the `asset_models` table builder DAG.
pub enum AssetModelBuilderDAG {
    ///Builder for the `asset_models` table.
    AssetModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
    ),
    ///Builder for the `ball_mill_machine_models` table.
    BallMillMachineModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelBuilder,
    ),
    ///Builder for the `bead_models` table.
    BeadModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableBeadModelBuilder,
    ),
    ///Builder for the `camera_models` table.
    CameraModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder,
    ),
    ///Builder for the `cap_models` table.
    CapModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableCapModelBuilder,
    ),
    ///Builder for the `centrifuge_models` table.
    CentrifugeModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeModelBuilder,
    ),
    ///Builder for the `commercial_ball_mill_machine_lots` table.
    CommercialBallMillMachineLot(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineLotBuilder,
    ),
    ///Builder for the `commercial_ball_mill_machine_models` table.
    CommercialBallMillMachineModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelBuilder,
    ),
    ///Builder for the `commercial_bead_lots` table.
    CommercialBeadLot(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadLotBuilder,
    ),
    ///Builder for the `commercial_bead_models` table.
    CommercialBeadModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadModelBuilder,
    ),
    ///Builder for the `commercial_camera_lots` table.
    CommercialCameraLot(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotBuilder,
    ),
    ///Builder for the `commercial_camera_models` table.
    CommercialCameraModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraModelBuilder,
    ),
    ///Builder for the `commercial_cap_lots` table.
    CommercialCapLot(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapLotBuilder,
    ),
    ///Builder for the `commercial_cap_models` table.
    CommercialCapModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapModelBuilder,
    ),
    ///Builder for the `commercial_centrifuge_lots` table.
    CommercialCentrifugeLot(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeLotBuilder,
    ),
    ///Builder for the `commercial_centrifuge_models` table.
    CommercialCentrifugeModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelBuilder,
    ),
    ///Builder for the `commercial_freeze_dryer_lots` table.
    CommercialFreezeDryerLot(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLotBuilder,
    ),
    ///Builder for the `commercial_freeze_dryer_models` table.
    CommercialFreezeDryerModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerModelBuilder,
    ),
    ///Builder for the `commercial_freezer_lots` table.
    CommercialFreezerLot(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerLotBuilder,
    ),
    ///Builder for the `commercial_freezer_models` table.
    CommercialFreezerModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelBuilder,
    ),
    ///Builder for the `commercial_packaging_lots` table.
    CommercialPackagingLot(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLotBuilder,
    ),
    ///Builder for the `commercial_packaging_models` table.
    CommercialPackagingModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelBuilder,
    ),
    ///Builder for the `commercial_pipette_lots` table.
    CommercialPipetteLot(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLotBuilder,
    ),
    ///Builder for the `commercial_pipette_models` table.
    CommercialPipetteModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteModelBuilder,
    ),
    ///Builder for the `commercial_pipette_tip_lots` table.
    CommercialPipetteTipLot(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLotBuilder,
    ),
    ///Builder for the `commercial_pipette_tip_models` table.
    CommercialPipetteTipModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelBuilder,
    ),
    ///Builder for the `commercial_positioning_device_lots` table.
    CommercialPositioningDeviceLot(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotBuilder,
    ),
    ///Builder for the `commercial_positioning_device_models` table.
    CommercialPositioningDeviceModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceModelBuilder,
    ),
    ///Builder for the `commercial_product_lots` table.
    CommercialProductLot(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder,
    ),
    ///Builder for the `commercial_products` table.
    CommercialProduct(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder,
    ),
    ///Builder for the `commercial_volume_measuring_device_lots` table.
    CommercialVolumeMeasuringDeviceLot(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceLotBuilder,
    ),
    ///Builder for the `commercial_volume_measuring_device_models` table.
    CommercialVolumeMeasuringDeviceModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceModelBuilder,
    ),
    ///Builder for the `commercial_weighing_device_lots` table.
    CommercialWeighingDeviceLot(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotBuilder,
    ),
    ///Builder for the `commercial_weighing_device_models` table.
    CommercialWeighingDeviceModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceModelBuilder,
    ),
    ///Builder for the `container_models` table.
    ContainerModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelBuilder,
    ),
    ///Builder for the `digital_asset_models` table.
    DigitalAssetModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModelBuilder,
    ),
    ///Builder for the `freeze_dryer_models` table.
    FreezeDryerModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelBuilder,
    ),
    ///Builder for the `freezer_models` table.
    FreezerModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableFreezerModelBuilder,
    ),
    ///Builder for the `packaging_models` table.
    PackagingModel(
        crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelBuilder,
    ),
    ///Builder for the `phone_models` table.
    PhoneModel(
        crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder,
    ),
    ///Builder for the `physical_asset_models` table.
    PhysicalAssetModel(
        crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder,
    ),
    ///Builder for the `pipette_models` table.
    PipetteModel(
        crate::codegen::structs_codegen::tables::insertables::InsertablePipetteModelBuilder,
    ),
    ///Builder for the `pipette_tip_models` table.
    PipetteTipModel(
        crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelBuilder,
    ),
    ///Builder for the `positioning_device_models` table.
    PositioningDeviceModel(
        crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder,
    ),
    ///Builder for the `reagent_models` table.
    ReagentModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelBuilder,
    ),
    ///Builder for the `sample_models` table.
    SampleModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableSampleModelBuilder,
    ),
    ///Builder for the `sample_source_models` table.
    SampleSourceModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceModelBuilder,
    ),
    ///Builder for the `volume_measuring_device_models` table.
    VolumeMeasuringDeviceModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelBuilder,
    ),
    ///Builder for the `volumetric_container_models` table.
    VolumetricContainerModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelBuilder,
    ),
    ///Builder for the `weighing_device_models` table.
    WeighingDeviceModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceModelBuilder,
    ),
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::AssetModel(value)
    }
}
impl
    From<
        crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelBuilder,
    > for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::BallMillMachineModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableBeadModelBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableBeadModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::BeadModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CameraModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableCapModelBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCapModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CapModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeModelBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CentrifugeModel(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineLotBuilder,
> for AssetModelBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineLotBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialBallMillMachineLot(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelBuilder,
> for AssetModelBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialBallMillMachineModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadLotBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadLotBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialBeadLot(value)
    }
}
impl
    From<crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadModelBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialBeadModel(value)
    }
}
impl
    From<crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialCameraLot(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraModelBuilder,
> for AssetModelBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialCameraModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapLotBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapLotBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialCapLot(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapModelBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialCapModel(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeLotBuilder,
> for AssetModelBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeLotBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialCentrifugeLot(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelBuilder,
> for AssetModelBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialCentrifugeModel(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLotBuilder,
> for AssetModelBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLotBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialFreezeDryerLot(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerModelBuilder,
> for AssetModelBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialFreezeDryerModel(value)
    }
}
impl
    From<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerLotBuilder,
    > for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerLotBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialFreezerLot(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelBuilder,
> for AssetModelBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialFreezerModel(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLotBuilder,
> for AssetModelBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLotBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialPackagingLot(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelBuilder,
> for AssetModelBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialPackagingModel(value)
    }
}
impl
    From<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLotBuilder,
    > for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLotBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialPipetteLot(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteModelBuilder,
> for AssetModelBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialPipetteModel(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLotBuilder,
> for AssetModelBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLotBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialPipetteTipLot(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelBuilder,
> for AssetModelBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialPipetteTipModel(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotBuilder,
> for AssetModelBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialPositioningDeviceLot(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceModelBuilder,
> for AssetModelBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialPositioningDeviceModel(value)
    }
}
impl
    From<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder,
    > for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialProductLot(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialProduct(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceLotBuilder,
> for AssetModelBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceLotBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialVolumeMeasuringDeviceLot(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceModelBuilder,
> for AssetModelBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialVolumeMeasuringDeviceModel(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotBuilder,
> for AssetModelBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialWeighingDeviceLot(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceModelBuilder,
> for AssetModelBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::CommercialWeighingDeviceModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::ContainerModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModelBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::DigitalAssetModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::FreezeDryerModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableFreezerModelBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableFreezerModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::FreezerModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::PackagingModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::PhoneModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::PhysicalAssetModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertablePipetteModelBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertablePipetteModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::PipetteModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::PipetteTipModel(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder,
> for AssetModelBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::PositioningDeviceModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::ReagentModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableSampleModelBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableSampleModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::SampleModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceModelBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::SampleSourceModel(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelBuilder,
> for AssetModelBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::VolumeMeasuringDeviceModel(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelBuilder,
> for AssetModelBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::VolumetricContainerModel(value)
    }
}
impl
    From<crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceModelBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::WeighingDeviceModel(value)
    }
}
