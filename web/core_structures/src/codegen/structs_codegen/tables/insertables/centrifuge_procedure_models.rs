#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCentrifugeProcedureModelAttributes {
    ProcedureModelId(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
    ),
    Kelvin,
    KelvinTolerancePercentage,
    Seconds,
    RotationPerMinute,
    CentrifugedWith,
    ProcedureCentrifugedWith(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
    CentrifugedContainerId,
    ProcedureCentrifugedContainerId(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
}
impl core::fmt::Display for InsertableCentrifugeProcedureModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableCentrifugeProcedureModelAttributes::ProcedureModelId(procedure_model_id) => {
                write!(f, "{}", procedure_model_id)
            }
            InsertableCentrifugeProcedureModelAttributes::Kelvin => write!(f, "kelvin"),
            InsertableCentrifugeProcedureModelAttributes::KelvinTolerancePercentage => {
                write!(f, "kelvin_tolerance_percentage")
            }
            InsertableCentrifugeProcedureModelAttributes::Seconds => write!(f, "seconds"),
            InsertableCentrifugeProcedureModelAttributes::RotationPerMinute => {
                write!(f, "rotation_per_minute")
            }
            InsertableCentrifugeProcedureModelAttributes::CentrifugedWith => {
                write!(f, "centrifuged_with")
            }
            InsertableCentrifugeProcedureModelAttributes::ProcedureCentrifugedWith(
                procedure_centrifuged_with,
            ) => write!(f, "{}", procedure_centrifuged_with),
            InsertableCentrifugeProcedureModelAttributes::CentrifugedContainerId => {
                write!(f, "centrifuged_container_id")
            }
            InsertableCentrifugeProcedureModelAttributes::ProcedureCentrifugedContainerId(
                procedure_centrifuged_container_id,
            ) => write!(f, "{}", procedure_centrifuged_container_id),
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
    kelvin: f32,
    kelvin_tolerance_percentage: f32,
    seconds: f32,
    rotation_per_minute: f32,
    centrifuged_with: ::rosetta_uuid::Uuid,
    procedure_centrifuged_with: i32,
    centrifuged_container_id: ::rosetta_uuid::Uuid,
    procedure_centrifuged_container_id: i32,
}
impl InsertableCentrifugeProcedureModel {
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
    pub fn procedure_centrifuged_with<C: diesel::connection::LoadConnection>(
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
                self.procedure_centrifuged_with,
            ),
            conn,
        )
    }
    pub fn centrifuged_container<C: diesel::connection::LoadConnection>(
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
                self.centrifuged_container_id,
            ),
            conn,
        )
    }
    pub fn procedure_centrifuged_container<C: diesel::connection::LoadConnection>(
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
                self.procedure_centrifuged_container_id,
            ),
            conn,
        )
    }
    pub fn centrifuge_procedure_models_centrifuged_with_centrifuged_c_fkey<
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
                (self.centrifuged_with, self.centrifuged_container_id),
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCentrifugeProcedureModelBuilder {
    pub(crate) procedure_model_id: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    pub(crate) kelvin: Option<f32>,
    pub(crate) kelvin_tolerance_percentage: Option<f32>,
    pub(crate) seconds: Option<f32>,
    pub(crate) rotation_per_minute: Option<f32>,
    pub(crate) centrifuged_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_centrifuged_with: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    pub(crate) centrifuged_container_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_centrifuged_container_id: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
}
impl Default for InsertableCentrifugeProcedureModelBuilder {
    fn default() -> Self {
        Self {
            procedure_model_id: Default::default(),
            kelvin: Some(293.15f32),
            kelvin_tolerance_percentage: Some(5f32),
            seconds: Some(120f32),
            rotation_per_minute: Some(13000f32),
            centrifuged_with: Default::default(),
            procedure_centrifuged_with: Default::default(),
            centrifuged_container_id: Default::default(),
            procedure_centrifuged_container_id: Default::default(),
        }
    }
}
impl InsertableCentrifugeProcedureModelBuilder {
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
        let kelvin = kelvin.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
            Into::into(err).rename_field(InsertableCentrifugeProcedureModelAttributes::Kelvin)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(kelvin)
            .map_err(|e| e.rename_field(InsertableCentrifugeProcedureModelAttributes::Kelvin))?;
        self.kelvin = Some(kelvin);
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
        let kelvin_tolerance_percentage =
            kelvin_tolerance_percentage.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
                Into::into(err).rename_field(
                    InsertableCentrifugeProcedureModelAttributes::KelvinTolerancePercentage,
                )
            })?;
        pgrx_validation::must_be_strictly_positive_f32(kelvin_tolerance_percentage)
            .map_err(|e| {
                e.rename_field(
                    InsertableCentrifugeProcedureModelAttributes::KelvinTolerancePercentage,
                )
            })
            .and_then(|_| {
                pgrx_validation::must_be_smaller_than_f32(kelvin_tolerance_percentage, 100f32)
                    .map_err(|e| {
                        e.rename_field(
                            InsertableCentrifugeProcedureModelAttributes::KelvinTolerancePercentage,
                        )
                    })
            })?;
        self.kelvin_tolerance_percentage = Some(kelvin_tolerance_percentage);
        Ok(self)
    }
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
        self.procedure_centrifuged_with =
            self.procedure_centrifuged_with.trackable_id(centrifuged_with).map_err(|err| {
                err.into_field_name(
                    InsertableCentrifugeProcedureModelAttributes::ProcedureCentrifugedWith,
                )
            })?;
        Ok(self)
    }
    pub fn centrifuged_container_id<P>(
        mut self,
        centrifuged_container_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let centrifuged_container_id = centrifuged_container_id.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(
                    InsertableCentrifugeProcedureModelAttributes::CentrifugedContainerId,
                )
            },
        )?;
        self.centrifuged_container_id = Some(centrifuged_container_id);
        self.procedure_centrifuged_container_id = self
            .procedure_centrifuged_container_id
            .trackable_id(centrifuged_container_id)
            .map_err(|err| {
                err.into_field_name(
                    InsertableCentrifugeProcedureModelAttributes::ProcedureCentrifugedContainerId,
                )
            })?;
        Ok(self)
    }
    pub fn procedure_centrifuged_container_id(
        mut self,
        procedure_centrifuged_container_id: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    > {
        if procedure_centrifuged_container_id.procedure_model_id.is_some() {
            return Err(
                web_common_traits::database::InsertError::BuilderError(
                    web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                        InsertableCentrifugeProcedureModelAttributes::ProcedureCentrifugedContainerId(
                            crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::ProcedureModelId,
                        ),
                    ),
                ),
            );
        }
        if let (Some(local), Some(foreign)) =
            (self.centrifuged_container_id, procedure_centrifuged_container_id.trackable_id)
        {
            if local != foreign {
                return Err(
                    web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            InsertableCentrifugeProcedureModelAttributes::ProcedureCentrifugedContainerId(
                                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::TrackableId,
                            ),
                        ),
                    ),
                );
            }
        } else if let Some(foreign) = procedure_centrifuged_container_id.trackable_id {
            self.centrifuged_container_id = Some(foreign);
        } else if let Some(local) = self.centrifuged_container_id {
            self.procedure_centrifuged_container_id = self
                .procedure_centrifuged_container_id
                .trackable_id(local)
                .map_err(|err| {
                    err.into_field_name(
                        InsertableCentrifugeProcedureModelAttributes::ProcedureCentrifugedContainerId,
                    )
                })?;
        }
        self.procedure_centrifuged_container_id = procedure_centrifuged_container_id;
        Ok(self)
    }
    pub fn procedure_centrifuged_with(
        mut self,
        procedure_centrifuged_with: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    > {
        if procedure_centrifuged_with.procedure_model_id.is_some() {
            return Err(
                web_common_traits::database::InsertError::BuilderError(
                    web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                        InsertableCentrifugeProcedureModelAttributes::ProcedureCentrifugedWith(
                            crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::ProcedureModelId,
                        ),
                    ),
                ),
            );
        }
        if let (Some(local), Some(foreign)) =
            (self.centrifuged_with, procedure_centrifuged_with.trackable_id)
        {
            if local != foreign {
                return Err(
                    web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            InsertableCentrifugeProcedureModelAttributes::ProcedureCentrifugedWith(
                                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::TrackableId,
                            ),
                        ),
                    ),
                );
            }
        } else if let Some(foreign) = procedure_centrifuged_with.trackable_id {
            self.centrifuged_with = Some(foreign);
        } else if let Some(local) = self.centrifuged_with {
            self.procedure_centrifuged_with =
                self.procedure_centrifuged_with.trackable_id(local).map_err(|err| {
                    err.into_field_name(
                        InsertableCentrifugeProcedureModelAttributes::ProcedureCentrifugedWith,
                    )
                })?;
        }
        self.procedure_centrifuged_with = procedure_centrifuged_with;
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
            InsertableCentrifugeProcedureModelAttributes::Kelvin,
        ))?;
        let kelvin_tolerance_percentage = self.kelvin_tolerance_percentage.ok_or(
            common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableCentrifugeProcedureModelAttributes::KelvinTolerancePercentage,
            ),
        )?;
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
        let centrifuged_container_id = self.centrifuged_container_id.ok_or(
            common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableCentrifugeProcedureModelAttributes::CentrifugedContainerId,
            ),
        )?;
        let procedure_model_id = self
            .procedure_model_id
            .insert(user_id, conn)
            .map_err(|err| {
                err.into_field_name(InsertableCentrifugeProcedureModelAttributes::ProcedureModelId)
            })?
            .id();
        let procedure_centrifuged_container_id = self
            .procedure_centrifuged_container_id
            .procedure_model_id(procedure_model_id)
            .map_err(|err| {
                err.into_field_name(
                    InsertableCentrifugeProcedureModelAttributes::ProcedureCentrifugedContainerId,
                )
            })?
            .insert(user_id, conn)
            .map_err(|err| {
                err.into_field_name(
                    InsertableCentrifugeProcedureModelAttributes::ProcedureCentrifugedContainerId,
                )
            })?
            .id();
        let procedure_centrifuged_with = self
            .procedure_centrifuged_with
            .procedure_model_id(procedure_model_id)
            .map_err(|err| {
                err.into_field_name(
                    InsertableCentrifugeProcedureModelAttributes::ProcedureCentrifugedWith,
                )
            })?
            .insert(user_id, conn)
            .map_err(|err| {
                err.into_field_name(
                    InsertableCentrifugeProcedureModelAttributes::ProcedureCentrifugedWith,
                )
            })?
            .id();
        Ok(InsertableCentrifugeProcedureModel {
            procedure_model_id,
            kelvin,
            kelvin_tolerance_percentage,
            seconds,
            rotation_per_minute,
            centrifuged_with,
            procedure_centrifuged_with,
            centrifuged_container_id,
            procedure_centrifuged_container_id,
        })
    }
}
