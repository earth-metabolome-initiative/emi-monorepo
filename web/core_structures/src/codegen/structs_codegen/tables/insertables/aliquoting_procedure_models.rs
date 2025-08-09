#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableAliquotingProcedureModelExtensionAttributes {
    ProcedureModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
    ),
}
impl core::fmt::Display for InsertableAliquotingProcedureModelExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ProcedureModel(e) => write!(f, "{e}"),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableAliquotingProcedureModelAttributes {
    Extension(InsertableAliquotingProcedureModelExtensionAttributes),
    ProcedureModelId,
    Liters,
    AliquotedFrom,
    ProcedureAliquotedFrom(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
    AliquotedInto,
    ProcedureAliquotedInto(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
    AliquotedWith,
    ProcedureAliquotedWith(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
    PipetteTip,
    ProcedurePipetteTip(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
}
impl core::fmt::Display for InsertableAliquotingProcedureModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureModelId => write!(f, "procedure_model_id"),
            Self::Liters => write!(f, "liters"),
            Self::AliquotedFrom => write!(f, "aliquoted_from"),
            Self::ProcedureAliquotedFrom(e) => write!(f, "{e}"),
            Self::AliquotedInto => write!(f, "aliquoted_into"),
            Self::ProcedureAliquotedInto(e) => write!(f, "{e}"),
            Self::AliquotedWith => write!(f, "aliquoted_with"),
            Self::ProcedureAliquotedWith(e) => write!(f, "{e}"),
            Self::PipetteTip => write!(f, "pipette_tip"),
            Self::ProcedurePipetteTip(e) => write!(f, "{e}"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::aliquoting_procedure_models::aliquoting_procedure_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableAliquotingProcedureModel {
    pub(crate) procedure_model_id: i32,
    pub(crate) liters: f32,
    pub(crate) aliquoted_from: ::rosetta_uuid::Uuid,
    pub(crate) procedure_aliquoted_from: i32,
    pub(crate) aliquoted_into: ::rosetta_uuid::Uuid,
    pub(crate) procedure_aliquoted_into: i32,
    pub(crate) aliquoted_with: ::rosetta_uuid::Uuid,
    pub(crate) procedure_aliquoted_with: i32,
    pub(crate) pipette_tip: ::rosetta_uuid::Uuid,
    pub(crate) procedure_pipette_tip: i32,
}
impl InsertableAliquotingProcedureModel {
    pub fn procedure_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel::table(),
                self.procedure_model_id,
            ),
            conn,
        )
    }
    pub fn aliquoted_from<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel::table(),
                self.aliquoted_from,
            ),
            conn,
        )
    }
    pub fn procedure_aliquoted_from<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable::table(),
                self.procedure_aliquoted_from,
            ),
            conn,
        )
    }
    pub fn aliquoted_into<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel::table(),
                self.aliquoted_into,
            ),
            conn,
        )
    }
    pub fn procedure_aliquoted_into<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable::table(),
                self.procedure_aliquoted_into,
            ),
            conn,
        )
    }
    pub fn aliquoted_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::pipette_models::PipetteModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::pipette_models::PipetteModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::pipette_models::PipetteModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::pipette_models::PipetteModel::table(),
                self.aliquoted_with,
            ),
            conn,
        )
    }
    pub fn procedure_aliquoted_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable::table(),
                self.procedure_aliquoted_with,
            ),
            conn,
        )
    }
    pub fn pipette_tip<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel::table(
                ),
                self.pipette_tip,
            ),
            conn,
        )
    }
    pub fn procedure_pipette_tip<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable::table(),
                self.procedure_pipette_tip,
            ),
            conn,
        )
    }
    pub fn aliquoting_procedure_models_aliquoted_with_pipette_tip_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule::table(),
                (self.aliquoted_with, self.pipette_tip),
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableAliquotingProcedureModelBuilder<
    ProcedureModel
        = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    pub(crate) liters: Option<f32>,
    pub(crate) aliquoted_from: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_aliquoted_from: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    pub(crate) aliquoted_into: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_aliquoted_into: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    pub(crate) aliquoted_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_aliquoted_with: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    pub(crate) pipette_tip: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_pipette_tip: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    pub(crate) procedure_model: ProcedureModel,
}
impl<ProcedureModel> web_common_traits::database::ExtendableBuilder
for InsertableAliquotingProcedureModelBuilder<ProcedureModel>
where
    ProcedureModel: web_common_traits::database::ExtendableBuilder<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
    >,
{
    type Attributes = InsertableAliquotingProcedureModelAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure_model = self
            .procedure_model
            .extend_builder(other.procedure_model)
            .map_err(|err| {
                err.into_field_name(|attribute| InsertableAliquotingProcedureModelAttributes::Extension(
                    InsertableAliquotingProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                ))
            })?;
        if let Some(liters) = other.liters {
            self = self.liters(liters)?;
        }
        if let Some(aliquoted_from) = other.aliquoted_from {
            self = self.aliquoted_from(aliquoted_from)?;
        }
        self = self.procedure_aliquoted_from(other.procedure_aliquoted_from)?;
        if let Some(aliquoted_into) = other.aliquoted_into {
            self = self.aliquoted_into(aliquoted_into)?;
        }
        self = self.procedure_aliquoted_into(other.procedure_aliquoted_into)?;
        if let Some(aliquoted_with) = other.aliquoted_with {
            self = self.aliquoted_with(aliquoted_with)?;
        }
        self = self.procedure_aliquoted_with(other.procedure_aliquoted_with)?;
        if let Some(pipette_tip) = other.pipette_tip {
            self = self.pipette_tip(pipette_tip)?;
        }
        self = self.procedure_pipette_tip(other.procedure_pipette_tip)?;
        Ok(self)
    }
}
impl<ProcedureModel> web_common_traits::prelude::SetPrimaryKey
    for InsertableAliquotingProcedureModelBuilder<ProcedureModel>
