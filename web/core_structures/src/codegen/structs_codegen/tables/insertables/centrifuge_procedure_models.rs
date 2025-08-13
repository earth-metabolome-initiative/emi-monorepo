#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCentrifugeProcedureModelExtensionAttributes {
    ProcedureModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
    ),
}
impl core::fmt::Display for InsertableCentrifugeProcedureModelExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ProcedureModel(e) => write!(f, "{e}"),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCentrifugeProcedureModelAttributes {
    Extension(InsertableCentrifugeProcedureModelExtensionAttributes),
    ProcedureModelId,
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
impl core::str::FromStr for InsertableCentrifugeProcedureModelAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Kelvin" => Ok(Self::Kelvin),
            "KelvinTolerancePercentage" => Ok(Self::KelvinTolerancePercentage),
            "Seconds" => Ok(Self::Seconds),
            "RotationPerMinute" => Ok(Self::RotationPerMinute),
            "CentrifugedWith" => Ok(Self::CentrifugedWith),
            "ProcedureCentrifugedWith" => {
                Ok(
                    Self::ProcedureCentrifugedWith(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::Id,
                    ),
                )
            }
            "CentrifugedContainerId" => Ok(Self::CentrifugedContainerId),
            "ProcedureCentrifugedContainerId" => {
                Ok(
                    Self::ProcedureCentrifugedContainerId(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::Id,
                    ),
                )
            }
            "kelvin" => Ok(Self::Kelvin),
            "kelvin_tolerance_percentage" => Ok(Self::KelvinTolerancePercentage),
            "seconds" => Ok(Self::Seconds),
            "rotation_per_minute" => Ok(Self::RotationPerMinute),
            "centrifuged_with" => Ok(Self::CentrifugedWith),
            "procedure_centrifuged_with" => {
                Ok(
                    Self::ProcedureCentrifugedWith(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::Id,
                    ),
                )
            }
            "centrifuged_container_id" => Ok(Self::CentrifugedContainerId),
            "procedure_centrifuged_container_id" => {
                Ok(
                    Self::ProcedureCentrifugedContainerId(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::Id,
                    ),
                )
            }
            _ => {
                Err(
                    web_common_traits::database::InsertError::UnknownAttribute(
                        s.to_owned(),
                    ),
                )
            }
        }
    }
}
impl core::fmt::Display for InsertableCentrifugeProcedureModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureModelId => write!(f, "procedure_model_id"),
            Self::Kelvin => write!(f, "kelvin"),
            Self::KelvinTolerancePercentage => write!(f, "kelvin_tolerance_percentage"),
            Self::Seconds => write!(f, "seconds"),
            Self::RotationPerMinute => write!(f, "rotation_per_minute"),
            Self::CentrifugedWith => write!(f, "centrifuged_with"),
            Self::ProcedureCentrifugedWith(e) => write!(f, "{e}"),
            Self::CentrifugedContainerId => write!(f, "centrifuged_container_id"),
            Self::ProcedureCentrifugedContainerId(e) => write!(f, "{e}"),
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
    pub(crate) procedure_model_id: i32,
    pub(crate) kelvin: f32,
    pub(crate) kelvin_tolerance_percentage: f32,
    pub(crate) seconds: f32,
    pub(crate) rotation_per_minute: f32,
    pub(crate) centrifuged_with: ::rosetta_uuid::Uuid,
    pub(crate) procedure_centrifuged_with: i32,
    pub(crate) centrifuged_container_id: ::rosetta_uuid::Uuid,
    pub(crate) procedure_centrifuged_container_id: i32,
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
    pub fn centrifuge_pm_compatibility_rules<C: diesel::connection::LoadConnection>(
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
pub struct InsertableCentrifugeProcedureModelBuilder<
    ProcedureModel
        = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    pub(crate) kelvin: Option<f32>,
    pub(crate) kelvin_tolerance_percentage: Option<f32>,
    pub(crate) seconds: Option<f32>,
    pub(crate) rotation_per_minute: Option<f32>,
    pub(crate) centrifuged_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_centrifuged_with: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    pub(crate) centrifuged_container_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_centrifuged_container_id: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    pub(crate) procedure_model: ProcedureModel,
}
impl<ProcedureModel> Default for InsertableCentrifugeProcedureModelBuilder<ProcedureModel>
where
    ProcedureModel: Default,
{
    fn default() -> Self {
        Self {
            procedure_model: Default::default(),
            kelvin: Some(293.15f32),
            kelvin_tolerance_percentage: Some(1f32),
            seconds: Some(120f32),
            rotation_per_minute: Some(13000f32),
            centrifuged_with: Default::default(),
            procedure_centrifuged_with: Default::default(),
            centrifuged_container_id: Default::default(),
            procedure_centrifuged_container_id: Default::default(),
        }
    }
}
impl<ProcedureModel> web_common_traits::database::ExtendableBuilder
for InsertableCentrifugeProcedureModelBuilder<ProcedureModel>
where
    ProcedureModel: web_common_traits::database::ExtendableBuilder<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
    >,
{
    type Attributes = InsertableCentrifugeProcedureModelAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure_model = self
            .procedure_model
            .extend_builder(other.procedure_model)
            .map_err(|err| {
                err.into_field_name(|attribute| InsertableCentrifugeProcedureModelAttributes::Extension(
                    InsertableCentrifugeProcedureModelExtensionAttributes::ProcedureModel(
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
        if let Some(seconds) = other.seconds {
            self = self.seconds(seconds)?;
        }
        if let Some(rotation_per_minute) = other.rotation_per_minute {
            self = self.rotation_per_minute(rotation_per_minute)?;
        }
        if let Some(centrifuged_with) = other.centrifuged_with {
            self = self.centrifuged_with(centrifuged_with)?;
        }
        self = self.procedure_centrifuged_with(other.procedure_centrifuged_with)?;
        if let Some(centrifuged_container_id) = other.centrifuged_container_id {
            self = self.centrifuged_container(centrifuged_container_id)?;
        }
        self = self
            .procedure_centrifuged_container(other.procedure_centrifuged_container_id)?;
        Ok(self)
    }
}
impl<ProcedureModel> web_common_traits::prelude::SetPrimaryKey
    for InsertableCentrifugeProcedureModelBuilder<ProcedureModel>
where
    ProcedureModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.procedure_model = self.procedure_model.set_primary_key(primary_key);
        self
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the
    /// `centrifuge_procedure_models.centrifuged_container_id` column from table
    /// `centrifuge_procedure_models`.
    pub fn centrifuged_container(
        mut self,
        centrifuged_container_id: ::rosetta_uuid::Uuid,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    > {
        self.centrifuged_container_id = Some(centrifuged_container_id);
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the `centrifuge_procedure_models.centrifuged_with`
    /// column from table `centrifuge_procedure_models`.
    pub fn centrifuged_with(
        mut self,
        centrifuged_with: ::rosetta_uuid::Uuid,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    > {
        self.centrifuged_with = Some(centrifuged_with);
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the
    /// `centrifuge_procedure_models.kelvin_tolerance_percentage` column from
    /// table `centrifuge_procedure_models`.
    pub fn kelvin_tolerance_percentage<KelvinTolerancePercentage>(
        mut self,
        kelvin_tolerance_percentage: KelvinTolerancePercentage,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        KelvinTolerancePercentage: TryInto<f32>,
        <KelvinTolerancePercentage as TryInto<f32>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let kelvin_tolerance_percentage = kelvin_tolerance_percentage.try_into().map_err(
            |err: <KelvinTolerancePercentage as TryInto<f32>>::Error| {
                Into::into(err).rename_field(
                    InsertableCentrifugeProcedureModelAttributes::KelvinTolerancePercentage,
                )
            },
        )?;
        pgrx_validation::must_be_strictly_positive_f32(kelvin_tolerance_percentage)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelAttributes::KelvinTolerancePercentage,
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
                                crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelAttributes::KelvinTolerancePercentage,
                            )
                    })
            })?;
        self.kelvin_tolerance_percentage = Some(kelvin_tolerance_percentage);
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the `centrifuge_procedure_models.kelvin` column from
    /// table `centrifuge_procedure_models`.
    pub fn kelvin<Kelvin>(
        mut self,
        kelvin: Kelvin,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        Kelvin: TryInto<f32>,
        <Kelvin as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let kelvin = kelvin.try_into().map_err(|err: <Kelvin as TryInto<f32>>::Error| {
            Into::into(err).rename_field(InsertableCentrifugeProcedureModelAttributes::Kelvin)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(kelvin)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelAttributes::Kelvin,
                    )
            })?;
        self.kelvin = Some(kelvin);
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the
    /// `centrifuge_procedure_models.procedure_centrifuged_container_id` column
    /// from table `centrifuge_procedure_models`.
    pub fn procedure_centrifuged_container(
        mut self,
        mut procedure_centrifuged_container_id: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableCentrifugeProcedureModelAttributes,
        >,
    >
    where
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::ExtendableBuilder<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        >,
    {
        use web_common_traits::database::ExtendableBuilder;
        if let (Some(local), Some(foreign)) =
            (self.centrifuged_container_id, procedure_centrifuged_container_id.trackable_id)
        {
            if local != foreign {
                return Err(
                    web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelAttributes::ProcedureCentrifugedContainerId(
                                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::TrackableId,
                            ),
                        ),
                    ),
                );
            }
        } else if let Some(foreign) = procedure_centrifuged_container_id.trackable_id {
            self.centrifuged_container_id = Some(foreign);
        } else if let Some(local) = self.centrifuged_container_id {
            procedure_centrifuged_container_id = procedure_centrifuged_container_id
                .trackable(local)
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelAttributes::ProcedureCentrifugedContainerId(
                            attribute,
                        )
                    })
                })?;
        }
        self.procedure_centrifuged_container_id = self
            .procedure_centrifuged_container_id
            .extend_builder(procedure_centrifuged_container_id)
            .map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertableCentrifugeProcedureModelAttributes::ProcedureCentrifugedContainerId(
                        attribute,
                    )
                })
            })?;
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the
    /// `centrifuge_procedure_models.procedure_centrifuged_with` column from
    /// table `centrifuge_procedure_models`.
    pub fn procedure_centrifuged_with(
        mut self,
        mut procedure_centrifuged_with: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableCentrifugeProcedureModelAttributes,
        >,
    >
    where
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::ExtendableBuilder<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        >,
    {
        use web_common_traits::database::ExtendableBuilder;
        if let (Some(local), Some(foreign)) =
            (self.centrifuged_with, procedure_centrifuged_with.trackable_id)
        {
            if local != foreign {
                return Err(
                    web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelAttributes::ProcedureCentrifugedWith(
                                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::TrackableId,
                            ),
                        ),
                    ),
                );
            }
        } else if let Some(foreign) = procedure_centrifuged_with.trackable_id {
            self.centrifuged_with = Some(foreign);
        } else if let Some(local) = self.centrifuged_with {
            procedure_centrifuged_with = procedure_centrifuged_with
                .trackable(local)
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelAttributes::ProcedureCentrifugedWith(
                            attribute,
                        )
                    })
                })?;
        }
        self.procedure_centrifuged_with = self
            .procedure_centrifuged_with
            .extend_builder(procedure_centrifuged_with)
            .map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertableCentrifugeProcedureModelAttributes::ProcedureCentrifugedWith(
                        attribute,
                    )
                })
            })?;
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the `centrifuge_procedure_models.rotation_per_minute`
    /// column from table `centrifuge_procedure_models`.
    pub fn rotation_per_minute<RotationPerMinute>(
        mut self,
        rotation_per_minute: RotationPerMinute,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        RotationPerMinute: TryInto<f32>,
        <RotationPerMinute as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let rotation_per_minute = rotation_per_minute.try_into().map_err(
            |err: <RotationPerMinute as TryInto<f32>>::Error| {
                Into::into(err)
                    .rename_field(InsertableCentrifugeProcedureModelAttributes::RotationPerMinute)
            },
        )?;
        pgrx_validation::must_be_greater_than_f32(rotation_per_minute, 5000f32)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelAttributes::RotationPerMinute,
                    )
            })
            .and_then(|_| {
                pgrx_validation::must_be_smaller_than_f32(rotation_per_minute, 30000f32)
                    .map_err(|e| {
                        e
                            .rename_field(
                                crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelAttributes::RotationPerMinute,
                            )
                    })
            })?;
        self.rotation_per_minute = Some(rotation_per_minute);
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the `centrifuge_procedure_models.seconds` column from
    /// table `centrifuge_procedure_models`.
    pub fn seconds<Seconds>(
        mut self,
        seconds: Seconds,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        Seconds: TryInto<f32>,
        <Seconds as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let seconds = seconds.try_into().map_err(|err: <Seconds as TryInto<f32>>::Error| {
            Into::into(err).rename_field(InsertableCentrifugeProcedureModelAttributes::Seconds)
        })?;
        pgrx_validation::must_be_greater_than_f32(seconds, 30f32)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelAttributes::Seconds,
                    )
            })
            .and_then(|_| {
                pgrx_validation::must_be_smaller_than_f32(seconds, 1800f32)
                    .map_err(|e| {
                        e
                            .rename_field(
                                crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelAttributes::Seconds,
                            )
                    })
            })?;
        self.seconds = Some(seconds);
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.created_at` column from table
    /// `centrifuge_procedure_models`.
    pub fn created_at<CreatedAt>(
        mut self,
        created_at: CreatedAt,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.created_at(created_at).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCentrifugeProcedureModelAttributes::Extension(
                    InsertableCentrifugeProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.created_at`,
    /// `procedure_models.updated_at` columns from table
    /// `centrifuge_procedure_models`.
    pub fn created_at_and_updated_at<CreatedAt, UpdatedAt>(
        mut self,
        created_at: CreatedAt,
        updated_at: UpdatedAt,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
        UpdatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self
            .procedure_model
            .created_at_and_updated_at(created_at, updated_at)
            .map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertableCentrifugeProcedureModelAttributes::Extension(
                        InsertableCentrifugeProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    )
                })
            })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.created_by` column from table
    /// `centrifuge_procedure_models`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    > {
        self.procedure_model = self.procedure_model.created_by(created_by).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCentrifugeProcedureModelAttributes::Extension(
                    InsertableCentrifugeProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        self = self.updated_by(created_by)?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.deprecated` column from table
    /// `centrifuge_procedure_models`.
    pub fn deprecated<Deprecated>(
        mut self,
        deprecated: Deprecated,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        Deprecated: TryInto<bool>,
        <Deprecated as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.deprecated(deprecated).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCentrifugeProcedureModelAttributes::Extension(
                    InsertableCentrifugeProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.description` column from table
    /// `centrifuge_procedure_models`.
    pub fn description<Description>(
        mut self,
        description: Description,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        Description: TryInto<String>,
        <Description as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.description(description).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCentrifugeProcedureModelAttributes::Extension(
                    InsertableCentrifugeProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.icon` column from table
    /// `centrifuge_procedure_models`.
    pub fn icon<Icon>(
        mut self,
        icon: Icon,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        Icon: TryInto<String>,
        <Icon as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.icon(icon).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCentrifugeProcedureModelAttributes::Extension(
                    InsertableCentrifugeProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.name` column from table
    /// `centrifuge_procedure_models`.
    pub fn name<Name>(
        mut self,
        name: Name,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        Name: TryInto<String>,
        <Name as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.name(name).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCentrifugeProcedureModelAttributes::Extension(
                    InsertableCentrifugeProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.name`,
    /// `procedure_models.description` columns from table
    /// `centrifuge_procedure_models`.
    pub fn name_and_description<Name, Description>(
        mut self,
        name: Name,
        description: Description,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        Name: TryInto<String>,
        <Name as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
        Description: TryInto<String>,
        <Description as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model =
            self.procedure_model.name_and_description(name, description).map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertableCentrifugeProcedureModelAttributes::Extension(
                        InsertableCentrifugeProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    )
                })
            })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.photograph_id` column from table
    /// `centrifuge_procedure_models`.
    pub fn photograph(
        mut self,
        photograph_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    > {
        self.procedure_model = self.procedure_model.photograph(photograph_id).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCentrifugeProcedureModelAttributes::Extension(
                    InsertableCentrifugeProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.updated_at` column from table
    /// `centrifuge_procedure_models`.
    pub fn updated_at<UpdatedAt>(
        mut self,
        updated_at: UpdatedAt,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    >
    where
        UpdatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.updated_at(updated_at).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCentrifugeProcedureModelAttributes::Extension(
                    InsertableCentrifugeProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.updated_by` column from table
    /// `centrifuge_procedure_models`.
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugeProcedureModelAttributes>,
    > {
        self.procedure_model = self.procedure_model.updated_by(updated_by).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCentrifugeProcedureModelAttributes::Extension(
                    InsertableCentrifugeProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl<ProcedureModel, C> web_common_traits::database::TryInsertGeneric<C>
for InsertableCentrifugeProcedureModelBuilder<ProcedureModel>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::centrifuge_procedure_models::CentrifugeProcedureModel,
        Error = web_common_traits::database::InsertError<
            InsertableCentrifugeProcedureModelAttributes,
        >,
    >,
    ProcedureModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::TryInsertGeneric<
        C,
    >,
{
    type Attributes = InsertableCentrifugeProcedureModelAttributes;
    fn is_complete(&self) -> bool {
        self.procedure_model.is_complete() && self.kelvin.is_some()
            && self.kelvin_tolerance_percentage.is_some() && self.seconds.is_some()
            && self.rotation_per_minute.is_some() && self.centrifuged_with.is_some()
            && self.procedure_centrifuged_with.is_complete()
            && self.centrifuged_container_id.is_some()
            && self.procedure_centrifuged_container_id.is_complete()
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
        let insertable: crate::codegen::structs_codegen::tables::centrifuge_procedure_models::CentrifugeProcedureModel = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
