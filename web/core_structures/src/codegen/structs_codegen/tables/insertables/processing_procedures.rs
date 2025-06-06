#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableProcessingProcedureAttributes {
    ProcedureId(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes,
    ),
    ProcessableId,
    InstrumentId,
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes>
    for InsertableProcessingProcedureAttributes
{
    fn from(
        extension: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes,
    ) -> Self {
        Self::ProcedureId(extension)
    }
}
impl core::fmt::Display for InsertableProcessingProcedureAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableProcessingProcedureAttributes::ProcedureId(procedure_id) => {
                write!(f, "{}", procedure_id)
            }
            InsertableProcessingProcedureAttributes::ProcessableId => {
                write!(f, "processable_id")
            }
            InsertableProcessingProcedureAttributes::InstrumentId => {
                write!(f, "instrument_id")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::processing_procedures::processing_procedures
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableProcessingProcedure {
    procedure_id: ::rosetta_uuid::Uuid,
    processable_id: ::rosetta_uuid::Uuid,
    instrument_id: ::rosetta_uuid::Uuid,
}
impl InsertableProcessingProcedure {
    pub fn instrument<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::instruments::Instrument,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::instruments::Instrument: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::instruments::Instrument as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::instruments::Instrument as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::instruments::Instrument as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::instruments::Instrument as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::instruments::Instrument as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::instruments::Instrument as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::instruments::Instrument,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::instruments::Instrument::table(),
                self.instrument_id,
            ),
            conn,
        )
    }
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
    pub fn processing_procedures_procedure_id_instrument_id_fkey<
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
                (self.procedure_id, self.instrument_id),
            ),
            conn,
        )
    }
    pub fn processing_procedures_procedure_id_processable_id_fkey<
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
                (self.procedure_id, self.processable_id),
            ),
            conn,
        )
    }
    pub fn processable<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::processables::Processable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::processables::Processable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::processables::Processable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::processables::Processable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::processables::Processable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::processables::Processable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::processables::Processable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::processables::Processable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::processables::Processable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::processables::Processable::table(),
                self.processable_id,
            ),
            conn,
        )
    }
}
#[derive(Default)]
pub struct InsertableProcessingProcedureBuilder {
    procedure_id: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
    processable_id: Option<::rosetta_uuid::Uuid>,
    instrument_id: Option<::rosetta_uuid::Uuid>,
}
impl InsertableProcessingProcedureBuilder {
    pub fn processable_id<P>(
        mut self,
        processable_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcessingProcedureAttributes>,
    >
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let processable_id = processable_id.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(InsertableProcessingProcedureAttributes::ProcessableId)
            },
        )?;
        self.processable_id = Some(processable_id);
        Ok(self)
    }
    pub fn instrument_id<P>(
        mut self,
        instrument_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcessingProcedureAttributes>,
    >
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let instrument_id = instrument_id.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(InsertableProcessingProcedureAttributes::InstrumentId)
            },
        )?;
        self.instrument_id = Some(instrument_id);
        Ok(self)
    }
    pub fn id<P>(
        mut self,
        id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcessingProcedureAttributes>,
    >
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_id = self.procedure_id.id(id).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn procedure_model_id<P>(
        mut self,
        procedure_model_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcessingProcedureAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_id = self
            .procedure_id
            .procedure_model_id(procedure_model_id)
            .map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcessingProcedureAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_id =
            self.procedure_id.created_by(created_by).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcessingProcedureAttributes>,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure_id =
            self.procedure_id.created_at(created_at).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn updated_by<P>(
        mut self,
        updated_by: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcessingProcedureAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_id =
            self.procedure_id.updated_by(updated_by).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcessingProcedureAttributes>,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure_id =
            self.procedure_id.updated_at(updated_at).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
}
impl InsertableProcessingProcedureBuilder {
    pub(crate) fn try_insert<C>(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        InsertableProcessingProcedure,
        web_common_traits::database::InsertError<InsertableProcessingProcedureAttributes>,
    >
    where
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::procedures::Procedure,
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes,
            >,
        >,
    {
        use diesel::associations::Identifiable;
        use web_common_traits::database::InsertableVariant;
        Ok(InsertableProcessingProcedure {
            processable_id: self.processable_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcessingProcedureAttributes::ProcessableId,
                ),
            )?,
            instrument_id: self.instrument_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcessingProcedureAttributes::InstrumentId,
                ),
            )?,
            procedure_id: self
                .procedure_id
                .insert(user_id, conn)
                .map_err(|err| err.into_field_name())?
                .id(),
        })
    }
}
