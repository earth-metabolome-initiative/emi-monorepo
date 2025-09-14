mod builder;
pub use builder::AssetModelBuilderDAG;
#[derive(Debug, Clone, PartialEq)]
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
/// v38@{shape: rect, label: "organism_models"}
/// v39@{shape: rect, label: "packaging_models"}
/// v40@{shape: rect, label: "phone_models"}
/// v41@{shape: rect, label: "physical_asset_models"}
/// v42@{shape: rect, label: "pipette_models"}
/// v43@{shape: rect, label: "pipette_tip_models"}
/// v44@{shape: rect, label: "positioning_device_models"}
/// v45@{shape: rect, label: "reagent_models"}
/// v46@{shape: rect, label: "sample_models"}
/// v47@{shape: rect, label: "sample_source_models"}
/// v48@{shape: rect, label: "soil_models"}
/// v49@{shape: rect, label: "volume_measuring_device_models"}
/// v50@{shape: rect, label: "volumetric_container_models"}
/// v51@{shape: rect, label: "weighing_device_models"}
/// v1 --->|"`extends`"| v41
/// v2 --->|"`extends`"| v41
/// v3 --->|"`extends`"| v41
/// v4 --->|"`extends`"| v41
/// v5 --->|"`extends`"| v41
/// v6 --->|"`extends`"| v28
/// v6 --->|"`extends`"| v1
/// v7 --->|"`extends`"| v1
/// v7 --->|"`extends`"| v29
/// v8 --->|"`extends`"| v28
/// v8 --->|"`extends`"| v2
/// v9 --->|"`extends`"| v2
/// v9 --->|"`extends`"| v29
/// v10 --->|"`extends`"| v28
/// v10 --->|"`extends`"| v3
/// v11 --->|"`extends`"| v3
/// v11 --->|"`extends`"| v29
/// v12 --->|"`extends`"| v28
/// v12 --->|"`extends`"| v4
/// v13 --->|"`extends`"| v4
/// v13 --->|"`extends`"| v29
/// v14 --->|"`extends`"| v28
/// v14 --->|"`extends`"| v5
/// v15 --->|"`extends`"| v5
/// v15 --->|"`extends`"| v29
/// v16 --->|"`extends`"| v28
/// v16 --->|"`extends`"| v36
/// v17 --->|"`extends`"| v36
/// v17 --->|"`extends`"| v29
/// v18 --->|"`extends`"| v28
/// v18 --->|"`extends`"| v37
/// v19 --->|"`extends`"| v37
/// v19 --->|"`extends`"| v29
/// v20 --->|"`extends`"| v28
/// v20 --->|"`extends`"| v39
/// v21 --->|"`extends`"| v39
/// v21 --->|"`extends`"| v29
/// v22 --->|"`extends`"| v28
/// v22 --->|"`extends`"| v42
/// v23 --->|"`extends`"| v42
/// v23 --->|"`extends`"| v29
/// v24 --->|"`extends`"| v28
/// v24 --->|"`extends`"| v43
/// v25 --->|"`extends`"| v43
/// v25 --->|"`extends`"| v29
/// v26 --->|"`extends`"| v28
/// v26 --->|"`extends`"| v44
/// v27 --->|"`extends`"| v44
/// v27 --->|"`extends`"| v29
/// v28 --->|"`extends`"| v41
/// v29 --->|"`extends`"| v0
/// v30 --->|"`extends`"| v28
/// v30 --->|"`extends`"| v49
/// v31 --->|"`extends`"| v49
/// v31 --->|"`extends`"| v29
/// v32 --->|"`extends`"| v28
/// v32 --->|"`extends`"| v51
/// v33 --->|"`extends`"| v51
/// v33 --->|"`extends`"| v29
/// v34 --->|"`extends`"| v41
/// v35 --->|"`extends`"| v0
/// v36 --->|"`extends`"| v41
/// v37 --->|"`extends`"| v41
/// v38 --->|"`extends`"| v47
/// v39 --->|"`extends`"| v41
/// v40 --->|"`extends`"| v3
/// v40 --->|"`extends`"| v44
/// v41 --->|"`extends`"| v0
/// v42 --->|"`extends`"| v41
/// v43 --->|"`extends`"| v41
/// v44 --->|"`extends`"| v41
/// v45 --->|"`extends`"| v0
/// v46 --->|"`extends`"| v41
/// v47 --->|"`extends`"| v41
/// v48 --->|"`extends`"| v47
/// v49 --->|"`extends`"| v41
/// v50 --->|"`extends`"| v34
/// v51 --->|"`extends`"| v41
/// ```
pub enum AssetModelDAG {
    ///Variant representing the `asset_models` table.
    AssetModel(crate::codegen::structs_codegen::tables::asset_models::AssetModel),
    ///Variant representing the `ball_mill_machine_models` table.
    BallMillMachineModel(
        crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
    ),
    ///Variant representing the `bead_models` table.
    BeadModel(crate::codegen::structs_codegen::tables::bead_models::BeadModel),
    ///Variant representing the `camera_models` table.
    CameraModel(crate::codegen::structs_codegen::tables::camera_models::CameraModel),
    ///Variant representing the `cap_models` table.
    CapModel(crate::codegen::structs_codegen::tables::cap_models::CapModel),
    ///Variant representing the `centrifuge_models` table.
    CentrifugeModel(
        crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel,
    ),
    ///Variant representing the `commercial_ball_mill_machine_lots` table.
    CommercialBallMillMachineLot(
        crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_lots::CommercialBallMillMachineLot,
    ),
    ///Variant representing the `commercial_ball_mill_machine_models` table.
    CommercialBallMillMachineModel(
        crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_models::CommercialBallMillMachineModel,
    ),
    ///Variant representing the `commercial_bead_lots` table.
    CommercialBeadLot(
        crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot,
    ),
    ///Variant representing the `commercial_bead_models` table.
    CommercialBeadModel(
        crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel,
    ),
    ///Variant representing the `commercial_camera_lots` table.
    CommercialCameraLot(
        crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot,
    ),
    ///Variant representing the `commercial_camera_models` table.
    CommercialCameraModel(
        crate::codegen::structs_codegen::tables::commercial_camera_models::CommercialCameraModel,
    ),
    ///Variant representing the `commercial_cap_lots` table.
    CommercialCapLot(
        crate::codegen::structs_codegen::tables::commercial_cap_lots::CommercialCapLot,
    ),
    ///Variant representing the `commercial_cap_models` table.
    CommercialCapModel(
        crate::codegen::structs_codegen::tables::commercial_cap_models::CommercialCapModel,
    ),
    ///Variant representing the `commercial_centrifuge_lots` table.
    CommercialCentrifugeLot(
        crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot,
    ),
    ///Variant representing the `commercial_centrifuge_models` table.
    CommercialCentrifugeModel(
        crate::codegen::structs_codegen::tables::commercial_centrifuge_models::CommercialCentrifugeModel,
    ),
    ///Variant representing the `commercial_freeze_dryer_lots` table.
    CommercialFreezeDryerLot(
        crate::codegen::structs_codegen::tables::commercial_freeze_dryer_lots::CommercialFreezeDryerLot,
    ),
    ///Variant representing the `commercial_freeze_dryer_models` table.
    CommercialFreezeDryerModel(
        crate::codegen::structs_codegen::tables::commercial_freeze_dryer_models::CommercialFreezeDryerModel,
    ),
    ///Variant representing the `commercial_freezer_lots` table.
    CommercialFreezerLot(
        crate::codegen::structs_codegen::tables::commercial_freezer_lots::CommercialFreezerLot,
    ),
    ///Variant representing the `commercial_freezer_models` table.
    CommercialFreezerModel(
        crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel,
    ),
    ///Variant representing the `commercial_packaging_lots` table.
    CommercialPackagingLot(
        crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot,
    ),
    ///Variant representing the `commercial_packaging_models` table.
    CommercialPackagingModel(
        crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel,
    ),
    ///Variant representing the `commercial_pipette_lots` table.
    CommercialPipetteLot(
        crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot,
    ),
    ///Variant representing the `commercial_pipette_models` table.
    CommercialPipetteModel(
        crate::codegen::structs_codegen::tables::commercial_pipette_models::CommercialPipetteModel,
    ),
    ///Variant representing the `commercial_pipette_tip_lots` table.
    CommercialPipetteTipLot(
        crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot,
    ),
    ///Variant representing the `commercial_pipette_tip_models` table.
    CommercialPipetteTipModel(
        crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel,
    ),
    ///Variant representing the `commercial_positioning_device_lots` table.
    CommercialPositioningDeviceLot(
        crate::codegen::structs_codegen::tables::commercial_positioning_device_lots::CommercialPositioningDeviceLot,
    ),
    ///Variant representing the `commercial_positioning_device_models` table.
    CommercialPositioningDeviceModel(
        crate::codegen::structs_codegen::tables::commercial_positioning_device_models::CommercialPositioningDeviceModel,
    ),
    ///Variant representing the `commercial_product_lots` table.
    CommercialProductLot(
        crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
    ),
    ///Variant representing the `commercial_products` table.
    CommercialProduct(
        crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
    ),
    ///Variant representing the `commercial_volume_measuring_device_lots` table.
    CommercialVolumeMeasuringDeviceLot(
        crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot,
    ),
    ///Variant representing the `commercial_volume_measuring_device_models` table.
    CommercialVolumeMeasuringDeviceModel(
        crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel,
    ),
    ///Variant representing the `commercial_weighing_device_lots` table.
    CommercialWeighingDeviceLot(
        crate::codegen::structs_codegen::tables::commercial_weighing_device_lots::CommercialWeighingDeviceLot,
    ),
    ///Variant representing the `commercial_weighing_device_models` table.
    CommercialWeighingDeviceModel(
        crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel,
    ),
    ///Variant representing the `container_models` table.
    ContainerModel(
        crate::codegen::structs_codegen::tables::container_models::ContainerModel,
    ),
    ///Variant representing the `digital_asset_models` table.
    DigitalAssetModel(
        crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel,
    ),
    ///Variant representing the `freeze_dryer_models` table.
    FreezeDryerModel(
        crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel,
    ),
    ///Variant representing the `freezer_models` table.
    FreezerModel(crate::codegen::structs_codegen::tables::freezer_models::FreezerModel),
    ///Variant representing the `organism_models` table.
    OrganismModel(
        crate::codegen::structs_codegen::tables::organism_models::OrganismModel,
    ),
    ///Variant representing the `packaging_models` table.
    PackagingModel(
        crate::codegen::structs_codegen::tables::packaging_models::PackagingModel,
    ),
    ///Variant representing the `phone_models` table.
    PhoneModel(crate::codegen::structs_codegen::tables::phone_models::PhoneModel),
    ///Variant representing the `physical_asset_models` table.
    PhysicalAssetModel(
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
    ),
    ///Variant representing the `pipette_models` table.
    PipetteModel(crate::codegen::structs_codegen::tables::pipette_models::PipetteModel),
    ///Variant representing the `pipette_tip_models` table.
    PipetteTipModel(
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
    ),
    ///Variant representing the `positioning_device_models` table.
    PositioningDeviceModel(
        crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel,
    ),
    ///Variant representing the `reagent_models` table.
    ReagentModel(crate::codegen::structs_codegen::tables::reagent_models::ReagentModel),
    ///Variant representing the `sample_models` table.
    SampleModel(crate::codegen::structs_codegen::tables::sample_models::SampleModel),
    ///Variant representing the `sample_source_models` table.
    SampleSourceModel(
        crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel,
    ),
    ///Variant representing the `soil_models` table.
    SoilModel(crate::codegen::structs_codegen::tables::soil_models::SoilModel),
    ///Variant representing the `volume_measuring_device_models` table.
    VolumeMeasuringDeviceModel(
        crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel,
    ),
    ///Variant representing the `volumetric_container_models` table.
    VolumetricContainerModel(
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    ),
    ///Variant representing the `weighing_device_models` table.
    WeighingDeviceModel(
        crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel,
    ),
}
impl From<crate::codegen::structs_codegen::tables::asset_models::AssetModel> for AssetModelDAG {
    fn from(value: crate::codegen::structs_codegen::tables::asset_models::AssetModel) -> Self {
        AssetModelDAG::AssetModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
    ) -> Self {
        AssetModelDAG::BallMillMachineModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::bead_models::BeadModel> for AssetModelDAG {
    fn from(value: crate::codegen::structs_codegen::tables::bead_models::BeadModel) -> Self {
        AssetModelDAG::BeadModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::camera_models::CameraModel> for AssetModelDAG {
    fn from(value: crate::codegen::structs_codegen::tables::camera_models::CameraModel) -> Self {
        AssetModelDAG::CameraModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::cap_models::CapModel> for AssetModelDAG {
    fn from(value: crate::codegen::structs_codegen::tables::cap_models::CapModel) -> Self {
        AssetModelDAG::CapModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel,
    ) -> Self {
        AssetModelDAG::CentrifugeModel(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_lots::CommercialBallMillMachineLot,
> for AssetModelDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_lots::CommercialBallMillMachineLot,
    ) -> Self {
        AssetModelDAG::CommercialBallMillMachineLot(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_models::CommercialBallMillMachineModel,
> for AssetModelDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_models::CommercialBallMillMachineModel,
    ) -> Self {
        AssetModelDAG::CommercialBallMillMachineModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot,
    ) -> Self {
        AssetModelDAG::CommercialBeadLot(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel,
    ) -> Self {
        AssetModelDAG::CommercialBeadModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot,
    ) -> Self {
        AssetModelDAG::CommercialCameraLot(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::commercial_camera_models::CommercialCameraModel>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_camera_models::CommercialCameraModel,
    ) -> Self {
        AssetModelDAG::CommercialCameraModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::commercial_cap_lots::CommercialCapLot>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_cap_lots::CommercialCapLot,
    ) -> Self {
        AssetModelDAG::CommercialCapLot(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::commercial_cap_models::CommercialCapModel>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_cap_models::CommercialCapModel,
    ) -> Self {
        AssetModelDAG::CommercialCapModel(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot,
> for AssetModelDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot,
    ) -> Self {
        AssetModelDAG::CommercialCentrifugeLot(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::commercial_centrifuge_models::CommercialCentrifugeModel,
> for AssetModelDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_centrifuge_models::CommercialCentrifugeModel,
    ) -> Self {
        AssetModelDAG::CommercialCentrifugeModel(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::commercial_freeze_dryer_lots::CommercialFreezeDryerLot,
> for AssetModelDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_freeze_dryer_lots::CommercialFreezeDryerLot,
    ) -> Self {
        AssetModelDAG::CommercialFreezeDryerLot(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::commercial_freeze_dryer_models::CommercialFreezeDryerModel,
> for AssetModelDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_freeze_dryer_models::CommercialFreezeDryerModel,
    ) -> Self {
        AssetModelDAG::CommercialFreezeDryerModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::commercial_freezer_lots::CommercialFreezerLot>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_freezer_lots::CommercialFreezerLot,
    ) -> Self {
        AssetModelDAG::CommercialFreezerLot(value)
    }
}
impl
    From<crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel,
    ) -> Self {
        AssetModelDAG::CommercialFreezerModel(value)
    }
}
impl
    From<crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot,
    ) -> Self {
        AssetModelDAG::CommercialPackagingLot(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel,
> for AssetModelDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel,
    ) -> Self {
        AssetModelDAG::CommercialPackagingModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot,
    ) -> Self {
        AssetModelDAG::CommercialPipetteLot(value)
    }
}
impl
    From<crate::codegen::structs_codegen::tables::commercial_pipette_models::CommercialPipetteModel>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_pipette_models::CommercialPipetteModel,
    ) -> Self {
        AssetModelDAG::CommercialPipetteModel(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot,
> for AssetModelDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot,
    ) -> Self {
        AssetModelDAG::CommercialPipetteTipLot(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel,
> for AssetModelDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel,
    ) -> Self {
        AssetModelDAG::CommercialPipetteTipModel(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::commercial_positioning_device_lots::CommercialPositioningDeviceLot,
> for AssetModelDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_positioning_device_lots::CommercialPositioningDeviceLot,
    ) -> Self {
        AssetModelDAG::CommercialPositioningDeviceLot(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::commercial_positioning_device_models::CommercialPositioningDeviceModel,
> for AssetModelDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_positioning_device_models::CommercialPositioningDeviceModel,
    ) -> Self {
        AssetModelDAG::CommercialPositioningDeviceModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
    ) -> Self {
        AssetModelDAG::CommercialProductLot(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
    ) -> Self {
        AssetModelDAG::CommercialProduct(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot,
> for AssetModelDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot,
    ) -> Self {
        AssetModelDAG::CommercialVolumeMeasuringDeviceLot(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel,
> for AssetModelDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel,
    ) -> Self {
        AssetModelDAG::CommercialVolumeMeasuringDeviceModel(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::commercial_weighing_device_lots::CommercialWeighingDeviceLot,
> for AssetModelDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_weighing_device_lots::CommercialWeighingDeviceLot,
    ) -> Self {
        AssetModelDAG::CommercialWeighingDeviceLot(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel,
> for AssetModelDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel,
    ) -> Self {
        AssetModelDAG::CommercialWeighingDeviceModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::container_models::ContainerModel>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::container_models::ContainerModel,
    ) -> Self {
        AssetModelDAG::ContainerModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel,
    ) -> Self {
        AssetModelDAG::DigitalAssetModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel,
    ) -> Self {
        AssetModelDAG::FreezeDryerModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::freezer_models::FreezerModel> for AssetModelDAG {
    fn from(value: crate::codegen::structs_codegen::tables::freezer_models::FreezerModel) -> Self {
        AssetModelDAG::FreezerModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::organism_models::OrganismModel>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::organism_models::OrganismModel,
    ) -> Self {
        AssetModelDAG::OrganismModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::packaging_models::PackagingModel>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::packaging_models::PackagingModel,
    ) -> Self {
        AssetModelDAG::PackagingModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::phone_models::PhoneModel> for AssetModelDAG {
    fn from(value: crate::codegen::structs_codegen::tables::phone_models::PhoneModel) -> Self {
        AssetModelDAG::PhoneModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
    ) -> Self {
        AssetModelDAG::PhysicalAssetModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel> for AssetModelDAG {
    fn from(value: crate::codegen::structs_codegen::tables::pipette_models::PipetteModel) -> Self {
        AssetModelDAG::PipetteModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
    ) -> Self {
        AssetModelDAG::PipetteTipModel(value)
    }
}
impl
    From<crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel,
    ) -> Self {
        AssetModelDAG::PositioningDeviceModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::reagent_models::ReagentModel> for AssetModelDAG {
    fn from(value: crate::codegen::structs_codegen::tables::reagent_models::ReagentModel) -> Self {
        AssetModelDAG::ReagentModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::sample_models::SampleModel> for AssetModelDAG {
    fn from(value: crate::codegen::structs_codegen::tables::sample_models::SampleModel) -> Self {
        AssetModelDAG::SampleModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel,
    ) -> Self {
        AssetModelDAG::SampleSourceModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::soil_models::SoilModel> for AssetModelDAG {
    fn from(value: crate::codegen::structs_codegen::tables::soil_models::SoilModel) -> Self {
        AssetModelDAG::SoilModel(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel,
> for AssetModelDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel,
    ) -> Self {
        AssetModelDAG::VolumeMeasuringDeviceModel(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
> for AssetModelDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    ) -> Self {
        AssetModelDAG::VolumetricContainerModel(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel>
    for AssetModelDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel,
    ) -> Self {
        AssetModelDAG::WeighingDeviceModel(value)
    }
}
impl web_common_traits::database::MostConcreteVariantMetadata
    for crate::codegen::structs_codegen::tables::asset_models::AssetModel
{
    type Variant = AssetModelDAG;
}
impl<C> web_common_traits::database::MostConcreteVariant<C>
for crate::codegen::structs_codegen::tables::asset_models::AssetModel
where
    crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::bead_models::BeadModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::camera_models::CameraModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::cap_models::CapModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_lots::CommercialBallMillMachineLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_models::CommercialBallMillMachineModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_camera_models::CommercialCameraModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_cap_lots::CommercialCapLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_cap_models::CommercialCapModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_centrifuge_models::CommercialCentrifugeModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_freeze_dryer_lots::CommercialFreezeDryerLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_freeze_dryer_models::CommercialFreezeDryerModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_freezer_lots::CommercialFreezerLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_pipette_models::CommercialPipetteModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_positioning_device_lots::CommercialPositioningDeviceLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_positioning_device_models::CommercialPositioningDeviceModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_weighing_device_lots::CommercialWeighingDeviceLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::container_models::ContainerModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::freezer_models::FreezerModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::organism_models::OrganismModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::packaging_models::PackagingModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::phone_models::PhoneModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::pipette_models::PipetteModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::reagent_models::ReagentModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::sample_models::SampleModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::soil_models::SoilModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel: web_common_traits::database::Read<
        C,
    >,
{
    fn most_concrete_variant(
        &self,
        conn: &mut C,
    ) -> Result<Self::Variant, diesel::result::Error> {
        use diesel::Identifiable;
        Ok(
            match self.most_concrete_table.as_str() {
                "asset_models" => self.clone().into(),
                "ball_mill_machine_models" => {
                    <crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "bead_models" => {
                    <crate::codegen::structs_codegen::tables::bead_models::BeadModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "camera_models" => {
                    <crate::codegen::structs_codegen::tables::camera_models::CameraModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "cap_models" => {
                    <crate::codegen::structs_codegen::tables::cap_models::CapModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "centrifuge_models" => {
                    <crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_ball_mill_machine_lots" => {
                    <crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_lots::CommercialBallMillMachineLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_ball_mill_machine_models" => {
                    <crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_models::CommercialBallMillMachineModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_bead_lots" => {
                    <crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_bead_models" => {
                    <crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_camera_lots" => {
                    <crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_camera_models" => {
                    <crate::codegen::structs_codegen::tables::commercial_camera_models::CommercialCameraModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_cap_lots" => {
                    <crate::codegen::structs_codegen::tables::commercial_cap_lots::CommercialCapLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_cap_models" => {
                    <crate::codegen::structs_codegen::tables::commercial_cap_models::CommercialCapModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_centrifuge_lots" => {
                    <crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_centrifuge_models" => {
                    <crate::codegen::structs_codegen::tables::commercial_centrifuge_models::CommercialCentrifugeModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_freeze_dryer_lots" => {
                    <crate::codegen::structs_codegen::tables::commercial_freeze_dryer_lots::CommercialFreezeDryerLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_freeze_dryer_models" => {
                    <crate::codegen::structs_codegen::tables::commercial_freeze_dryer_models::CommercialFreezeDryerModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_freezer_lots" => {
                    <crate::codegen::structs_codegen::tables::commercial_freezer_lots::CommercialFreezerLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_freezer_models" => {
                    <crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_packaging_lots" => {
                    <crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_packaging_models" => {
                    <crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_pipette_lots" => {
                    <crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_pipette_models" => {
                    <crate::codegen::structs_codegen::tables::commercial_pipette_models::CommercialPipetteModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_pipette_tip_lots" => {
                    <crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_pipette_tip_models" => {
                    <crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_positioning_device_lots" => {
                    <crate::codegen::structs_codegen::tables::commercial_positioning_device_lots::CommercialPositioningDeviceLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_positioning_device_models" => {
                    <crate::codegen::structs_codegen::tables::commercial_positioning_device_models::CommercialPositioningDeviceModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_product_lots" => {
                    <crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_products" => {
                    <crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_volume_measuring_device_lots" => {
                    <crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_volume_measuring_device_models" => {
                    <crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_weighing_device_lots" => {
                    <crate::codegen::structs_codegen::tables::commercial_weighing_device_lots::CommercialWeighingDeviceLot as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "commercial_weighing_device_models" => {
                    <crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "container_models" => {
                    <crate::codegen::structs_codegen::tables::container_models::ContainerModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "digital_asset_models" => {
                    <crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "freeze_dryer_models" => {
                    <crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "freezer_models" => {
                    <crate::codegen::structs_codegen::tables::freezer_models::FreezerModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "organism_models" => {
                    <crate::codegen::structs_codegen::tables::organism_models::OrganismModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "packaging_models" => {
                    <crate::codegen::structs_codegen::tables::packaging_models::PackagingModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "phone_models" => {
                    <crate::codegen::structs_codegen::tables::phone_models::PhoneModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "physical_asset_models" => {
                    <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "pipette_models" => {
                    <crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "pipette_tip_models" => {
                    <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "positioning_device_models" => {
                    <crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "reagent_models" => {
                    <crate::codegen::structs_codegen::tables::reagent_models::ReagentModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "sample_models" => {
                    <crate::codegen::structs_codegen::tables::sample_models::SampleModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "sample_source_models" => {
                    <crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "soil_models" => {
                    <crate::codegen::structs_codegen::tables::soil_models::SoilModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "volume_measuring_device_models" => {
                    <crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "volumetric_container_models" => {
                    <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "weighing_device_models" => {
                    <crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                _ => unreachable!("Database and codegen are out of sync."),
            },
        )
    }
}
