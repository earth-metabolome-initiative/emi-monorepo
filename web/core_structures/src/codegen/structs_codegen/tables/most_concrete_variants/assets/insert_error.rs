#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enum representing insert errors which may occur when inserting an entry from
/// the `assets` table DAG.
pub enum AssetInsertErrorDAG {
    /// Insert error associated with the `assets` table.
    Asset(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::AssetAttribute,
        >,
    ),
    /// Insert error associated with the `ball_mill_machines` table.
    BallMillMachine(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::BallMillMachineAttribute,
        >,
    ),
    /// Insert error associated with the `cameras` table.
    Camera(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CameraAttribute,
        >,
    ),
    /// Insert error associated with the `centrifuges` table.
    Centrifuge(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CentrifugeAttribute,
        >,
    ),
    /// Insert error associated with the `containers` table.
    Container(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::ContainerAttribute,
        >,
    ),
    /// Insert error associated with the `digital_assets` table.
    DigitalAsset(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute,
        >,
    ),
    /// Insert error associated with the `freeze_dryers` table.
    FreezeDryer(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezeDryerAttribute,
        >,
    ),
    /// Insert error associated with the `freezers` table.
    Freezer(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezerAttribute,
        >,
    ),
    /// Insert error associated with the `organisms` table.
    Organism(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::OrganismAttribute,
        >,
    ),
    /// Insert error associated with the `photographs` table.
    Photograph(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhotographAttribute,
        >,
    ),
    /// Insert error associated with the `physical_assets` table.
    PhysicalAsset(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhysicalAssetAttribute,
        >,
    ),
    /// Insert error associated with the `pipettes` table.
    Pipette(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PipetteAttribute,
        >,
    ),
    /// Insert error associated with the `positioning_devices` table.
    PositioningDevice(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PositioningDeviceAttribute,
        >,
    ),
    /// Insert error associated with the `sample_sources` table.
    SampleSource(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SampleSourceAttribute,
        >,
    ),
    /// Insert error associated with the `samples` table.
    Sample(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SampleAttribute,
        >,
    ),
    /// Insert error associated with the `soils` table.
    Soil(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SoilAttribute,
        >,
    ),
    /// Insert error associated with the `spectra` table.
    Spectrum(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SpectrumAttribute,
        >,
    ),
    /// Insert error associated with the `spectra_collections` table.
    SpectraCollection(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SpectraCollectionAttribute,
        >,
    ),
    /// Insert error associated with the `volume_measuring_devices` table.
    VolumeMeasuringDevice(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceAttribute,
        >,
    ),
    /// Insert error associated with the `volumetric_containers` table.
    VolumetricContainer(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::VolumetricContainerAttribute,
        >,
    ),
    /// Insert error associated with the `weighing_devices` table.
    WeighingDevice(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::WeighingDeviceAttribute,
        >,
    ),
}
impl std::fmt::Display for AssetInsertErrorDAG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Asset(e) => write!(f, "{e}"),
            Self::BallMillMachine(e) => write!(f, "{e}"),
            Self::Camera(e) => write!(f, "{e}"),
            Self::Centrifuge(e) => write!(f, "{e}"),
            Self::Container(e) => write!(f, "{e}"),
            Self::DigitalAsset(e) => write!(f, "{e}"),
            Self::FreezeDryer(e) => write!(f, "{e}"),
            Self::Freezer(e) => write!(f, "{e}"),
            Self::Organism(e) => write!(f, "{e}"),
            Self::Photograph(e) => write!(f, "{e}"),
            Self::PhysicalAsset(e) => write!(f, "{e}"),
            Self::Pipette(e) => write!(f, "{e}"),
            Self::PositioningDevice(e) => write!(f, "{e}"),
            Self::SampleSource(e) => write!(f, "{e}"),
            Self::Sample(e) => write!(f, "{e}"),
            Self::Soil(e) => write!(f, "{e}"),
            Self::Spectrum(e) => write!(f, "{e}"),
            Self::SpectraCollection(e) => write!(f, "{e}"),
            Self::VolumeMeasuringDevice(e) => write!(f, "{e}"),
            Self::VolumetricContainer(e) => write!(f, "{e}"),
            Self::WeighingDevice(e) => write!(f, "{e}"),
        }
    }
}
impl std::error::Error for AssetInsertErrorDAG {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Asset(e) => Some(e),
            Self::BallMillMachine(e) => Some(e),
            Self::Camera(e) => Some(e),
            Self::Centrifuge(e) => Some(e),
            Self::Container(e) => Some(e),
            Self::DigitalAsset(e) => Some(e),
            Self::FreezeDryer(e) => Some(e),
            Self::Freezer(e) => Some(e),
            Self::Organism(e) => Some(e),
            Self::Photograph(e) => Some(e),
            Self::PhysicalAsset(e) => Some(e),
            Self::Pipette(e) => Some(e),
            Self::PositioningDevice(e) => Some(e),
            Self::SampleSource(e) => Some(e),
            Self::Sample(e) => Some(e),
            Self::Soil(e) => Some(e),
            Self::Spectrum(e) => Some(e),
            Self::SpectraCollection(e) => Some(e),
            Self::VolumeMeasuringDevice(e) => Some(e),
            Self::VolumetricContainer(e) => Some(e),
            Self::WeighingDevice(e) => Some(e),
        }
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::AssetAttribute,
        >,
    > for AssetInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::AssetAttribute,
        >,
    ) -> Self {
        AssetInsertErrorDAG::Asset(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::BallMillMachineAttribute,
        >,
    > for AssetInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::BallMillMachineAttribute,
        >,
    ) -> Self {
        AssetInsertErrorDAG::BallMillMachine(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CameraAttribute,
        >,
    > for AssetInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CameraAttribute,
        >,
    ) -> Self {
        AssetInsertErrorDAG::Camera(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CentrifugeAttribute,
        >,
    > for AssetInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CentrifugeAttribute,
        >,
    ) -> Self {
        AssetInsertErrorDAG::Centrifuge(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::ContainerAttribute,
        >,
    > for AssetInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::ContainerAttribute,
        >,
    ) -> Self {
        AssetInsertErrorDAG::Container(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute,
        >,
    > for AssetInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute,
        >,
    ) -> Self {
        AssetInsertErrorDAG::DigitalAsset(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezeDryerAttribute,
        >,
    > for AssetInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezeDryerAttribute,
        >,
    ) -> Self {
        AssetInsertErrorDAG::FreezeDryer(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezerAttribute,
        >,
    > for AssetInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezerAttribute,
        >,
    ) -> Self {
        AssetInsertErrorDAG::Freezer(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::OrganismAttribute,
        >,
    > for AssetInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::OrganismAttribute,
        >,
    ) -> Self {
        AssetInsertErrorDAG::Organism(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhotographAttribute,
        >,
    > for AssetInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhotographAttribute,
        >,
    ) -> Self {
        AssetInsertErrorDAG::Photograph(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhysicalAssetAttribute,
        >,
    > for AssetInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhysicalAssetAttribute,
        >,
    ) -> Self {
        AssetInsertErrorDAG::PhysicalAsset(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PipetteAttribute,
        >,
    > for AssetInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PipetteAttribute,
        >,
    ) -> Self {
        AssetInsertErrorDAG::Pipette(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PositioningDeviceAttribute,
        >,
    > for AssetInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PositioningDeviceAttribute,
        >,
    ) -> Self {
        AssetInsertErrorDAG::PositioningDevice(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SampleSourceAttribute,
        >,
    > for AssetInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SampleSourceAttribute,
        >,
    ) -> Self {
        AssetInsertErrorDAG::SampleSource(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SampleAttribute,
        >,
    > for AssetInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SampleAttribute,
        >,
    ) -> Self {
        AssetInsertErrorDAG::Sample(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SoilAttribute,
        >,
    > for AssetInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SoilAttribute,
        >,
    ) -> Self {
        AssetInsertErrorDAG::Soil(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SpectrumAttribute,
        >,
    > for AssetInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SpectrumAttribute,
        >,
    ) -> Self {
        AssetInsertErrorDAG::Spectrum(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SpectraCollectionAttribute,
        >,
    > for AssetInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SpectraCollectionAttribute,
        >,
    ) -> Self {
        AssetInsertErrorDAG::SpectraCollection(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceAttribute,
        >,
    > for AssetInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceAttribute,
        >,
    ) -> Self {
        AssetInsertErrorDAG::VolumeMeasuringDevice(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::VolumetricContainerAttribute,
        >,
    > for AssetInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::VolumetricContainerAttribute,
        >,
    ) -> Self {
        AssetInsertErrorDAG::VolumetricContainer(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::WeighingDeviceAttribute,
        >,
    > for AssetInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::WeighingDeviceAttribute,
        >,
    ) -> Self {
        AssetInsertErrorDAG::WeighingDevice(error)
    }
}
