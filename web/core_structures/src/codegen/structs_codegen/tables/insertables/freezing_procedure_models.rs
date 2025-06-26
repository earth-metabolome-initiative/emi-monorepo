#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableFreezingProcedureModelAttributes {
    ProcedureModelId(
        crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureModelAttributes,
    ),
    Seconds,
    FrozenWith,
}
impl core::fmt::Display for InsertableFreezingProcedureModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableFreezingProcedureModelAttributes::ProcedureModelId(procedure_model_id) => {
                write!(f, "{}", procedure_model_id)
            }
            InsertableFreezingProcedureModelAttributes::Seconds => write!(f, "seconds"),
            InsertableFreezingProcedureModelAttributes::FrozenWith => {
                write!(f, "frozen_with")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::freezing_procedure_models::freezing_procedure_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableFreezingProcedureModel {
    procedure_model_id: i32,
    seconds: Option<f32>,
    frozen_with: ::rosetta_uuid::Uuid,
}
impl InsertableFreezingProcedureModel {
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
    pub fn frozen_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::freezer_models::FreezerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::freezer_models::FreezerModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::freezer_models::FreezerModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freezer_models::FreezerModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::freezer_models::FreezerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freezer_models::FreezerModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::freezer_models::FreezerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freezer_models::FreezerModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::freezer_models::FreezerModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::freezer_models::FreezerModel::table(),
                self.frozen_with,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableFreezingProcedureModelBuilder {
    pub(crate) procedure_model_id: crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureModelBuilder,
    pub(crate) seconds: Option<f32>,
    pub(crate) frozen_with: Option<::rosetta_uuid::Uuid>,
}
impl Default for InsertableFreezingProcedureModelBuilder {
    fn default() -> Self {
        Self {
            procedure_model_id: Default::default(),
            seconds: Some(43200f32),
            frozen_with: Default::default(),
        }
    }
}
impl InsertableFreezingProcedureModelBuilder {
    pub fn seconds<P>(
        mut self,
        seconds: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezingProcedureModelAttributes>,
    >
    where
        P: TryInto<Option<f32>>,
        <P as TryInto<Option<f32>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let seconds = seconds.try_into().map_err(|err: <P as TryInto<Option<f32>>>::Error| {
            Into::into(err).rename_field(InsertableFreezingProcedureModelAttributes::Seconds)
        })?;
        if let Some(seconds) = seconds {
            pgrx_validation::must_be_strictly_positive_f32(seconds)
                .map_err(|e| e.rename_field(InsertableFreezingProcedureModelAttributes::Seconds))
                .and_then(|_| {
                    pgrx_validation::must_be_strictly_greater_than_f32(seconds, 1800f32).map_err(
                        |e| e.rename_field(InsertableFreezingProcedureModelAttributes::Seconds),
                    )
                })?;
        }
        self.seconds = seconds;
        Ok(self)
    }
    pub fn frozen_with<P>(
        mut self,
        frozen_with: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezingProcedureModelAttributes>,
    >
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let frozen_with = frozen_with.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(InsertableFreezingProcedureModelAttributes::FrozenWith)
            },
        )?;
        self.frozen_with = Some(frozen_with);
        self.procedure_model_id =
            self.procedure_model_id.parent_container_id(frozen_with).map_err(|err| {
                err.into_field_name(InsertableFreezingProcedureModelAttributes::ProcedureModelId)
            })?;
        Ok(self)
    }
    pub fn kelvin<P>(
        mut self,
        kelvin: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezingProcedureModelAttributes>,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id = self.procedure_model_id.kelvin(kelvin).map_err(|err| {
            err.into_field_name(InsertableFreezingProcedureModelAttributes::ProcedureModelId)
        })?;
        Ok(self)
    }
    pub fn kelvin_tolerance_percentage<P>(
        mut self,
        kelvin_tolerance_percentage: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezingProcedureModelAttributes>,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id = self
            .procedure_model_id
            .kelvin_tolerance_percentage(kelvin_tolerance_percentage)
            .map_err(|err| {
                err.into_field_name(InsertableFreezingProcedureModelAttributes::ProcedureModelId)
            })?;
        Ok(self)
    }
    pub fn procedure_parent_container_id(
        mut self,
        procedure_parent_container_id: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezingProcedureModelAttributes>,
    > {
        if let (Some(local), Some(foreign)) =
            (self.frozen_with, procedure_parent_container_id.trackable_id)
        {
            if local != foreign {
                return Err(
                    web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            InsertableFreezingProcedureModelAttributes::ProcedureModelId(
                                crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureModelAttributes::ProcedureParentContainerId(
                                    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::TrackableId,
                                ),
                            ),
                        ),
                    ),
                );
            }
        } else if let Some(foreign) = procedure_parent_container_id.trackable_id {
            self.frozen_with = Some(foreign);
        } else if let Some(local) = self.frozen_with {
            self.procedure_model_id.procedure_parent_container_id = self
                .procedure_model_id
                .procedure_parent_container_id
                .trackable_id(local)
                .map_err(|err| {
                    err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureModelAttributes::ProcedureParentContainerId,
                        )
                        .into_field_name(
                            InsertableFreezingProcedureModelAttributes::ProcedureModelId,
                        )
                })?;
        }
        self.procedure_model_id = self
            .procedure_model_id
            .procedure_parent_container_id(procedure_parent_container_id)
            .map_err(|err| {
                err.into_field_name(InsertableFreezingProcedureModelAttributes::ProcedureModelId)
            })?;
        Ok(self)
    }
    pub fn child_container_id<P>(
        mut self,
        child_container_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezingProcedureModelAttributes>,
    >
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id =
            self.procedure_model_id.child_container_id(child_container_id).map_err(|err| {
                err.into_field_name(InsertableFreezingProcedureModelAttributes::ProcedureModelId)
            })?;
        Ok(self)
    }
    pub fn procedure_child_container_id(
        mut self,
        procedure_child_container_id: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezingProcedureModelAttributes>,
    > {
        self.procedure_model_id = self
            .procedure_model_id
            .procedure_child_container_id(procedure_child_container_id)
            .map_err(|err| {
                err.into_field_name(InsertableFreezingProcedureModelAttributes::ProcedureModelId)
            })?;
        Ok(self)
    }
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezingProcedureModelAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id = self.procedure_model_id.name(name).map_err(|err| {
            err.into_field_name(InsertableFreezingProcedureModelAttributes::ProcedureModelId)
        })?;
        Ok(self)
    }
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezingProcedureModelAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id =
            self.procedure_model_id.description(description).map_err(|err| {
                err.into_field_name(InsertableFreezingProcedureModelAttributes::ProcedureModelId)
            })?;
        Ok(self)
    }
    pub fn deprecated<P>(
        mut self,
        deprecated: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezingProcedureModelAttributes>,
    >
    where
        P: TryInto<bool>,
        <P as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id =
            self.procedure_model_id.deprecated(deprecated).map_err(|err| {
                err.into_field_name(InsertableFreezingProcedureModelAttributes::ProcedureModelId)
            })?;
        Ok(self)
    }
    pub fn photograph_id<P>(
        mut self,
        photograph_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezingProcedureModelAttributes>,
    >
    where
        P: TryInto<Option<::rosetta_uuid::Uuid>>,
        <P as TryInto<Option<::rosetta_uuid::Uuid>>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id =
            self.procedure_model_id.photograph_id(photograph_id).map_err(|err| {
                err.into_field_name(InsertableFreezingProcedureModelAttributes::ProcedureModelId)
            })?;
        Ok(self)
    }
    pub fn icon<P>(
        mut self,
        icon: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezingProcedureModelAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id = self.procedure_model_id.icon(icon).map_err(|err| {
            err.into_field_name(InsertableFreezingProcedureModelAttributes::ProcedureModelId)
        })?;
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezingProcedureModelAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id =
            self.procedure_model_id.created_by(created_by).map_err(|err| {
                err.into_field_name(InsertableFreezingProcedureModelAttributes::ProcedureModelId)
            })?;
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezingProcedureModelAttributes>,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id =
            self.procedure_model_id.created_at(created_at).map_err(|err| {
                err.into_field_name(InsertableFreezingProcedureModelAttributes::ProcedureModelId)
            })?;
        Ok(self)
    }
    pub fn updated_by<P>(
        mut self,
        updated_by: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezingProcedureModelAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id =
            self.procedure_model_id.updated_by(updated_by).map_err(|err| {
                err.into_field_name(InsertableFreezingProcedureModelAttributes::ProcedureModelId)
            })?;
        Ok(self)
    }
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezingProcedureModelAttributes>,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model_id =
            self.procedure_model_id.updated_at(updated_at).map_err(|err| {
                err.into_field_name(InsertableFreezingProcedureModelAttributes::ProcedureModelId)
            })?;
        Ok(self)
    }
}
impl InsertableFreezingProcedureModelBuilder {
    pub(crate) fn try_insert<C>(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        InsertableFreezingProcedureModel,
        web_common_traits::database::InsertError<
            InsertableFreezingProcedureModelAttributes,
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
        let frozen_with =
            self.frozen_with.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableFreezingProcedureModelAttributes::FrozenWith,
            ))?;
        let procedure_model_id = self
            .procedure_model_id
            .insert(user_id, conn)
            .map_err(|err| {
                err.into_field_name(InsertableFreezingProcedureModelAttributes::ProcedureModelId)
            })?
            .id();
        Ok(InsertableFreezingProcedureModel {
            procedure_model_id,
            seconds: self.seconds,
            frozen_with,
        })
    }
}
