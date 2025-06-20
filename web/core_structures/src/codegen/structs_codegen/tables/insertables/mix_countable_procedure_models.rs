#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableMixCountableProcedureModelAttributes {
    ProcedureModelId(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
    ),
    Source(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
    Destination(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
    Quantity,
}
impl core::fmt::Display for InsertableMixCountableProcedureModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableMixCountableProcedureModelAttributes::ProcedureModelId(
                procedure_model_id,
            ) => write!(f, "{}", procedure_model_id),
            InsertableMixCountableProcedureModelAttributes::Source(source) => {
                write!(f, "{}", source)
            }
            InsertableMixCountableProcedureModelAttributes::Destination(destination) => {
                write!(f, "{}", destination)
            }
            InsertableMixCountableProcedureModelAttributes::Quantity => {
                write!(f, "quantity")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::mix_countable_procedure_models::mix_countable_procedure_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableMixCountableProcedureModel {
    procedure_model_id: i32,
    source: i32,
    destination: i32,
    quantity: i16,
}
impl InsertableMixCountableProcedureModel {
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
    pub fn source<C: diesel::connection::LoadConnection>(
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
                self.source,
            ),
            conn,
        )
    }
    pub fn destination<C: diesel::connection::LoadConnection>(
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
                self.destination,
            ),
            conn,
        )
    }
}
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableMixCountableProcedureModelBuilder {
    pub(crate) procedure_model_id: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    pub(crate) source: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    pub(crate) destination: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    pub(crate) quantity: Option<i16>,
}
impl InsertableMixCountableProcedureModelBuilder {
    pub fn quantity<P>(
        mut self,
        quantity: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableMixCountableProcedureModelAttributes>,
    >
    where
        P: TryInto<i16>,
        <P as TryInto<i16>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let quantity = quantity.try_into().map_err(|err: <P as TryInto<i16>>::Error| {
            Into::into(err).rename_field(InsertableMixCountableProcedureModelAttributes::Quantity)
        })?;
        pgrx_validation::must_be_strictly_positive_i16(quantity).map_err(|e| {
            e.rename_field(InsertableMixCountableProcedureModelAttributes::Quantity)
        })?;
        self.quantity = Some(quantity);
        Ok(self)
    }
    pub fn destination(
        mut self,
        destination: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableMixCountableProcedureModelAttributes>,
    > {
        if destination.procedure_model_id.is_some() {
            return Err(
                web_common_traits::database::InsertError::BuilderError(
                    web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                        InsertableMixCountableProcedureModelAttributes::Destination(
                            crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::ProcedureModelId,
                        ),
                    ),
                ),
            );
        }
        self.destination = destination;
        Ok(self)
    }
    pub fn source(
        mut self,
        source: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableMixCountableProcedureModelAttributes>,
    > {
        if source.procedure_model_id.is_some() {
            return Err(
                web_common_traits::database::InsertError::BuilderError(
                    web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                        InsertableMixCountableProcedureModelAttributes::Source(
                            crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::ProcedureModelId,
                        ),
                    ),
                ),
            );
        }
        self.source = source;
        Ok(self)
    }
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableMixCountableProcedureModelAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id = self.procedure_model_id.name(name).map_err(|err| {
            err.into_field_name(InsertableMixCountableProcedureModelAttributes::ProcedureModelId)
        })?;
        Ok(self)
    }
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableMixCountableProcedureModelAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id =
            self.procedure_model_id.description(description).map_err(|err| {
                err.into_field_name(
                    InsertableMixCountableProcedureModelAttributes::ProcedureModelId,
                )
            })?;
        Ok(self)
    }
    pub fn deprecated<P>(
        mut self,
        deprecated: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableMixCountableProcedureModelAttributes>,
    >
    where
        P: TryInto<bool>,
        <P as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id =
            self.procedure_model_id.deprecated(deprecated).map_err(|err| {
                err.into_field_name(
                    InsertableMixCountableProcedureModelAttributes::ProcedureModelId,
                )
            })?;
        Ok(self)
    }
    pub fn photograph_id<P>(
        mut self,
        photograph_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableMixCountableProcedureModelAttributes>,
    >
    where
        P: TryInto<Option<::rosetta_uuid::Uuid>>,
        <P as TryInto<Option<::rosetta_uuid::Uuid>>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id =
            self.procedure_model_id.photograph_id(photograph_id).map_err(|err| {
                err.into_field_name(
                    InsertableMixCountableProcedureModelAttributes::ProcedureModelId,
                )
            })?;
        Ok(self)
    }
    pub fn icon<P>(
        mut self,
        icon: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableMixCountableProcedureModelAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id = self.procedure_model_id.icon(icon).map_err(|err| {
            err.into_field_name(InsertableMixCountableProcedureModelAttributes::ProcedureModelId)
        })?;
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableMixCountableProcedureModelAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id =
            self.procedure_model_id.created_by(created_by).map_err(|err| {
                err.into_field_name(
                    InsertableMixCountableProcedureModelAttributes::ProcedureModelId,
                )
            })?;
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableMixCountableProcedureModelAttributes>,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id =
            self.procedure_model_id.created_at(created_at).map_err(|err| {
                err.into_field_name(
                    InsertableMixCountableProcedureModelAttributes::ProcedureModelId,
                )
            })?;
        Ok(self)
    }
    pub fn updated_by<P>(
        mut self,
        updated_by: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableMixCountableProcedureModelAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id =
            self.procedure_model_id.updated_by(updated_by).map_err(|err| {
                err.into_field_name(
                    InsertableMixCountableProcedureModelAttributes::ProcedureModelId,
                )
            })?;
        Ok(self)
    }
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableMixCountableProcedureModelAttributes>,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id =
            self.procedure_model_id.updated_at(updated_at).map_err(|err| {
                err.into_field_name(
                    InsertableMixCountableProcedureModelAttributes::ProcedureModelId,
                )
            })?;
        Ok(self)
    }
}
impl InsertableMixCountableProcedureModelBuilder {
    pub(crate) fn try_insert<C>(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        InsertableMixCountableProcedureModel,
        web_common_traits::database::InsertError<
            InsertableMixCountableProcedureModelAttributes,
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
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
            >,
        >,
    {
        use diesel::associations::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let quantity =
            self.quantity.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableMixCountableProcedureModelAttributes::Quantity,
            ))?;
        let procedure_model_id = self
            .procedure_model_id
            .insert(user_id, conn)
            .map_err(|err| {
                err.into_field_name(
                    InsertableMixCountableProcedureModelAttributes::ProcedureModelId,
                )
            })?
            .id();
        let destination = self
            .destination
            .procedure_model_id(procedure_model_id)
            .map_err(|err| {
                err.into_field_name(InsertableMixCountableProcedureModelAttributes::Destination)
            })?
            .insert(user_id, conn)
            .map_err(|err| {
                err.into_field_name(InsertableMixCountableProcedureModelAttributes::Destination)
            })?
            .id();
        let source = self
            .source
            .procedure_model_id(procedure_model_id)
            .map_err(|err| {
                err.into_field_name(InsertableMixCountableProcedureModelAttributes::Source)
            })?
            .insert(user_id, conn)
            .map_err(|err| {
                err.into_field_name(InsertableMixCountableProcedureModelAttributes::Source)
            })?
            .id();
        Ok(InsertableMixCountableProcedureModel {
            procedure_model_id,
            source,
            destination,
            quantity,
        })
    }
}