where
    ProcedureModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.procedure_model = self.procedure_model.set_primary_key(primary_key);
        self
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the `aliquoting_procedure_models.aliquoted_from`
    /// column from table `aliquoting_procedure_models`.
    pub fn aliquoted_from(
        mut self,
        aliquoted_from: ::rosetta_uuid::Uuid,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableAliquotingProcedureModelAttributes>,
    > {
        self.aliquoted_from = Some(aliquoted_from);
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the `aliquoting_procedure_models.aliquoted_into`
    /// column from table `aliquoting_procedure_models`.
    pub fn aliquoted_into(
        mut self,
        aliquoted_into: ::rosetta_uuid::Uuid,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableAliquotingProcedureModelAttributes>,
    > {
        self.aliquoted_into = Some(aliquoted_into);
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the `aliquoting_procedure_models.aliquoted_with`
    /// column from table `aliquoting_procedure_models`.
    pub fn aliquoted_with(
        mut self,
        aliquoted_with: ::rosetta_uuid::Uuid,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableAliquotingProcedureModelAttributes>,
    > {
        self.aliquoted_with = Some(aliquoted_with);
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the `aliquoting_procedure_models.liters` column from
    /// table `aliquoting_procedure_models`.
    pub fn liters<Liters>(
        mut self,
        liters: Liters,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableAliquotingProcedureModelAttributes>,
    >
    where
        Liters: TryInto<f32>,
        <Liters as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let liters = liters.try_into().map_err(|err: <Liters as TryInto<f32>>::Error| {
            Into::into(err).rename_field(InsertableAliquotingProcedureModelAttributes::Liters)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(liters)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes::Liters,
                    )
            })?;
        self.liters = Some(liters);
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the `aliquoting_procedure_models.pipette_tip` column
    /// from table `aliquoting_procedure_models`.
    pub fn pipette_tip(
        mut self,
        pipette_tip: ::rosetta_uuid::Uuid,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableAliquotingProcedureModelAttributes>,
    > {
        self.pipette_tip = Some(pipette_tip);
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the
    /// `aliquoting_procedure_models.procedure_aliquoted_from` column from table
    /// `aliquoting_procedure_models`.
    pub fn procedure_aliquoted_from(
        mut self,
        mut procedure_aliquoted_from: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableAliquotingProcedureModelAttributes,
        >,
    >
    where
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::ExtendableBuilder<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        >,
    {
        use web_common_traits::database::ExtendableBuilder;
        if let (Some(local), Some(foreign)) =
            (self.aliquoted_from, procedure_aliquoted_from.trackable_id)
        {
            if local != foreign {
                return Err(
                    web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes::ProcedureAliquotedFrom(
                                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::TrackableId,
                            ),
                        ),
                    ),
                );
            }
        } else if let Some(foreign) = procedure_aliquoted_from.trackable_id {
            self.aliquoted_from = Some(foreign);
        } else if let Some(local) = self.aliquoted_from {
            procedure_aliquoted_from = procedure_aliquoted_from
                .trackable(local)
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes::ProcedureAliquotedFrom(
                            attribute,
                        )
                    })
                })?;
        }
        self.procedure_aliquoted_from = self
            .procedure_aliquoted_from
            .extend_builder(procedure_aliquoted_from)
            .map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertableAliquotingProcedureModelAttributes::ProcedureAliquotedFrom(attribute)
                })
            })?;
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the
    /// `aliquoting_procedure_models.procedure_aliquoted_into` column from table
    /// `aliquoting_procedure_models`.
    pub fn procedure_aliquoted_into(
        mut self,
        mut procedure_aliquoted_into: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableAliquotingProcedureModelAttributes,
        >,
    >
    where
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::ExtendableBuilder<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        >,
    {
        use web_common_traits::database::ExtendableBuilder;
        if let (Some(local), Some(foreign)) =
            (self.aliquoted_into, procedure_aliquoted_into.trackable_id)
        {
            if local != foreign {
                return Err(
                    web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes::ProcedureAliquotedInto(
                                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::TrackableId,
                            ),
                        ),
                    ),
                );
            }
        } else if let Some(foreign) = procedure_aliquoted_into.trackable_id {
            self.aliquoted_into = Some(foreign);
        } else if let Some(local) = self.aliquoted_into {
            procedure_aliquoted_into = procedure_aliquoted_into
                .trackable(local)
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes::ProcedureAliquotedInto(
                            attribute,
                        )
                    })
                })?;
        }
        self.procedure_aliquoted_into = self
            .procedure_aliquoted_into
            .extend_builder(procedure_aliquoted_into)
            .map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertableAliquotingProcedureModelAttributes::ProcedureAliquotedInto(attribute)
                })
            })?;
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the
    /// `aliquoting_procedure_models.procedure_aliquoted_with` column from table
    /// `aliquoting_procedure_models`.
    pub fn procedure_aliquoted_with(
        mut self,
        mut procedure_aliquoted_with: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableAliquotingProcedureModelAttributes,
        >,
    >
    where
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::ExtendableBuilder<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        >,
    {
        use web_common_traits::database::ExtendableBuilder;
        if let (Some(local), Some(foreign)) =
            (self.aliquoted_with, procedure_aliquoted_with.trackable_id)
        {
            if local != foreign {
                return Err(
                    web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes::ProcedureAliquotedWith(
                                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::TrackableId,
                            ),
                        ),
                    ),
                );
            }
        } else if let Some(foreign) = procedure_aliquoted_with.trackable_id {
            self.aliquoted_with = Some(foreign);
        } else if let Some(local) = self.aliquoted_with {
            procedure_aliquoted_with = procedure_aliquoted_with
                .trackable(local)
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes::ProcedureAliquotedWith(
                            attribute,
                        )
                    })
                })?;
        }
        self.procedure_aliquoted_with = self
            .procedure_aliquoted_with
            .extend_builder(procedure_aliquoted_with)
            .map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertableAliquotingProcedureModelAttributes::ProcedureAliquotedWith(attribute)
                })
            })?;
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the
    /// `aliquoting_procedure_models.procedure_pipette_tip` column from table
    /// `aliquoting_procedure_models`.
    pub fn procedure_pipette_tip(
        mut self,
        mut procedure_pipette_tip: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableAliquotingProcedureModelAttributes,
        >,
    >
    where
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::ExtendableBuilder<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        >,
    {
        use web_common_traits::database::ExtendableBuilder;
        if let (Some(local), Some(foreign)) = (self.pipette_tip, procedure_pipette_tip.trackable_id)
        {
            if local != foreign {
                return Err(
                    web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes::ProcedurePipetteTip(
                                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::TrackableId,
                            ),
                        ),
                    ),
                );
            }
        } else if let Some(foreign) = procedure_pipette_tip.trackable_id {
            self.pipette_tip = Some(foreign);
        } else if let Some(local) = self.pipette_tip {
            procedure_pipette_tip = procedure_pipette_tip
                .trackable(local)
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes::ProcedurePipetteTip(
                            attribute,
                        )
                    })
                })?;
        }
        self.procedure_pipette_tip =
            self.procedure_pipette_tip.extend_builder(procedure_pipette_tip).map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertableAliquotingProcedureModelAttributes::ProcedurePipetteTip(attribute)
                })
            })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.created_at` column from table
    /// `aliquoting_procedure_models`.
    pub fn created_at<CreatedAt>(
        mut self,
        created_at: CreatedAt,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableAliquotingProcedureModelAttributes>,
    >
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.created_at(created_at).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableAliquotingProcedureModelAttributes::Extension(
                    InsertableAliquotingProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.created_at`,
    /// `procedure_models.updated_at` columns from table
    /// `aliquoting_procedure_models`.
    pub fn created_at_and_updated_at<CreatedAt, UpdatedAt>(
        mut self,
        created_at: CreatedAt,
        updated_at: UpdatedAt,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableAliquotingProcedureModelAttributes>,
    >
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
        UpdatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self
            .procedure_model
            .created_at_and_updated_at(created_at, updated_at)
            .map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertableAliquotingProcedureModelAttributes::Extension(
                        InsertableAliquotingProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    )
                })
            })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.created_by` column from table
    /// `aliquoting_procedure_models`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableAliquotingProcedureModelAttributes>,
    > {
        self.procedure_model = self.procedure_model.created_by(created_by).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableAliquotingProcedureModelAttributes::Extension(
                    InsertableAliquotingProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        self = self.updated_by(created_by)?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.deprecated` column from table
    /// `aliquoting_procedure_models`.
    pub fn deprecated<Deprecated>(
        mut self,
        deprecated: Deprecated,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableAliquotingProcedureModelAttributes>,
    >
    where
        Deprecated: TryInto<bool>,
        <Deprecated as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.deprecated(deprecated).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableAliquotingProcedureModelAttributes::Extension(
                    InsertableAliquotingProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.description` column from table
    /// `aliquoting_procedure_models`.
    pub fn description<Description>(
        mut self,
        description: Description,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableAliquotingProcedureModelAttributes>,
    >
    where
        Description: TryInto<String>,
        <Description as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.description(description).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableAliquotingProcedureModelAttributes::Extension(
                    InsertableAliquotingProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.icon` column from table
    /// `aliquoting_procedure_models`.
    pub fn icon<Icon>(
        mut self,
        icon: Icon,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableAliquotingProcedureModelAttributes>,
    >
    where
        Icon: TryInto<String>,
        <Icon as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.icon(icon).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableAliquotingProcedureModelAttributes::Extension(
                    InsertableAliquotingProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.name` column from table
    /// `aliquoting_procedure_models`.
    pub fn name<Name>(
        mut self,
        name: Name,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableAliquotingProcedureModelAttributes>,
    >
    where
        Name: TryInto<String>,
        <Name as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.name(name).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableAliquotingProcedureModelAttributes::Extension(
                    InsertableAliquotingProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.name`,
    /// `procedure_models.description` columns from table
    /// `aliquoting_procedure_models`.
    pub fn name_and_description<Name, Description>(
        mut self,
        name: Name,
        description: Description,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableAliquotingProcedureModelAttributes>,
    >
    where
        Name: TryInto<String>,
        <Name as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
        Description: TryInto<String>,
        <Description as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model =
            self.procedure_model.name_and_description(name, description).map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertableAliquotingProcedureModelAttributes::Extension(
                        InsertableAliquotingProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    )
                })
            })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.photograph_id` column from table
    /// `aliquoting_procedure_models`.
    pub fn photograph(
        mut self,
        photograph_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableAliquotingProcedureModelAttributes>,
    > {
        self.procedure_model = self.procedure_model.photograph(photograph_id).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableAliquotingProcedureModelAttributes::Extension(
                    InsertableAliquotingProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.updated_at` column from table
    /// `aliquoting_procedure_models`.
    pub fn updated_at<UpdatedAt>(
        mut self,
        updated_at: UpdatedAt,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableAliquotingProcedureModelAttributes>,
    >
    where
        UpdatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.updated_at(updated_at).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableAliquotingProcedureModelAttributes::Extension(
                    InsertableAliquotingProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.updated_by` column from table
    /// `aliquoting_procedure_models`.
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableAliquotingProcedureModelAttributes>,
    > {
        self.procedure_model = self.procedure_model.updated_by(updated_by).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableAliquotingProcedureModelAttributes::Extension(
                    InsertableAliquotingProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl<ProcedureModel, C> web_common_traits::database::TryInsertGeneric<C>
for InsertableAliquotingProcedureModelBuilder<ProcedureModel>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel,
        Error = web_common_traits::database::InsertError<
            InsertableAliquotingProcedureModelAttributes,
        >,
    >,
    ProcedureModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::TryInsertGeneric<
        C,
    >,
{
    type Attributes = InsertableAliquotingProcedureModelAttributes;
    fn is_complete(&self) -> bool {
        self.procedure_model.is_complete() && self.liters.is_some()
            && self.aliquoted_from.is_some()
            && self.procedure_aliquoted_from.is_complete()
            && self.aliquoted_into.is_some()
            && self.procedure_aliquoted_into.is_complete()
            && self.aliquoted_with.is_some()
            && self.procedure_aliquoted_with.is_complete() && self.pipette_tip.is_some()
            && self.procedure_pipette_tip.is_complete()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::PrimaryKey,
        web_common_traits::database::InsertError<Self::Attributes>,
    > {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
