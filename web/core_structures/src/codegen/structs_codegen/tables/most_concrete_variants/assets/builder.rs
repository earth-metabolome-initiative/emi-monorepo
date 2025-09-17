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
impl AssetBuilderDAG {
    /// Returns the type name of the variant contained within the enum.
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Asset(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder,
                >()
            }
            Self::BallMillMachine(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineBuilder,
                >()
            }
            Self::Camera(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCameraBuilder,
                >()
            }
            Self::Centrifuge(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeBuilder,
                >()
            }
            Self::Container(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder,
                >()
            }
            Self::DigitalAsset(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetBuilder,
                >()
            }
            Self::FreezeDryer(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerBuilder,
                >()
            }
            Self::Freezer(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezerBuilder,
                >()
            }
            Self::Organism(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder,
                >()
            }
            Self::Photograph(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographBuilder,
                >()
            }
            Self::PhysicalAsset(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder,
                >()
            }
            Self::Pipette(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertablePipetteBuilder,
                >()
            }
            Self::PositioningDevice(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceBuilder,
                >()
            }
            Self::SampleSource(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceBuilder,
                >()
            }
            Self::Sample(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableSampleBuilder,
                >()
            }
            Self::Soil(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableSoilBuilder,
                >()
            }
            Self::Spectrum(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumBuilder,
                >()
            }
            Self::SpectraCollection(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder,
                >()
            }
            Self::VolumeMeasuringDevice(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceBuilder,
                >()
            }
            Self::VolumetricContainer(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerBuilder,
                >()
            }
            Self::WeighingDevice(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceBuilder,
                >()
            }
        }
    }
}
impl common_traits::builder::IsCompleteBuilder for AssetBuilderDAG {
    fn is_complete(&self) -> bool {
        match self {
            Self::Asset(builder) => builder.is_complete(),
            Self::BallMillMachine(builder) => builder.is_complete(),
            Self::Camera(builder) => builder.is_complete(),
            Self::Centrifuge(builder) => builder.is_complete(),
            Self::Container(builder) => builder.is_complete(),
            Self::DigitalAsset(builder) => builder.is_complete(),
            Self::FreezeDryer(builder) => builder.is_complete(),
            Self::Freezer(builder) => builder.is_complete(),
            Self::Organism(builder) => builder.is_complete(),
            Self::Photograph(builder) => builder.is_complete(),
            Self::PhysicalAsset(builder) => builder.is_complete(),
            Self::Pipette(builder) => builder.is_complete(),
            Self::PositioningDevice(builder) => builder.is_complete(),
            Self::SampleSource(builder) => builder.is_complete(),
            Self::Sample(builder) => builder.is_complete(),
            Self::Soil(builder) => builder.is_complete(),
            Self::Spectrum(builder) => builder.is_complete(),
            Self::SpectraCollection(builder) => builder.is_complete(),
            Self::VolumeMeasuringDevice(builder) => builder.is_complete(),
            Self::VolumetricContainer(builder) => builder.is_complete(),
            Self::WeighingDevice(builder) => builder.is_complete(),
        }
    }
}
impl web_common_traits::database::DispatchableInsertVariantMetadata for AssetBuilderDAG {
    type Row = crate::codegen::structs_codegen::tables::most_concrete_variants::AssetDAG;
    type Error =
        crate::codegen::structs_codegen::tables::most_concrete_variants::AssetInsertErrorDAG;
}
impl<C> web_common_traits::database::DispatchableInsertableVariant<C> for AssetBuilderDAG
where
    crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableCameraBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezerBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertablePipetteBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableSampleBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableSoilBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        Ok(match self {
            Self::Asset(variant) => variant.insert(user_id, conn)?.into(),
            Self::BallMillMachine(variant) => variant.insert(user_id, conn)?.into(),
            Self::Camera(variant) => variant.insert(user_id, conn)?.into(),
            Self::Centrifuge(variant) => variant.insert(user_id, conn)?.into(),
            Self::Container(variant) => variant.insert(user_id, conn)?.into(),
            Self::DigitalAsset(variant) => variant.insert(user_id, conn)?.into(),
            Self::FreezeDryer(variant) => variant.insert(user_id, conn)?.into(),
            Self::Freezer(variant) => variant.insert(user_id, conn)?.into(),
            Self::Organism(variant) => variant.insert(user_id, conn)?.into(),
            Self::Photograph(variant) => variant.insert(user_id, conn)?.into(),
            Self::PhysicalAsset(variant) => variant.insert(user_id, conn)?.into(),
            Self::Pipette(variant) => variant.insert(user_id, conn)?.into(),
            Self::PositioningDevice(variant) => variant.insert(user_id, conn)?.into(),
            Self::SampleSource(variant) => variant.insert(user_id, conn)?.into(),
            Self::Sample(variant) => variant.insert(user_id, conn)?.into(),
            Self::Soil(variant) => variant.insert(user_id, conn)?.into(),
            Self::Spectrum(variant) => variant.insert(user_id, conn)?.into(),
            Self::SpectraCollection(variant) => variant.insert(user_id, conn)?.into(),
            Self::VolumeMeasuringDevice(variant) => variant.insert(user_id, conn)?.into(),
            Self::VolumetricContainer(variant) => variant.insert(user_id, conn)?.into(),
            Self::WeighingDevice(variant) => variant.insert(user_id, conn)?.into(),
        })
    }
}
impl crate::codegen::structs_codegen::tables::insertables::AssetSettable for AssetBuilderDAG
where
    crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder:
        crate::codegen::structs_codegen::tables::insertables::AssetSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineBuilder:
        crate::codegen::structs_codegen::tables::insertables::AssetSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCameraBuilder:
        crate::codegen::structs_codegen::tables::insertables::AssetSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeBuilder:
        crate::codegen::structs_codegen::tables::insertables::AssetSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder:
        crate::codegen::structs_codegen::tables::insertables::AssetSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetBuilder:
        crate::codegen::structs_codegen::tables::insertables::AssetSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerBuilder:
        crate::codegen::structs_codegen::tables::insertables::AssetSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezerBuilder:
        crate::codegen::structs_codegen::tables::insertables::AssetSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder:
        crate::codegen::structs_codegen::tables::insertables::AssetSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographBuilder:
        crate::codegen::structs_codegen::tables::insertables::AssetSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder:
        crate::codegen::structs_codegen::tables::insertables::AssetSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertablePipetteBuilder:
        crate::codegen::structs_codegen::tables::insertables::AssetSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceBuilder:
        crate::codegen::structs_codegen::tables::insertables::AssetSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceBuilder:
        crate::codegen::structs_codegen::tables::insertables::AssetSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableSampleBuilder:
        crate::codegen::structs_codegen::tables::insertables::AssetSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableSoilBuilder:
        crate::codegen::structs_codegen::tables::insertables::AssetSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumBuilder:
        crate::codegen::structs_codegen::tables::insertables::AssetSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder:
        crate::codegen::structs_codegen::tables::insertables::AssetSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceBuilder:
        crate::codegen::structs_codegen::tables::insertables::AssetSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerBuilder:
        crate::codegen::structs_codegen::tables::insertables::AssetSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceBuilder:
        crate::codegen::structs_codegen::tables::insertables::AssetSettable,
{
    type Error =
        crate::codegen::structs_codegen::tables::most_concrete_variants::AssetInsertErrorDAG;
    #[inline]
    /// Sets the value of the `public.assets.id` column.
    fn id<I>(self, id: I) -> Result<Self, Self::Error>
    where
        I: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
    {
        Ok(
            match self {
                Self::Asset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                            builder,
                            id,
                        )?
                        .into()
                }
                Self::BallMillMachine(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                            builder,
                            id,
                        )?
                        .into()
                }
                Self::Camera(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCameraBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                            builder,
                            id,
                        )?
                        .into()
                }
                Self::Centrifuge(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                            builder,
                            id,
                        )?
                        .into()
                }
                Self::Container(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                            builder,
                            id,
                        )?
                        .into()
                }
                Self::DigitalAsset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                            builder,
                            id,
                        )?
                        .into()
                }
                Self::FreezeDryer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                            builder,
                            id,
                        )?
                        .into()
                }
                Self::Freezer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                            builder,
                            id,
                        )?
                        .into()
                }
                Self::Organism(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                            builder,
                            id,
                        )?
                        .into()
                }
                Self::Photograph(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                            builder,
                            id,
                        )?
                        .into()
                }
                Self::PhysicalAsset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                            builder,
                            id,
                        )?
                        .into()
                }
                Self::Pipette(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePipetteBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                            builder,
                            id,
                        )?
                        .into()
                }
                Self::PositioningDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                            builder,
                            id,
                        )?
                        .into()
                }
                Self::SampleSource(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                            builder,
                            id,
                        )?
                        .into()
                }
                Self::Sample(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                            builder,
                            id,
                        )?
                        .into()
                }
                Self::Soil(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSoilBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                            builder,
                            id,
                        )?
                        .into()
                }
                Self::Spectrum(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                            builder,
                            id,
                        )?
                        .into()
                }
                Self::SpectraCollection(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                            builder,
                            id,
                        )?
                        .into()
                }
                Self::VolumeMeasuringDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                            builder,
                            id,
                        )?
                        .into()
                }
                Self::VolumetricContainer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                            builder,
                            id,
                        )?
                        .into()
                }
                Self::WeighingDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                            builder,
                            id,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    /// Sets the value of the `public.assets.name` column.
    fn name<N>(self, name: N) -> Result<Self, Self::Error>
    where
        N: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<N as TryInto<Option<String>>>::Error>,
    {
        Ok(
            match self {
                Self::Asset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::BallMillMachine(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::Camera(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCameraBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::Centrifuge(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::Container(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::DigitalAsset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::FreezeDryer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::Freezer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::Organism(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::Photograph(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::PhysicalAsset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::Pipette(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePipetteBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::PositioningDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::SampleSource(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::Sample(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::Soil(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSoilBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::Spectrum(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::SpectraCollection(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::VolumeMeasuringDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::VolumetricContainer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::WeighingDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    /// Sets the value of the `public.assets.description` column.
    fn description<D>(self, description: D) -> Result<Self, Self::Error>
    where
        D: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<D as TryInto<Option<String>>>::Error>,
    {
        Ok(
            match self {
                Self::Asset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::BallMillMachine(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::Camera(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCameraBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::Centrifuge(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::Container(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::DigitalAsset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::FreezeDryer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::Freezer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::Organism(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::Photograph(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::PhysicalAsset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::Pipette(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePipetteBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::PositioningDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::SampleSource(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::Sample(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::Soil(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSoilBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::Spectrum(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::SpectraCollection(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::VolumeMeasuringDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::VolumetricContainer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::WeighingDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    /// Sets the value of the `public.assets.model` column.
    fn model<M>(self, model: M) -> Result<Self, Self::Error>
    where
        M: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        Ok(
            match self {
                Self::Asset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::model(
                            builder,
                            model,
                        )?
                        .into()
                }
                Self::BallMillMachine(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::model(
                            builder,
                            model,
                        )?
                        .into()
                }
                Self::Camera(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCameraBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::model(
                            builder,
                            model,
                        )?
                        .into()
                }
                Self::Centrifuge(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::model(
                            builder,
                            model,
                        )?
                        .into()
                }
                Self::Container(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::model(
                            builder,
                            model,
                        )?
                        .into()
                }
                Self::DigitalAsset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::model(
                            builder,
                            model,
                        )?
                        .into()
                }
                Self::FreezeDryer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::model(
                            builder,
                            model,
                        )?
                        .into()
                }
                Self::Freezer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::model(
                            builder,
                            model,
                        )?
                        .into()
                }
                Self::Organism(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::model(
                            builder,
                            model,
                        )?
                        .into()
                }
                Self::Photograph(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::model(
                            builder,
                            model,
                        )?
                        .into()
                }
                Self::PhysicalAsset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::model(
                            builder,
                            model,
                        )?
                        .into()
                }
                Self::Pipette(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePipetteBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::model(
                            builder,
                            model,
                        )?
                        .into()
                }
                Self::PositioningDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::model(
                            builder,
                            model,
                        )?
                        .into()
                }
                Self::SampleSource(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::model(
                            builder,
                            model,
                        )?
                        .into()
                }
                Self::Sample(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::model(
                            builder,
                            model,
                        )?
                        .into()
                }
                Self::Soil(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSoilBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::model(
                            builder,
                            model,
                        )?
                        .into()
                }
                Self::Spectrum(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::model(
                            builder,
                            model,
                        )?
                        .into()
                }
                Self::SpectraCollection(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::model(
                            builder,
                            model,
                        )?
                        .into()
                }
                Self::VolumeMeasuringDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::model(
                            builder,
                            model,
                        )?
                        .into()
                }
                Self::VolumetricContainer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::model(
                            builder,
                            model,
                        )?
                        .into()
                }
                Self::WeighingDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::model(
                            builder,
                            model,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    /// Sets the value of the `public.assets.created_by` column.
    fn created_by<CB>(self, created_by: CB) -> Result<Self, Self::Error>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        Ok(
            match self {
                Self::Asset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::BallMillMachine(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::Camera(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCameraBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::Centrifuge(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::Container(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::DigitalAsset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::FreezeDryer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::Freezer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::Organism(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::Photograph(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::PhysicalAsset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::Pipette(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePipetteBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::PositioningDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::SampleSource(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::Sample(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::Soil(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSoilBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::Spectrum(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::SpectraCollection(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::VolumeMeasuringDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::VolumetricContainer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::WeighingDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    /// Sets the value of the `public.assets.created_at` column.
    fn created_at<CA>(self, created_at: CA) -> Result<Self, Self::Error>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        Ok(
            match self {
                Self::Asset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::BallMillMachine(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::Camera(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCameraBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::Centrifuge(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::Container(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::DigitalAsset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::FreezeDryer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::Freezer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::Organism(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::Photograph(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::PhysicalAsset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::Pipette(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePipetteBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::PositioningDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::SampleSource(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::Sample(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::Soil(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSoilBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::Spectrum(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::SpectraCollection(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::VolumeMeasuringDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::VolumetricContainer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::WeighingDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    /// Sets the value of the `public.assets.updated_by` column.
    fn updated_by<UB>(self, updated_by: UB) -> Result<Self, Self::Error>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        Ok(
            match self {
                Self::Asset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::BallMillMachine(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::Camera(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCameraBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::Centrifuge(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::Container(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::DigitalAsset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::FreezeDryer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::Freezer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::Organism(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::Photograph(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::PhysicalAsset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::Pipette(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePipetteBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::PositioningDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::SampleSource(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::Sample(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::Soil(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSoilBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::Spectrum(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::SpectraCollection(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::VolumeMeasuringDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::VolumetricContainer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::WeighingDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    /// Sets the value of the `public.assets.updated_at` column.
    fn updated_at<UA>(self, updated_at: UA) -> Result<Self, Self::Error>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        Ok(
            match self {
                Self::Asset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::BallMillMachine(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::Camera(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCameraBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::Centrifuge(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::Container(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::DigitalAsset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::FreezeDryer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::Freezer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::Organism(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::Photograph(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::PhysicalAsset(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::Pipette(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePipetteBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::PositioningDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::SampleSource(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::Sample(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSampleBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::Soil(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSoilBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::Spectrum(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::SpectraCollection(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::VolumeMeasuringDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::VolumetricContainer(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::WeighingDevice(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceBuilder as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
            },
        )
    }
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
impl From<AssetBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder>
{
    fn from(value: AssetBuilderDAG) -> Self {
        match value {
            AssetBuilderDAG::Asset(v) => Some(v),
            _ => None,
        }
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
impl From<AssetBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineBuilder,
    >
{
    fn from(value: AssetBuilderDAG) -> Self {
        match value {
            AssetBuilderDAG::BallMillMachine(v) => Some(v),
            _ => None,
        }
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
impl From<AssetBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertableCameraBuilder>
{
    fn from(value: AssetBuilderDAG) -> Self {
        match value {
            AssetBuilderDAG::Camera(v) => Some(v),
            _ => None,
        }
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
impl From<AssetBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeBuilder>
{
    fn from(value: AssetBuilderDAG) -> Self {
        match value {
            AssetBuilderDAG::Centrifuge(v) => Some(v),
            _ => None,
        }
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
impl From<AssetBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder>
{
    fn from(value: AssetBuilderDAG) -> Self {
        match value {
            AssetBuilderDAG::Container(v) => Some(v),
            _ => None,
        }
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
impl From<AssetBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetBuilder>
{
    fn from(value: AssetBuilderDAG) -> Self {
        match value {
            AssetBuilderDAG::DigitalAsset(v) => Some(v),
            _ => None,
        }
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
impl From<AssetBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerBuilder>
{
    fn from(value: AssetBuilderDAG) -> Self {
        match value {
            AssetBuilderDAG::FreezeDryer(v) => Some(v),
            _ => None,
        }
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
impl From<AssetBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertableFreezerBuilder>
{
    fn from(value: AssetBuilderDAG) -> Self {
        match value {
            AssetBuilderDAG::Freezer(v) => Some(v),
            _ => None,
        }
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
impl From<AssetBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder>
{
    fn from(value: AssetBuilderDAG) -> Self {
        match value {
            AssetBuilderDAG::Organism(v) => Some(v),
            _ => None,
        }
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
impl From<AssetBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertablePhotographBuilder>
{
    fn from(value: AssetBuilderDAG) -> Self {
        match value {
            AssetBuilderDAG::Photograph(v) => Some(v),
            _ => None,
        }
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
impl From<AssetBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder>
{
    fn from(value: AssetBuilderDAG) -> Self {
        match value {
            AssetBuilderDAG::PhysicalAsset(v) => Some(v),
            _ => None,
        }
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
impl From<AssetBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertablePipetteBuilder>
{
    fn from(value: AssetBuilderDAG) -> Self {
        match value {
            AssetBuilderDAG::Pipette(v) => Some(v),
            _ => None,
        }
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
impl From<AssetBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceBuilder,
    >
{
    fn from(value: AssetBuilderDAG) -> Self {
        match value {
            AssetBuilderDAG::PositioningDevice(v) => Some(v),
            _ => None,
        }
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
impl From<AssetBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceBuilder>
{
    fn from(value: AssetBuilderDAG) -> Self {
        match value {
            AssetBuilderDAG::SampleSource(v) => Some(v),
            _ => None,
        }
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
impl From<AssetBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertableSampleBuilder>
{
    fn from(value: AssetBuilderDAG) -> Self {
        match value {
            AssetBuilderDAG::Sample(v) => Some(v),
            _ => None,
        }
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
impl From<AssetBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertableSoilBuilder>
{
    fn from(value: AssetBuilderDAG) -> Self {
        match value {
            AssetBuilderDAG::Soil(v) => Some(v),
            _ => None,
        }
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
impl From<AssetBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumBuilder>
{
    fn from(value: AssetBuilderDAG) -> Self {
        match value {
            AssetBuilderDAG::Spectrum(v) => Some(v),
            _ => None,
        }
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
impl From<AssetBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder,
    >
{
    fn from(value: AssetBuilderDAG) -> Self {
        match value {
            AssetBuilderDAG::SpectraCollection(v) => Some(v),
            _ => None,
        }
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
impl From<AssetBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceBuilder,
> {
    fn from(value: AssetBuilderDAG) -> Self {
        match value {
            AssetBuilderDAG::VolumeMeasuringDevice(v) => Some(v),
            _ => None,
        }
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
impl From<AssetBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerBuilder,
    >
{
    fn from(value: AssetBuilderDAG) -> Self {
        match value {
            AssetBuilderDAG::VolumetricContainer(v) => Some(v),
            _ => None,
        }
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
impl From<AssetBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceBuilder,
    >
{
    fn from(value: AssetBuilderDAG) -> Self {
        match value {
            AssetBuilderDAG::WeighingDevice(v) => Some(v),
            _ => None,
        }
    }
}
