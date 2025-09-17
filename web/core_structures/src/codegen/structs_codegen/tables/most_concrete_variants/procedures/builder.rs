#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enum representing the `procedures` table builder DAG.
pub enum ProcedureBuilderDAG {
    ///Builder for the `aliquoting_procedures` table.
    AliquotingProcedure(
        crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureBuilder,
    ),
    ///Builder for the `ball_mill_procedures` table.
    BallMillProcedure(
        crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureBuilder,
    ),
    ///Builder for the `capping_procedures` table.
    CappingProcedure(
        crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureBuilder,
    ),
    ///Builder for the `centrifuge_procedures` table.
    CentrifugeProcedure(
        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder,
    ),
    ///Builder for the `disposal_procedures` table.
    DisposalProcedure(
        crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder,
    ),
    ///Builder for the `fractioning_procedures` table.
    FractioningProcedure(
        crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureBuilder,
    ),
    ///Builder for the `freeze_drying_procedures` table.
    FreezeDryingProcedure(
        crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder,
    ),
    ///Builder for the `freezing_procedures` table.
    FreezingProcedure(
        crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureBuilder,
    ),
    ///Builder for the `geolocation_procedures` table.
    GeolocationProcedure(
        crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder,
    ),
    ///Builder for the `harvesting_procedures` table.
    HarvestingProcedure(
        crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder,
    ),
    ///Builder for the `packaging_procedures` table.
    PackagingProcedure(
        crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder,
    ),
    ///Builder for the `photograph_procedures` table.
    PhotographProcedure(
        crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder,
    ),
    ///Builder for the `pouring_procedures` table.
    PouringProcedure(
        crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder,
    ),
    ///Builder for the `procedures` table.
    Procedure(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
    ),
    ///Builder for the `storage_procedures` table.
    StorageProcedure(
        crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureBuilder,
    ),
    ///Builder for the `supernatant_procedures` table.
    SupernatantProcedure(
        crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder,
    ),
    ///Builder for the `weighing_procedures` table.
    WeighingProcedure(
        crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder,
    ),
}
impl ProcedureBuilderDAG {
    /// Returns the type name of the variant contained within the enum.
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::AliquotingProcedure(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureBuilder,
                >()
            }
            Self::BallMillProcedure(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureBuilder,
                >()
            }
            Self::CappingProcedure(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureBuilder,
                >()
            }
            Self::CentrifugeProcedure(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder,
                >()
            }
            Self::DisposalProcedure(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder,
                >()
            }
            Self::FractioningProcedure(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureBuilder,
                >()
            }
            Self::FreezeDryingProcedure(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder,
                >()
            }
            Self::FreezingProcedure(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureBuilder,
                >()
            }
            Self::GeolocationProcedure(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder,
                >()
            }
            Self::HarvestingProcedure(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder,
                >()
            }
            Self::PackagingProcedure(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder,
                >()
            }
            Self::PhotographProcedure(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder,
                >()
            }
            Self::PouringProcedure(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder,
                >()
            }
            Self::Procedure(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
                >()
            }
            Self::StorageProcedure(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureBuilder,
                >()
            }
            Self::SupernatantProcedure(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder,
                >()
            }
            Self::WeighingProcedure(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder,
                >()
            }
        }
    }
}
impl common_traits::builder::IsCompleteBuilder for ProcedureBuilderDAG {
    fn is_complete(&self) -> bool {
        match self {
            Self::AliquotingProcedure(builder) => builder.is_complete(),
            Self::BallMillProcedure(builder) => builder.is_complete(),
            Self::CappingProcedure(builder) => builder.is_complete(),
            Self::CentrifugeProcedure(builder) => builder.is_complete(),
            Self::DisposalProcedure(builder) => builder.is_complete(),
            Self::FractioningProcedure(builder) => builder.is_complete(),
            Self::FreezeDryingProcedure(builder) => builder.is_complete(),
            Self::FreezingProcedure(builder) => builder.is_complete(),
            Self::GeolocationProcedure(builder) => builder.is_complete(),
            Self::HarvestingProcedure(builder) => builder.is_complete(),
            Self::PackagingProcedure(builder) => builder.is_complete(),
            Self::PhotographProcedure(builder) => builder.is_complete(),
            Self::PouringProcedure(builder) => builder.is_complete(),
            Self::Procedure(builder) => builder.is_complete(),
            Self::StorageProcedure(builder) => builder.is_complete(),
            Self::SupernatantProcedure(builder) => builder.is_complete(),
            Self::WeighingProcedure(builder) => builder.is_complete(),
        }
    }
}
impl web_common_traits::database::DispatchableInsertVariantMetadata for ProcedureBuilderDAG {
    type Row = crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureDAG;
    type Error =
        crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureInsertErrorDAG;
}
impl<C> web_common_traits::database::DispatchableInsertableVariant<C> for ProcedureBuilderDAG
where
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder:
        web_common_traits::database::DispatchableInsertableVariant<C>,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        Ok(match self {
            Self::AliquotingProcedure(variant) => variant.insert(user_id, conn)?.into(),
            Self::BallMillProcedure(variant) => variant.insert(user_id, conn)?.into(),
            Self::CappingProcedure(variant) => variant.insert(user_id, conn)?.into(),
            Self::CentrifugeProcedure(variant) => variant.insert(user_id, conn)?.into(),
            Self::DisposalProcedure(variant) => variant.insert(user_id, conn)?.into(),
            Self::FractioningProcedure(variant) => variant.insert(user_id, conn)?.into(),
            Self::FreezeDryingProcedure(variant) => variant.insert(user_id, conn)?.into(),
            Self::FreezingProcedure(variant) => variant.insert(user_id, conn)?.into(),
            Self::GeolocationProcedure(variant) => variant.insert(user_id, conn)?.into(),
            Self::HarvestingProcedure(variant) => variant.insert(user_id, conn)?.into(),
            Self::PackagingProcedure(variant) => variant.insert(user_id, conn)?.into(),
            Self::PhotographProcedure(variant) => variant.insert(user_id, conn)?.into(),
            Self::PouringProcedure(variant) => variant.insert(user_id, conn)?.into(),
            Self::Procedure(variant) => variant.insert(user_id, conn)?.into(),
            Self::StorageProcedure(variant) => variant.insert(user_id, conn)?.into(),
            Self::SupernatantProcedure(variant) => variant.insert(user_id, conn)?.into(),
            Self::WeighingProcedure(variant) => variant.insert(user_id, conn)?.into(),
        })
    }
}
impl crate::codegen::structs_codegen::tables::insertables::ProcedureSettable for ProcedureBuilderDAG
where
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureBuilder:
        crate::codegen::structs_codegen::tables::insertables::ProcedureSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureBuilder:
        crate::codegen::structs_codegen::tables::insertables::ProcedureSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureBuilder:
        crate::codegen::structs_codegen::tables::insertables::ProcedureSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder:
        crate::codegen::structs_codegen::tables::insertables::ProcedureSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder:
        crate::codegen::structs_codegen::tables::insertables::ProcedureSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureBuilder:
        crate::codegen::structs_codegen::tables::insertables::ProcedureSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder:
        crate::codegen::structs_codegen::tables::insertables::ProcedureSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureBuilder:
        crate::codegen::structs_codegen::tables::insertables::ProcedureSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder:
        crate::codegen::structs_codegen::tables::insertables::ProcedureSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder:
        crate::codegen::structs_codegen::tables::insertables::ProcedureSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder:
        crate::codegen::structs_codegen::tables::insertables::ProcedureSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder:
        crate::codegen::structs_codegen::tables::insertables::ProcedureSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder:
        crate::codegen::structs_codegen::tables::insertables::ProcedureSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder:
        crate::codegen::structs_codegen::tables::insertables::ProcedureSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureBuilder:
        crate::codegen::structs_codegen::tables::insertables::ProcedureSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder:
        crate::codegen::structs_codegen::tables::insertables::ProcedureSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder:
        crate::codegen::structs_codegen::tables::insertables::ProcedureSettable,
{
    type Error =
        crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureInsertErrorDAG;
    #[inline]
    /// Sets the value of the `public.procedures.procedure` column.
    fn procedure<P>(self, procedure: P) -> Result<Self, Self::Error>
    where
        P: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
    {
        Ok(
            match self {
                Self::AliquotingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure(
                            builder,
                            procedure,
                        )?
                        .into()
                }
                Self::BallMillProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure(
                            builder,
                            procedure,
                        )?
                        .into()
                }
                Self::CappingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure(
                            builder,
                            procedure,
                        )?
                        .into()
                }
                Self::CentrifugeProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure(
                            builder,
                            procedure,
                        )?
                        .into()
                }
                Self::DisposalProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure(
                            builder,
                            procedure,
                        )?
                        .into()
                }
                Self::FractioningProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure(
                            builder,
                            procedure,
                        )?
                        .into()
                }
                Self::FreezeDryingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure(
                            builder,
                            procedure,
                        )?
                        .into()
                }
                Self::FreezingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure(
                            builder,
                            procedure,
                        )?
                        .into()
                }
                Self::GeolocationProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure(
                            builder,
                            procedure,
                        )?
                        .into()
                }
                Self::HarvestingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure(
                            builder,
                            procedure,
                        )?
                        .into()
                }
                Self::PackagingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure(
                            builder,
                            procedure,
                        )?
                        .into()
                }
                Self::PhotographProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure(
                            builder,
                            procedure,
                        )?
                        .into()
                }
                Self::PouringProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure(
                            builder,
                            procedure,
                        )?
                        .into()
                }
                Self::Procedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure(
                            builder,
                            procedure,
                        )?
                        .into()
                }
                Self::StorageProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure(
                            builder,
                            procedure,
                        )?
                        .into()
                }
                Self::SupernatantProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure(
                            builder,
                            procedure,
                        )?
                        .into()
                }
                Self::WeighingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure(
                            builder,
                            procedure,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    /// Sets the value of the `public.procedures.procedure_template` column.
    fn procedure_template<PT>(self, procedure_template: PT) -> Result<Self, Self::Error>
    where
        PT: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        Ok(
            match self {
                Self::AliquotingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure_template(
                            builder,
                            procedure_template,
                        )?
                        .into()
                }
                Self::BallMillProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure_template(
                            builder,
                            procedure_template,
                        )?
                        .into()
                }
                Self::CappingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure_template(
                            builder,
                            procedure_template,
                        )?
                        .into()
                }
                Self::CentrifugeProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure_template(
                            builder,
                            procedure_template,
                        )?
                        .into()
                }
                Self::DisposalProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure_template(
                            builder,
                            procedure_template,
                        )?
                        .into()
                }
                Self::FractioningProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure_template(
                            builder,
                            procedure_template,
                        )?
                        .into()
                }
                Self::FreezeDryingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure_template(
                            builder,
                            procedure_template,
                        )?
                        .into()
                }
                Self::FreezingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure_template(
                            builder,
                            procedure_template,
                        )?
                        .into()
                }
                Self::GeolocationProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure_template(
                            builder,
                            procedure_template,
                        )?
                        .into()
                }
                Self::HarvestingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure_template(
                            builder,
                            procedure_template,
                        )?
                        .into()
                }
                Self::PackagingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure_template(
                            builder,
                            procedure_template,
                        )?
                        .into()
                }
                Self::PhotographProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure_template(
                            builder,
                            procedure_template,
                        )?
                        .into()
                }
                Self::PouringProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure_template(
                            builder,
                            procedure_template,
                        )?
                        .into()
                }
                Self::Procedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure_template(
                            builder,
                            procedure_template,
                        )?
                        .into()
                }
                Self::StorageProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure_template(
                            builder,
                            procedure_template,
                        )?
                        .into()
                }
                Self::SupernatantProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure_template(
                            builder,
                            procedure_template,
                        )?
                        .into()
                }
                Self::WeighingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure_template(
                            builder,
                            procedure_template,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    /// Sets the value of the `public.procedures.parent_procedure` column.
    fn parent_procedure<PP>(self, parent_procedure: PP) -> Result<Self, Self::Error>
    where
        PP: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
    {
        Ok(
            match self {
                Self::AliquotingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure(
                            builder,
                            parent_procedure,
                        )?
                        .into()
                }
                Self::BallMillProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure(
                            builder,
                            parent_procedure,
                        )?
                        .into()
                }
                Self::CappingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure(
                            builder,
                            parent_procedure,
                        )?
                        .into()
                }
                Self::CentrifugeProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure(
                            builder,
                            parent_procedure,
                        )?
                        .into()
                }
                Self::DisposalProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure(
                            builder,
                            parent_procedure,
                        )?
                        .into()
                }
                Self::FractioningProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure(
                            builder,
                            parent_procedure,
                        )?
                        .into()
                }
                Self::FreezeDryingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure(
                            builder,
                            parent_procedure,
                        )?
                        .into()
                }
                Self::FreezingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure(
                            builder,
                            parent_procedure,
                        )?
                        .into()
                }
                Self::GeolocationProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure(
                            builder,
                            parent_procedure,
                        )?
                        .into()
                }
                Self::HarvestingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure(
                            builder,
                            parent_procedure,
                        )?
                        .into()
                }
                Self::PackagingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure(
                            builder,
                            parent_procedure,
                        )?
                        .into()
                }
                Self::PhotographProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure(
                            builder,
                            parent_procedure,
                        )?
                        .into()
                }
                Self::PouringProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure(
                            builder,
                            parent_procedure,
                        )?
                        .into()
                }
                Self::Procedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure(
                            builder,
                            parent_procedure,
                        )?
                        .into()
                }
                Self::StorageProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure(
                            builder,
                            parent_procedure,
                        )?
                        .into()
                }
                Self::SupernatantProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure(
                            builder,
                            parent_procedure,
                        )?
                        .into()
                }
                Self::WeighingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure(
                            builder,
                            parent_procedure,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    /// Sets the value of the `public.procedures.parent_procedure_template`
    /// column.
    fn parent_procedure_template<PPT>(
        self,
        parent_procedure_template: PPT,
    ) -> Result<Self, Self::Error>
    where
        PPT: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
        Ok(
            match self {
                Self::AliquotingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure_template(
                            builder,
                            parent_procedure_template,
                        )?
                        .into()
                }
                Self::BallMillProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure_template(
                            builder,
                            parent_procedure_template,
                        )?
                        .into()
                }
                Self::CappingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure_template(
                            builder,
                            parent_procedure_template,
                        )?
                        .into()
                }
                Self::CentrifugeProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure_template(
                            builder,
                            parent_procedure_template,
                        )?
                        .into()
                }
                Self::DisposalProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure_template(
                            builder,
                            parent_procedure_template,
                        )?
                        .into()
                }
                Self::FractioningProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure_template(
                            builder,
                            parent_procedure_template,
                        )?
                        .into()
                }
                Self::FreezeDryingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure_template(
                            builder,
                            parent_procedure_template,
                        )?
                        .into()
                }
                Self::FreezingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure_template(
                            builder,
                            parent_procedure_template,
                        )?
                        .into()
                }
                Self::GeolocationProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure_template(
                            builder,
                            parent_procedure_template,
                        )?
                        .into()
                }
                Self::HarvestingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure_template(
                            builder,
                            parent_procedure_template,
                        )?
                        .into()
                }
                Self::PackagingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure_template(
                            builder,
                            parent_procedure_template,
                        )?
                        .into()
                }
                Self::PhotographProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure_template(
                            builder,
                            parent_procedure_template,
                        )?
                        .into()
                }
                Self::PouringProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure_template(
                            builder,
                            parent_procedure_template,
                        )?
                        .into()
                }
                Self::Procedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure_template(
                            builder,
                            parent_procedure_template,
                        )?
                        .into()
                }
                Self::StorageProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure_template(
                            builder,
                            parent_procedure_template,
                        )?
                        .into()
                }
                Self::SupernatantProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure_template(
                            builder,
                            parent_procedure_template,
                        )?
                        .into()
                }
                Self::WeighingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure_template(
                            builder,
                            parent_procedure_template,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    /// Sets the value of the `public.procedures.predecessor_procedure` column.
    fn predecessor_procedure<PP>(self, predecessor_procedure: PP) -> Result<Self, Self::Error>
    where
        PP: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
    {
        Ok(
            match self {
                Self::AliquotingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure(
                            builder,
                            predecessor_procedure,
                        )?
                        .into()
                }
                Self::BallMillProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure(
                            builder,
                            predecessor_procedure,
                        )?
                        .into()
                }
                Self::CappingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure(
                            builder,
                            predecessor_procedure,
                        )?
                        .into()
                }
                Self::CentrifugeProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure(
                            builder,
                            predecessor_procedure,
                        )?
                        .into()
                }
                Self::DisposalProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure(
                            builder,
                            predecessor_procedure,
                        )?
                        .into()
                }
                Self::FractioningProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure(
                            builder,
                            predecessor_procedure,
                        )?
                        .into()
                }
                Self::FreezeDryingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure(
                            builder,
                            predecessor_procedure,
                        )?
                        .into()
                }
                Self::FreezingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure(
                            builder,
                            predecessor_procedure,
                        )?
                        .into()
                }
                Self::GeolocationProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure(
                            builder,
                            predecessor_procedure,
                        )?
                        .into()
                }
                Self::HarvestingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure(
                            builder,
                            predecessor_procedure,
                        )?
                        .into()
                }
                Self::PackagingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure(
                            builder,
                            predecessor_procedure,
                        )?
                        .into()
                }
                Self::PhotographProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure(
                            builder,
                            predecessor_procedure,
                        )?
                        .into()
                }
                Self::PouringProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure(
                            builder,
                            predecessor_procedure,
                        )?
                        .into()
                }
                Self::Procedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure(
                            builder,
                            predecessor_procedure,
                        )?
                        .into()
                }
                Self::StorageProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure(
                            builder,
                            predecessor_procedure,
                        )?
                        .into()
                }
                Self::SupernatantProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure(
                            builder,
                            predecessor_procedure,
                        )?
                        .into()
                }
                Self::WeighingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure(
                            builder,
                            predecessor_procedure,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    /// Sets the value of the `public.procedures.predecessor_procedure_template`
    /// column.
    fn predecessor_procedure_template<PPT>(
        self,
        predecessor_procedure_template: PPT,
    ) -> Result<Self, Self::Error>
    where
        PPT: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
        Ok(
            match self {
                Self::AliquotingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure_template(
                            builder,
                            predecessor_procedure_template,
                        )?
                        .into()
                }
                Self::BallMillProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure_template(
                            builder,
                            predecessor_procedure_template,
                        )?
                        .into()
                }
                Self::CappingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure_template(
                            builder,
                            predecessor_procedure_template,
                        )?
                        .into()
                }
                Self::CentrifugeProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure_template(
                            builder,
                            predecessor_procedure_template,
                        )?
                        .into()
                }
                Self::DisposalProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure_template(
                            builder,
                            predecessor_procedure_template,
                        )?
                        .into()
                }
                Self::FractioningProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure_template(
                            builder,
                            predecessor_procedure_template,
                        )?
                        .into()
                }
                Self::FreezeDryingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure_template(
                            builder,
                            predecessor_procedure_template,
                        )?
                        .into()
                }
                Self::FreezingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure_template(
                            builder,
                            predecessor_procedure_template,
                        )?
                        .into()
                }
                Self::GeolocationProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure_template(
                            builder,
                            predecessor_procedure_template,
                        )?
                        .into()
                }
                Self::HarvestingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure_template(
                            builder,
                            predecessor_procedure_template,
                        )?
                        .into()
                }
                Self::PackagingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure_template(
                            builder,
                            predecessor_procedure_template,
                        )?
                        .into()
                }
                Self::PhotographProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure_template(
                            builder,
                            predecessor_procedure_template,
                        )?
                        .into()
                }
                Self::PouringProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure_template(
                            builder,
                            predecessor_procedure_template,
                        )?
                        .into()
                }
                Self::Procedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure_template(
                            builder,
                            predecessor_procedure_template,
                        )?
                        .into()
                }
                Self::StorageProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure_template(
                            builder,
                            predecessor_procedure_template,
                        )?
                        .into()
                }
                Self::SupernatantProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure_template(
                            builder,
                            predecessor_procedure_template,
                        )?
                        .into()
                }
                Self::WeighingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure_template(
                            builder,
                            predecessor_procedure_template,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    /// Sets the value of the `public.procedures.created_by` column.
    fn created_by<CB>(self, created_by: CB) -> Result<Self, Self::Error>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        Ok(
            match self {
                Self::AliquotingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::BallMillProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CappingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CentrifugeProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::DisposalProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::FractioningProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::FreezeDryingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::FreezingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::GeolocationProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::HarvestingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::PackagingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::PhotographProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::PouringProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::Procedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::StorageProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::SupernatantProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::WeighingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    /// Sets the value of the `public.procedures.created_at` column.
    fn created_at<CA>(self, created_at: CA) -> Result<Self, Self::Error>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        Ok(
            match self {
                Self::AliquotingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::BallMillProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CappingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CentrifugeProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::DisposalProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::FractioningProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::FreezeDryingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::FreezingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::GeolocationProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::HarvestingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::PackagingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::PhotographProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::PouringProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::Procedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::StorageProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::SupernatantProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::WeighingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    /// Sets the value of the `public.procedures.updated_by` column.
    fn updated_by<UB>(self, updated_by: UB) -> Result<Self, Self::Error>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        Ok(
            match self {
                Self::AliquotingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::BallMillProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CappingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CentrifugeProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::DisposalProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::FractioningProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::FreezeDryingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::FreezingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::GeolocationProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::HarvestingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::PackagingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::PhotographProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::PouringProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::Procedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::StorageProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::SupernatantProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::WeighingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    /// Sets the value of the `public.procedures.updated_at` column.
    fn updated_at<UA>(self, updated_at: UA) -> Result<Self, Self::Error>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        Ok(
            match self {
                Self::AliquotingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::BallMillProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CappingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CentrifugeProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::DisposalProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::FractioningProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::FreezeDryingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::FreezingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::GeolocationProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::HarvestingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::PackagingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::PhotographProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::PouringProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::Procedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::StorageProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::SupernatantProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::WeighingProcedure(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
            },
        )
    }
}
impl
    From<crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureBuilder>
    for ProcedureBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::AliquotingProcedure(value)
    }
}
impl From<ProcedureBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureBuilder,
    >
{
    fn from(value: ProcedureBuilderDAG) -> Self {
        match value {
            ProcedureBuilderDAG::AliquotingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureBuilder>
    for ProcedureBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::BallMillProcedure(value)
    }
}
impl From<ProcedureBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureBuilder,
    >
{
    fn from(value: ProcedureBuilderDAG) -> Self {
        match value {
            ProcedureBuilderDAG::BallMillProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureBuilder>
    for ProcedureBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::CappingProcedure(value)
    }
}
impl From<ProcedureBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureBuilder,
    >
{
    fn from(value: ProcedureBuilderDAG) -> Self {
        match value {
            ProcedureBuilderDAG::CappingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl
    From<crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder>
    for ProcedureBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::CentrifugeProcedure(value)
    }
}
impl From<ProcedureBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder,
    >
{
    fn from(value: ProcedureBuilderDAG) -> Self {
        match value {
            ProcedureBuilderDAG::CentrifugeProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder>
    for ProcedureBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::DisposalProcedure(value)
    }
}
impl From<ProcedureBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder,
    >
{
    fn from(value: ProcedureBuilderDAG) -> Self {
        match value {
            ProcedureBuilderDAG::DisposalProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl
    From<
        crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureBuilder,
    > for ProcedureBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::FractioningProcedure(value)
    }
}
impl From<ProcedureBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureBuilder,
    >
{
    fn from(value: ProcedureBuilderDAG) -> Self {
        match value {
            ProcedureBuilderDAG::FractioningProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder,
> for ProcedureBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::FreezeDryingProcedure(value)
    }
}
impl From<ProcedureBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder,
> {
    fn from(value: ProcedureBuilderDAG) -> Self {
        match value {
            ProcedureBuilderDAG::FreezeDryingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureBuilder>
    for ProcedureBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::FreezingProcedure(value)
    }
}
impl From<ProcedureBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureBuilder,
    >
{
    fn from(value: ProcedureBuilderDAG) -> Self {
        match value {
            ProcedureBuilderDAG::FreezingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl
    From<
        crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder,
    > for ProcedureBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::GeolocationProcedure(value)
    }
}
impl From<ProcedureBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder,
    >
{
    fn from(value: ProcedureBuilderDAG) -> Self {
        match value {
            ProcedureBuilderDAG::GeolocationProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl
    From<crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder>
    for ProcedureBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::HarvestingProcedure(value)
    }
}
impl From<ProcedureBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder,
    >
{
    fn from(value: ProcedureBuilderDAG) -> Self {
        match value {
            ProcedureBuilderDAG::HarvestingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder>
    for ProcedureBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::PackagingProcedure(value)
    }
}
impl From<ProcedureBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder,
    >
{
    fn from(value: ProcedureBuilderDAG) -> Self {
        match value {
            ProcedureBuilderDAG::PackagingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl
    From<crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder>
    for ProcedureBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::PhotographProcedure(value)
    }
}
impl From<ProcedureBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder,
    >
{
    fn from(value: ProcedureBuilderDAG) -> Self {
        match value {
            ProcedureBuilderDAG::PhotographProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder>
    for ProcedureBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::PouringProcedure(value)
    }
}
impl From<ProcedureBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder,
    >
{
    fn from(value: ProcedureBuilderDAG) -> Self {
        match value {
            ProcedureBuilderDAG::PouringProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder>
    for ProcedureBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::Procedure(value)
    }
}
impl From<ProcedureBuilderDAG>
    for Option<crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder>
{
    fn from(value: ProcedureBuilderDAG) -> Self {
        match value {
            ProcedureBuilderDAG::Procedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureBuilder>
    for ProcedureBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::StorageProcedure(value)
    }
}
impl From<ProcedureBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureBuilder,
    >
{
    fn from(value: ProcedureBuilderDAG) -> Self {
        match value {
            ProcedureBuilderDAG::StorageProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl
    From<
        crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder,
    > for ProcedureBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::SupernatantProcedure(value)
    }
}
impl From<ProcedureBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder,
    >
{
    fn from(value: ProcedureBuilderDAG) -> Self {
        match value {
            ProcedureBuilderDAG::SupernatantProcedure(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder>
    for ProcedureBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::WeighingProcedure(value)
    }
}
impl From<ProcedureBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder,
    >
{
    fn from(value: ProcedureBuilderDAG) -> Self {
        match value {
            ProcedureBuilderDAG::WeighingProcedure(v) => Some(v),
            _ => None,
        }
    }
}
