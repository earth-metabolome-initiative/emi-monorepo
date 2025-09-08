#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enum representing the most concrete variant of the `asset_models` table DAG.
///
/// # Mermaid illustration of the DAG:
/// ```mermaid
/// flowchart BT
/// v0@{shape: rect, label: "asset_models"}
/// v1@{shape: rect, label: "ball_mill_machine_models"}
/// v2@{shape: rect, label: "bead_models"}
/// v3@{shape: rect, label: "camera_models"}
/// v4@{shape: rect, label: "cap_models"}
/// v5@{shape: rect, label: "centrifuge_models"}
/// v6@{shape: rect, label: "commercial_ball_mill_machine_lots"}
/// v7@{shape: rect, label: "commercial_ball_mill_machine_models"}
/// v8@{shape: rect, label: "commercial_bead_lots"}
/// v9@{shape: rect, label: "commercial_bead_models"}
/// v10@{shape: rect, label: "commercial_camera_lots"}
/// v11@{shape: rect, label: "commercial_camera_models"}
/// v12@{shape: rect, label: "commercial_cap_lots"}
/// v13@{shape: rect, label: "commercial_cap_models"}
/// v14@{shape: rect, label: "commercial_centrifuge_lots"}
/// v15@{shape: rect, label: "commercial_centrifuge_models"}
/// v16@{shape: rect, label: "commercial_freeze_dryer_lots"}
/// v17@{shape: rect, label: "commercial_freeze_dryer_models"}
/// v18@{shape: rect, label: "commercial_freezer_lots"}
/// v19@{shape: rect, label: "commercial_freezer_models"}
/// v20@{shape: rect, label: "commercial_packaging_lots"}
/// v21@{shape: rect, label: "commercial_packaging_models"}
/// v22@{shape: rect, label: "commercial_pipette_lots"}
/// v23@{shape: rect, label: "commercial_pipette_models"}
/// v24@{shape: rect, label: "commercial_pipette_tip_lots"}
/// v25@{shape: rect, label: "commercial_pipette_tip_models"}
/// v26@{shape: rect, label: "commercial_positioning_device_lots"}
/// v27@{shape: rect, label: "commercial_positioning_device_models"}
/// v28@{shape: rect, label: "commercial_product_lots"}
/// v29@{shape: rect, label: "commercial_products"}
/// v30@{shape: rect, label: "commercial_volume_measuring_device_lots"}
/// v31@{shape: rect, label: "commercial_volume_measuring_device_models"}
/// v32@{shape: rect, label: "commercial_weighing_device_lots"}
/// v33@{shape: rect, label: "commercial_weighing_device_models"}
/// v34@{shape: rect, label: "container_models"}
/// v35@{shape: rect, label: "digital_asset_models"}
/// v36@{shape: rect, label: "freeze_dryer_models"}
/// v37@{shape: rect, label: "freezer_models"}
/// v38@{shape: rect, label: "packaging_models"}
/// v39@{shape: rect, label: "phone_models"}
/// v40@{shape: rect, label: "physical_asset_models"}
/// v41@{shape: rect, label: "pipette_models"}
/// v42@{shape: rect, label: "pipette_tip_models"}
/// v43@{shape: rect, label: "positioning_device_models"}
/// v44@{shape: rect, label: "reagent_models"}
/// v45@{shape: rect, label: "volume_measuring_device_models"}
/// v46@{shape: rect, label: "volumetric_container_models"}
/// v47@{shape: rect, label: "weighing_device_models"}
/// v27 --->|"`extends`"| v43
/// v27 --->|"`extends`"| v29
/// v22 --->|"`extends`"| v28
/// v22 --->|"`extends`"| v41
/// v37 --->|"`extends`"| v40
/// v5 --->|"`extends`"| v40
/// v8 --->|"`extends`"| v28
/// v8 --->|"`extends`"| v2
/// v10 --->|"`extends`"| v28
/// v10 --->|"`extends`"| v3
/// v6 --->|"`extends`"| v28
/// v6 --->|"`extends`"| v1
/// v3 --->|"`extends`"| v40
/// v15 --->|"`extends`"| v5
/// v15 --->|"`extends`"| v29
/// v19 --->|"`extends`"| v37
/// v19 --->|"`extends`"| v29
/// v28 --->|"`extends`"| v40
/// v39 --->|"`extends`"| v3
/// v39 --->|"`extends`"| v43
/// v1 --->|"`extends`"| v40
/// v9 --->|"`extends`"| v2
/// v9 --->|"`extends`"| v29
/// v43 --->|"`extends`"| v40
/// v46 --->|"`extends`"| v34
/// v41 --->|"`extends`"| v40
/// v26 --->|"`extends`"| v28
/// v26 --->|"`extends`"| v43
/// v16 --->|"`extends`"| v28
/// v16 --->|"`extends`"| v36
/// v14 --->|"`extends`"| v28
/// v14 --->|"`extends`"| v5
/// v36 --->|"`extends`"| v40
/// v4 --->|"`extends`"| v40
/// v38 --->|"`extends`"| v40
/// v42 --->|"`extends`"| v40
/// v7 --->|"`extends`"| v1
/// v7 --->|"`extends`"| v29
/// v20 --->|"`extends`"| v28
/// v20 --->|"`extends`"| v38
/// v31 --->|"`extends`"| v45
/// v31 --->|"`extends`"| v29
/// v25 --->|"`extends`"| v42
/// v25 --->|"`extends`"| v29
/// v17 --->|"`extends`"| v36
/// v17 --->|"`extends`"| v29
/// v21 --->|"`extends`"| v38
/// v21 --->|"`extends`"| v29
/// v30 --->|"`extends`"| v28
/// v30 --->|"`extends`"| v45
/// v24 --->|"`extends`"| v28
/// v24 --->|"`extends`"| v42
/// v32 --->|"`extends`"| v28
/// v32 --->|"`extends`"| v47
/// v33 --->|"`extends`"| v47
/// v33 --->|"`extends`"| v29
/// v34 --->|"`extends`"| v40
/// v12 --->|"`extends`"| v28
/// v12 --->|"`extends`"| v4
/// v18 --->|"`extends`"| v28
/// v18 --->|"`extends`"| v37
/// v23 --->|"`extends`"| v41
/// v23 --->|"`extends`"| v29
/// v35 --->|"`extends`"| v0
/// v40 --->|"`extends`"| v0
/// v44 --->|"`extends`"| v0
/// v45 --->|"`extends`"| v40
/// v47 --->|"`extends`"| v40
/// v11 --->|"`extends`"| v3
/// v11 --->|"`extends`"| v29
/// v13 --->|"`extends`"| v4
/// v13 --->|"`extends`"| v29
/// v2 --->|"`extends`"| v40
/// v29 --->|"`extends`"| v0
/// ```
pub enum AssetModelDAG {
    /// Variant representing the `asset_models` table.
    AssetModel(crate::AssetModel),
    /// Variant representing the `ball_mill_machine_models` table.
    BallMillMachineModel(crate::BallMillMachineModel),
    /// Variant representing the `bead_models` table.
    BeadModel(crate::BeadModel),
    /// Variant representing the `camera_models` table.
    CameraModel(crate::CameraModel),
    /// Variant representing the `cap_models` table.
    CapModel(crate::CapModel),
    /// Variant representing the `centrifuge_models` table.
    CentrifugeModel(crate::CentrifugeModel),
    /// Variant representing the `commercial_ball_mill_machine_lots` table.
    CommercialBallMillMachineLot(crate::CommercialBallMillMachineLot),
    /// Variant representing the `commercial_ball_mill_machine_models` table.
    CommercialBallMillMachineModel(crate::CommercialBallMillMachineModel),
    /// Variant representing the `commercial_bead_lots` table.
    CommercialBeadLot(crate::CommercialBeadLot),
    /// Variant representing the `commercial_bead_models` table.
    CommercialBeadModel(crate::CommercialBeadModel),
    /// Variant representing the `commercial_camera_lots` table.
    CommercialCameraLot(crate::CommercialCameraLot),
    /// Variant representing the `commercial_camera_models` table.
    CommercialCameraModel(crate::CommercialCameraModel),
    /// Variant representing the `commercial_cap_lots` table.
    CommercialCapLot(crate::CommercialCapLot),
    /// Variant representing the `commercial_cap_models` table.
    CommercialCapModel(crate::CommercialCapModel),
    /// Variant representing the `commercial_centrifuge_lots` table.
    CommercialCentrifugeLot(crate::CommercialCentrifugeLot),
    /// Variant representing the `commercial_centrifuge_models` table.
    CommercialCentrifugeModel(crate::CommercialCentrifugeModel),
    /// Variant representing the `commercial_freeze_dryer_lots` table.
    CommercialFreezeDryerLot(crate::CommercialFreezeDryerLot),
    /// Variant representing the `commercial_freeze_dryer_models` table.
    CommercialFreezeDryerModel(crate::CommercialFreezeDryerModel),
    /// Variant representing the `commercial_freezer_lots` table.
    CommercialFreezerLot(crate::CommercialFreezerLot),
    /// Variant representing the `commercial_freezer_models` table.
    CommercialFreezerModel(crate::CommercialFreezerModel),
    /// Variant representing the `commercial_packaging_lots` table.
    CommercialPackagingLot(crate::CommercialPackagingLot),
    /// Variant representing the `commercial_packaging_models` table.
    CommercialPackagingModel(crate::CommercialPackagingModel),
    /// Variant representing the `commercial_pipette_lots` table.
    CommercialPipetteLot(crate::CommercialPipetteLot),
    /// Variant representing the `commercial_pipette_models` table.
    CommercialPipetteModel(crate::CommercialPipetteModel),
    /// Variant representing the `commercial_pipette_tip_lots` table.
    CommercialPipetteTipLot(crate::CommercialPipetteTipLot),
    /// Variant representing the `commercial_pipette_tip_models` table.
    CommercialPipetteTipModel(crate::CommercialPipetteTipModel),
    /// Variant representing the `commercial_positioning_device_lots` table.
    CommercialPositioningDeviceLot(crate::CommercialPositioningDeviceLot),
    /// Variant representing the `commercial_positioning_device_models` table.
    CommercialPositioningDeviceModel(crate::CommercialPositioningDeviceModel),
    /// Variant representing the `commercial_product_lots` table.
    CommercialProductLot(crate::CommercialProductLot),
    /// Variant representing the `commercial_products` table.
    CommercialProduct(crate::CommercialProduct),
    /// Variant representing the `commercial_volume_measuring_device_lots`
    /// table.
    CommercialVolumeMeasuringDeviceLot(crate::CommercialVolumeMeasuringDeviceLot),
    /// Variant representing the `commercial_volume_measuring_device_models`
    /// table.
    CommercialVolumeMeasuringDeviceModel(crate::CommercialVolumeMeasuringDeviceModel),
    /// Variant representing the `commercial_weighing_device_lots` table.
    CommercialWeighingDeviceLot(crate::CommercialWeighingDeviceLot),
    /// Variant representing the `commercial_weighing_device_models` table.
    CommercialWeighingDeviceModel(crate::CommercialWeighingDeviceModel),
    /// Variant representing the `container_models` table.
    ContainerModel(crate::ContainerModel),
    /// Variant representing the `digital_asset_models` table.
    DigitalAssetModel(crate::DigitalAssetModel),
    /// Variant representing the `freeze_dryer_models` table.
    FreezeDryerModel(crate::FreezeDryerModel),
    /// Variant representing the `freezer_models` table.
    FreezerModel(crate::FreezerModel),
    /// Variant representing the `packaging_models` table.
    PackagingModel(crate::PackagingModel),
    /// Variant representing the `phone_models` table.
    PhoneModel(crate::PhoneModel),
    /// Variant representing the `physical_asset_models` table.
    PhysicalAssetModel(crate::PhysicalAssetModel),
    /// Variant representing the `pipette_models` table.
    PipetteModel(crate::PipetteModel),
    /// Variant representing the `pipette_tip_models` table.
    PipetteTipModel(crate::PipetteTipModel),
    /// Variant representing the `positioning_device_models` table.
    PositioningDeviceModel(crate::PositioningDeviceModel),
    /// Variant representing the `reagent_models` table.
    ReagentModel(crate::ReagentModel),
    /// Variant representing the `volume_measuring_device_models` table.
    VolumeMeasuringDeviceModel(crate::VolumeMeasuringDeviceModel),
    /// Variant representing the `volumetric_container_models` table.
    VolumetricContainerModel(crate::VolumetricContainerModel),
    /// Variant representing the `weighing_device_models` table.
    WeighingDeviceModel(crate::WeighingDeviceModel),
}
impl From<crate::AssetModel> for AssetModelDAG {
    fn from(value: crate::AssetModel) -> Self {
        AssetModelDAG::AssetModel(value)
    }
}
impl From<crate::BallMillMachineModel> for AssetModelDAG {
    fn from(value: crate::BallMillMachineModel) -> Self {
        AssetModelDAG::BallMillMachineModel(value)
    }
}
impl From<crate::BeadModel> for AssetModelDAG {
    fn from(value: crate::BeadModel) -> Self {
        AssetModelDAG::BeadModel(value)
    }
}
impl From<crate::CameraModel> for AssetModelDAG {
    fn from(value: crate::CameraModel) -> Self {
        AssetModelDAG::CameraModel(value)
    }
}
impl From<crate::CapModel> for AssetModelDAG {
    fn from(value: crate::CapModel) -> Self {
        AssetModelDAG::CapModel(value)
    }
}
impl From<crate::CentrifugeModel> for AssetModelDAG {
    fn from(value: crate::CentrifugeModel) -> Self {
        AssetModelDAG::CentrifugeModel(value)
    }
}
impl From<crate::CommercialBallMillMachineLot> for AssetModelDAG {
    fn from(value: crate::CommercialBallMillMachineLot) -> Self {
        AssetModelDAG::CommercialBallMillMachineLot(value)
    }
}
impl From<crate::CommercialBallMillMachineModel> for AssetModelDAG {
    fn from(value: crate::CommercialBallMillMachineModel) -> Self {
        AssetModelDAG::CommercialBallMillMachineModel(value)
    }
}
impl From<crate::CommercialBeadLot> for AssetModelDAG {
    fn from(value: crate::CommercialBeadLot) -> Self {
        AssetModelDAG::CommercialBeadLot(value)
    }
}
impl From<crate::CommercialBeadModel> for AssetModelDAG {
    fn from(value: crate::CommercialBeadModel) -> Self {
        AssetModelDAG::CommercialBeadModel(value)
    }
}
impl From<crate::CommercialCameraLot> for AssetModelDAG {
    fn from(value: crate::CommercialCameraLot) -> Self {
        AssetModelDAG::CommercialCameraLot(value)
    }
}
impl From<crate::CommercialCameraModel> for AssetModelDAG {
    fn from(value: crate::CommercialCameraModel) -> Self {
        AssetModelDAG::CommercialCameraModel(value)
    }
}
impl From<crate::CommercialCapLot> for AssetModelDAG {
    fn from(value: crate::CommercialCapLot) -> Self {
        AssetModelDAG::CommercialCapLot(value)
    }
}
impl From<crate::CommercialCapModel> for AssetModelDAG {
    fn from(value: crate::CommercialCapModel) -> Self {
        AssetModelDAG::CommercialCapModel(value)
    }
}
impl From<crate::CommercialCentrifugeLot> for AssetModelDAG {
    fn from(value: crate::CommercialCentrifugeLot) -> Self {
        AssetModelDAG::CommercialCentrifugeLot(value)
    }
}
impl From<crate::CommercialCentrifugeModel> for AssetModelDAG {
    fn from(value: crate::CommercialCentrifugeModel) -> Self {
        AssetModelDAG::CommercialCentrifugeModel(value)
    }
}
impl From<crate::CommercialFreezeDryerLot> for AssetModelDAG {
    fn from(value: crate::CommercialFreezeDryerLot) -> Self {
        AssetModelDAG::CommercialFreezeDryerLot(value)
    }
}
impl From<crate::CommercialFreezeDryerModel> for AssetModelDAG {
    fn from(value: crate::CommercialFreezeDryerModel) -> Self {
        AssetModelDAG::CommercialFreezeDryerModel(value)
    }
}
impl From<crate::CommercialFreezerLot> for AssetModelDAG {
    fn from(value: crate::CommercialFreezerLot) -> Self {
        AssetModelDAG::CommercialFreezerLot(value)
    }
}
impl From<crate::CommercialFreezerModel> for AssetModelDAG {
    fn from(value: crate::CommercialFreezerModel) -> Self {
        AssetModelDAG::CommercialFreezerModel(value)
    }
}
impl From<crate::CommercialPackagingLot> for AssetModelDAG {
    fn from(value: crate::CommercialPackagingLot) -> Self {
        AssetModelDAG::CommercialPackagingLot(value)
    }
}
impl From<crate::CommercialPackagingModel> for AssetModelDAG {
    fn from(value: crate::CommercialPackagingModel) -> Self {
        AssetModelDAG::CommercialPackagingModel(value)
    }
}
impl From<crate::CommercialPipetteLot> for AssetModelDAG {
    fn from(value: crate::CommercialPipetteLot) -> Self {
        AssetModelDAG::CommercialPipetteLot(value)
    }
}
impl From<crate::CommercialPipetteModel> for AssetModelDAG {
    fn from(value: crate::CommercialPipetteModel) -> Self {
        AssetModelDAG::CommercialPipetteModel(value)
    }
}
impl From<crate::CommercialPipetteTipLot> for AssetModelDAG {
    fn from(value: crate::CommercialPipetteTipLot) -> Self {
        AssetModelDAG::CommercialPipetteTipLot(value)
    }
}
impl From<crate::CommercialPipetteTipModel> for AssetModelDAG {
    fn from(value: crate::CommercialPipetteTipModel) -> Self {
        AssetModelDAG::CommercialPipetteTipModel(value)
    }
}
impl From<crate::CommercialPositioningDeviceLot> for AssetModelDAG {
    fn from(value: crate::CommercialPositioningDeviceLot) -> Self {
        AssetModelDAG::CommercialPositioningDeviceLot(value)
    }
}
impl From<crate::CommercialPositioningDeviceModel> for AssetModelDAG {
    fn from(value: crate::CommercialPositioningDeviceModel) -> Self {
        AssetModelDAG::CommercialPositioningDeviceModel(value)
    }
}
impl From<crate::CommercialProductLot> for AssetModelDAG {
    fn from(value: crate::CommercialProductLot) -> Self {
        AssetModelDAG::CommercialProductLot(value)
    }
}
impl From<crate::CommercialProduct> for AssetModelDAG {
    fn from(value: crate::CommercialProduct) -> Self {
        AssetModelDAG::CommercialProduct(value)
    }
}
impl From<crate::CommercialVolumeMeasuringDeviceLot> for AssetModelDAG {
    fn from(value: crate::CommercialVolumeMeasuringDeviceLot) -> Self {
        AssetModelDAG::CommercialVolumeMeasuringDeviceLot(value)
    }
}
impl From<crate::CommercialVolumeMeasuringDeviceModel> for AssetModelDAG {
    fn from(value: crate::CommercialVolumeMeasuringDeviceModel) -> Self {
        AssetModelDAG::CommercialVolumeMeasuringDeviceModel(value)
    }
}
impl From<crate::CommercialWeighingDeviceLot> for AssetModelDAG {
    fn from(value: crate::CommercialWeighingDeviceLot) -> Self {
        AssetModelDAG::CommercialWeighingDeviceLot(value)
    }
}
impl From<crate::CommercialWeighingDeviceModel> for AssetModelDAG {
    fn from(value: crate::CommercialWeighingDeviceModel) -> Self {
        AssetModelDAG::CommercialWeighingDeviceModel(value)
    }
}
impl From<crate::ContainerModel> for AssetModelDAG {
    fn from(value: crate::ContainerModel) -> Self {
        AssetModelDAG::ContainerModel(value)
    }
}
impl From<crate::DigitalAssetModel> for AssetModelDAG {
    fn from(value: crate::DigitalAssetModel) -> Self {
        AssetModelDAG::DigitalAssetModel(value)
    }
}
impl From<crate::FreezeDryerModel> for AssetModelDAG {
    fn from(value: crate::FreezeDryerModel) -> Self {
        AssetModelDAG::FreezeDryerModel(value)
    }
}
impl From<crate::FreezerModel> for AssetModelDAG {
    fn from(value: crate::FreezerModel) -> Self {
        AssetModelDAG::FreezerModel(value)
    }
}
impl From<crate::PackagingModel> for AssetModelDAG {
    fn from(value: crate::PackagingModel) -> Self {
        AssetModelDAG::PackagingModel(value)
    }
}
impl From<crate::PhoneModel> for AssetModelDAG {
    fn from(value: crate::PhoneModel) -> Self {
        AssetModelDAG::PhoneModel(value)
    }
}
impl From<crate::PhysicalAssetModel> for AssetModelDAG {
    fn from(value: crate::PhysicalAssetModel) -> Self {
        AssetModelDAG::PhysicalAssetModel(value)
    }
}
impl From<crate::PipetteModel> for AssetModelDAG {
    fn from(value: crate::PipetteModel) -> Self {
        AssetModelDAG::PipetteModel(value)
    }
}
impl From<crate::PipetteTipModel> for AssetModelDAG {
    fn from(value: crate::PipetteTipModel) -> Self {
        AssetModelDAG::PipetteTipModel(value)
    }
}
impl From<crate::PositioningDeviceModel> for AssetModelDAG {
    fn from(value: crate::PositioningDeviceModel) -> Self {
        AssetModelDAG::PositioningDeviceModel(value)
    }
}
impl From<crate::ReagentModel> for AssetModelDAG {
    fn from(value: crate::ReagentModel) -> Self {
        AssetModelDAG::ReagentModel(value)
    }
}
impl From<crate::VolumeMeasuringDeviceModel> for AssetModelDAG {
    fn from(value: crate::VolumeMeasuringDeviceModel) -> Self {
        AssetModelDAG::VolumeMeasuringDeviceModel(value)
    }
}
impl From<crate::VolumetricContainerModel> for AssetModelDAG {
    fn from(value: crate::VolumetricContainerModel) -> Self {
        AssetModelDAG::VolumetricContainerModel(value)
    }
}
impl From<crate::WeighingDeviceModel> for AssetModelDAG {
    fn from(value: crate::WeighingDeviceModel) -> Self {
        AssetModelDAG::WeighingDeviceModel(value)
    }
}
impl<C> web_common_traits::database::MostConcreteVariant<C> for crate::AssetModel
where
    crate::BallMillMachineModel: web_common_traits::database::Read<C>,
    crate::BeadModel: web_common_traits::database::Read<C>,
    crate::CameraModel: web_common_traits::database::Read<C>,
    crate::CapModel: web_common_traits::database::Read<C>,
    crate::CentrifugeModel: web_common_traits::database::Read<C>,
    crate::CommercialBallMillMachineLot: web_common_traits::database::Read<C>,
    crate::CommercialBallMillMachineModel: web_common_traits::database::Read<C>,
    crate::CommercialBeadLot: web_common_traits::database::Read<C>,
    crate::CommercialBeadModel: web_common_traits::database::Read<C>,
    crate::CommercialCameraLot: web_common_traits::database::Read<C>,
    crate::CommercialCameraModel: web_common_traits::database::Read<C>,
    crate::CommercialCapLot: web_common_traits::database::Read<C>,
    crate::CommercialCapModel: web_common_traits::database::Read<C>,
    crate::CommercialCentrifugeLot: web_common_traits::database::Read<C>,
    crate::CommercialCentrifugeModel: web_common_traits::database::Read<C>,
    crate::CommercialFreezeDryerLot: web_common_traits::database::Read<C>,
    crate::CommercialFreezeDryerModel: web_common_traits::database::Read<C>,
    crate::CommercialFreezerLot: web_common_traits::database::Read<C>,
    crate::CommercialFreezerModel: web_common_traits::database::Read<C>,
    crate::CommercialPackagingLot: web_common_traits::database::Read<C>,
    crate::CommercialPackagingModel: web_common_traits::database::Read<C>,
    crate::CommercialPipetteLot: web_common_traits::database::Read<C>,
    crate::CommercialPipetteModel: web_common_traits::database::Read<C>,
    crate::CommercialPipetteTipLot: web_common_traits::database::Read<C>,
    crate::CommercialPipetteTipModel: web_common_traits::database::Read<C>,
    crate::CommercialPositioningDeviceLot: web_common_traits::database::Read<C>,
    crate::CommercialPositioningDeviceModel: web_common_traits::database::Read<C>,
    crate::CommercialProductLot: web_common_traits::database::Read<C>,
    crate::CommercialProduct: web_common_traits::database::Read<C>,
    crate::CommercialVolumeMeasuringDeviceLot: web_common_traits::database::Read<C>,
    crate::CommercialVolumeMeasuringDeviceModel: web_common_traits::database::Read<C>,
    crate::CommercialWeighingDeviceLot: web_common_traits::database::Read<C>,
    crate::CommercialWeighingDeviceModel: web_common_traits::database::Read<C>,
    crate::ContainerModel: web_common_traits::database::Read<C>,
    crate::DigitalAssetModel: web_common_traits::database::Read<C>,
    crate::FreezeDryerModel: web_common_traits::database::Read<C>,
    crate::FreezerModel: web_common_traits::database::Read<C>,
    crate::PackagingModel: web_common_traits::database::Read<C>,
    crate::PhoneModel: web_common_traits::database::Read<C>,
    crate::PhysicalAssetModel: web_common_traits::database::Read<C>,
    crate::PipetteModel: web_common_traits::database::Read<C>,
    crate::PipetteTipModel: web_common_traits::database::Read<C>,
    crate::PositioningDeviceModel: web_common_traits::database::Read<C>,
    crate::ReagentModel: web_common_traits::database::Read<C>,
    crate::VolumeMeasuringDeviceModel: web_common_traits::database::Read<C>,
    crate::VolumetricContainerModel: web_common_traits::database::Read<C>,
    crate::WeighingDeviceModel: web_common_traits::database::Read<C>,
{
    type Variant = AssetModelDAG;
    fn most_concrete_variant(&self, conn: &mut C) -> Result<Self::Variant, diesel::result::Error> {
        use diesel::Identifiable;
        Ok(
            match self.most_concrete_table.as_str() {
                "asset_models" => self.clone().into(),
                "ball_mill_machine_models" => {
                    <crate::BallMillMachineModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "bead_models" => {
                    <crate::BeadModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "camera_models" => {
                    <crate::CameraModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "cap_models" => {
                    <crate::CapModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "centrifuge_models" => {
                    <crate::CentrifugeModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_ball_mill_machine_lots" => {
                    <crate::CommercialBallMillMachineLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_ball_mill_machine_models" => {
                    <crate::CommercialBallMillMachineModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_bead_lots" => {
                    <crate::CommercialBeadLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_bead_models" => {
                    <crate::CommercialBeadModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_camera_lots" => {
                    <crate::CommercialCameraLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_camera_models" => {
                    <crate::CommercialCameraModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_cap_lots" => {
                    <crate::CommercialCapLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_cap_models" => {
                    <crate::CommercialCapModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_centrifuge_lots" => {
                    <crate::CommercialCentrifugeLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_centrifuge_models" => {
                    <crate::CommercialCentrifugeModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_freeze_dryer_lots" => {
                    <crate::CommercialFreezeDryerLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_freeze_dryer_models" => {
                    <crate::CommercialFreezeDryerModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_freezer_lots" => {
                    <crate::CommercialFreezerLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_freezer_models" => {
                    <crate::CommercialFreezerModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_packaging_lots" => {
                    <crate::CommercialPackagingLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_packaging_models" => {
                    <crate::CommercialPackagingModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_pipette_lots" => {
                    <crate::CommercialPipetteLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_pipette_models" => {
                    <crate::CommercialPipetteModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_pipette_tip_lots" => {
                    <crate::CommercialPipetteTipLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_pipette_tip_models" => {
                    <crate::CommercialPipetteTipModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_positioning_device_lots" => {
                    <crate::CommercialPositioningDeviceLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_positioning_device_models" => {
                    <crate::CommercialPositioningDeviceModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_product_lots" => {
                    <crate::CommercialProductLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_products" => {
                    <crate::CommercialProduct as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_volume_measuring_device_lots" => {
                    <crate::CommercialVolumeMeasuringDeviceLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_volume_measuring_device_models" => {
                    <crate::CommercialVolumeMeasuringDeviceModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_weighing_device_lots" => {
                    <crate::CommercialWeighingDeviceLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_weighing_device_models" => {
                    <crate::CommercialWeighingDeviceModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "container_models" => {
                    <crate::ContainerModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "digital_asset_models" => {
                    <crate::DigitalAssetModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "freeze_dryer_models" => {
                    <crate::FreezeDryerModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "freezer_models" => {
                    <crate::FreezerModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "packaging_models" => {
                    <crate::PackagingModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "phone_models" => {
                    <crate::PhoneModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "physical_asset_models" => {
                    <crate::PhysicalAssetModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "pipette_models" => {
                    <crate::PipetteModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "pipette_tip_models" => {
                    <crate::PipetteTipModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "positioning_device_models" => {
                    <crate::PositioningDeviceModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "reagent_models" => {
                    <crate::ReagentModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "volume_measuring_device_models" => {
                    <crate::VolumeMeasuringDeviceModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "volumetric_container_models" => {
                    <crate::VolumetricContainerModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "weighing_device_models" => {
                    <crate::WeighingDeviceModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                _ => unreachable!("Database and codegen are out of sync."),
            },
        )
    }
}
