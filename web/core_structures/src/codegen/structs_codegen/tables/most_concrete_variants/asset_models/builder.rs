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
    ///Builder for the `organism_models` table.
    OrganismModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModelBuilder,
    ),
    ///Builder for the `packaging_models` table.
    PackagingModel(
        crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelBuilder,
    ),
    ///Builder for the `personal_protective_equipment_models` table.
    PersonalProtectiveEquipmentModel(
        crate::codegen::structs_codegen::tables::insertables::InsertablePersonalProtectiveEquipmentModelBuilder,
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
    ///Builder for the `soil_models` table.
    SoilModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableSoilModelBuilder,
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
impl AssetModelBuilderDAG {
    /// Returns the type name of the variant contained within the enum.
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::AssetModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
                >()
            }
            Self::BallMillMachineModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelBuilder,
                >()
            }
            Self::BeadModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableBeadModelBuilder,
                >()
            }
            Self::CameraModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder,
                >()
            }
            Self::CapModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCapModelBuilder,
                >()
            }
            Self::CentrifugeModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeModelBuilder,
                >()
            }
            Self::CommercialBallMillMachineLot(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineLotBuilder,
                >()
            }
            Self::CommercialBallMillMachineModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelBuilder,
                >()
            }
            Self::CommercialBeadLot(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadLotBuilder,
                >()
            }
            Self::CommercialBeadModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadModelBuilder,
                >()
            }
            Self::CommercialCameraLot(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotBuilder,
                >()
            }
            Self::CommercialCameraModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraModelBuilder,
                >()
            }
            Self::CommercialCapLot(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapLotBuilder,
                >()
            }
            Self::CommercialCapModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapModelBuilder,
                >()
            }
            Self::CommercialCentrifugeLot(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeLotBuilder,
                >()
            }
            Self::CommercialCentrifugeModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelBuilder,
                >()
            }
            Self::CommercialFreezeDryerLot(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLotBuilder,
                >()
            }
            Self::CommercialFreezeDryerModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerModelBuilder,
                >()
            }
            Self::CommercialFreezerLot(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerLotBuilder,
                >()
            }
            Self::CommercialFreezerModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelBuilder,
                >()
            }
            Self::CommercialPackagingLot(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLotBuilder,
                >()
            }
            Self::CommercialPackagingModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelBuilder,
                >()
            }
            Self::CommercialPipetteLot(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLotBuilder,
                >()
            }
            Self::CommercialPipetteModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteModelBuilder,
                >()
            }
            Self::CommercialPipetteTipLot(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLotBuilder,
                >()
            }
            Self::CommercialPipetteTipModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelBuilder,
                >()
            }
            Self::CommercialPositioningDeviceLot(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotBuilder,
                >()
            }
            Self::CommercialPositioningDeviceModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceModelBuilder,
                >()
            }
            Self::CommercialProductLot(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder,
                >()
            }
            Self::CommercialProduct(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder,
                >()
            }
            Self::CommercialVolumeMeasuringDeviceLot(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceLotBuilder,
                >()
            }
            Self::CommercialVolumeMeasuringDeviceModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceModelBuilder,
                >()
            }
            Self::CommercialWeighingDeviceLot(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotBuilder,
                >()
            }
            Self::CommercialWeighingDeviceModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceModelBuilder,
                >()
            }
            Self::ContainerModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelBuilder,
                >()
            }
            Self::DigitalAssetModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModelBuilder,
                >()
            }
            Self::FreezeDryerModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelBuilder,
                >()
            }
            Self::FreezerModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezerModelBuilder,
                >()
            }
            Self::OrganismModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModelBuilder,
                >()
            }
            Self::PackagingModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelBuilder,
                >()
            }
            Self::PersonalProtectiveEquipmentModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertablePersonalProtectiveEquipmentModelBuilder,
                >()
            }
            Self::PhoneModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder,
                >()
            }
            Self::PhysicalAssetModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder,
                >()
            }
            Self::PipetteModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertablePipetteModelBuilder,
                >()
            }
            Self::PipetteTipModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelBuilder,
                >()
            }
            Self::PositioningDeviceModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder,
                >()
            }
            Self::ReagentModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelBuilder,
                >()
            }
            Self::SampleModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableSampleModelBuilder,
                >()
            }
            Self::SampleSourceModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceModelBuilder,
                >()
            }
            Self::SoilModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableSoilModelBuilder,
                >()
            }
            Self::VolumeMeasuringDeviceModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelBuilder,
                >()
            }
            Self::VolumetricContainerModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelBuilder,
                >()
            }
            Self::WeighingDeviceModel(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceModelBuilder,
                >()
            }
        }
    }
}
impl common_traits::builder::IsCompleteBuilder for AssetModelBuilderDAG {
    fn is_complete(&self) -> bool {
        match self {
            Self::AssetModel(builder) => builder.is_complete(),
            Self::BallMillMachineModel(builder) => builder.is_complete(),
            Self::BeadModel(builder) => builder.is_complete(),
            Self::CameraModel(builder) => builder.is_complete(),
            Self::CapModel(builder) => builder.is_complete(),
            Self::CentrifugeModel(builder) => builder.is_complete(),
            Self::CommercialBallMillMachineLot(builder) => builder.is_complete(),
            Self::CommercialBallMillMachineModel(builder) => builder.is_complete(),
            Self::CommercialBeadLot(builder) => builder.is_complete(),
            Self::CommercialBeadModel(builder) => builder.is_complete(),
            Self::CommercialCameraLot(builder) => builder.is_complete(),
            Self::CommercialCameraModel(builder) => builder.is_complete(),
            Self::CommercialCapLot(builder) => builder.is_complete(),
            Self::CommercialCapModel(builder) => builder.is_complete(),
            Self::CommercialCentrifugeLot(builder) => builder.is_complete(),
            Self::CommercialCentrifugeModel(builder) => builder.is_complete(),
            Self::CommercialFreezeDryerLot(builder) => builder.is_complete(),
            Self::CommercialFreezeDryerModel(builder) => builder.is_complete(),
            Self::CommercialFreezerLot(builder) => builder.is_complete(),
            Self::CommercialFreezerModel(builder) => builder.is_complete(),
            Self::CommercialPackagingLot(builder) => builder.is_complete(),
            Self::CommercialPackagingModel(builder) => builder.is_complete(),
            Self::CommercialPipetteLot(builder) => builder.is_complete(),
            Self::CommercialPipetteModel(builder) => builder.is_complete(),
            Self::CommercialPipetteTipLot(builder) => builder.is_complete(),
            Self::CommercialPipetteTipModel(builder) => builder.is_complete(),
            Self::CommercialPositioningDeviceLot(builder) => builder.is_complete(),
            Self::CommercialPositioningDeviceModel(builder) => builder.is_complete(),
            Self::CommercialProductLot(builder) => builder.is_complete(),
            Self::CommercialProduct(builder) => builder.is_complete(),
            Self::CommercialVolumeMeasuringDeviceLot(builder) => builder.is_complete(),
            Self::CommercialVolumeMeasuringDeviceModel(builder) => builder.is_complete(),
            Self::CommercialWeighingDeviceLot(builder) => builder.is_complete(),
            Self::CommercialWeighingDeviceModel(builder) => builder.is_complete(),
            Self::ContainerModel(builder) => builder.is_complete(),
            Self::DigitalAssetModel(builder) => builder.is_complete(),
            Self::FreezeDryerModel(builder) => builder.is_complete(),
            Self::FreezerModel(builder) => builder.is_complete(),
            Self::OrganismModel(builder) => builder.is_complete(),
            Self::PackagingModel(builder) => builder.is_complete(),
            Self::PersonalProtectiveEquipmentModel(builder) => builder.is_complete(),
            Self::PhoneModel(builder) => builder.is_complete(),
            Self::PhysicalAssetModel(builder) => builder.is_complete(),
            Self::PipetteModel(builder) => builder.is_complete(),
            Self::PipetteTipModel(builder) => builder.is_complete(),
            Self::PositioningDeviceModel(builder) => builder.is_complete(),
            Self::ReagentModel(builder) => builder.is_complete(),
            Self::SampleModel(builder) => builder.is_complete(),
            Self::SampleSourceModel(builder) => builder.is_complete(),
            Self::SoilModel(builder) => builder.is_complete(),
            Self::VolumeMeasuringDeviceModel(builder) => builder.is_complete(),
            Self::VolumetricContainerModel(builder) => builder.is_complete(),
            Self::WeighingDeviceModel(builder) => builder.is_complete(),
        }
    }
}
impl web_common_traits::database::DispatchableInsertVariantMetadata for AssetModelBuilderDAG {
    type Row = crate::codegen::structs_codegen::tables::most_concrete_variants::AssetModelDAG;
    type Error =
        crate::codegen::structs_codegen::tables::most_concrete_variants::AssetModelInsertErrorDAG;
}
impl<C> web_common_traits::database::DispatchableInsertableVariant<C>
for AssetModelBuilderDAG
where
    crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableBeadModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCapModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineLotBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadLotBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapLotBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeLotBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLotBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerLotBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLotBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLotBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLotBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceLotBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezerModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertablePersonalProtectiveEquipmentModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertablePipetteModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableSampleModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableSoilModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        Ok(
            match self {
                Self::AssetModel(variant) => variant.insert(user_id, conn)?.into(),
                Self::BallMillMachineModel(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::BeadModel(variant) => variant.insert(user_id, conn)?.into(),
                Self::CameraModel(variant) => variant.insert(user_id, conn)?.into(),
                Self::CapModel(variant) => variant.insert(user_id, conn)?.into(),
                Self::CentrifugeModel(variant) => variant.insert(user_id, conn)?.into(),
                Self::CommercialBallMillMachineLot(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialBallMillMachineModel(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialBeadLot(variant) => variant.insert(user_id, conn)?.into(),
                Self::CommercialBeadModel(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialCameraLot(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialCameraModel(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialCapLot(variant) => variant.insert(user_id, conn)?.into(),
                Self::CommercialCapModel(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialCentrifugeLot(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialCentrifugeModel(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialFreezeDryerLot(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialFreezeDryerModel(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialFreezerLot(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialFreezerModel(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialPackagingLot(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialPackagingModel(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialPipetteLot(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialPipetteModel(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialPipetteTipLot(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialPipetteTipModel(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialPositioningDeviceLot(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialPositioningDeviceModel(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialProductLot(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialProduct(variant) => variant.insert(user_id, conn)?.into(),
                Self::CommercialVolumeMeasuringDeviceLot(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialVolumeMeasuringDeviceModel(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialWeighingDeviceLot(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CommercialWeighingDeviceModel(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::ContainerModel(variant) => variant.insert(user_id, conn)?.into(),
                Self::DigitalAssetModel(variant) => variant.insert(user_id, conn)?.into(),
                Self::FreezeDryerModel(variant) => variant.insert(user_id, conn)?.into(),
                Self::FreezerModel(variant) => variant.insert(user_id, conn)?.into(),
                Self::OrganismModel(variant) => variant.insert(user_id, conn)?.into(),
                Self::PackagingModel(variant) => variant.insert(user_id, conn)?.into(),
                Self::PersonalProtectiveEquipmentModel(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::PhoneModel(variant) => variant.insert(user_id, conn)?.into(),
                Self::PhysicalAssetModel(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::PipetteModel(variant) => variant.insert(user_id, conn)?.into(),
                Self::PipetteTipModel(variant) => variant.insert(user_id, conn)?.into(),
                Self::PositioningDeviceModel(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::ReagentModel(variant) => variant.insert(user_id, conn)?.into(),
                Self::SampleModel(variant) => variant.insert(user_id, conn)?.into(),
                Self::SampleSourceModel(variant) => variant.insert(user_id, conn)?.into(),
                Self::SoilModel(variant) => variant.insert(user_id, conn)?.into(),
                Self::VolumeMeasuringDeviceModel(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::VolumetricContainerModel(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::WeighingDeviceModel(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
            },
        )
    }
}
impl crate::codegen::structs_codegen::tables::insertables::AssetModelSettable
for AssetModelBuilderDAG
where
    crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableBeadModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCapModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineLotBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadLotBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapLotBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeLotBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLotBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerLotBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLotBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLotBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLotBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceLotBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezerModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertablePersonalProtectiveEquipmentModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertablePipetteModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableSampleModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableSoilModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceModelBuilder: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable,
{
    type Error = crate::codegen::structs_codegen::tables::most_concrete_variants::AssetModelInsertErrorDAG;
    #[inline]
    ///Sets the value of the `public.asset_models.name` column.
    fn name<N>(self, name: N) -> Result<Self, Self::Error>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        Ok(
            match self {
                Self::AssetModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::BallMillMachineModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::BeadModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBeadModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CameraModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CapModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCapModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CentrifugeModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialBallMillMachineLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialBallMillMachineModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialBeadLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialBeadModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialCameraLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialCameraModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialCapLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialCapModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialCentrifugeLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialCentrifugeModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialFreezeDryerLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialFreezeDryerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialFreezerLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialFreezerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialPackagingLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialPackagingModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialPipetteLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialPipetteModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialPipetteTipLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialPipetteTipModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialPositioningDeviceLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialPositioningDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialProductLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialProduct(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialVolumeMeasuringDeviceLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialVolumeMeasuringDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialWeighingDeviceLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CommercialWeighingDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::ContainerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::DigitalAssetModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::FreezeDryerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::FreezerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::OrganismModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::PackagingModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::PersonalProtectiveEquipmentModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePersonalProtectiveEquipmentModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::PhoneModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::PhysicalAssetModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::PipetteModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePipetteModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::PipetteTipModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::PositioningDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::ReagentModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::SampleModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::SampleSourceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::SoilModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSoilModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::VolumeMeasuringDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::VolumetricContainerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::WeighingDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    ///Sets the value of the `public.asset_models.description` column.
    fn description<D>(self, description: D) -> Result<Self, Self::Error>
    where
        D: TryInto<String>,
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>,
    {
        Ok(
            match self {
                Self::AssetModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::BallMillMachineModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::BeadModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBeadModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CameraModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CapModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCapModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CentrifugeModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialBallMillMachineLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialBallMillMachineModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialBeadLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialBeadModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialCameraLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialCameraModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialCapLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialCapModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialCentrifugeLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialCentrifugeModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialFreezeDryerLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialFreezeDryerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialFreezerLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialFreezerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialPackagingLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialPackagingModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialPipetteLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialPipetteModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialPipetteTipLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialPipetteTipModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialPositioningDeviceLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialPositioningDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialProductLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialProduct(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialVolumeMeasuringDeviceLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialVolumeMeasuringDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialWeighingDeviceLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CommercialWeighingDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::ContainerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::DigitalAssetModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::FreezeDryerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::FreezerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::OrganismModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::PackagingModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::PersonalProtectiveEquipmentModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePersonalProtectiveEquipmentModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::PhoneModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::PhysicalAssetModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::PipetteModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePipetteModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::PipetteTipModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::PositioningDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::ReagentModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::SampleModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::SampleSourceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::SoilModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSoilModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::VolumeMeasuringDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::VolumetricContainerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::WeighingDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    ///Sets the value of the `public.asset_models.parent_model` column.
    fn parent_model<PM>(self, parent_model: PM) -> Result<Self, Self::Error>
    where
        PM: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
        Ok(
            match self {
                Self::AssetModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::BallMillMachineModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::BeadModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBeadModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CameraModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CapModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCapModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CentrifugeModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialBallMillMachineLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialBallMillMachineModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialBeadLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialBeadModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialCameraLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialCameraModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialCapLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialCapModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialCentrifugeLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialCentrifugeModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialFreezeDryerLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialFreezeDryerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialFreezerLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialFreezerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialPackagingLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialPackagingModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialPipetteLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialPipetteModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialPipetteTipLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialPipetteTipModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialPositioningDeviceLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialPositioningDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialProductLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialProduct(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialVolumeMeasuringDeviceLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialVolumeMeasuringDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialWeighingDeviceLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::CommercialWeighingDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::ContainerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::DigitalAssetModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::FreezeDryerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::FreezerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::OrganismModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::PackagingModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::PersonalProtectiveEquipmentModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePersonalProtectiveEquipmentModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::PhoneModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::PhysicalAssetModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::PipetteModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePipetteModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::PipetteTipModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::PositioningDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::ReagentModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::SampleModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::SampleSourceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::SoilModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSoilModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::VolumeMeasuringDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::VolumetricContainerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
                Self::WeighingDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                            builder,
                            parent_model,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    ///Sets the value of the `public.asset_models.created_by` column.
    fn created_by<CB>(self, created_by: CB) -> Result<Self, Self::Error>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        Ok(
            match self {
                Self::AssetModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::BallMillMachineModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::BeadModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBeadModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CameraModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CapModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCapModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CentrifugeModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialBallMillMachineLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialBallMillMachineModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialBeadLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialBeadModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialCameraLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialCameraModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialCapLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialCapModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialCentrifugeLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialCentrifugeModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialFreezeDryerLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialFreezeDryerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialFreezerLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialFreezerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialPackagingLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialPackagingModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialPipetteLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialPipetteModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialPipetteTipLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialPipetteTipModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialPositioningDeviceLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialPositioningDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialProductLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialProduct(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialVolumeMeasuringDeviceLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialVolumeMeasuringDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialWeighingDeviceLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CommercialWeighingDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::ContainerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::DigitalAssetModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::FreezeDryerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::FreezerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::OrganismModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::PackagingModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::PersonalProtectiveEquipmentModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePersonalProtectiveEquipmentModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::PhoneModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::PhysicalAssetModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::PipetteModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePipetteModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::PipetteTipModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::PositioningDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::ReagentModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::SampleModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::SampleSourceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::SoilModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSoilModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::VolumeMeasuringDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::VolumetricContainerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::WeighingDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    ///Sets the value of the `public.asset_models.created_at` column.
    fn created_at<CA>(self, created_at: CA) -> Result<Self, Self::Error>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        Ok(
            match self {
                Self::AssetModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::BallMillMachineModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::BeadModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBeadModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CameraModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CapModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCapModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CentrifugeModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialBallMillMachineLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialBallMillMachineModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialBeadLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialBeadModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialCameraLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialCameraModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialCapLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialCapModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialCentrifugeLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialCentrifugeModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialFreezeDryerLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialFreezeDryerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialFreezerLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialFreezerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialPackagingLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialPackagingModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialPipetteLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialPipetteModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialPipetteTipLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialPipetteTipModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialPositioningDeviceLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialPositioningDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialProductLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialProduct(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialVolumeMeasuringDeviceLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialVolumeMeasuringDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialWeighingDeviceLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CommercialWeighingDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::ContainerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::DigitalAssetModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::FreezeDryerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::FreezerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::OrganismModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::PackagingModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::PersonalProtectiveEquipmentModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePersonalProtectiveEquipmentModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::PhoneModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::PhysicalAssetModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::PipetteModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePipetteModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::PipetteTipModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::PositioningDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::ReagentModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::SampleModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::SampleSourceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::SoilModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSoilModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::VolumeMeasuringDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::VolumetricContainerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::WeighingDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    ///Sets the value of the `public.asset_models.updated_by` column.
    fn updated_by<UB>(self, updated_by: UB) -> Result<Self, Self::Error>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        Ok(
            match self {
                Self::AssetModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::BallMillMachineModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::BeadModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBeadModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CameraModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CapModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCapModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CentrifugeModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialBallMillMachineLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialBallMillMachineModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialBeadLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialBeadModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialCameraLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialCameraModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialCapLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialCapModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialCentrifugeLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialCentrifugeModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialFreezeDryerLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialFreezeDryerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialFreezerLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialFreezerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialPackagingLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialPackagingModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialPipetteLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialPipetteModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialPipetteTipLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialPipetteTipModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialPositioningDeviceLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialPositioningDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialProductLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialProduct(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialVolumeMeasuringDeviceLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialVolumeMeasuringDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialWeighingDeviceLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CommercialWeighingDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::ContainerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::DigitalAssetModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::FreezeDryerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::FreezerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::OrganismModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::PackagingModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::PersonalProtectiveEquipmentModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePersonalProtectiveEquipmentModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::PhoneModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::PhysicalAssetModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::PipetteModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePipetteModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::PipetteTipModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::PositioningDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::ReagentModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::SampleModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::SampleSourceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::SoilModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSoilModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::VolumeMeasuringDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::VolumetricContainerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::WeighingDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    ///Sets the value of the `public.asset_models.updated_at` column.
    fn updated_at<UA>(self, updated_at: UA) -> Result<Self, Self::Error>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        Ok(
            match self {
                Self::AssetModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::BallMillMachineModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::BeadModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBeadModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CameraModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CapModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCapModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CentrifugeModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialBallMillMachineLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialBallMillMachineModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialBeadLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialBeadModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialCameraLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialCameraModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialCapLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialCapModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialCentrifugeLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialCentrifugeModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialFreezeDryerLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialFreezeDryerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialFreezerLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialFreezerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialPackagingLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialPackagingModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialPipetteLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialPipetteModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialPipetteTipLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialPipetteTipModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialPositioningDeviceLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialPositioningDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialProductLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialProduct(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialVolumeMeasuringDeviceLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialVolumeMeasuringDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialWeighingDeviceLot(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CommercialWeighingDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::ContainerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::DigitalAssetModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::FreezeDryerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::FreezerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::OrganismModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::PackagingModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::PersonalProtectiveEquipmentModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePersonalProtectiveEquipmentModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::PhoneModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::PhysicalAssetModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::PipetteModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePipetteModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::PipetteTipModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::PositioningDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::ReagentModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::SampleModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::SampleSourceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::SoilModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSoilModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::VolumeMeasuringDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::VolumetricContainerModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::WeighingDeviceModel(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceModelBuilder as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
            },
        )
    }
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
impl From<AssetModelBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder>
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::AssetModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelBuilder,
    >
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::BallMillMachineModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertableBeadModelBuilder>
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::BeadModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder>
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CameraModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertableCapModelBuilder>
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CapModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeModelBuilder,
    >
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CentrifugeModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineLotBuilder,
> {
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialBallMillMachineLot(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelBuilder,
> {
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialBallMillMachineModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadLotBuilder,
    >
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialBeadLot(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadModelBuilder,
    >
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialBeadModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotBuilder,
    >
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialCameraLot(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraModelBuilder,
> {
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialCameraModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapLotBuilder,
    >
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialCapLot(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapModelBuilder,
    >
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialCapModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeLotBuilder,
> {
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialCentrifugeLot(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelBuilder,
> {
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialCentrifugeModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLotBuilder,
> {
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialFreezeDryerLot(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerModelBuilder,
> {
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialFreezeDryerModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerLotBuilder,
    >
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialFreezerLot(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelBuilder,
> {
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialFreezerModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLotBuilder,
> {
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialPackagingLot(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelBuilder,
> {
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialPackagingModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLotBuilder,
    >
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialPipetteLot(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteModelBuilder,
> {
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialPipetteModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLotBuilder,
> {
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialPipetteTipLot(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelBuilder,
> {
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialPipetteTipModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotBuilder,
> {
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialPositioningDeviceLot(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceModelBuilder,
> {
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialPositioningDeviceModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder,
    >
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialProductLot(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder,
    >
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialProduct(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceLotBuilder,
> {
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialVolumeMeasuringDeviceLot(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceModelBuilder,
> {
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialVolumeMeasuringDeviceModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotBuilder,
> {
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialWeighingDeviceLot(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceModelBuilder,
> {
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::CommercialWeighingDeviceModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelBuilder,
    >
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::ContainerModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModelBuilder,
    >
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::DigitalAssetModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelBuilder,
    >
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::FreezeDryerModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertableFreezerModelBuilder>
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::FreezerModel(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModelBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::OrganismModel(value)
    }
}
impl From<AssetModelBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModelBuilder>
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::OrganismModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelBuilder,
    >
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::PackagingModel(v) => Some(v),
            _ => None,
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertablePersonalProtectiveEquipmentModelBuilder,
> for AssetModelBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertablePersonalProtectiveEquipmentModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::PersonalProtectiveEquipmentModel(value)
    }
}
impl From<AssetModelBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertablePersonalProtectiveEquipmentModelBuilder,
> {
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::PersonalProtectiveEquipmentModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder>
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::PhoneModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder,
    >
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::PhysicalAssetModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertablePipetteModelBuilder>
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::PipetteModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelBuilder,
    >
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::PipetteTipModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder,
> {
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::PositioningDeviceModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelBuilder>
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::ReagentModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertableSampleModelBuilder>
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::SampleModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceModelBuilder,
    >
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::SampleSourceModel(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableSoilModelBuilder>
    for AssetModelBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableSoilModelBuilder,
    ) -> Self {
        AssetModelBuilderDAG::SoilModel(value)
    }
}
impl From<AssetModelBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertableSoilModelBuilder>
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::SoilModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelBuilder,
> {
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::VolumeMeasuringDeviceModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelBuilder,
> {
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::VolumetricContainerModel(v) => Some(v),
            _ => None,
        }
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
impl From<AssetModelBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceModelBuilder,
    >
{
    fn from(value: AssetModelBuilderDAG) -> Self {
        match value {
            AssetModelBuilderDAG::WeighingDeviceModel(v) => Some(v),
            _ => None,
        }
    }
}
