#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCentrifugeProcedureModelAttributes {
    Id(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
    ),
    Seconds,
    RotationPerMinute,
    CentrifugedWith(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
    ContainerId(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
}
impl core::fmt::Display for InsertableCentrifugeProcedureModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableCentrifugeProcedureModelAttributes::Id(id) => write!(f, "{}", id),
            InsertableCentrifugeProcedureModelAttributes::Seconds => write!(f, "seconds"),
            InsertableCentrifugeProcedureModelAttributes::RotationPerMinute => {
                write!(f, "rotation_per_minute")
            }
            InsertableCentrifugeProcedureModelAttributes::CentrifugedWith(centrifuged_with) => {
                write!(f, "{}", centrifuged_with)
            }
            InsertableCentrifugeProcedureModelAttributes::ContainerId(container_id) => {
                write!(f, "{}", container_id)
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::centrifuge_procedure_models::centrifuge_procedure_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCentrifugeProcedureModel {
    id: i32,
    seconds: f32,
    rotation_per_minute: f32,
    centrifuged_with: i32,
    container_id: i32,
}
impl InsertableCentrifugeProcedureModel {
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
    pub fn centrifuged_with<C: diesel::connection::LoadConnection>(
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
                self.centrifuged_with,
            ),
            conn,
        )
    }
    pub fn container<C: diesel::connection::LoadConnection>(
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
                self.container_id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCentrifugeProcedureModelBuilder {
    pub(crate) id: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    pub(crate) seconds: Option<f32>,
    pub(crate) rotation_per_minute: Option<f32>,
    pub(crate) centrifuged_with: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    pub(crate) container_id: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
}
impl Default for InsertableCentrifugeProcedureModelBuilder {
    fn default() -> Self {
        Self {
            id: Default::default(),
            seconds: Some(120f32),
            rotation_per_minute: Some(13000f32),
            centrifuged_with: Default::default(),
            container_id: Default::default(),
        }
    }
}
impl InsertableCentrifugeProcedureModelBuilder {
    pub fn seconds<P>(
        mut self,
        seconds: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let seconds = seconds.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
            Into::into(err).rename_field(InsertableCentrifugeProcedureModelAttributes::Seconds)
        })?;
        pgrx_validation::must_be_strictly_smaller_than_f32(seconds, 1800f32)
            .map_err(|e| e.rename_field(InsertableCentrifugeProcedureModelAttributes::Seconds))?;
        pgrx_validation::must_be_greater_than_f32(seconds, 30f32)
            .map_err(|e| e.rename_field(InsertableCentrifugeProcedureModelAttributes::Seconds))?;
        self.seconds = Some(seconds);
        Ok(self)
    }
    pub fn rotation_per_minute<P>(
        mut self,
        rotation_per_minute: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let rotation_per_minute =
            rotation_per_minute.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
                Into::into(err)
                    .rename_field(InsertableCentrifugeProcedureModelAttributes::RotationPerMinute)
            })?;
        pgrx_validation::must_be_smaller_than_f32(rotation_per_minute, 30000f32).map_err(|e| {
            e.rename_field(InsertableCentrifugeProcedureModelAttributes::RotationPerMinute)
        })?;
        pgrx_validation::must_be_greater_than_f32(rotation_per_minute, 5000f32).map_err(|e| {
            e.rename_field(InsertableCentrifugeProcedureModelAttributes::RotationPerMinute)
        })?;
        self.rotation_per_minute = Some(rotation_per_minute);
        Ok(self)
    }
    pub fn centrifuged_with(
        mut self,
        centrifuged_with: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    > {
        if centrifuged_with.procedure_model_id.is_some() {
            return Err(
                web_common_traits::database::InsertError::BuilderError(
                    web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                        InsertableCentrifugeProcedureModelAttributes::CentrifugedWith(
                            crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::ProcedureModelId,
                        ),
                    ),
                ),
            );
        }
        self.centrifuged_with = centrifuged_with;
        Ok(self)
    }
    pub fn container_id(
        mut self,
        container_id: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    > {
        if container_id.procedure_model_id.is_some() {
            return Err(
                web_common_traits::database::InsertError::BuilderError(
                    web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                        InsertableCentrifugeProcedureModelAttributes::ContainerId(
                            crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::ProcedureModelId,
                        ),
                    ),
                ),
            );
        }
        self.container_id = container_id;
        Ok(self)
    }
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self
            .id
            .name(name)
            .map_err(|err| err.into_field_name(InsertableCentrifugeProcedureModelAttributes::Id))?;
        Ok(self)
    }
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self
            .id
            .description(description)
            .map_err(|err| err.into_field_name(InsertableCentrifugeProcedureModelAttributes::Id))?;
        Ok(self)
    }
    pub fn deprecated<P>(
        mut self,
        deprecated: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        P: TryInto<bool>,
        <P as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self
            .id
            .deprecated(deprecated)
            .map_err(|err| err.into_field_name(InsertableCentrifugeProcedureModelAttributes::Id))?;
        Ok(self)
    }
    pub fn photograph_id<P>(
        mut self,
        photograph_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        P: TryInto<Option<::rosetta_uuid::Uuid>>,
        <P as TryInto<Option<::rosetta_uuid::Uuid>>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self
            .id
            .photograph_id(photograph_id)
            .map_err(|err| err.into_field_name(InsertableCentrifugeProcedureModelAttributes::Id))?;
        Ok(self)
    }
    pub fn icon<P>(
        mut self,
        icon: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self
            .id
            .icon(icon)
            .map_err(|err| err.into_field_name(InsertableCentrifugeProcedureModelAttributes::Id))?;
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self
            .id
            .created_by(created_by)
            .map_err(|err| err.into_field_name(InsertableCentrifugeProcedureModelAttributes::Id))?;
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self
            .id
            .created_at(created_at)
            .map_err(|err| err.into_field_name(InsertableCentrifugeProcedureModelAttributes::Id))?;
        Ok(self)
    }
    pub fn updated_by<P>(
        mut self,
        updated_by: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self
            .id
            .updated_by(updated_by)
            .map_err(|err| err.into_field_name(InsertableCentrifugeProcedureModelAttributes::Id))?;
        Ok(self)
    }
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self
            .id
            .updated_at(updated_at)
            .map_err(|err| err.into_field_name(InsertableCentrifugeProcedureModelAttributes::Id))?;
        Ok(self)
    }
}
impl InsertableCentrifugeProcedureModelBuilder {
    pub(crate) fn try_insert<C>(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        InsertableCentrifugeProcedureModel,
        web_common_traits::database::InsertError<
            InsertableCentrifugeProcedureModelAttributes,
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
        let seconds = self.seconds.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
            InsertableCentrifugeProcedureModelAttributes::Seconds,
        ))?;
        let rotation_per_minute = self.rotation_per_minute.ok_or(
            common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableCentrifugeProcedureModelAttributes::RotationPerMinute,
            ),
        )?;
        let id = self
            .id
            .insert(user_id, conn)
            .map_err(|err| err.into_field_name(InsertableCentrifugeProcedureModelAttributes::Id))?
            .id();
        let centrifuged_with = self
            .centrifuged_with
            .procedure_model_id(id)
            .map_err(|err| {
                err.into_field_name(InsertableCentrifugeProcedureModelAttributes::CentrifugedWith)
            })?
            .insert(user_id, conn)
            .map_err(|err| {
                err.into_field_name(InsertableCentrifugeProcedureModelAttributes::CentrifugedWith)
            })?
            .id();
        let container_id = self
            .container_id
            .procedure_model_id(id)
            .map_err(|err| {
                err.into_field_name(InsertableCentrifugeProcedureModelAttributes::ContainerId)
            })?
            .insert(user_id, conn)
            .map_err(|err| {
                err.into_field_name(InsertableCentrifugeProcedureModelAttributes::ContainerId)
            })?
            .id();
        Ok(InsertableCentrifugeProcedureModel {
            id,
            seconds,
            rotation_per_minute,
            centrifuged_with,
            container_id,
        })
    }
}
