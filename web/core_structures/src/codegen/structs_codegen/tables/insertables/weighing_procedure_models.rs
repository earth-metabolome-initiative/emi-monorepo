#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableWeighingProcedureModelAttributes {
    Id(crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes),
    Kilograms,
    InstrumentId,
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes>
    for InsertableWeighingProcedureModelAttributes
{
    fn from(
        extension: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
    ) -> Self {
        Self::Id(extension)
    }
}
impl core::fmt::Display for InsertableWeighingProcedureModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableWeighingProcedureModelAttributes::Id(id) => write!(f, "{}", id),
            InsertableWeighingProcedureModelAttributes::Kilograms => {
                write!(f, "kilograms")
            }
            InsertableWeighingProcedureModelAttributes::InstrumentId => {
                write!(f, "instrument_id")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::weighing_procedure_models::weighing_procedure_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableWeighingProcedureModel {
    id: i32,
    kilograms: f32,
    instrument_id: i32,
}
impl InsertableWeighingProcedureModel {
    pub fn id<C: diesel::connection::LoadConnection>(
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
                self.id,
            ),
            conn,
        )
    }
    pub fn instrument<C: diesel::connection::LoadConnection>(
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
                self.instrument_id,
            ),
            conn,
        )
    }
}
#[derive(Default)]
pub struct InsertableWeighingProcedureModelBuilder {
    pub(crate) id:
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    pub(crate) kilograms: Option<f32>,
    pub(crate) instrument_id: Option<i32>,
}
impl InsertableWeighingProcedureModelBuilder {
    pub fn kilograms<P>(
        mut self,
        kilograms: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let kilograms = kilograms.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
            Into::into(err).rename_field(InsertableWeighingProcedureModelAttributes::Kilograms)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(kilograms)
            .map_err(|e| e.rename_field(InsertableWeighingProcedureModelAttributes::Kilograms))?;
        self.kilograms = Some(kilograms);
        Ok(self)
    }
    pub fn instrument_id<P>(
        mut self,
        instrument_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let instrument_id =
            instrument_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
                Into::into(err)
                    .rename_field(InsertableWeighingProcedureModelAttributes::InstrumentId)
            })?;
        self.instrument_id = Some(instrument_id);
        Ok(self)
    }
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.name(name).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.description(description).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn deprecated<P>(
        mut self,
        deprecated: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
    >
    where
        P: TryInto<bool>,
        <P as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.deprecated(deprecated).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn photograph_id<P>(
        mut self,
        photograph_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
    >
    where
        P: TryInto<Option<::rosetta_uuid::Uuid>>,
        <P as TryInto<Option<::rosetta_uuid::Uuid>>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.photograph_id(photograph_id).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn icon<P>(
        mut self,
        icon: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.icon(icon).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.created_by(created_by).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.created_at(created_at).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn updated_by<P>(
        mut self,
        updated_by: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.updated_by(updated_by).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.updated_at(updated_at).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
}
impl InsertableWeighingProcedureModelBuilder {
    pub(crate) fn try_insert<C>(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        InsertableWeighingProcedureModel,
        web_common_traits::database::InsertError<
            InsertableWeighingProcedureModelAttributes,
        >,
    >
    where
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
            >,
        >,
    {
        use diesel::associations::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let kilograms =
            self.kilograms.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableWeighingProcedureModelAttributes::Kilograms,
            ))?;
        let instrument_id =
            self.instrument_id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableWeighingProcedureModelAttributes::InstrumentId,
            ))?;
        let id = self.id.insert(user_id, conn).map_err(|err| err.into_field_name())?.id();
        Ok(InsertableWeighingProcedureModel { id, kilograms, instrument_id })
    }
}
