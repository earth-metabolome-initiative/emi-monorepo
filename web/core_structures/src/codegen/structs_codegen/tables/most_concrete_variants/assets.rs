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
/// v16 --->|"`extends`"| v9
/// v9 --->|"`extends`"| v0
/// v15 --->|"`extends`"| v4
/// v5 --->|"`extends`"| v0
/// v6 --->|"`extends`"| v9
/// v2 --->|"`extends`"| v9
/// v11 --->|"`extends`"| v9
/// v8 --->|"`extends`"| v9
/// v4 --->|"`extends`"| v9
/// v12 --->|"`extends`"| v5
/// v13 --->|"`extends`"| v5
/// v3 --->|"`extends`"| v9
/// v7 --->|"`extends`"| v9
/// v10 --->|"`extends`"| v9
/// v1 --->|"`extends`"| v9
/// v14 --->|"`extends`"| v9
/// ```
pub enum AssetDAG {
    /// Variant representing the `assets` table.
    Asset(crate::Asset),
    /// Variant representing the `ball_mill_machines` table.
    BallMillMachine(crate::BallMillMachine),
    /// Variant representing the `cameras` table.
    Camera(crate::Camera),
    /// Variant representing the `centrifuges` table.
    Centrifuge(crate::Centrifuge),
    /// Variant representing the `containers` table.
    Container(crate::Container),
    /// Variant representing the `digital_assets` table.
    DigitalAsset(crate::DigitalAsset),
    /// Variant representing the `freeze_dryers` table.
    FreezeDryer(crate::FreezeDryer),
    /// Variant representing the `freezers` table.
    Freezer(crate::Freezer),
    /// Variant representing the `organisms` table.
    Organism(crate::Organism),
    /// Variant representing the `physical_assets` table.
    PhysicalAsset(crate::PhysicalAsset),
    /// Variant representing the `pipettes` table.
    Pipette(crate::Pipette),
    /// Variant representing the `positioning_devices` table.
    PositioningDevice(crate::PositioningDevice),
    /// Variant representing the `spectra` table.
    Spectrum(crate::Spectrum),
    /// Variant representing the `spectra_collections` table.
    SpectraCollection(crate::SpectraCollection),
    /// Variant representing the `volume_measuring_devices` table.
    VolumeMeasuringDevice(crate::VolumeMeasuringDevice),
    /// Variant representing the `volumetric_containers` table.
    VolumetricContainer(crate::VolumetricContainer),
    /// Variant representing the `weighing_devices` table.
    WeighingDevice(crate::WeighingDevice),
}
impl From<crate::Asset> for AssetDAG {
    fn from(value: crate::Asset) -> Self {
        AssetDAG::Asset(value)
    }
}
impl From<crate::BallMillMachine> for AssetDAG {
    fn from(value: crate::BallMillMachine) -> Self {
        AssetDAG::BallMillMachine(value)
    }
}
impl From<crate::Camera> for AssetDAG {
    fn from(value: crate::Camera) -> Self {
        AssetDAG::Camera(value)
    }
}
impl From<crate::Centrifuge> for AssetDAG {
    fn from(value: crate::Centrifuge) -> Self {
        AssetDAG::Centrifuge(value)
    }
}
impl From<crate::Container> for AssetDAG {
    fn from(value: crate::Container) -> Self {
        AssetDAG::Container(value)
    }
}
impl From<crate::DigitalAsset> for AssetDAG {
    fn from(value: crate::DigitalAsset) -> Self {
        AssetDAG::DigitalAsset(value)
    }
}
impl From<crate::FreezeDryer> for AssetDAG {
    fn from(value: crate::FreezeDryer) -> Self {
        AssetDAG::FreezeDryer(value)
    }
}
impl From<crate::Freezer> for AssetDAG {
    fn from(value: crate::Freezer) -> Self {
        AssetDAG::Freezer(value)
    }
}
impl From<crate::Organism> for AssetDAG {
    fn from(value: crate::Organism) -> Self {
        AssetDAG::Organism(value)
    }
}
impl From<crate::PhysicalAsset> for AssetDAG {
    fn from(value: crate::PhysicalAsset) -> Self {
        AssetDAG::PhysicalAsset(value)
    }
}
impl From<crate::Pipette> for AssetDAG {
    fn from(value: crate::Pipette) -> Self {
        AssetDAG::Pipette(value)
    }
}
impl From<crate::PositioningDevice> for AssetDAG {
    fn from(value: crate::PositioningDevice) -> Self {
        AssetDAG::PositioningDevice(value)
    }
}
impl From<crate::Spectrum> for AssetDAG {
    fn from(value: crate::Spectrum) -> Self {
        AssetDAG::Spectrum(value)
    }
}
impl From<crate::SpectraCollection> for AssetDAG {
    fn from(value: crate::SpectraCollection) -> Self {
        AssetDAG::SpectraCollection(value)
    }
}
impl From<crate::VolumeMeasuringDevice> for AssetDAG {
    fn from(value: crate::VolumeMeasuringDevice) -> Self {
        AssetDAG::VolumeMeasuringDevice(value)
    }
}
impl From<crate::VolumetricContainer> for AssetDAG {
    fn from(value: crate::VolumetricContainer) -> Self {
        AssetDAG::VolumetricContainer(value)
    }
}
impl From<crate::WeighingDevice> for AssetDAG {
    fn from(value: crate::WeighingDevice) -> Self {
        AssetDAG::WeighingDevice(value)
    }
}
impl<C> web_common_traits::database::MostConcreteVariant<C> for crate::Asset
where
    crate::BallMillMachine: web_common_traits::database::Read<C>,
    crate::Camera: web_common_traits::database::Read<C>,
    crate::Centrifuge: web_common_traits::database::Read<C>,
    crate::Container: web_common_traits::database::Read<C>,
    crate::DigitalAsset: web_common_traits::database::Read<C>,
    crate::FreezeDryer: web_common_traits::database::Read<C>,
    crate::Freezer: web_common_traits::database::Read<C>,
    crate::Organism: web_common_traits::database::Read<C>,
    crate::PhysicalAsset: web_common_traits::database::Read<C>,
    crate::Pipette: web_common_traits::database::Read<C>,
    crate::PositioningDevice: web_common_traits::database::Read<C>,
    crate::Spectrum: web_common_traits::database::Read<C>,
    crate::SpectraCollection: web_common_traits::database::Read<C>,
    crate::VolumeMeasuringDevice: web_common_traits::database::Read<C>,
    crate::VolumetricContainer: web_common_traits::database::Read<C>,
    crate::WeighingDevice: web_common_traits::database::Read<C>,
{
    type Variant = AssetDAG;
    fn most_concrete_variant(&self, conn: &mut C) -> Result<Self::Variant, diesel::result::Error> {
        use diesel::Identifiable;
        Ok(match self.most_concrete_table.as_str() {
            "assets" => self.clone().into(),
            "ball_mill_machines" => {
                <crate::BallMillMachine as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            "cameras" => {
                <crate::Camera as web_common_traits::database::Read<C>>::read(*self.id(), conn)?
                    .into()
            }
            "centrifuges" => {
                <crate::Centrifuge as web_common_traits::database::Read<C>>::read(*self.id(), conn)?
                    .into()
            }
            "containers" => {
                <crate::Container as web_common_traits::database::Read<C>>::read(*self.id(), conn)?
                    .into()
            }
            "digital_assets" => {
                <crate::DigitalAsset as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            "freeze_dryers" => {
                <crate::FreezeDryer as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            "freezers" => {
                <crate::Freezer as web_common_traits::database::Read<C>>::read(*self.id(), conn)?
                    .into()
            }
            "organisms" => {
                <crate::Organism as web_common_traits::database::Read<C>>::read(*self.id(), conn)?
                    .into()
            }
            "physical_assets" => {
                <crate::PhysicalAsset as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            "pipettes" => {
                <crate::Pipette as web_common_traits::database::Read<C>>::read(*self.id(), conn)?
                    .into()
            }
            "positioning_devices" => {
                <crate::PositioningDevice as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            "spectra" => {
                <crate::Spectrum as web_common_traits::database::Read<C>>::read(*self.id(), conn)?
                    .into()
            }
            "spectra_collections" => {
                <crate::SpectraCollection as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            "volume_measuring_devices" => {
                <crate::VolumeMeasuringDevice as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            "volumetric_containers" => {
                <crate::VolumetricContainer as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            "weighing_devices" => {
                <crate::WeighingDevice as web_common_traits::database::Read<C>>::read(
                    *self.id(),
                    conn,
                )?
                .into()
            }
            _ => unreachable!("Database and codegen are out of sync."),
        })
    }
}
