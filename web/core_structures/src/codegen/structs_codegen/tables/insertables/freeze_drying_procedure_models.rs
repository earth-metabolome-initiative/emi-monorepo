#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableFreezeDryingProcedureModelAttributes {
    ProcedureModelId(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
    ),
    Kelvin,
    KelvinTolerancePercentage,
    Pascal,
    Seconds,
    FreezeDriedWith,
    ProcedureFreezeDriedWith(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
    FreezeDriedContainerId,
    ProcedureFreezeDriedContainerId(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
}
impl core::fmt::Display for InsertableFreezeDryingProcedureModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableFreezeDryingProcedureModelAttributes::ProcedureModelId(
                procedure_model_id,
            ) => write!(f, "{}", procedure_model_id),
            InsertableFreezeDryingProcedureModelAttributes::Kelvin => write!(f, "kelvin"),
            InsertableFreezeDryingProcedureModelAttributes::KelvinTolerancePercentage => {
                write!(f, "kelvin_tolerance_percentage")
            }
            InsertableFreezeDryingProcedureModelAttributes::Pascal => write!(f, "pascal"),
            InsertableFreezeDryingProcedureModelAttributes::Seconds => {
                write!(f, "seconds")
            }
            InsertableFreezeDryingProcedureModelAttributes::FreezeDriedWith => {
                write!(f, "freeze_dried_with")
            }
            InsertableFreezeDryingProcedureModelAttributes::ProcedureFreezeDriedWith(
                procedure_freeze_dried_with,
            ) => write!(f, "{}", procedure_freeze_dried_with),
            InsertableFreezeDryingProcedureModelAttributes::FreezeDriedContainerId => {
                write!(f, "freeze_dried_container_id")
            }
            InsertableFreezeDryingProcedureModelAttributes::ProcedureFreezeDriedContainerId(
                procedure_freeze_dried_container_id,
            ) => write!(f, "{}", procedure_freeze_dried_container_id),
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
    procedure_model_id: i32,
    kelvin: f32,
    kelvin_tolerance_percentage: f32,
    pascal: f32,
    seconds: f32,
    freeze_dried_with: ::rosetta_uuid::Uuid,
    procedure_freeze_dried_with: i32,
    freeze_dried_container_id: ::rosetta_uuid::Uuid,
    procedure_freeze_dried_container_id: i32,
}
impl InsertableFreezeDryingProcedureModel {
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
    pub fn freeze_dried_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel::table(),
                self.freeze_dried_with,
            ),
            conn,
        )
    }
    pub fn procedure_freeze_dried_with<C: diesel::connection::LoadConnection>(
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
                self.procedure_freeze_dried_with,
            ),
            conn,
        )
    }
    pub fn freeze_dried_container<C: diesel::connection::LoadConnection>(
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
                self.freeze_dried_container_id,
            ),
            conn,
        )
    }
    pub fn procedure_freeze_dried_container<C: diesel::connection::LoadConnection>(
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
                self.procedure_freeze_dried_container_id,
            ),
            conn,
        )
    }
    pub fn freeze_drying_procedure_model_freeze_dried_with_freeze_dri_fkey<
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
                (self.freeze_dried_with, self.freeze_dried_container_id),
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableFreezeDryingProcedureModelBuilder {
    pub(crate) procedure_model_id: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    pub(crate) kelvin: Option<f32>,
    pub(crate) kelvin_tolerance_percentage: Option<f32>,
    pub(crate) pascal: Option<f32>,
    pub(crate) seconds: Option<f32>,
    pub(crate) freeze_dried_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_freeze_dried_with: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    pub(crate) freeze_dried_container_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_freeze_dried_container_id: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
}
impl Default for InsertableFreezeDryingProcedureModelBuilder {
    fn default() -> Self {
        Self {
            procedure_model_id: Default::default(),
            kelvin: Some(223.15f32),
            kelvin_tolerance_percentage: Some(5f32),
            pascal: Some(4f32),
            seconds: Some(259200f32),
            freeze_dried_with: Default::default(),
            procedure_freeze_dried_with: Default::default(),
            freeze_dried_container_id: Default::default(),
            procedure_freeze_dried_container_id: Default::default(),
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
        self.kelvin = Some(kelvin);
        Ok(self)
    }
    pub fn kelvin_tolerance_percentage<P>(
        mut self,
        kelvin_tolerance_percentage: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezeDryingProcedureModelAttributes>,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let kelvin_tolerance_percentage =
            kelvin_tolerance_percentage.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
                Into::into(err).rename_field(
                    InsertableFreezeDryingProcedureModelAttributes::KelvinTolerancePercentage,
                )
            })?;
        pgrx_validation::must_be_strictly_positive_f32(kelvin_tolerance_percentage)
            .map_err(|e| {
                e
                    .rename_field(
                        InsertableFreezeDryingProcedureModelAttributes::KelvinTolerancePercentage,
                    )
            })
            .and_then(|_| {
                pgrx_validation::must_be_smaller_than_f32(
                        kelvin_tolerance_percentage,
                        100f32,
                    )
                    .map_err(|e| {
                        e
                            .rename_field(
                                InsertableFreezeDryingProcedureModelAttributes::KelvinTolerancePercentage,
                            )
                    })
            })?;
        self.kelvin_tolerance_percentage = Some(kelvin_tolerance_percentage);
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
            .map_err(|e| e.rename_field(InsertableFreezeDryingProcedureModelAttributes::Pascal))
            .and_then(|_| {
                pgrx_validation::must_be_smaller_than_f32(pascal, 500f32).map_err(|e| {
                    e.rename_field(InsertableFreezeDryingProcedureModelAttributes::Pascal)
                })
            })?;
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
        pgrx_validation::must_be_strictly_greater_than_f32(seconds, 7200f32)
            .map_err(|e| e.rename_field(InsertableFreezeDryingProcedureModelAttributes::Seconds))
            .and_then(|_| {
                pgrx_validation::must_be_strictly_smaller_than_f32(seconds, 604800f32).map_err(
                    |e| e.rename_field(InsertableFreezeDryingProcedureModelAttributes::Seconds),
                )
            })?;
        self.seconds = Some(seconds);
        Ok(self)
    }
    pub fn freeze_dried_with<P>(
        mut self,
        freeze_dried_with: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezeDryingProcedureModelAttributes>,
    >
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let freeze_dried_with = freeze_dried_with.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err)
                    .rename_field(InsertableFreezeDryingProcedureModelAttributes::FreezeDriedWith)
            },
        )?;
        self.freeze_dried_with = Some(freeze_dried_with);
        self.procedure_freeze_dried_with =
            self.procedure_freeze_dried_with.trackable_id(freeze_dried_with).map_err(|err| {
                err.into_field_name(
                    InsertableFreezeDryingProcedureModelAttributes::ProcedureFreezeDriedWith,
                )
            })?;
        Ok(self)
    }
    pub fn freeze_dried_container_id<P>(
        mut self,
        freeze_dried_container_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezeDryingProcedureModelAttributes>,
    >
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let freeze_dried_container_id = freeze_dried_container_id.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(
                    InsertableFreezeDryingProcedureModelAttributes::FreezeDriedContainerId,
                )
            },
        )?;
        self.freeze_dried_container_id = Some(freeze_dried_container_id);
        self.procedure_freeze_dried_container_id = self
            .procedure_freeze_dried_container_id
            .trackable_id(freeze_dried_container_id)
            .map_err(|err| {
                err.into_field_name(
                    InsertableFreezeDryingProcedureModelAttributes::ProcedureFreezeDriedContainerId,
                )
            })?;
        Ok(self)
    }
    pub fn procedure_freeze_dried_container_id(
        mut self,
        procedure_freeze_dried_container_id: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezeDryingProcedureModelAttributes>,
    > {
        if procedure_freeze_dried_container_id.procedure_model_id.is_some() {
            return Err(
                web_common_traits::database::InsertError::BuilderError(
                    web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                        InsertableFreezeDryingProcedureModelAttributes::ProcedureFreezeDriedContainerId(
                            crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::ProcedureModelId,
                        ),
                    ),
                ),
            );
        }
        if let (Some(local), Some(foreign)) =
            (self.freeze_dried_container_id, procedure_freeze_dried_container_id.trackable_id)
        {
            if local != foreign {
                return Err(
                    web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            InsertableFreezeDryingProcedureModelAttributes::ProcedureFreezeDriedContainerId(
                                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::TrackableId,
                            ),
                        ),
                    ),
                );
            }
        } else if let Some(foreign) = procedure_freeze_dried_container_id.trackable_id {
            self.freeze_dried_container_id = Some(foreign);
        } else if let Some(local) = self.freeze_dried_container_id {
            self.procedure_freeze_dried_container_id = self
                .procedure_freeze_dried_container_id
                .trackable_id(local)
                .map_err(|err| {
                    err.into_field_name(
                        InsertableFreezeDryingProcedureModelAttributes::ProcedureFreezeDriedContainerId,
                    )
                })?;
        }
        self.procedure_freeze_dried_container_id = procedure_freeze_dried_container_id;
        Ok(self)
    }
    pub fn procedure_freeze_dried_with(
        mut self,
        procedure_freeze_dried_with: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFreezeDryingProcedureModelAttributes>,
    > {
        if procedure_freeze_dried_with.procedure_model_id.is_some() {
            return Err(
                web_common_traits::database::InsertError::BuilderError(
                    web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                        InsertableFreezeDryingProcedureModelAttributes::ProcedureFreezeDriedWith(
                            crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::ProcedureModelId,
                        ),
                    ),
                ),
            );
        }
        if let (Some(local), Some(foreign)) =
            (self.freeze_dried_with, procedure_freeze_dried_with.trackable_id)
        {
            if local != foreign {
                return Err(
                    web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            InsertableFreezeDryingProcedureModelAttributes::ProcedureFreezeDriedWith(
                                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::TrackableId,
                            ),
                        ),
                    ),
                );
            }
        } else if let Some(foreign) = procedure_freeze_dried_with.trackable_id {
            self.freeze_dried_with = Some(foreign);
        } else if let Some(local) = self.freeze_dried_with {
            self.procedure_freeze_dried_with =
                self.procedure_freeze_dried_with.trackable_id(local).map_err(|err| {
                    err.into_field_name(
                        InsertableFreezeDryingProcedureModelAttributes::ProcedureFreezeDriedWith,
                    )
                })?;
        }
        self.procedure_freeze_dried_with = procedure_freeze_dried_with;
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
        self.procedure_model_id = self.procedure_model_id.name(name).map_err(|err| {
            err.into_field_name(InsertableFreezeDryingProcedureModelAttributes::ProcedureModelId)
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
        self.procedure_model_id =
            self.procedure_model_id.description(description).map_err(|err| {
                err.into_field_name(
                    InsertableFreezeDryingProcedureModelAttributes::ProcedureModelId,
                )
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
        self.procedure_model_id =
            self.procedure_model_id.deprecated(deprecated).map_err(|err| {
                err.into_field_name(
                    InsertableFreezeDryingProcedureModelAttributes::ProcedureModelId,
                )
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
        self.procedure_model_id =
            self.procedure_model_id.photograph_id(photograph_id).map_err(|err| {
                err.into_field_name(
                    InsertableFreezeDryingProcedureModelAttributes::ProcedureModelId,
                )
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
        self.procedure_model_id = self.procedure_model_id.icon(icon).map_err(|err| {
            err.into_field_name(InsertableFreezeDryingProcedureModelAttributes::ProcedureModelId)
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
        self.procedure_model_id =
            self.procedure_model_id.created_by(created_by).map_err(|err| {
                err.into_field_name(
                    InsertableFreezeDryingProcedureModelAttributes::ProcedureModelId,
                )
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
        self.procedure_model_id =
            self.procedure_model_id.created_at(created_at).map_err(|err| {
                err.into_field_name(
                    InsertableFreezeDryingProcedureModelAttributes::ProcedureModelId,
                )
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
        self.procedure_model_id =
            self.procedure_model_id.updated_by(updated_by).map_err(|err| {
                err.into_field_name(
                    InsertableFreezeDryingProcedureModelAttributes::ProcedureModelId,
                )
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
        self.procedure_model_id =
            self.procedure_model_id.updated_at(updated_at).map_err(|err| {
                err.into_field_name(
                    InsertableFreezeDryingProcedureModelAttributes::ProcedureModelId,
                )
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
        let kelvin_tolerance_percentage = self.kelvin_tolerance_percentage.ok_or(
            common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableFreezeDryingProcedureModelAttributes::KelvinTolerancePercentage,
            ),
        )?;
        let pascal = self.pascal.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
            InsertableFreezeDryingProcedureModelAttributes::Pascal,
        ))?;
        let seconds = self.seconds.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
            InsertableFreezeDryingProcedureModelAttributes::Seconds,
        ))?;
        let freeze_dried_with =
            self.freeze_dried_with.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableFreezeDryingProcedureModelAttributes::FreezeDriedWith,
            ))?;
        let freeze_dried_container_id = self.freeze_dried_container_id.ok_or(
            common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableFreezeDryingProcedureModelAttributes::FreezeDriedContainerId,
            ),
        )?;
        let procedure_model_id = self
            .procedure_model_id
            .insert(user_id, conn)
            .map_err(|err| {
                err.into_field_name(
                    InsertableFreezeDryingProcedureModelAttributes::ProcedureModelId,
                )
            })?
            .id();
        let procedure_freeze_dried_container_id = self
            .procedure_freeze_dried_container_id
            .procedure_model_id(procedure_model_id)
            .map_err(|err| {
                err.into_field_name(
                    InsertableFreezeDryingProcedureModelAttributes::ProcedureFreezeDriedContainerId,
                )
            })?
            .insert(user_id, conn)
            .map_err(|err| {
                err.into_field_name(
                    InsertableFreezeDryingProcedureModelAttributes::ProcedureFreezeDriedContainerId,
                )
            })?
            .id();
        let procedure_freeze_dried_with = self
            .procedure_freeze_dried_with
            .procedure_model_id(procedure_model_id)
            .map_err(|err| {
                err.into_field_name(
                    InsertableFreezeDryingProcedureModelAttributes::ProcedureFreezeDriedWith,
                )
            })?
            .insert(user_id, conn)
            .map_err(|err| {
                err.into_field_name(
                    InsertableFreezeDryingProcedureModelAttributes::ProcedureFreezeDriedWith,
                )
            })?
            .id();
        Ok(InsertableFreezeDryingProcedureModel {
            procedure_model_id,
            kelvin,
            kelvin_tolerance_percentage,
            pascal,
            seconds,
            freeze_dried_with,
            procedure_freeze_dried_with,
            freeze_dried_container_id,
            procedure_freeze_dried_container_id,
        })
    }
}
