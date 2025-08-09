#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableSupernatantProcedureExtensionAttributes {
    Procedure(crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes),
}
impl core::fmt::Display for InsertableSupernatantProcedureExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Procedure(e) => write!(f, "{e}"),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableSupernatantProcedureAttributes {
    Extension(InsertableSupernatantProcedureExtensionAttributes),
    ProcedureId,
    ProcedureModelId,
    StratifiedSource,
    SupernatantDestination,
    TransferredWith,
    PipetteTip,
}
impl core::fmt::Display for InsertableSupernatantProcedureAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureId => write!(f, "procedure_id"),
            Self::ProcedureModelId => write!(f, "procedure_model_id"),
            Self::StratifiedSource => write!(f, "stratified_source"),
            Self::SupernatantDestination => write!(f, "supernatant_destination"),
            Self::TransferredWith => write!(f, "transferred_with"),
            Self::PipetteTip => write!(f, "pipette_tip"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::supernatant_procedures::supernatant_procedures
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSupernatantProcedure {
    pub(crate) procedure_id: ::rosetta_uuid::Uuid,
    pub(crate) procedure_model_id: i32,
    pub(crate) stratified_source: ::rosetta_uuid::Uuid,
    pub(crate) supernatant_destination: ::rosetta_uuid::Uuid,
    pub(crate) transferred_with: ::rosetta_uuid::Uuid,
    pub(crate) pipette_tip: ::rosetta_uuid::Uuid,
}
impl InsertableSupernatantProcedure {
    pub fn procedure<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedures::Procedure: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedures::Procedure,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedures::Procedure::table(),
                self.procedure_id,
            ),
            conn,
        )
    }
    pub fn procedure_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel::table(),
                self.procedure_model_id,
            ),
            conn,
        )
    }
    pub fn stratified_source<C: diesel::connection::LoadConnection>(
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
                self.stratified_source,
            ),
            conn,
        )
    }
    pub fn supernatant_destination<C: diesel::connection::LoadConnection>(
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
                self.supernatant_destination,
            ),
            conn,
        )
    }
    pub fn transferred_with<C: diesel::connection::LoadConnection>(
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
                self.transferred_with,
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
    pub fn supernatant_procedures_procedure_id_stratified_source_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable::table(),
                (self.procedure_id, self.stratified_source),
            ),
            conn,
        )
    }
    pub fn supernatant_procedures_procedure_id_supernatant_destinatio_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable::table(),
                (self.procedure_id, self.supernatant_destination),
            ),
            conn,
        )
    }
    pub fn supernatant_procedures_procedure_id_transferred_with_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable::table(),
                (self.procedure_id, self.transferred_with),
            ),
            conn,
        )
    }
    pub fn supernatant_procedures_procedure_id_pipette_tip_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable::table(),
                (self.procedure_id, self.pipette_tip),
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSupernatantProcedureBuilder<
    Procedure = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
> {
    pub(crate) procedure_model_id: Option<i32>,
    pub(crate) stratified_source: Option<::rosetta_uuid::Uuid>,
    pub(crate) supernatant_destination: Option<::rosetta_uuid::Uuid>,
    pub(crate) transferred_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) pipette_tip: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure: Procedure,
}
impl<Procedure> web_common_traits::database::ExtendableBuilder
for InsertableSupernatantProcedureBuilder<Procedure>
where
    Procedure: web_common_traits::database::ExtendableBuilder<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes,
    >,
{
    type Attributes = InsertableSupernatantProcedureAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = self
            .procedure
            .extend_builder(other.procedure)
            .map_err(|err| {
                err.into_field_name(|attribute| InsertableSupernatantProcedureAttributes::Extension(
                    InsertableSupernatantProcedureExtensionAttributes::Procedure(
                        attribute,
                    ),
                ))
            })?;
        if let Some(procedure_model_id) = other.procedure_model_id {
            self = self.procedure_model(procedure_model_id)?;
        }
        if let Some(stratified_source) = other.stratified_source {
            self = self.stratified_source(stratified_source)?;
        }
        if let Some(supernatant_destination) = other.supernatant_destination {
            self = self.supernatant_destination(supernatant_destination)?;
        }
        if let Some(transferred_with) = other.transferred_with {
            self = self.transferred_with(transferred_with)?;
        }
        if let Some(pipette_tip) = other.pipette_tip {
            self = self.pipette_tip(pipette_tip)?;
        }
        Ok(self)
    }
}
impl<Procedure> web_common_traits::prelude::SetPrimaryKey
    for InsertableSupernatantProcedureBuilder<Procedure>
