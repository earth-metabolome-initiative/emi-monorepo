#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enum representing the `assets` table builder DAG.
pub enum AssetBuilderDAG {
    ///Builder for the `assets` table.
    Asset(crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder),
    ///Builder for the `ball_mill_machines` table.
    BallMillMachine(
        crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineBuilder,
    ),
    ///Builder for the `cameras` table.
    Camera(
        crate::codegen::structs_codegen::tables::insertables::InsertableCameraBuilder,
    ),
    ///Builder for the `centrifuges` table.
    Centrifuge(
        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeBuilder,
    ),
    ///Builder for the `containers` table.
    Container(
        crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder,
    ),
    ///Builder for the `digital_assets` table.
    DigitalAsset(
        crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetBuilder,
    ),
    ///Builder for the `freeze_dryers` table.
    FreezeDryer(
        crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerBuilder,
    ),
    ///Builder for the `freezers` table.
    Freezer(
        crate::codegen::structs_codegen::tables::insertables::InsertableFreezerBuilder,
    ),
    ///Builder for the `organisms` table.
    Organism(
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder,
    ),
    ///Builder for the `photographs` table.
    Photograph(
        crate::codegen::structs_codegen::tables::insertables::InsertablePhotographBuilder,
    ),
    ///Builder for the `physical_assets` table.
    PhysicalAsset(
        crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder,
    ),
    ///Builder for the `pipettes` table.
    Pipette(
        crate::codegen::structs_codegen::tables::insertables::InsertablePipetteBuilder,
    ),
    ///Builder for the `positioning_devices` table.
    PositioningDevice(
        crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceBuilder,
    ),
    ///Builder for the `sample_sources` table.
    SampleSource(
        crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceBuilder,
    ),
    ///Builder for the `samples` table.
    Sample(
        crate::codegen::structs_codegen::tables::insertables::InsertableSampleBuilder,
    ),
    ///Builder for the `soils` table.
    Soil(crate::codegen::structs_codegen::tables::insertables::InsertableSoilBuilder),
    ///Builder for the `spectra` table.
    Spectrum(
        crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumBuilder,
    ),
    ///Builder for the `spectra_collections` table.
    SpectraCollection(
        crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder,
    ),
    ///Builder for the `volume_measuring_devices` table.
    VolumeMeasuringDevice(
        crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceBuilder,
    ),
    ///Builder for the `volumetric_containers` table.
    VolumetricContainer(
        crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerBuilder,
    ),
    ///Builder for the `weighing_devices` table.
    WeighingDevice(
        crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceBuilder,
    ),
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder>
    for AssetBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder,
    ) -> Self {
        AssetBuilderDAG::Asset(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineBuilder>
    for AssetBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineBuilder,
    ) -> Self {
        AssetBuilderDAG::BallMillMachine(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableCameraBuilder>
    for AssetBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCameraBuilder,
    ) -> Self {
        AssetBuilderDAG::Camera(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeBuilder>
    for AssetBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeBuilder,
    ) -> Self {
        AssetBuilderDAG::Centrifuge(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder>
    for AssetBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder,
    ) -> Self {
        AssetBuilderDAG::Container(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetBuilder>
    for AssetBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetBuilder,
    ) -> Self {
        AssetBuilderDAG::DigitalAsset(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerBuilder>
    for AssetBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerBuilder,
    ) -> Self {
        AssetBuilderDAG::FreezeDryer(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableFreezerBuilder>
    for AssetBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableFreezerBuilder,
    ) -> Self {
        AssetBuilderDAG::Freezer(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder>
    for AssetBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder,
    ) -> Self {
        AssetBuilderDAG::Organism(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertablePhotographBuilder>
    for AssetBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertablePhotographBuilder,
    ) -> Self {
        AssetBuilderDAG::Photograph(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder>
    for AssetBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder,
    ) -> Self {
        AssetBuilderDAG::PhysicalAsset(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertablePipetteBuilder>
    for AssetBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertablePipetteBuilder,
    ) -> Self {
        AssetBuilderDAG::Pipette(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceBuilder>
    for AssetBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceBuilder,
    ) -> Self {
        AssetBuilderDAG::PositioningDevice(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceBuilder>
    for AssetBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceBuilder,
    ) -> Self {
        AssetBuilderDAG::SampleSource(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableSampleBuilder>
    for AssetBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableSampleBuilder,
    ) -> Self {
        AssetBuilderDAG::Sample(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableSoilBuilder>
    for AssetBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableSoilBuilder,
    ) -> Self {
        AssetBuilderDAG::Soil(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumBuilder>
    for AssetBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumBuilder,
    ) -> Self {
        AssetBuilderDAG::Spectrum(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder>
    for AssetBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder,
    ) -> Self {
        AssetBuilderDAG::SpectraCollection(value)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceBuilder,
> for AssetBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceBuilder,
    ) -> Self {
        AssetBuilderDAG::VolumeMeasuringDevice(value)
    }
}
impl
    From<crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerBuilder>
    for AssetBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerBuilder,
    ) -> Self {
        AssetBuilderDAG::VolumetricContainer(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceBuilder>
    for AssetBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceBuilder,
    ) -> Self {
        AssetBuilderDAG::WeighingDevice(value)
    }
}
