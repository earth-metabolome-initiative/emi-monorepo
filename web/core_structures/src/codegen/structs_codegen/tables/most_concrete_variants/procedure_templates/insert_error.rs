#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enum representing insert errors which may occur when inserting an entry from
/// the `procedure_templates` table DAG.
pub enum ProcedureTemplateInsertErrorDAG {
    ///Insert error associated with the `aliquoting_procedure_templates` table.
    AliquotingProcedureTemplate(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateAttribute,
        >,
    ),
    ///Insert error associated with the `ball_mill_procedure_templates` table.
    BallMillProcedureTemplate(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute,
        >,
    ),
    ///Insert error associated with the `capping_procedure_templates` table.
    CappingProcedureTemplate(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CappingProcedureTemplateAttribute,
        >,
    ),
    ///Insert error associated with the `centrifuge_procedure_templates` table.
    CentrifugeProcedureTemplate(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute,
        >,
    ),
    ///Insert error associated with the `cleaning_procedure_templates` table.
    CleaningProcedureTemplate(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CleaningProcedureTemplateAttribute,
        >,
    ),
    ///Insert error associated with the `disposal_procedure_templates` table.
    DisposalProcedureTemplate(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::DisposalProcedureTemplateAttribute,
        >,
    ),
    ///Insert error associated with the `fractioning_procedure_templates` table.
    FractioningProcedureTemplate(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateAttribute,
        >,
    ),
    ///Insert error associated with the `freeze_drying_procedure_templates` table.
    FreezeDryingProcedureTemplate(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureTemplateAttribute,
        >,
    ),
    ///Insert error associated with the `freezing_procedure_templates` table.
    FreezingProcedureTemplate(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateAttribute,
        >,
    ),
    ///Insert error associated with the `geolocation_procedure_templates` table.
    GeolocationProcedureTemplate(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureTemplateAttribute,
        >,
    ),
    ///Insert error associated with the `harvesting_procedure_templates` table.
    HarvestingProcedureTemplate(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureTemplateAttribute,
        >,
    ),
    ///Insert error associated with the `packaging_procedure_templates` table.
    PackagingProcedureTemplate(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PackagingProcedureTemplateAttribute,
        >,
    ),
    ///Insert error associated with the `photograph_procedure_templates` table.
    PhotographProcedureTemplate(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureTemplateAttribute,
        >,
    ),
    ///Insert error associated with the `pouring_procedure_templates` table.
    PouringProcedureTemplate(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PouringProcedureTemplateAttribute,
        >,
    ),
    ///Insert error associated with the `ppe_reminder_procedure_templates` table.
    PpeReminderProcedureTemplate(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PpeReminderProcedureTemplateAttribute,
        >,
    ),
    ///Insert error associated with the `procedure_templates` table.
    ProcedureTemplate(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
        >,
    ),
    ///Insert error associated with the `storage_procedure_templates` table.
    StorageProcedureTemplate(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::StorageProcedureTemplateAttribute,
        >,
    ),
    ///Insert error associated with the `supernatant_procedure_templates` table.
    SupernatantProcedureTemplate(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateAttribute,
        >,
    ),
    ///Insert error associated with the `tagging_procedure_templates` table.
    TaggingProcedureTemplate(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::TaggingProcedureTemplateAttribute,
        >,
    ),
    ///Insert error associated with the `weighing_procedure_templates` table.
    WeighingProcedureTemplate(
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::WeighingProcedureTemplateAttribute,
        >,
    ),
}
impl std::fmt::Display for ProcedureTemplateInsertErrorDAG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AliquotingProcedureTemplate(e) => write!(f, "{e}"),
            Self::BallMillProcedureTemplate(e) => write!(f, "{e}"),
            Self::CappingProcedureTemplate(e) => write!(f, "{e}"),
            Self::CentrifugeProcedureTemplate(e) => write!(f, "{e}"),
            Self::CleaningProcedureTemplate(e) => write!(f, "{e}"),
            Self::DisposalProcedureTemplate(e) => write!(f, "{e}"),
            Self::FractioningProcedureTemplate(e) => write!(f, "{e}"),
            Self::FreezeDryingProcedureTemplate(e) => write!(f, "{e}"),
            Self::FreezingProcedureTemplate(e) => write!(f, "{e}"),
            Self::GeolocationProcedureTemplate(e) => write!(f, "{e}"),
            Self::HarvestingProcedureTemplate(e) => write!(f, "{e}"),
            Self::PackagingProcedureTemplate(e) => write!(f, "{e}"),
            Self::PhotographProcedureTemplate(e) => write!(f, "{e}"),
            Self::PouringProcedureTemplate(e) => write!(f, "{e}"),
            Self::PpeReminderProcedureTemplate(e) => write!(f, "{e}"),
            Self::ProcedureTemplate(e) => write!(f, "{e}"),
            Self::StorageProcedureTemplate(e) => write!(f, "{e}"),
            Self::SupernatantProcedureTemplate(e) => write!(f, "{e}"),
            Self::TaggingProcedureTemplate(e) => write!(f, "{e}"),
            Self::WeighingProcedureTemplate(e) => write!(f, "{e}"),
        }
    }
}
impl std::error::Error for ProcedureTemplateInsertErrorDAG {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::AliquotingProcedureTemplate(e) => Some(e),
            Self::BallMillProcedureTemplate(e) => Some(e),
            Self::CappingProcedureTemplate(e) => Some(e),
            Self::CentrifugeProcedureTemplate(e) => Some(e),
            Self::CleaningProcedureTemplate(e) => Some(e),
            Self::DisposalProcedureTemplate(e) => Some(e),
            Self::FractioningProcedureTemplate(e) => Some(e),
            Self::FreezeDryingProcedureTemplate(e) => Some(e),
            Self::FreezingProcedureTemplate(e) => Some(e),
            Self::GeolocationProcedureTemplate(e) => Some(e),
            Self::HarvestingProcedureTemplate(e) => Some(e),
            Self::PackagingProcedureTemplate(e) => Some(e),
            Self::PhotographProcedureTemplate(e) => Some(e),
            Self::PouringProcedureTemplate(e) => Some(e),
            Self::PpeReminderProcedureTemplate(e) => Some(e),
            Self::ProcedureTemplate(e) => Some(e),
            Self::StorageProcedureTemplate(e) => Some(e),
            Self::SupernatantProcedureTemplate(e) => Some(e),
            Self::TaggingProcedureTemplate(e) => Some(e),
            Self::WeighingProcedureTemplate(e) => Some(e),
        }
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateAttribute,
    >,
