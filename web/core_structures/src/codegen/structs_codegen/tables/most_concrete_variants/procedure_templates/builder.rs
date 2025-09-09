#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
    ///Builder for the `weighing_procedure_templates` table.
    WeighingProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateBuilder,
    ),
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
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::BallMillProcedureTemplate(value)
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
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::CentrifugeProcedureTemplate(value)
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
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::FractioningProcedureTemplate(value)
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
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::FreezingProcedureTemplate(value)
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
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::PackagingProcedureTemplate(value)
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
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::PouringProcedureTemplate(value)
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
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::StorageProcedureTemplate(value)
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
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateBuilder,
> for ProcedureTemplateBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateBuilder,
    ) -> Self {
        ProcedureTemplateBuilderDAG::WeighingProcedureTemplate(value)
    }
}
