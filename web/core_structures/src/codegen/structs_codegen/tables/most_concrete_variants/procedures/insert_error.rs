#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enum representing insert errors which may occur when inserting an entry from
/// the `procedures` table DAG.
pub enum ProcedureInsertErrorDAG {
    /// Insert error associated with the `aliquoting_procedures` table.
    AliquotingProcedure(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute,
        >,
    ),
    /// Insert error associated with the `ball_mill_procedures` table.
    BallMillProcedure(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute,
        >,
    ),
    /// Insert error associated with the `capping_procedures` table.
    CappingProcedure(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CappingProcedureAttribute,
        >,
    ),
    /// Insert error associated with the `centrifuge_procedures` table.
    CentrifugeProcedure(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureAttribute,
        >,
    ),
    /// Insert error associated with the `disposal_procedures` table.
    DisposalProcedure(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::DisposalProcedureAttribute,
        >,
    ),
    /// Insert error associated with the `fractioning_procedures` table.
    FractioningProcedure(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute,
        >,
    ),
    /// Insert error associated with the `freeze_drying_procedures` table.
    FreezeDryingProcedure(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureAttribute,
        >,
    ),
    /// Insert error associated with the `freezing_procedures` table.
    FreezingProcedure(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezingProcedureAttribute,
        >,
    ),
    /// Insert error associated with the `geolocation_procedures` table.
    GeolocationProcedure(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureAttribute,
        >,
    ),
    /// Insert error associated with the `harvesting_procedures` table.
    HarvestingProcedure(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureAttribute,
        >,
    ),
    /// Insert error associated with the `packaging_procedures` table.
    PackagingProcedure(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PackagingProcedureAttribute,
        >,
    ),
    /// Insert error associated with the `photograph_procedures` table.
    PhotographProcedure(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute,
        >,
    ),
    /// Insert error associated with the `pouring_procedures` table.
    PouringProcedure(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PouringProcedureAttribute,
        >,
    ),
    /// Insert error associated with the `procedures` table.
    Procedure(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
        >,
    ),
    /// Insert error associated with the `storage_procedures` table.
    StorageProcedure(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::StorageProcedureAttribute,
        >,
    ),
    /// Insert error associated with the `supernatant_procedures` table.
    SupernatantProcedure(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute,
        >,
    ),
    /// Insert error associated with the `weighing_procedures` table.
    WeighingProcedure(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::WeighingProcedureAttribute,
        >,
    ),
}
impl std::fmt::Display for ProcedureInsertErrorDAG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AliquotingProcedure(e) => write!(f, "{e}"),
            Self::BallMillProcedure(e) => write!(f, "{e}"),
            Self::CappingProcedure(e) => write!(f, "{e}"),
            Self::CentrifugeProcedure(e) => write!(f, "{e}"),
            Self::DisposalProcedure(e) => write!(f, "{e}"),
            Self::FractioningProcedure(e) => write!(f, "{e}"),
            Self::FreezeDryingProcedure(e) => write!(f, "{e}"),
            Self::FreezingProcedure(e) => write!(f, "{e}"),
            Self::GeolocationProcedure(e) => write!(f, "{e}"),
            Self::HarvestingProcedure(e) => write!(f, "{e}"),
            Self::PackagingProcedure(e) => write!(f, "{e}"),
            Self::PhotographProcedure(e) => write!(f, "{e}"),
            Self::PouringProcedure(e) => write!(f, "{e}"),
            Self::Procedure(e) => write!(f, "{e}"),
            Self::StorageProcedure(e) => write!(f, "{e}"),
            Self::SupernatantProcedure(e) => write!(f, "{e}"),
            Self::WeighingProcedure(e) => write!(f, "{e}"),
        }
    }
}
impl std::error::Error for ProcedureInsertErrorDAG {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::AliquotingProcedure(e) => Some(e),
            Self::BallMillProcedure(e) => Some(e),
            Self::CappingProcedure(e) => Some(e),
            Self::CentrifugeProcedure(e) => Some(e),
            Self::DisposalProcedure(e) => Some(e),
            Self::FractioningProcedure(e) => Some(e),
            Self::FreezeDryingProcedure(e) => Some(e),
            Self::FreezingProcedure(e) => Some(e),
            Self::GeolocationProcedure(e) => Some(e),
            Self::HarvestingProcedure(e) => Some(e),
            Self::PackagingProcedure(e) => Some(e),
            Self::PhotographProcedure(e) => Some(e),
            Self::PouringProcedure(e) => Some(e),
            Self::Procedure(e) => Some(e),
            Self::StorageProcedure(e) => Some(e),
            Self::SupernatantProcedure(e) => Some(e),
            Self::WeighingProcedure(e) => Some(e),
        }
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute,
        >,
    > for ProcedureInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute,
        >,
    ) -> Self {
        ProcedureInsertErrorDAG::AliquotingProcedure(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute,
        >,
    > for ProcedureInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute,
        >,
    ) -> Self {
        ProcedureInsertErrorDAG::BallMillProcedure(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CappingProcedureAttribute,
        >,
    > for ProcedureInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CappingProcedureAttribute,
        >,
    ) -> Self {
        ProcedureInsertErrorDAG::CappingProcedure(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureAttribute,
        >,
    > for ProcedureInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureAttribute,
        >,
    ) -> Self {
        ProcedureInsertErrorDAG::CentrifugeProcedure(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::DisposalProcedureAttribute,
        >,
    > for ProcedureInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::DisposalProcedureAttribute,
        >,
    ) -> Self {
        ProcedureInsertErrorDAG::DisposalProcedure(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute,
        >,
    > for ProcedureInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute,
        >,
    ) -> Self {
        ProcedureInsertErrorDAG::FractioningProcedure(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureAttribute,
        >,
    > for ProcedureInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureAttribute,
        >,
    ) -> Self {
        ProcedureInsertErrorDAG::FreezeDryingProcedure(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezingProcedureAttribute,
        >,
    > for ProcedureInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezingProcedureAttribute,
        >,
    ) -> Self {
        ProcedureInsertErrorDAG::FreezingProcedure(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureAttribute,
        >,
    > for ProcedureInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureAttribute,
        >,
    ) -> Self {
        ProcedureInsertErrorDAG::GeolocationProcedure(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureAttribute,
        >,
    > for ProcedureInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureAttribute,
        >,
    ) -> Self {
        ProcedureInsertErrorDAG::HarvestingProcedure(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PackagingProcedureAttribute,
        >,
    > for ProcedureInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PackagingProcedureAttribute,
        >,
    ) -> Self {
        ProcedureInsertErrorDAG::PackagingProcedure(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute,
        >,
    > for ProcedureInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute,
        >,
    ) -> Self {
        ProcedureInsertErrorDAG::PhotographProcedure(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PouringProcedureAttribute,
        >,
    > for ProcedureInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PouringProcedureAttribute,
        >,
    ) -> Self {
        ProcedureInsertErrorDAG::PouringProcedure(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
        >,
    > for ProcedureInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
        >,
    ) -> Self {
        ProcedureInsertErrorDAG::Procedure(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::StorageProcedureAttribute,
        >,
    > for ProcedureInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::StorageProcedureAttribute,
        >,
    ) -> Self {
        ProcedureInsertErrorDAG::StorageProcedure(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute,
        >,
    > for ProcedureInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute,
        >,
    ) -> Self {
        ProcedureInsertErrorDAG::SupernatantProcedure(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::WeighingProcedureAttribute,
        >,
    > for ProcedureInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::WeighingProcedureAttribute,
        >,
    ) -> Self {
        ProcedureInsertErrorDAG::WeighingProcedure(error)
    }
}
