#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableFreezeDryingProcedureModelExtensionAttributes {
    ProcedureModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
    ),
}
impl core::fmt::Display for InsertableFreezeDryingProcedureModelExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ProcedureModel(e) => write!(f, "ProcedureModel.{e}"),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableFreezeDryingProcedureModelAttributes {
    Extension(InsertableFreezeDryingProcedureModelExtensionAttributes),
    ProcedureModelId,
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
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureModelId => write!(f, "procedure_model_id"),
            Self::Kelvin => write!(f, "kelvin"),
            Self::KelvinTolerancePercentage => write!(f, "kelvin_tolerance_percentage"),
            Self::Pascal => write!(f, "pascal"),
            Self::Seconds => write!(f, "seconds"),
            Self::FreezeDriedWith => write!(f, "freeze_dried_with"),
            Self::ProcedureFreezeDriedWith(e) => write!(f, "{e}"),
            Self::FreezeDriedContainerId => write!(f, "freeze_dried_container_id"),
            Self::ProcedureFreezeDriedContainerId(e) => write!(f, "{e}"),
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
    pub(crate) procedure_model_id: i32,
    pub(crate) kelvin: f32,
    pub(crate) kelvin_tolerance_percentage: f32,
    pub(crate) pascal: f32,
    pub(crate) seconds: f32,
    pub(crate) freeze_dried_with: ::rosetta_uuid::Uuid,
    pub(crate) procedure_freeze_dried_with: i32,
    pub(crate) freeze_dried_container_id: ::rosetta_uuid::Uuid,
    pub(crate) procedure_freeze_dried_container_id: i32,
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
pub struct InsertableFreezeDryingProcedureModelBuilder<
    ProcedureModel
        = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    pub(crate) kelvin: Option<f32>,
    pub(crate) kelvin_tolerance_percentage: Option<f32>,
    pub(crate) pascal: Option<f32>,
    pub(crate) seconds: Option<f32>,
    pub(crate) freeze_dried_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_freeze_dried_with: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    pub(crate) freeze_dried_container_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_freeze_dried_container_id: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    pub(crate) procedure_model: ProcedureModel,
}
impl<ProcedureModel> Default for InsertableFreezeDryingProcedureModelBuilder<ProcedureModel>
where
    ProcedureModel: Default,
{
    fn default() -> Self {
        Self {
            procedure_model: Default::default(),
            kelvin: Default::default(),
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
impl<ProcedureModel> web_common_traits::database::ExtendableBuilder
for InsertableFreezeDryingProcedureModelBuilder<ProcedureModel>
where
    ProcedureModel: web_common_traits::database::ExtendableBuilder<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
    >,
{
    type Attributes = InsertableFreezeDryingProcedureModelAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure_model = self
            .procedure_model
            .extend_builder(other.procedure_model)
            .map_err(|err| {
                err.into_field_name(|attribute| InsertableFreezeDryingProcedureModelAttributes::Extension(
                    InsertableFreezeDryingProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                ))
            })?;
        if let Some(kelvin) = other.kelvin {
            self = self.kelvin(kelvin)?;
        }
        if let Some(kelvin_tolerance_percentage) = other.kelvin_tolerance_percentage {
            self = self.kelvin_tolerance_percentage(kelvin_tolerance_percentage)?;
        }
        if let Some(pascal) = other.pascal {
            self = self.pascal(pascal)?;
        }
        if let Some(seconds) = other.seconds {
            self = self.seconds(seconds)?;
        }
        if let Some(freeze_dried_with) = other.freeze_dried_with {
            self = self.freeze_dried_with(freeze_dried_with)?;
        }
        self = self.procedure_freeze_dried_with(other.procedure_freeze_dried_with)?;
        if let Some(freeze_dried_container_id) = other.freeze_dried_container_id {
            self = self.freeze_dried_container(freeze_dried_container_id)?;
        }
        self = self
            .procedure_freeze_dried_container(
                other.procedure_freeze_dried_container_id,
            )?;
        Ok(self)
    }
}
impl<ProcedureModel> web_common_traits::prelude::SetPrimaryKey
    for InsertableFreezeDryingProcedureModelBuilder<ProcedureModel>
where
    ProcedureModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.procedure_model = self.procedure_model.set_primary_key(primary_key);
        self
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureModelBuilder<
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    ///Sets the value of the `procedure_models.name` column from table `freeze_drying_procedure_models`.
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableFreezeDryingProcedureModelAttributes,
        >,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self
            .procedure_model
            .name(name)
            .map_err(|e| {
                e
                    .into_field_name(|attribute| InsertableFreezeDryingProcedureModelAttributes::Extension(
                        InsertableFreezeDryingProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    ))
            })?;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureModelBuilder<
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    ///Sets the value of the `procedure_models.description` column from table `freeze_drying_procedure_models`.
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableFreezeDryingProcedureModelAttributes,
        >,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self
            .procedure_model
            .description(description)
            .map_err(|e| {
                e
                    .into_field_name(|attribute| InsertableFreezeDryingProcedureModelAttributes::Extension(
                        InsertableFreezeDryingProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    ))
            })?;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureModelBuilder<
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    ///Sets the value of the `procedure_models.deprecated` column from table `freeze_drying_procedure_models`.
    pub fn deprecated<P>(
        mut self,
        deprecated: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableFreezeDryingProcedureModelAttributes,
        >,
    >
    where
        P: TryInto<bool>,
        <P as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self
            .procedure_model
            .deprecated(deprecated)
            .map_err(|e| {
                e
                    .into_field_name(|attribute| InsertableFreezeDryingProcedureModelAttributes::Extension(
                        InsertableFreezeDryingProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    ))
            })?;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureModelBuilder<
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    ///Sets the value of the `procedure_models.photograph_id` column from table `freeze_drying_procedure_models`.
    pub fn photograph(
        mut self,
        photograph_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableFreezeDryingProcedureModelAttributes,
        >,
    > {
        self.procedure_model = self
            .procedure_model
            .photograph(photograph_id)
            .map_err(|e| {
                e
                    .into_field_name(|attribute| InsertableFreezeDryingProcedureModelAttributes::Extension(
                        InsertableFreezeDryingProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    ))
            })?;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureModelBuilder<
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    ///Sets the value of the `procedure_models.icon` column from table `freeze_drying_procedure_models`.
    pub fn icon<P>(
        mut self,
        icon: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableFreezeDryingProcedureModelAttributes,
        >,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self
            .procedure_model
            .icon(icon)
            .map_err(|e| {
                e
                    .into_field_name(|attribute| InsertableFreezeDryingProcedureModelAttributes::Extension(
                        InsertableFreezeDryingProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    ))
            })?;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureModelBuilder<
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    ///Sets the value of the `procedure_models.created_by` column from table `freeze_drying_procedure_models`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableFreezeDryingProcedureModelAttributes,
        >,
    > {
        self.procedure_model = self
            .procedure_model
            .created_by(created_by)
            .map_err(|e| {
                e
                    .into_field_name(|attribute| InsertableFreezeDryingProcedureModelAttributes::Extension(
                        InsertableFreezeDryingProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    ))
            })?;
        self = self.updated_by(created_by)?;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureModelBuilder<
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    ///Sets the value of the `procedure_models.created_at` column from table `freeze_drying_procedure_models`.
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableFreezeDryingProcedureModelAttributes,
        >,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<
            ::rosetta_timestamp::TimestampUTC,
        >>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self
            .procedure_model
            .created_at(created_at)
            .map_err(|e| {
                e
                    .into_field_name(|attribute| InsertableFreezeDryingProcedureModelAttributes::Extension(
                        InsertableFreezeDryingProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    ))
            })?;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureModelBuilder<
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    ///Sets the value of the `procedure_models.updated_by` column from table `freeze_drying_procedure_models`.
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableFreezeDryingProcedureModelAttributes,
        >,
    > {
        self.procedure_model = self
            .procedure_model
            .updated_by(updated_by)
            .map_err(|e| {
                e
                    .into_field_name(|attribute| InsertableFreezeDryingProcedureModelAttributes::Extension(
                        InsertableFreezeDryingProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    ))
            })?;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureModelBuilder<
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    ///Sets the value of the `procedure_models.updated_at` column from table `freeze_drying_procedure_models`.
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableFreezeDryingProcedureModelAttributes,
        >,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<
            ::rosetta_timestamp::TimestampUTC,
        >>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self
            .procedure_model
            .updated_at(updated_at)
            .map_err(|e| {
                e
                    .into_field_name(|attribute| InsertableFreezeDryingProcedureModelAttributes::Extension(
                        InsertableFreezeDryingProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    ))
            })?;
        Ok(self)
    }
}
impl<
    ProcedureModel,
> crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureModelBuilder<
    ProcedureModel,
> {
    ///Sets the value of the `freeze_drying_procedure_models.kelvin` column from table `freeze_drying_procedure_models`.
    pub fn kelvin<P>(
        mut self,
        kelvin: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableFreezeDryingProcedureModelAttributes,
        >,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let kelvin = kelvin
            .try_into()
            .map_err(|err: <P as TryInto<f32>>::Error| {
                Into::into(err)
                    .rename_field(InsertableFreezeDryingProcedureModelAttributes::Kelvin)
            })?;
        pgrx_validation::must_be_strictly_positive_f32(kelvin)
            .map_err(|e| {
                e.rename_field(InsertableFreezeDryingProcedureModelAttributes::Kelvin)
            })?;
        self.kelvin = Some(kelvin);
        Ok(self)
    }
}
impl<
    ProcedureModel,
> crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureModelBuilder<
    ProcedureModel,
> {
    ///Sets the value of the `freeze_drying_procedure_models.kelvin_tolerance_percentage` column from table `freeze_drying_procedure_models`.
    pub fn kelvin_tolerance_percentage<P>(
        mut self,
        kelvin_tolerance_percentage: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableFreezeDryingProcedureModelAttributes,
        >,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let kelvin_tolerance_percentage = kelvin_tolerance_percentage
            .try_into()
            .map_err(|err: <P as TryInto<f32>>::Error| {
                Into::into(err)
                    .rename_field(
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
}
impl<
    ProcedureModel,
> crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureModelBuilder<
    ProcedureModel,
> {
    ///Sets the value of the `freeze_drying_procedure_models.pascal` column from table `freeze_drying_procedure_models`.
    pub fn pascal<P>(
        mut self,
        pascal: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableFreezeDryingProcedureModelAttributes,
        >,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let pascal = pascal
            .try_into()
            .map_err(|err: <P as TryInto<f32>>::Error| {
                Into::into(err)
                    .rename_field(InsertableFreezeDryingProcedureModelAttributes::Pascal)
            })?;
        pgrx_validation::must_be_strictly_positive_f32(pascal)
            .map_err(|e| {
                e.rename_field(InsertableFreezeDryingProcedureModelAttributes::Pascal)
            })
            .and_then(|_| {
                pgrx_validation::must_be_smaller_than_f32(pascal, 500f32)
                    .map_err(|e| {
                        e
                            .rename_field(
                                InsertableFreezeDryingProcedureModelAttributes::Pascal,
                            )
                    })
            })?;
        self.pascal = Some(pascal);
        Ok(self)
    }
}
impl<
    ProcedureModel,
> crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureModelBuilder<
    ProcedureModel,
> {
    ///Sets the value of the `freeze_drying_procedure_models.seconds` column from table `freeze_drying_procedure_models`.
    pub fn seconds<P>(
        mut self,
        seconds: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableFreezeDryingProcedureModelAttributes,
        >,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let seconds = seconds
            .try_into()
            .map_err(|err: <P as TryInto<f32>>::Error| {
                Into::into(err)
                    .rename_field(
                        InsertableFreezeDryingProcedureModelAttributes::Seconds,
                    )
            })?;
        pgrx_validation::must_be_strictly_greater_than_f32(seconds, 7200f32)
            .map_err(|e| {
                e.rename_field(InsertableFreezeDryingProcedureModelAttributes::Seconds)
            })
            .and_then(|_| {
                pgrx_validation::must_be_strictly_smaller_than_f32(seconds, 604800f32)
                    .map_err(|e| {
                        e
                            .rename_field(
                                InsertableFreezeDryingProcedureModelAttributes::Seconds,
                            )
                    })
            })?;
        self.seconds = Some(seconds);
        Ok(self)
    }
}
impl<
    ProcedureModel,
> crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureModelBuilder<
    ProcedureModel,
> {
    ///Sets the value of the `freeze_drying_procedure_models.freeze_dried_with` column from table `freeze_drying_procedure_models`.
    pub fn freeze_dried_with(
        mut self,
        freeze_dried_with: ::rosetta_uuid::Uuid,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableFreezeDryingProcedureModelAttributes,
        >,
    > {
        self.freeze_dried_with = Some(freeze_dried_with);
        Ok(self)
    }
}
impl<
    ProcedureModel,
> crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureModelBuilder<
    ProcedureModel,
> {
    ///Sets the value of the `freeze_drying_procedure_models.procedure_freeze_dried_with` column from table `freeze_drying_procedure_models`.
    pub fn procedure_freeze_dried_with(
        mut self,
        procedure_freeze_dried_with: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableFreezeDryingProcedureModelAttributes,
        >,
    >
    where
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::ExtendableBuilder<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        >,
    {
        use web_common_traits::database::ExtendableBuilder;
        self.procedure_freeze_dried_with = self
            .procedure_freeze_dried_with
            .extend_builder(procedure_freeze_dried_with)
            .map_err(|e| {
                e
                    .into_field_name(|attribute| InsertableFreezeDryingProcedureModelAttributes::ProcedureFreezeDriedWith(
                        attribute,
                    ))
            })?;
        Ok(self)
    }
}
impl<
    ProcedureModel,
> crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureModelBuilder<
    ProcedureModel,
> {
    ///Sets the value of the `freeze_drying_procedure_models.freeze_dried_container_id` column from table `freeze_drying_procedure_models`.
    pub fn freeze_dried_container(
        mut self,
        freeze_dried_container_id: ::rosetta_uuid::Uuid,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableFreezeDryingProcedureModelAttributes,
        >,
    > {
        self.freeze_dried_container_id = Some(freeze_dried_container_id);
        Ok(self)
    }
}
impl<
    ProcedureModel,
> crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureModelBuilder<
    ProcedureModel,
> {
    ///Sets the value of the `freeze_drying_procedure_models.procedure_freeze_dried_container_id` column from table `freeze_drying_procedure_models`.
    pub fn procedure_freeze_dried_container(
        mut self,
        procedure_freeze_dried_container_id: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableFreezeDryingProcedureModelAttributes,
        >,
    >
    where
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::ExtendableBuilder<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        >,
    {
        use web_common_traits::database::ExtendableBuilder;
        self.procedure_freeze_dried_container_id = self
            .procedure_freeze_dried_container_id
            .extend_builder(procedure_freeze_dried_container_id)
            .map_err(|e| {
                e
                    .into_field_name(|attribute| InsertableFreezeDryingProcedureModelAttributes::ProcedureFreezeDriedContainerId(
                        attribute,
                    ))
            })?;
        Ok(self)
    }
}
impl<ProcedureModel, C> web_common_traits::database::TryInsertGeneric<C>
for InsertableFreezeDryingProcedureModelBuilder<ProcedureModel>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::freeze_drying_procedure_models::FreezeDryingProcedureModel,
        Error = web_common_traits::database::InsertError<
            InsertableFreezeDryingProcedureModelAttributes,
        >,
    >,
    ProcedureModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::TryInsertGeneric<
        C,
    >,
{
    type Attributes = InsertableFreezeDryingProcedureModelAttributes;
    fn is_complete(&self) -> bool {
        self.procedure_model.is_complete() && self.kelvin.is_some()
            && self.kelvin_tolerance_percentage.is_some() && self.pascal.is_some()
            && self.seconds.is_some() && self.freeze_dried_with.is_some()
            && self.procedure_freeze_dried_with.is_complete()
            && self.freeze_dried_container_id.is_some()
            && self.procedure_freeze_dried_container_id.is_complete()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::PrimaryKey,
        web_common_traits::database::InsertError<Self::Attributes>,
    > {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::freeze_drying_procedure_models::FreezeDryingProcedureModel = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
