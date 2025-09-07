#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enum representing the most concrete variant of the `assets` table DAG.
///
/// # Mermaid illustration of the DAG:
/// ```mermaid
/// flowchart BT
/// v0@{shape: rect, label: "assets"}
/// v1@{shape: rect, label: "ball_mill_machines"}
/// v2@{shape: rect, label: "cameras"}
/// v3@{shape: rect, label: "centrifuges"}
/// v4@{shape: rect, label: "containers"}
/// v5@{shape: rect, label: "digital_assets"}
/// v6@{shape: rect, label: "freeze_dryers"}
/// v7@{shape: rect, label: "freezers"}
/// v8@{shape: rect, label: "organisms"}
/// v9@{shape: rect, label: "physical_assets"}
/// v10@{shape: rect, label: "pipettes"}
/// v11@{shape: rect, label: "positioning_devices"}
/// v12@{shape: rect, label: "spectra"}
/// v13@{shape: rect, label: "spectra_collections"}
/// v14@{shape: rect, label: "volume_measuring_devices"}
/// v15@{shape: rect, label: "volumetric_containers"}
/// v16@{shape: rect, label: "weighing_devices"}
/// v11 --->|"`extends`"| v9
/// v16 --->|"`extends`"| v9
/// v10 --->|"`extends`"| v9
/// v7 --->|"`extends`"| v9
/// v12 --->|"`extends`"| v5
/// v13 --->|"`extends`"| v5
/// v1 --->|"`extends`"| v9
/// v6 --->|"`extends`"| v9
/// v2 --->|"`extends`"| v9
/// v4 --->|"`extends`"| v9
/// v15 --->|"`extends`"| v4
/// v9 --->|"`extends`"| v0
/// v8 --->|"`extends`"| v9
/// v3 --->|"`extends`"| v9
/// v5 --->|"`extends`"| v0
/// v14 --->|"`extends`"| v9
/// ```
pub enum AssetDAG {
    /// Variant representing the `assets` table.
    Asset(crate::codegen::structs_codegen::tables::assets::Asset),
    /// Variant representing the `ball_mill_machines` table.
    BallMillMachine(crate::codegen::structs_codegen::tables::ball_mill_machines::BallMillMachine),
    /// Variant representing the `cameras` table.
    Camera(crate::codegen::structs_codegen::tables::cameras::Camera),
    /// Variant representing the `centrifuges` table.
    Centrifuge(crate::codegen::structs_codegen::tables::centrifuges::Centrifuge),
    /// Variant representing the `containers` table.
    Container(crate::codegen::structs_codegen::tables::containers::Container),
    /// Variant representing the `digital_assets` table.
    DigitalAsset(crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset),
    /// Variant representing the `freeze_dryers` table.
    FreezeDryer(crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer),
    /// Variant representing the `freezers` table.
    Freezer(crate::codegen::structs_codegen::tables::freezers::Freezer),
    /// Variant representing the `organisms` table.
    Organism(crate::codegen::structs_codegen::tables::organisms::Organism),
    /// Variant representing the `physical_assets` table.
    PhysicalAsset(crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset),
    /// Variant representing the `pipettes` table.
    Pipette(crate::codegen::structs_codegen::tables::pipettes::Pipette),
    /// Variant representing the `positioning_devices` table.
    PositioningDevice(
        crate::codegen::structs_codegen::tables::positioning_devices::PositioningDevice,
    ),
    /// Variant representing the `spectra` table.
    Spectrum(crate::codegen::structs_codegen::tables::spectra::Spectrum),
    /// Variant representing the `spectra_collections` table.
    SpectraCollection(
        crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
    ),
    /// Variant representing the `volume_measuring_devices` table.
    VolumeMeasuringDevice(
        crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice,
    ),
    /// Variant representing the `volumetric_containers` table.
    VolumetricContainer(
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
    ),
    /// Variant representing the `weighing_devices` table.
    WeighingDevice(crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice),
}
impl From<crate::codegen::structs_codegen::tables::assets::Asset> for AssetDAG {
    fn from(value: crate::codegen::structs_codegen::tables::assets::Asset) -> Self {
        AssetDAG::Asset(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::ball_mill_machines::BallMillMachine>
    for AssetDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::ball_mill_machines::BallMillMachine,
    ) -> Self {
        AssetDAG::BallMillMachine(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::cameras::Camera> for AssetDAG {
    fn from(value: crate::codegen::structs_codegen::tables::cameras::Camera) -> Self {
        AssetDAG::Camera(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::centrifuges::Centrifuge> for AssetDAG {
    fn from(value: crate::codegen::structs_codegen::tables::centrifuges::Centrifuge) -> Self {
        AssetDAG::Centrifuge(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::containers::Container> for AssetDAG {
    fn from(value: crate::codegen::structs_codegen::tables::containers::Container) -> Self {
        AssetDAG::Container(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset> for AssetDAG {
    fn from(value: crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset) -> Self {
        AssetDAG::DigitalAsset(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer> for AssetDAG {
    fn from(value: crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer) -> Self {
        AssetDAG::FreezeDryer(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::freezers::Freezer> for AssetDAG {
    fn from(value: crate::codegen::structs_codegen::tables::freezers::Freezer) -> Self {
        AssetDAG::Freezer(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::organisms::Organism> for AssetDAG {
    fn from(value: crate::codegen::structs_codegen::tables::organisms::Organism) -> Self {
        AssetDAG::Organism(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset> for AssetDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
    ) -> Self {
        AssetDAG::PhysicalAsset(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::pipettes::Pipette> for AssetDAG {
    fn from(value: crate::codegen::structs_codegen::tables::pipettes::Pipette) -> Self {
        AssetDAG::Pipette(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::positioning_devices::PositioningDevice>
    for AssetDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::positioning_devices::PositioningDevice,
    ) -> Self {
        AssetDAG::PositioningDevice(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::spectra::Spectrum> for AssetDAG {
    fn from(value: crate::codegen::structs_codegen::tables::spectra::Spectrum) -> Self {
        AssetDAG::Spectrum(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection>
    for AssetDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
    ) -> Self {
        AssetDAG::SpectraCollection(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice>
    for AssetDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice,
    ) -> Self {
        AssetDAG::VolumeMeasuringDevice(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer>
    for AssetDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
    ) -> Self {
        AssetDAG::VolumetricContainer(value)
    }
}
impl From<crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice> for AssetDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice,
    ) -> Self {
        AssetDAG::WeighingDevice(value)
    }
}
impl<C> web_common_traits::database::MostConcreteVariant<C>
    for crate::codegen::structs_codegen::tables::assets::Asset
where
    crate::codegen::structs_codegen::tables::ball_mill_machines::BallMillMachine:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::cameras::Camera: web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::centrifuges::Centrifuge:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::containers::Container:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::freezers::Freezer:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::organisms::Organism:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::pipettes::Pipette:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::positioning_devices::PositioningDevice:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::spectra::Spectrum:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice:
        web_common_traits::database::Read<C>,
{
    type Variant = AssetDAG;
    fn most_concrete_variant(&self, conn: &mut C) -> Result<Self::Variant, diesel::result::Error> {
        use diesel::Identifiable;
        Ok(
            match self.most_concrete_table.as_str() {
                "assets" => self.clone().into(),
                "ball_mill_machines" => {
                    <crate::codegen::structs_codegen::tables::ball_mill_machines::BallMillMachine as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "cameras" => {
                    <crate::codegen::structs_codegen::tables::cameras::Camera as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "centrifuges" => {
                    <crate::codegen::structs_codegen::tables::centrifuges::Centrifuge as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "containers" => {
                    <crate::codegen::structs_codegen::tables::containers::Container as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "digital_assets" => {
                    <crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "freeze_dryers" => {
                    <crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "freezers" => {
                    <crate::codegen::structs_codegen::tables::freezers::Freezer as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "organisms" => {
                    <crate::codegen::structs_codegen::tables::organisms::Organism as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "physical_assets" => {
                    <crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "pipettes" => {
                    <crate::codegen::structs_codegen::tables::pipettes::Pipette as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "positioning_devices" => {
                    <crate::codegen::structs_codegen::tables::positioning_devices::PositioningDevice as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "spectra" => {
                    <crate::codegen::structs_codegen::tables::spectra::Spectrum as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "spectra_collections" => {
                    <crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "volume_measuring_devices" => {
                    <crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "volumetric_containers" => {
                    <crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                "weighing_devices" => {
                    <crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice as web_common_traits::database::Read<
                        C,
                    >>::read(*self.id(), conn)?
                        .into()
                }
                _ => unreachable!("Database and codegen are out of sync."),
            },
        )
    }
}
