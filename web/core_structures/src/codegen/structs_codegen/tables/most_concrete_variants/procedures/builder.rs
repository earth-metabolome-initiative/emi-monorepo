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
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureBuilder>
    for ProcedureBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::BallMillProcedure(value)
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
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder>
    for ProcedureBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::DisposalProcedure(value)
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
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder,
> for ProcedureBuilderDAG {
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::FreezeDryingProcedure(value)
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
impl From<crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder>
    for ProcedureBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::PackagingProcedure(value)
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
impl From<crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder>
    for ProcedureBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::PouringProcedure(value)
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
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureBuilder>
    for ProcedureBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::StorageProcedure(value)
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
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder>
    for ProcedureBuilderDAG
{
    fn from(
        value: crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder,
    ) -> Self {
        ProcedureBuilderDAG::WeighingProcedure(value)
    }
}