> for ProcedureTemplateInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateAttribute,
        >,
    ) -> Self {
        ProcedureTemplateInsertErrorDAG::AliquotingProcedureTemplate(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute,
    >,
> for ProcedureTemplateInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute,
        >,
    ) -> Self {
        ProcedureTemplateInsertErrorDAG::BallMillProcedureTemplate(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CappingProcedureTemplateAttribute,
        >,
    > for ProcedureTemplateInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CappingProcedureTemplateAttribute,
        >,
    ) -> Self {
        ProcedureTemplateInsertErrorDAG::CappingProcedureTemplate(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute,
    >,
> for ProcedureTemplateInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute,
        >,
    ) -> Self {
        ProcedureTemplateInsertErrorDAG::CentrifugeProcedureTemplate(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CleaningProcedureTemplateAttribute,
    >,
> for ProcedureTemplateInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CleaningProcedureTemplateAttribute,
        >,
    ) -> Self {
        ProcedureTemplateInsertErrorDAG::CleaningProcedureTemplate(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::DisposalProcedureTemplateAttribute,
    >,
> for ProcedureTemplateInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::DisposalProcedureTemplateAttribute,
        >,
    ) -> Self {
        ProcedureTemplateInsertErrorDAG::DisposalProcedureTemplate(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateAttribute,
    >,
> for ProcedureTemplateInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateAttribute,
        >,
    ) -> Self {
        ProcedureTemplateInsertErrorDAG::FractioningProcedureTemplate(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureTemplateAttribute,
    >,
> for ProcedureTemplateInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureTemplateAttribute,
        >,
    ) -> Self {
        ProcedureTemplateInsertErrorDAG::FreezeDryingProcedureTemplate(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateAttribute,
    >,
> for ProcedureTemplateInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateAttribute,
        >,
    ) -> Self {
        ProcedureTemplateInsertErrorDAG::FreezingProcedureTemplate(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureTemplateAttribute,
    >,
> for ProcedureTemplateInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureTemplateAttribute,
        >,
    ) -> Self {
        ProcedureTemplateInsertErrorDAG::GeolocationProcedureTemplate(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureTemplateAttribute,
    >,
> for ProcedureTemplateInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureTemplateAttribute,
        >,
    ) -> Self {
        ProcedureTemplateInsertErrorDAG::HarvestingProcedureTemplate(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::PackagingProcedureTemplateAttribute,
    >,
> for ProcedureTemplateInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PackagingProcedureTemplateAttribute,
        >,
    ) -> Self {
        ProcedureTemplateInsertErrorDAG::PackagingProcedureTemplate(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::PhotographProcedureTemplateAttribute,
    >,
> for ProcedureTemplateInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureTemplateAttribute,
        >,
    ) -> Self {
        ProcedureTemplateInsertErrorDAG::PhotographProcedureTemplate(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PouringProcedureTemplateAttribute,
        >,
    > for ProcedureTemplateInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PouringProcedureTemplateAttribute,
        >,
    ) -> Self {
        ProcedureTemplateInsertErrorDAG::PouringProcedureTemplate(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::PpeReminderProcedureTemplateAttribute,
    >,
> for ProcedureTemplateInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PpeReminderProcedureTemplateAttribute,
        >,
    ) -> Self {
        ProcedureTemplateInsertErrorDAG::PpeReminderProcedureTemplate(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
        >,
    > for ProcedureTemplateInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
        >,
    ) -> Self {
        ProcedureTemplateInsertErrorDAG::ProcedureTemplate(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::StorageProcedureTemplateAttribute,
        >,
    > for ProcedureTemplateInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::StorageProcedureTemplateAttribute,
        >,
    ) -> Self {
        ProcedureTemplateInsertErrorDAG::StorageProcedureTemplate(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateAttribute,
    >,
> for ProcedureTemplateInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateAttribute,
        >,
    ) -> Self {
        ProcedureTemplateInsertErrorDAG::SupernatantProcedureTemplate(error)
    }
}
impl
    From<
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::TaggingProcedureTemplateAttribute,
        >,
    > for ProcedureTemplateInsertErrorDAG
{
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::TaggingProcedureTemplateAttribute,
        >,
    ) -> Self {
        ProcedureTemplateInsertErrorDAG::TaggingProcedureTemplate(error)
    }
}
impl From<
    web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::WeighingProcedureTemplateAttribute,
    >,
> for ProcedureTemplateInsertErrorDAG {
    fn from(
        error: web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::WeighingProcedureTemplateAttribute,
        >,
    ) -> Self {
        ProcedureTemplateInsertErrorDAG::WeighingProcedureTemplate(error)
    }
}
