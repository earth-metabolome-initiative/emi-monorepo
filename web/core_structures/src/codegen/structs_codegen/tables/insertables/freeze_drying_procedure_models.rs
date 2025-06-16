#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableFreezeDryingProcedureModelAttributes {
    Id(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
    ),
    Kelvin,
    Pascal,
    Seconds,
    FreezeDriedWith(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
    SourceContainer(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
}
impl core::fmt::Display for InsertableFreezeDryingProcedureModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableFreezeDryingProcedureModelAttributes::Id(id) => write!(f, "{}", id),
            InsertableFreezeDryingProcedureModelAttributes::Kelvin => write!(f, "kelvin"),
            InsertableFreezeDryingProcedureModelAttributes::Pascal => write!(f, "pascal"),
            InsertableFreezeDryingProcedureModelAttributes::Seconds => {
                write!(f, "seconds")
            }
            InsertableFreezeDryingProcedureModelAttributes::FreezeDriedWith(freeze_dried_with) => {
                write!(f, "{}", freeze_dried_with)
            }
            InsertableFreezeDryingProcedureModelAttributes::SourceContainer(source_container) => {
                write!(f, "{}", source_container)
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::freeze_drying_procedure_models::freeze_drying_procedure_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableFreezeDryingProcedureModel {
    id: i32,
    kelvin: f32,
    pascal: f32,
    seconds: f32,
    freeze_dried_with: i32,
    source_container: i32,
}
impl InsertableFreezeDryingProcedureModel {
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
    pub fn freeze_dried_with<C: diesel::connection::LoadConnection>(
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
                self.freeze_dried_with,
            ),
            conn,
        )
    }
    pub fn source_container<C: diesel::connection::LoadConnection>(
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
                self.source_container,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableFreezeDryingProcedureModelBuilder {
    pub(crate) id: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    pub(crate) kelvin: Option<f32>,
    pub(crate) pascal: Option<f32>,
    pub(crate) seconds: Option<f32>,
    pub(crate) freeze_dried_with: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    pub(crate) source_container: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
}
impl Default for InsertableFreezeDryingProcedureModelBuilder {
    fn default() -> Self {
        Self {
            id: Default::default(),
            kelvin: Some(213.15f32),
            pascal: Some(4f32),
            seconds: Some(259200f32),
            freeze_dried_with: Default::default(),
            source_container: Default::default(),
        }
    }
}
impl InsertableFreezeDryingProcedureModelBuilder {
    pub fn kelvin<P>(
        mut self,
        kelvin: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezeDryingProcedureModelAttributes>,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let kelvin = kelvin.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
            Into::into(err).rename_field(InsertableFreezeDryingProcedureModelAttributes::Kelvin)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(kelvin)
            .map_err(|e| e.rename_field(InsertableFreezeDryingProcedureModelAttributes::Kelvin))?;
        pgrx_validation::must_be_strictly_smaller_than_f32(kelvin, 250f32)
            .map_err(|e| e.rename_field(InsertableFreezeDryingProcedureModelAttributes::Kelvin))?;
        self.kelvin = Some(kelvin);
        Ok(self)
    }
    pub fn pascal<P>(
        mut self,
        pascal: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezeDryingProcedureModelAttributes>,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let pascal = pascal.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
            Into::into(err).rename_field(InsertableFreezeDryingProcedureModelAttributes::Pascal)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(pascal)
            .map_err(|e| e.rename_field(InsertableFreezeDryingProcedureModelAttributes::Pascal))?;
        pgrx_validation::must_be_strictly_smaller_than_f32(pascal, 500f32)
            .map_err(|e| e.rename_field(InsertableFreezeDryingProcedureModelAttributes::Pascal))?;
        self.pascal = Some(pascal);
        Ok(self)
    }
    pub fn seconds<P>(
        mut self,
        seconds: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezeDryingProcedureModelAttributes>,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let seconds = seconds.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
            Into::into(err).rename_field(InsertableFreezeDryingProcedureModelAttributes::Seconds)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(seconds)
            .map_err(|e| e.rename_field(InsertableFreezeDryingProcedureModelAttributes::Seconds))?;
        pgrx_validation::must_be_strictly_greater_than_f32(seconds, 7200f32)
            .map_err(|e| e.rename_field(InsertableFreezeDryingProcedureModelAttributes::Seconds))?;
        pgrx_validation::must_be_strictly_smaller_than_f32(seconds, 604800f32)
            .map_err(|e| e.rename_field(InsertableFreezeDryingProcedureModelAttributes::Seconds))?;
        self.seconds = Some(seconds);
        Ok(self)
    }
    pub fn freeze_dried_with(
        mut self,
        freeze_dried_with: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezeDryingProcedureModelAttributes>,
    > {
        if freeze_dried_with.procedure_model_id.is_some() {
            return Err(
                web_common_traits::database::InsertError::BuilderError(
                    web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                        InsertableFreezeDryingProcedureModelAttributes::FreezeDriedWith(
                            crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::ProcedureModelId,
                        ),
                    ),
                ),
            );
        }
        self.freeze_dried_with = freeze_dried_with;
        Ok(self)
    }
    pub fn source_container(
        mut self,
        source_container: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezeDryingProcedureModelAttributes>,
    > {
        if source_container.procedure_model_id.is_some() {
            return Err(
                web_common_traits::database::InsertError::BuilderError(
                    web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                        InsertableFreezeDryingProcedureModelAttributes::SourceContainer(
                            crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::ProcedureModelId,
                        ),
                    ),
                ),
            );
        }
        self.source_container = source_container;
        Ok(self)
    }
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezeDryingProcedureModelAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.name(name).map_err(|err| {
            err.into_field_name(InsertableFreezeDryingProcedureModelAttributes::Id)
        })?;
        Ok(self)
    }
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezeDryingProcedureModelAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.description(description).map_err(|err| {
            err.into_field_name(InsertableFreezeDryingProcedureModelAttributes::Id)
        })?;
        Ok(self)
    }
    pub fn deprecated<P>(
        mut self,
        deprecated: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezeDryingProcedureModelAttributes>,
    >
    where
        P: TryInto<bool>,
        <P as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.deprecated(deprecated).map_err(|err| {
            err.into_field_name(InsertableFreezeDryingProcedureModelAttributes::Id)
        })?;
        Ok(self)
    }
    pub fn photograph_id<P>(
        mut self,
        photograph_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezeDryingProcedureModelAttributes>,
    >
    where
        P: TryInto<Option<::rosetta_uuid::Uuid>>,
        <P as TryInto<Option<::rosetta_uuid::Uuid>>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.photograph_id(photograph_id).map_err(|err| {
            err.into_field_name(InsertableFreezeDryingProcedureModelAttributes::Id)
        })?;
        Ok(self)
    }
    pub fn icon<P>(
        mut self,
        icon: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezeDryingProcedureModelAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.icon(icon).map_err(|err| {
            err.into_field_name(InsertableFreezeDryingProcedureModelAttributes::Id)
        })?;
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezeDryingProcedureModelAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.created_by(created_by).map_err(|err| {
            err.into_field_name(InsertableFreezeDryingProcedureModelAttributes::Id)
        })?;
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezeDryingProcedureModelAttributes>,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.created_at(created_at).map_err(|err| {
            err.into_field_name(InsertableFreezeDryingProcedureModelAttributes::Id)
        })?;
        Ok(self)
    }
    pub fn updated_by<P>(
        mut self,
        updated_by: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezeDryingProcedureModelAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.updated_by(updated_by).map_err(|err| {
            err.into_field_name(InsertableFreezeDryingProcedureModelAttributes::Id)
        })?;
        Ok(self)
    }
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezeDryingProcedureModelAttributes>,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.updated_at(updated_at).map_err(|err| {
            err.into_field_name(InsertableFreezeDryingProcedureModelAttributes::Id)
        })?;
        Ok(self)
    }
}
impl InsertableFreezeDryingProcedureModelBuilder {
    pub(crate) fn try_insert<C>(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        InsertableFreezeDryingProcedureModel,
        web_common_traits::database::InsertError<
            InsertableFreezeDryingProcedureModelAttributes,
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
        let kelvin = self.kelvin.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
            InsertableFreezeDryingProcedureModelAttributes::Kelvin,
        ))?;
        let pascal = self.pascal.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
            InsertableFreezeDryingProcedureModelAttributes::Pascal,
        ))?;
        let seconds = self.seconds.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
            InsertableFreezeDryingProcedureModelAttributes::Seconds,
        ))?;
        let id = self
            .id
            .insert(user_id, conn)
            .map_err(|err| err.into_field_name(InsertableFreezeDryingProcedureModelAttributes::Id))?
            .id();
        let freeze_dried_with = self
            .freeze_dried_with
            .procedure_model_id(id)
            .map_err(|err| {
                err.into_field_name(InsertableFreezeDryingProcedureModelAttributes::FreezeDriedWith)
            })?
            .insert(user_id, conn)
            .map_err(|err| {
                err.into_field_name(InsertableFreezeDryingProcedureModelAttributes::FreezeDriedWith)
            })?
            .id();
        let source_container = self
            .source_container
            .procedure_model_id(id)
            .map_err(|err| {
                err.into_field_name(InsertableFreezeDryingProcedureModelAttributes::SourceContainer)
            })?
            .insert(user_id, conn)
            .map_err(|err| {
                err.into_field_name(InsertableFreezeDryingProcedureModelAttributes::SourceContainer)
            })?
            .id();
        Ok(InsertableFreezeDryingProcedureModel {
            id,
            kelvin,
            pascal,
            seconds,
            freeze_dried_with,
            source_container,
        })
    }
}
