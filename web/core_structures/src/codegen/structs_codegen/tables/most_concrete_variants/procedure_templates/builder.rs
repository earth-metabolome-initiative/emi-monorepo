#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enum representing the `procedure_templates` table builder DAG.
pub enum ProcedureTemplateBuilderDAG {
    ///Builder for the `aliquoting_procedure_templates` table.
    AliquotingProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureTemplateBuilder,
    ),
    ///Builder for the `ball_mill_procedure_templates` table.
    BallMillProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateBuilder,
    ),
    ///Builder for the `capping_procedure_templates` table.
    CappingProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateBuilder,
    ),
    ///Builder for the `centrifuge_procedure_templates` table.
    CentrifugeProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder,
    ),
    ///Builder for the `cleaning_procedure_templates` table.
    CleaningProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedureTemplateBuilder,
    ),
    ///Builder for the `disposal_procedure_templates` table.
    DisposalProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplateBuilder,
    ),
    ///Builder for the `fractioning_procedure_templates` table.
    FractioningProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateBuilder,
    ),
    ///Builder for the `freeze_drying_procedure_templates` table.
    FreezeDryingProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateBuilder,
    ),
    ///Builder for the `freezing_procedure_templates` table.
    FreezingProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateBuilder,
    ),
    ///Builder for the `geolocation_procedure_templates` table.
    GeolocationProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureTemplateBuilder,
    ),
    ///Builder for the `harvesting_procedure_templates` table.
    HarvestingProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplateBuilder,
    ),
    ///Builder for the `packaging_procedure_templates` table.
    PackagingProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureTemplateBuilder,
    ),
    ///Builder for the `photograph_procedure_templates` table.
    PhotographProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateBuilder,
    ),
    ///Builder for the `pouring_procedure_templates` table.
    PouringProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateBuilder,
    ),
    ///Builder for the `ppe_reminder_procedure_templates` table.
    PpeReminderProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertablePpeReminderProcedureTemplateBuilder,
    ),
    ///Builder for the `procedure_templates` table.
    ProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder,
    ),
    ///Builder for the `storage_procedure_templates` table.
    StorageProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureTemplateBuilder,
    ),
    ///Builder for the `supernatant_procedure_templates` table.
    SupernatantProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureTemplateBuilder,
    ),
    ///Builder for the `tagging_procedure_templates` table.
    TaggingProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertableTaggingProcedureTemplateBuilder,
    ),
    ///Builder for the `weighing_procedure_templates` table.
    WeighingProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateBuilder,
    ),
}
impl ProcedureTemplateBuilderDAG {
    /// Returns the type name of the variant contained within the enum.
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::AliquotingProcedureTemplate(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureTemplateBuilder,
                >()
            }
            Self::BallMillProcedureTemplate(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateBuilder,
                >()
            }
            Self::CappingProcedureTemplate(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateBuilder,
                >()
            }
            Self::CentrifugeProcedureTemplate(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder,
                >()
            }
            Self::CleaningProcedureTemplate(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedureTemplateBuilder,
                >()
            }
            Self::DisposalProcedureTemplate(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplateBuilder,
                >()
            }
            Self::FractioningProcedureTemplate(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateBuilder,
                >()
            }
            Self::FreezeDryingProcedureTemplate(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateBuilder,
                >()
            }
            Self::FreezingProcedureTemplate(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateBuilder,
                >()
            }
            Self::GeolocationProcedureTemplate(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureTemplateBuilder,
                >()
            }
            Self::HarvestingProcedureTemplate(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplateBuilder,
                >()
            }
            Self::PackagingProcedureTemplate(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureTemplateBuilder,
                >()
            }
            Self::PhotographProcedureTemplate(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateBuilder,
                >()
            }
            Self::PouringProcedureTemplate(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateBuilder,
                >()
            }
            Self::PpeReminderProcedureTemplate(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertablePpeReminderProcedureTemplateBuilder,
                >()
            }
            Self::ProcedureTemplate(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder,
                >()
            }
            Self::StorageProcedureTemplate(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureTemplateBuilder,
                >()
            }
            Self::SupernatantProcedureTemplate(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureTemplateBuilder,
                >()
            }
            Self::TaggingProcedureTemplate(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableTaggingProcedureTemplateBuilder,
                >()
            }
            Self::WeighingProcedureTemplate(_) => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateBuilder,
                >()
            }
        }
    }
}
impl common_traits::builder::IsCompleteBuilder for ProcedureTemplateBuilderDAG {
    fn is_complete(&self) -> bool {
        match self {
            Self::AliquotingProcedureTemplate(builder) => builder.is_complete(),
            Self::BallMillProcedureTemplate(builder) => builder.is_complete(),
            Self::CappingProcedureTemplate(builder) => builder.is_complete(),
            Self::CentrifugeProcedureTemplate(builder) => builder.is_complete(),
            Self::CleaningProcedureTemplate(builder) => builder.is_complete(),
            Self::DisposalProcedureTemplate(builder) => builder.is_complete(),
            Self::FractioningProcedureTemplate(builder) => builder.is_complete(),
            Self::FreezeDryingProcedureTemplate(builder) => builder.is_complete(),
            Self::FreezingProcedureTemplate(builder) => builder.is_complete(),
            Self::GeolocationProcedureTemplate(builder) => builder.is_complete(),
            Self::HarvestingProcedureTemplate(builder) => builder.is_complete(),
            Self::PackagingProcedureTemplate(builder) => builder.is_complete(),
            Self::PhotographProcedureTemplate(builder) => builder.is_complete(),
            Self::PouringProcedureTemplate(builder) => builder.is_complete(),
            Self::PpeReminderProcedureTemplate(builder) => builder.is_complete(),
            Self::ProcedureTemplate(builder) => builder.is_complete(),
            Self::StorageProcedureTemplate(builder) => builder.is_complete(),
            Self::SupernatantProcedureTemplate(builder) => builder.is_complete(),
            Self::TaggingProcedureTemplate(builder) => builder.is_complete(),
            Self::WeighingProcedureTemplate(builder) => builder.is_complete(),
        }
    }
}
impl web_common_traits::database::DispatchableInsertVariantMetadata
    for ProcedureTemplateBuilderDAG
{
    type Row =
        crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG;
    type Error = crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateInsertErrorDAG;
}
impl<C> web_common_traits::database::DispatchableInsertableVariant<C>
for ProcedureTemplateBuilderDAG
where
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureTemplateBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedureTemplateBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplateBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureTemplateBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplateBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureTemplateBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertablePpeReminderProcedureTemplateBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureTemplateBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureTemplateBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableTaggingProcedureTemplateBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        Ok(
            match self {
                Self::AliquotingProcedureTemplate(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::BallMillProcedureTemplate(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CappingProcedureTemplate(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CentrifugeProcedureTemplate(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::CleaningProcedureTemplate(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::DisposalProcedureTemplate(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::FractioningProcedureTemplate(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::FreezeDryingProcedureTemplate(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::FreezingProcedureTemplate(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::GeolocationProcedureTemplate(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::HarvestingProcedureTemplate(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::PackagingProcedureTemplate(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::PhotographProcedureTemplate(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::PouringProcedureTemplate(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::PpeReminderProcedureTemplate(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::ProcedureTemplate(variant) => variant.insert(user_id, conn)?.into(),
                Self::StorageProcedureTemplate(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::SupernatantProcedureTemplate(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::TaggingProcedureTemplate(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
                Self::WeighingProcedureTemplate(variant) => {
                    variant.insert(user_id, conn)?.into()
                }
            },
        )
    }
}
impl crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable
for ProcedureTemplateBuilderDAG
where
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureTemplateBuilder: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateBuilder: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateBuilder: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedureTemplateBuilder: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplateBuilder: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateBuilder: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateBuilder: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateBuilder: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureTemplateBuilder: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplateBuilder: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureTemplateBuilder: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateBuilder: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateBuilder: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertablePpeReminderProcedureTemplateBuilder: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureTemplateBuilder: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureTemplateBuilder: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableTaggingProcedureTemplateBuilder: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable,
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateBuilder: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable,
{
    type Error = crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateInsertErrorDAG;
    #[inline]
    ///Sets the value of the `public.procedure_templates.name` column.
    fn name<N>(self, name: N) -> Result<Self, Self::Error>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        Ok(
            match self {
                Self::AliquotingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::BallMillProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CappingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CentrifugeProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::CleaningProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::DisposalProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::FractioningProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::FreezeDryingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::FreezingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::GeolocationProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::HarvestingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::PackagingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::PhotographProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::PouringProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::PpeReminderProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePpeReminderProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::ProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::StorageProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::SupernatantProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::TaggingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableTaggingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
                Self::WeighingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                            builder,
                            name,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    ///Sets the value of the `public.procedure_templates.description` column.
    fn description<D>(self, description: D) -> Result<Self, Self::Error>
    where
        D: TryInto<String>,
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>,
    {
        Ok(
            match self {
                Self::AliquotingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::BallMillProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CappingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CentrifugeProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::CleaningProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::DisposalProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::FractioningProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::FreezeDryingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::FreezingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::GeolocationProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::HarvestingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::PackagingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::PhotographProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::PouringProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::PpeReminderProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePpeReminderProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::ProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::StorageProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::SupernatantProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::TaggingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableTaggingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
                Self::WeighingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
                            builder,
                            description,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    ///Sets the value of the `public.procedure_templates.created_by` column.
    fn created_by<CB>(self, created_by: CB) -> Result<Self, Self::Error>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        Ok(
            match self {
                Self::AliquotingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::BallMillProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CappingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CentrifugeProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::CleaningProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::DisposalProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::FractioningProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::FreezeDryingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::FreezingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::GeolocationProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::HarvestingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::PackagingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::PhotographProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::PouringProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::PpeReminderProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePpeReminderProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::ProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::StorageProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::SupernatantProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::TaggingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableTaggingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
                Self::WeighingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
                            builder,
                            created_by,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    ///Sets the value of the `public.procedure_templates.created_at` column.
    fn created_at<CA>(self, created_at: CA) -> Result<Self, Self::Error>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        Ok(
            match self {
                Self::AliquotingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::BallMillProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CappingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CentrifugeProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::CleaningProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::DisposalProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::FractioningProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::FreezeDryingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::FreezingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::GeolocationProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::HarvestingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::PackagingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::PhotographProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::PouringProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::PpeReminderProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePpeReminderProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::ProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::StorageProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::SupernatantProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::TaggingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableTaggingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
                Self::WeighingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
                            builder,
                            created_at,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    ///Sets the value of the `public.procedure_templates.updated_by` column.
    fn updated_by<UB>(self, updated_by: UB) -> Result<Self, Self::Error>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        Ok(
            match self {
                Self::AliquotingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::BallMillProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CappingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CentrifugeProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::CleaningProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::DisposalProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::FractioningProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::FreezeDryingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::FreezingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::GeolocationProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::HarvestingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::PackagingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::PhotographProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::PouringProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::PpeReminderProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePpeReminderProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::ProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::StorageProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::SupernatantProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::TaggingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableTaggingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
                Self::WeighingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
                            builder,
                            updated_by,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    ///Sets the value of the `public.procedure_templates.updated_at` column.
    fn updated_at<UA>(self, updated_at: UA) -> Result<Self, Self::Error>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        Ok(
            match self {
                Self::AliquotingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::BallMillProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CappingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CentrifugeProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::CleaningProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::DisposalProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::FractioningProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::FreezeDryingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::FreezingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::GeolocationProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::HarvestingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::PackagingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::PhotographProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::PouringProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::PpeReminderProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePpeReminderProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::ProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::StorageProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::SupernatantProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::TaggingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableTaggingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
                Self::WeighingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
                            builder,
                            updated_at,
                        )?
                        .into()
                }
            },
        )
    }
    #[inline]
    ///Sets the value of the `public.procedure_templates.deprecated` column.
    fn deprecated<D>(self, deprecated: D) -> Result<Self, Self::Error>
    where
        D: TryInto<bool>,
        validation_errors::SingleFieldError: From<<D as TryInto<bool>>::Error>,
    {
        Ok(
            match self {
                Self::AliquotingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                            builder,
                            deprecated,
                        )?
                        .into()
                }
                Self::BallMillProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                            builder,
                            deprecated,
                        )?
                        .into()
                }
                Self::CappingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                            builder,
                            deprecated,
                        )?
                        .into()
                }
                Self::CentrifugeProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                            builder,
                            deprecated,
                        )?
                        .into()
                }
                Self::CleaningProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                            builder,
                            deprecated,
                        )?
                        .into()
                }
                Self::DisposalProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                            builder,
                            deprecated,
                        )?
                        .into()
                }
                Self::FractioningProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                            builder,
                            deprecated,
                        )?
                        .into()
                }
                Self::FreezeDryingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                            builder,
                            deprecated,
                        )?
                        .into()
                }
                Self::FreezingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                            builder,
                            deprecated,
                        )?
                        .into()
                }
                Self::GeolocationProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                            builder,
                            deprecated,
                        )?
                        .into()
                }
                Self::HarvestingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                            builder,
                            deprecated,
                        )?
                        .into()
                }
                Self::PackagingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                            builder,
                            deprecated,
                        )?
                        .into()
                }
                Self::PhotographProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                            builder,
                            deprecated,
                        )?
                        .into()
                }
                Self::PouringProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                            builder,
                            deprecated,
                        )?
                        .into()
                }
                Self::PpeReminderProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertablePpeReminderProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                            builder,
                            deprecated,
                        )?
                        .into()
                }
                Self::ProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                            builder,
                            deprecated,
                        )?
                        .into()
                }
                Self::StorageProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                            builder,
                            deprecated,
                        )?
                        .into()
                }
                Self::SupernatantProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                            builder,
                            deprecated,
                        )?
                        .into()
                }
                Self::TaggingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableTaggingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                            builder,
                            deprecated,
                        )?
                        .into()
                }
                Self::WeighingProcedureTemplate(builder) => {
                    <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                            builder,
                            deprecated,
                        )?
                        .into()
                }
            },
        )
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::AliquotingProcedureTemplate(value)
    }
}
impl From<ProcedureTemplateBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureTemplateBuilder,
> {
    fn from(value: ProcedureTemplateBuilderDAG) -> Self {
        match value {
            ProcedureTemplateBuilderDAG::AliquotingProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::BallMillProcedureTemplate(value)
    }
}
impl From<ProcedureTemplateBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateBuilder,
> {
    fn from(value: ProcedureTemplateBuilderDAG) -> Self {
        match value {
            ProcedureTemplateBuilderDAG::BallMillProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::CappingProcedureTemplate(value)
    }
}
impl From<ProcedureTemplateBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateBuilder,
> {
    fn from(value: ProcedureTemplateBuilderDAG) -> Self {
        match value {
            ProcedureTemplateBuilderDAG::CappingProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::CentrifugeProcedureTemplate(value)
    }
}
impl From<ProcedureTemplateBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder,
> {
    fn from(value: ProcedureTemplateBuilderDAG) -> Self {
        match value {
            ProcedureTemplateBuilderDAG::CentrifugeProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::CleaningProcedureTemplate(value)
    }
}
impl From<ProcedureTemplateBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedureTemplateBuilder,
> {
    fn from(value: ProcedureTemplateBuilderDAG) -> Self {
        match value {
            ProcedureTemplateBuilderDAG::CleaningProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::DisposalProcedureTemplate(value)
    }
}
impl From<ProcedureTemplateBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplateBuilder,
> {
    fn from(value: ProcedureTemplateBuilderDAG) -> Self {
        match value {
            ProcedureTemplateBuilderDAG::DisposalProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::FractioningProcedureTemplate(value)
    }
}
impl From<ProcedureTemplateBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateBuilder,
> {
    fn from(value: ProcedureTemplateBuilderDAG) -> Self {
        match value {
            ProcedureTemplateBuilderDAG::FractioningProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::FreezeDryingProcedureTemplate(value)
    }
}
impl From<ProcedureTemplateBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateBuilder,
> {
    fn from(value: ProcedureTemplateBuilderDAG) -> Self {
        match value {
            ProcedureTemplateBuilderDAG::FreezeDryingProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::FreezingProcedureTemplate(value)
    }
}
impl From<ProcedureTemplateBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateBuilder,
> {
    fn from(value: ProcedureTemplateBuilderDAG) -> Self {
        match value {
            ProcedureTemplateBuilderDAG::FreezingProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::GeolocationProcedureTemplate(value)
    }
}
impl From<ProcedureTemplateBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureTemplateBuilder,
> {
    fn from(value: ProcedureTemplateBuilderDAG) -> Self {
        match value {
            ProcedureTemplateBuilderDAG::GeolocationProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::HarvestingProcedureTemplate(value)
    }
}
impl From<ProcedureTemplateBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplateBuilder,
> {
    fn from(value: ProcedureTemplateBuilderDAG) -> Self {
        match value {
            ProcedureTemplateBuilderDAG::HarvestingProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::PackagingProcedureTemplate(value)
    }
}
impl From<ProcedureTemplateBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureTemplateBuilder,
> {
    fn from(value: ProcedureTemplateBuilderDAG) -> Self {
        match value {
            ProcedureTemplateBuilderDAG::PackagingProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::PhotographProcedureTemplate(value)
    }
}
impl From<ProcedureTemplateBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateBuilder,
> {
    fn from(value: ProcedureTemplateBuilderDAG) -> Self {
        match value {
            ProcedureTemplateBuilderDAG::PhotographProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::PouringProcedureTemplate(value)
    }
}
impl From<ProcedureTemplateBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateBuilder,
> {
    fn from(value: ProcedureTemplateBuilderDAG) -> Self {
        match value {
            ProcedureTemplateBuilderDAG::PouringProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertablePpeReminderProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertablePpeReminderProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::PpeReminderProcedureTemplate(value)
    }
}
impl From<ProcedureTemplateBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertablePpeReminderProcedureTemplateBuilder,
> {
    fn from(value: ProcedureTemplateBuilderDAG) -> Self {
        match value {
            ProcedureTemplateBuilderDAG::PpeReminderProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder>
    for ProcedureTemplateBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::ProcedureTemplate(value)
    }
}
impl From<ProcedureTemplateBuilderDAG>
    for Option<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder,
    >
{
    fn from(value: ProcedureTemplateBuilderDAG) -> Self {
        match value {
            ProcedureTemplateBuilderDAG::ProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::StorageProcedureTemplate(value)
    }
}
impl From<ProcedureTemplateBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureTemplateBuilder,
> {
    fn from(value: ProcedureTemplateBuilderDAG) -> Self {
        match value {
            ProcedureTemplateBuilderDAG::StorageProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::SupernatantProcedureTemplate(value)
    }
}
impl From<ProcedureTemplateBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureTemplateBuilder,
> {
    fn from(value: ProcedureTemplateBuilderDAG) -> Self {
        match value {
            ProcedureTemplateBuilderDAG::SupernatantProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableTaggingProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableTaggingProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::TaggingProcedureTemplate(value)
    }
}
impl From<ProcedureTemplateBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableTaggingProcedureTemplateBuilder,
> {
    fn from(value: ProcedureTemplateBuilderDAG) -> Self {
        match value {
            ProcedureTemplateBuilderDAG::TaggingProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::WeighingProcedureTemplate(value)
    }
}
impl From<ProcedureTemplateBuilderDAG>
for Option<
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateBuilder,
> {
    fn from(value: ProcedureTemplateBuilderDAG) -> Self {
        match value {
            ProcedureTemplateBuilderDAG::WeighingProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