where
    Procedure: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.procedure = self.procedure.set_primary_key(primary_key);
        self
    }
}
impl<Procedure>
    crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder<
        Procedure,
    >
{
    /// Sets the value of the `supernatant_procedures.pipette_tip` column from
    /// table `supernatant_procedures`.
    pub fn pipette_tip(
        mut self,
        pipette_tip: ::rosetta_uuid::Uuid,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableSupernatantProcedureAttributes>,
    > {
        self.pipette_tip = Some(pipette_tip);
        Ok(self)
    }
}
impl<Procedure>
    crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder<
        Procedure,
    >
{
    /// Sets the value of the `supernatant_procedures.procedure_model_id` column
    /// from table `supernatant_procedures`.
    pub fn procedure_model(
        mut self,
        procedure_model_id: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableSupernatantProcedureAttributes>,
    > {
        self.procedure_model_id = Some(procedure_model_id);
        Ok(self)
    }
}
impl<Procedure>
    crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder<
        Procedure,
    >
{
    /// Sets the value of the `supernatant_procedures.stratified_source` column
    /// from table `supernatant_procedures`.
    pub fn stratified_source(
        mut self,
        stratified_source: ::rosetta_uuid::Uuid,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableSupernatantProcedureAttributes>,
    > {
        self.stratified_source = Some(stratified_source);
        Ok(self)
    }
}
impl<Procedure>
    crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder<
        Procedure,
    >
{
    /// Sets the value of the `supernatant_procedures.supernatant_destination`
    /// column from table `supernatant_procedures`.
    pub fn supernatant_destination(
        mut self,
        supernatant_destination: ::rosetta_uuid::Uuid,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableSupernatantProcedureAttributes>,
    > {
        self.supernatant_destination = Some(supernatant_destination);
        Ok(self)
    }
}
impl<Procedure>
    crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder<
        Procedure,
    >
{
    /// Sets the value of the `supernatant_procedures.transferred_with` column
    /// from table `supernatant_procedures`.
    pub fn transferred_with(
        mut self,
        transferred_with: ::rosetta_uuid::Uuid,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableSupernatantProcedureAttributes>,
    > {
        self.transferred_with = Some(transferred_with);
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
    >
{
    /// Sets the value of the `procedures.created_at` column from table
    /// `supernatant_procedures`.
    pub fn created_at<CreatedAt>(
        mut self,
        created_at: CreatedAt,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableSupernatantProcedureAttributes>,
    >
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure = self.procedure.created_at(created_at).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableSupernatantProcedureAttributes::Extension(
                    InsertableSupernatantProcedureExtensionAttributes::Procedure(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
    >
{
    /// Sets the value of the `procedures.created_at`, `procedures.updated_at`
    /// columns from table `supernatant_procedures`.
    pub fn created_at_and_updated_at<CreatedAt, UpdatedAt>(
        mut self,
        created_at: CreatedAt,
        updated_at: UpdatedAt,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableSupernatantProcedureAttributes>,
    >
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
        UpdatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure =
            self.procedure.created_at_and_updated_at(created_at, updated_at).map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertableSupernatantProcedureAttributes::Extension(
                        InsertableSupernatantProcedureExtensionAttributes::Procedure(attribute),
                    )
                })
            })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
    >
{
    /// Sets the value of the `procedures.created_by` column from table
    /// `supernatant_procedures`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableSupernatantProcedureAttributes>,
    > {
        self.procedure = self.procedure.created_by(created_by).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableSupernatantProcedureAttributes::Extension(
                    InsertableSupernatantProcedureExtensionAttributes::Procedure(attribute),
                )
            })
        })?;
        self = self.updated_by(created_by)?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
    >
{
    /// Sets the value of the `procedures.updated_at` column from table
    /// `supernatant_procedures`.
    pub fn updated_at<UpdatedAt>(
        mut self,
        updated_at: UpdatedAt,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableSupernatantProcedureAttributes>,
    >
    where
        UpdatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure = self.procedure.updated_at(updated_at).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableSupernatantProcedureAttributes::Extension(
                    InsertableSupernatantProcedureExtensionAttributes::Procedure(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
    >
{
    /// Sets the value of the `procedures.updated_by` column from table
    /// `supernatant_procedures`.
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableSupernatantProcedureAttributes>,
    > {
        self.procedure = self.procedure.updated_by(updated_by).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableSupernatantProcedureAttributes::Extension(
                    InsertableSupernatantProcedureExtensionAttributes::Procedure(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl<Procedure, C> web_common_traits::database::TryInsertGeneric<C>
for InsertableSupernatantProcedureBuilder<Procedure>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure,
        Error = web_common_traits::database::InsertError<
            InsertableSupernatantProcedureAttributes,
        >,
    >,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
{
    type Attributes = InsertableSupernatantProcedureAttributes;
    fn is_complete(&self) -> bool {
        self.procedure.is_complete() && self.procedure_model_id.is_some()
            && self.stratified_source.is_some() && self.supernatant_destination.is_some()
            && self.transferred_with.is_some() && self.pipette_tip.is_some()
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
        let insertable: crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
