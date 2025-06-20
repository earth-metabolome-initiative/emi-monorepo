#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCentrifugeProcedureModelAttributes {
    ProcedureModelId(
        crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureModelAttributes,
    ),
    Seconds,
    RotationPerMinute,
    CentrifugedWith,
}
impl core::fmt::Display for InsertableCentrifugeProcedureModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableCentrifugeProcedureModelAttributes::ProcedureModelId(procedure_model_id) => {
                write!(f, "{}", procedure_model_id)
            }
            InsertableCentrifugeProcedureModelAttributes::Seconds => write!(f, "seconds"),
            InsertableCentrifugeProcedureModelAttributes::RotationPerMinute => {
                write!(f, "rotation_per_minute")
            }
            InsertableCentrifugeProcedureModelAttributes::CentrifugedWith => {
                write!(f, "centrifuged_with")
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
    procedure_model_id: i32,
    seconds: f32,
    rotation_per_minute: f32,
    centrifuged_with: ::rosetta_uuid::Uuid,
}
impl InsertableCentrifugeProcedureModel {
    pub fn procedure_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel::table(),
                self.procedure_model_id,
            ),
            conn,
        )
    }
    pub fn centrifuged_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel::table(
                ),
                self.centrifuged_with,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCentrifugeProcedureModelBuilder {
    pub(crate) procedure_model_id: crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureModelBuilder,
    pub(crate) seconds: Option<f32>,
    pub(crate) rotation_per_minute: Option<f32>,
    pub(crate) centrifuged_with: Option<::rosetta_uuid::Uuid>,
}
impl Default for InsertableCentrifugeProcedureModelBuilder {
    fn default() -> Self {
        Self {
            procedure_model_id: Default::default(),
            seconds: Some(120f32),
            rotation_per_minute: Some(13000f32),
            centrifuged_with: Default::default(),
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
        pgrx_validation::must_be_greater_than_f32(seconds, 30f32)
            .map_err(|e| e.rename_field(InsertableCentrifugeProcedureModelAttributes::Seconds))
            .and_then(|_| {
                pgrx_validation::must_be_smaller_than_f32(seconds, 1800f32).map_err(|e| {
                    e.rename_field(InsertableCentrifugeProcedureModelAttributes::Seconds)
                })
            })?;
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
        pgrx_validation::must_be_greater_than_f32(rotation_per_minute, 5000f32)
            .map_err(|e| {
                e.rename_field(InsertableCentrifugeProcedureModelAttributes::RotationPerMinute)
            })
            .and_then(|_| {
                pgrx_validation::must_be_smaller_than_f32(rotation_per_minute, 30000f32).map_err(
                    |e| {
                        e.rename_field(
                            InsertableCentrifugeProcedureModelAttributes::RotationPerMinute,
                        )
                    },
                )
            })?;
        self.rotation_per_minute = Some(rotation_per_minute);
        Ok(self)
    }
    pub fn centrifuged_with<P>(
        mut self,
        centrifuged_with: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let centrifuged_with = centrifuged_with.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err)
                    .rename_field(InsertableCentrifugeProcedureModelAttributes::CentrifugedWith)
            },
        )?;
        self.centrifuged_with = Some(centrifuged_with);
        self.procedure_model_id =
            self.procedure_model_id.parent_container_id(centrifuged_with).map_err(|err| {
                err.into_field_name(InsertableCentrifugeProcedureModelAttributes::ProcedureModelId)
            })?;
        Ok(self)
    }
    pub fn kelvin<P>(
        mut self,
        kelvin: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id = self.procedure_model_id.kelvin(kelvin).map_err(|err| {
            err.into_field_name(InsertableCentrifugeProcedureModelAttributes::ProcedureModelId)
        })?;
        Ok(self)
    }
    pub fn kelvin_tolerance_percentage<P>(
        mut self,
        kelvin_tolerance_percentage: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id = self
            .procedure_model_id
            .kelvin_tolerance_percentage(kelvin_tolerance_percentage)
            .map_err(|err| {
                err.into_field_name(InsertableCentrifugeProcedureModelAttributes::ProcedureModelId)
            })?;
        Ok(self)
    }
    pub fn procedure_parent_container_id(
        mut self,
        procedure_parent_container_id: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    > {
        self.procedure_model_id = self
            .procedure_model_id
            .procedure_parent_container_id(procedure_parent_container_id)
            .map_err(|err| {
                err.into_field_name(InsertableCentrifugeProcedureModelAttributes::ProcedureModelId)
            })?;
        Ok(self)
    }
    pub fn child_container_id<P>(
        mut self,
        child_container_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id =
            self.procedure_model_id.child_container_id(child_container_id).map_err(|err| {
                err.into_field_name(InsertableCentrifugeProcedureModelAttributes::ProcedureModelId)
            })?;
        Ok(self)
    }
    pub fn procedure_child_container_id(
        mut self,
        procedure_child_container_id: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    > {
        self.procedure_model_id = self
            .procedure_model_id
            .procedure_child_container_id(procedure_child_container_id)
            .map_err(|err| {
                err.into_field_name(InsertableCentrifugeProcedureModelAttributes::ProcedureModelId)
            })?;
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
        self.procedure_model_id = self.procedure_model_id.name(name).map_err(|err| {
            err.into_field_name(InsertableCentrifugeProcedureModelAttributes::ProcedureModelId)
        })?;
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
        self.procedure_model_id =
            self.procedure_model_id.description(description).map_err(|err| {
                err.into_field_name(InsertableCentrifugeProcedureModelAttributes::ProcedureModelId)
            })?;
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
        self.procedure_model_id =
            self.procedure_model_id.deprecated(deprecated).map_err(|err| {
                err.into_field_name(InsertableCentrifugeProcedureModelAttributes::ProcedureModelId)
            })?;
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
        self.procedure_model_id =
            self.procedure_model_id.photograph_id(photograph_id).map_err(|err| {
                err.into_field_name(InsertableCentrifugeProcedureModelAttributes::ProcedureModelId)
            })?;
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
        self.procedure_model_id = self.procedure_model_id.icon(icon).map_err(|err| {
            err.into_field_name(InsertableCentrifugeProcedureModelAttributes::ProcedureModelId)
        })?;
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
        self.procedure_model_id =
            self.procedure_model_id.created_by(created_by).map_err(|err| {
                err.into_field_name(InsertableCentrifugeProcedureModelAttributes::ProcedureModelId)
            })?;
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
        self.procedure_model_id =
            self.procedure_model_id.created_at(created_at).map_err(|err| {
                err.into_field_name(InsertableCentrifugeProcedureModelAttributes::ProcedureModelId)
            })?;
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
        self.procedure_model_id =
            self.procedure_model_id.updated_by(updated_by).map_err(|err| {
                err.into_field_name(InsertableCentrifugeProcedureModelAttributes::ProcedureModelId)
            })?;
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
        self.procedure_model_id =
            self.procedure_model_id.updated_at(updated_at).map_err(|err| {
                err.into_field_name(InsertableCentrifugeProcedureModelAttributes::ProcedureModelId)
            })?;
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
        crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureModelBuilder: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel,
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureModelAttributes,
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
        let centrifuged_with =
            self.centrifuged_with.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableCentrifugeProcedureModelAttributes::CentrifugedWith,
            ))?;
        let procedure_model_id = self
            .procedure_model_id
            .insert(user_id, conn)
            .map_err(|err| {
                err.into_field_name(InsertableCentrifugeProcedureModelAttributes::ProcedureModelId)
            })?
            .id();
        Ok(InsertableCentrifugeProcedureModel {
            procedure_model_id,
            seconds,
            rotation_per_minute,
            centrifuged_with,
        })
    }
}
