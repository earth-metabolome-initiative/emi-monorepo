#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableWeighingProcedureExtensionAttributes {
    Procedure(crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes),
}
impl core::fmt::Display for InsertableWeighingProcedureExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Procedure(e) => write!(f, "Procedure.{e}"),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableWeighingProcedureAttributes {
    Extension(InsertableWeighingProcedureExtensionAttributes),
    ProcedureId,
    ProcedureModelId,
    WeightedWith,
    WeightedContainerId,
    Kilograms,
}
impl core::fmt::Display for InsertableWeighingProcedureAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureId => write!(f, "procedure_id"),
            Self::ProcedureModelId => write!(f, "procedure_model_id"),
            Self::WeightedWith => write!(f, "weighted_with"),
            Self::WeightedContainerId => write!(f, "weighted_container_id"),
            Self::Kilograms => write!(f, "kilograms"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::weighing_procedures::weighing_procedures
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableWeighingProcedure {
    pub(crate) procedure_id: ::rosetta_uuid::Uuid,
    pub(crate) procedure_model_id: i32,
    pub(crate) weighted_with: ::rosetta_uuid::Uuid,
    pub(crate) weighted_container_id: ::rosetta_uuid::Uuid,
    pub(crate) kilograms: f32,
}
impl InsertableWeighingProcedure {
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
        crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel::table(),
                self.procedure_model_id,
            ),
            conn,
        )
    }
    pub fn weighted_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel::table(),
                self.weighted_with,
            ),
            conn,
        )
    }
    pub fn weighted_container<C: diesel::connection::LoadConnection>(
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
                self.weighted_container_id,
            ),
            conn,
        )
    }
    pub fn weighing_procedures_procedure_id_weighted_with_fkey<
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
                (self.procedure_id, self.weighted_with),
            ),
            conn,
        )
    }
    pub fn weighing_procedures_procedure_id_weighted_container_id_fkey<
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
                (self.procedure_id, self.weighted_container_id),
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableWeighingProcedureBuilder<
    Procedure = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
> {
    pub(crate) procedure_model_id: Option<i32>,
    pub(crate) weighted_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) weighted_container_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) kilograms: Option<f32>,
    pub(crate) procedure: Procedure,
}
impl<Procedure> web_common_traits::database::ExtendableBuilder
for InsertableWeighingProcedureBuilder<Procedure>
where
    Procedure: web_common_traits::database::ExtendableBuilder<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes,
    >,
{
    type Attributes = InsertableWeighingProcedureAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = self
            .procedure
            .extend_builder(other.procedure)
            .map_err(|err| {
                err.into_field_name(|attribute| InsertableWeighingProcedureAttributes::Extension(
                    InsertableWeighingProcedureExtensionAttributes::Procedure(attribute),
                ))
            })?;
        if let Some(procedure_model_id) = other.procedure_model_id {
            self = self.procedure_model(procedure_model_id)?;
        }
        if let Some(weighted_with) = other.weighted_with {
            self = self.weighted_with(weighted_with)?;
        }
        if let Some(weighted_container_id) = other.weighted_container_id {
            self = self.weighted_container(weighted_container_id)?;
        }
        if let Some(kilograms) = other.kilograms {
            self = self.kilograms(kilograms)?;
        }
        Ok(self)
    }
}
impl<Procedure> web_common_traits::prelude::SetPrimaryKey
    for InsertableWeighingProcedureBuilder<Procedure>
where
    Procedure: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.procedure = self.procedure.set_primary_key(primary_key);
        self
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
    >
{
    /// Sets the value of the `procedures.created_at` column from table
    /// `weighing_procedures`.
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableWeighingProcedureAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure = self.procedure.created_at(created_at).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableWeighingProcedureAttributes::Extension(
                    InsertableWeighingProcedureExtensionAttributes::Procedure(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
    >
{
    /// Sets the value of the `procedures.created_by` column from table
    /// `weighing_procedures`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableWeighingProcedureAttributes>>
    {
        self.procedure = self.procedure.created_by(created_by).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableWeighingProcedureAttributes::Extension(
                    InsertableWeighingProcedureExtensionAttributes::Procedure(attribute),
                )
            })
        })?;
        self = self.updated_by(created_by)?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
    >
{
    /// Sets the value of the `procedures.updated_at` column from table
    /// `weighing_procedures`.
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableWeighingProcedureAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure = self.procedure.updated_at(updated_at).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableWeighingProcedureAttributes::Extension(
                    InsertableWeighingProcedureExtensionAttributes::Procedure(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
    >
{
    /// Sets the value of the `procedures.updated_by` column from table
    /// `weighing_procedures`.
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableWeighingProcedureAttributes>>
    {
        self.procedure = self.procedure.updated_by(updated_by).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableWeighingProcedureAttributes::Extension(
                    InsertableWeighingProcedureExtensionAttributes::Procedure(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl<Procedure>
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder<
        Procedure,
    >
{
    /// Sets the value of the `weighing_procedures.kilograms` column from table
    /// `weighing_procedures`.
    pub fn kilograms<P>(
        mut self,
        kilograms: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableWeighingProcedureAttributes>>
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let kilograms = kilograms.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
            Into::into(err).rename_field(InsertableWeighingProcedureAttributes::Kilograms)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(kilograms)
            .map_err(|e| e.rename_field(InsertableWeighingProcedureAttributes::Kilograms))?;
        self.kilograms = Some(kilograms);
        Ok(self)
    }
}
impl<Procedure>
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder<
        Procedure,
    >
{
    /// Sets the value of the `weighing_procedures.procedure_model_id` column
    /// from table `weighing_procedures`.
    pub fn procedure_model(
        mut self,
        procedure_model_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableWeighingProcedureAttributes>>
    {
        self.procedure_model_id = Some(procedure_model_id);
        Ok(self)
    }
}
impl<Procedure>
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder<
        Procedure,
    >
{
    /// Sets the value of the `weighing_procedures.weighted_container_id` column
    /// from table `weighing_procedures`.
    pub fn weighted_container(
        mut self,
        weighted_container_id: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableWeighingProcedureAttributes>>
    {
        self.weighted_container_id = Some(weighted_container_id);
        Ok(self)
    }
}
impl<Procedure>
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder<
        Procedure,
    >
{
    /// Sets the value of the `weighing_procedures.weighted_with` column from
    /// table `weighing_procedures`.
    pub fn weighted_with(
        mut self,
        weighted_with: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableWeighingProcedureAttributes>>
    {
        self.weighted_with = Some(weighted_with);
        Ok(self)
    }
}
impl<Procedure, C> web_common_traits::database::TryInsertGeneric<C>
    for InsertableWeighingProcedureBuilder<Procedure>
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure,
            Error = web_common_traits::database::InsertError<InsertableWeighingProcedureAttributes>,
        >,
    Procedure: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type Attributes = InsertableWeighingProcedureAttributes;
    fn is_complete(&self) -> bool {
        self.procedure.is_complete()
            && self.procedure_model_id.is_some()
            && self.weighted_with.is_some()
            && self.weighted_container_id.is_some()
            && self.kilograms.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
