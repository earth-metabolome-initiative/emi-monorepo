#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableWeighingProcedureModelExtensionAttributes {
    ProcedureModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
    ),
}
impl core::fmt::Display for InsertableWeighingProcedureModelExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ProcedureModel(e) => write!(f, "{e}"),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableWeighingProcedureModelAttributes {
    Extension(InsertableWeighingProcedureModelExtensionAttributes),
    ProcedureModelId,
    WeighedWith,
    ProcedureWeighedWith(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
    SampleContainerId,
    ProcedureSampleContainer(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
}
impl core::fmt::Display for InsertableWeighingProcedureModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureModelId => write!(f, "procedure_model_id"),
            Self::WeighedWith => write!(f, "weighed_with"),
            Self::ProcedureWeighedWith(e) => write!(f, "{e}"),
            Self::SampleContainerId => write!(f, "sample_container_id"),
            Self::ProcedureSampleContainer(e) => write!(f, "{e}"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::weighing_procedure_models::weighing_procedure_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableWeighingProcedureModel {
    pub(crate) procedure_model_id: i32,
    pub(crate) weighed_with: ::rosetta_uuid::Uuid,
    pub(crate) procedure_weighed_with: i32,
    pub(crate) sample_container_id: ::rosetta_uuid::Uuid,
    pub(crate) procedure_sample_container: i32,
}
impl InsertableWeighingProcedureModel {
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
    pub fn weighed_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel::table(),
                self.weighed_with,
            ),
            conn,
        )
    }
    pub fn procedure_weighed_with<C: diesel::connection::LoadConnection>(
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
                self.procedure_weighed_with,
            ),
            conn,
        )
    }
    pub fn sample_container<C: diesel::connection::LoadConnection>(
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
                self.sample_container_id,
            ),
            conn,
        )
    }
    pub fn procedure_sample_container<C: diesel::connection::LoadConnection>(
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
                self.procedure_sample_container,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableWeighingProcedureModelBuilder<
    ProcedureModel
        = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    pub(crate) weighed_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_weighed_with: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    pub(crate) sample_container_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_sample_container: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    pub(crate) procedure_model: ProcedureModel,
}
impl<ProcedureModel> web_common_traits::database::ExtendableBuilder
for InsertableWeighingProcedureModelBuilder<ProcedureModel>
where
    ProcedureModel: web_common_traits::database::ExtendableBuilder<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
    >,
{
    type Attributes = InsertableWeighingProcedureModelAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure_model = self
            .procedure_model
            .extend_builder(other.procedure_model)
            .map_err(|err| {
                err.into_field_name(|attribute| InsertableWeighingProcedureModelAttributes::Extension(
                    InsertableWeighingProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                ))
            })?;
        if let Some(weighed_with) = other.weighed_with {
            self = self.weighed_with(weighed_with)?;
        }
        self = self.procedure_weighed_with(other.procedure_weighed_with)?;
        if let Some(sample_container_id) = other.sample_container_id {
            self = self.sample_container(sample_container_id)?;
        }
        self = self.procedure_sample_container(other.procedure_sample_container)?;
        Ok(self)
    }
}
impl<ProcedureModel> web_common_traits::prelude::SetPrimaryKey
    for InsertableWeighingProcedureModelBuilder<ProcedureModel>
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
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the
    /// `weighing_procedure_models.procedure_sample_container` column from table
    /// `weighing_procedure_models`.
    pub fn procedure_sample_container(
        mut self,
        mut procedure_sample_container: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableWeighingProcedureModelAttributes,
        >,
    >
    where
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::ExtendableBuilder<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        >,
    {
        use web_common_traits::database::ExtendableBuilder;
        if let (Some(local), Some(foreign)) =
            (self.sample_container_id, procedure_sample_container.trackable_id)
        {
            if local != foreign {
                return Err(
                    web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelAttributes::ProcedureSampleContainer(
                                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::TrackableId,
                            ),
                        ),
                    ),
                );
            }
        } else if let Some(foreign) = procedure_sample_container.trackable_id {
            self.sample_container_id = Some(foreign);
        } else if let Some(local) = self.sample_container_id {
            procedure_sample_container = procedure_sample_container
                .trackable(local)
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelAttributes::ProcedureSampleContainer(
                            attribute,
                        )
                    })
                })?;
        }
        self.procedure_sample_container = self
            .procedure_sample_container
            .extend_builder(procedure_sample_container)
            .map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertableWeighingProcedureModelAttributes::ProcedureSampleContainer(attribute)
                })
            })?;
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the `weighing_procedure_models.procedure_weighed_with`
    /// column from table `weighing_procedure_models`.
    pub fn procedure_weighed_with(
        mut self,
        mut procedure_weighed_with: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableWeighingProcedureModelAttributes,
        >,
    >
    where
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::ExtendableBuilder<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        >,
    {
        use web_common_traits::database::ExtendableBuilder;
        if let (Some(local), Some(foreign)) =
            (self.weighed_with, procedure_weighed_with.trackable_id)
        {
            if local != foreign {
                return Err(
                    web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelAttributes::ProcedureWeighedWith(
                                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::TrackableId,
                            ),
                        ),
                    ),
                );
            }
        } else if let Some(foreign) = procedure_weighed_with.trackable_id {
            self.weighed_with = Some(foreign);
        } else if let Some(local) = self.weighed_with {
            procedure_weighed_with = procedure_weighed_with
                .trackable(local)
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelAttributes::ProcedureWeighedWith(
                            attribute,
                        )
                    })
                })?;
        }
        self.procedure_weighed_with =
            self.procedure_weighed_with.extend_builder(procedure_weighed_with).map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertableWeighingProcedureModelAttributes::ProcedureWeighedWith(attribute)
                })
            })?;
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the `weighing_procedure_models.sample_container_id`
    /// column from table `weighing_procedure_models`.
    pub fn sample_container(
        mut self,
        sample_container_id: ::rosetta_uuid::Uuid,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
    > {
        self.sample_container_id = Some(sample_container_id);
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the `weighing_procedure_models.weighed_with` column
    /// from table `weighing_procedure_models`.
    pub fn weighed_with(
        mut self,
        weighed_with: ::rosetta_uuid::Uuid,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
    > {
        self.weighed_with = Some(weighed_with);
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.created_at` column from table
    /// `weighing_procedure_models`.
    pub fn created_at<CreatedAt>(
        mut self,
        created_at: CreatedAt,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
    >
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.created_at(created_at).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableWeighingProcedureModelAttributes::Extension(
                    InsertableWeighingProcedureModelExtensionAttributes::ProcedureModel(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.created_at`,
    /// `procedure_models.updated_at` columns from table
    /// `weighing_procedure_models`.
    pub fn created_at_and_updated_at<CreatedAt, UpdatedAt>(
        mut self,
        created_at: CreatedAt,
        updated_at: UpdatedAt,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
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
                    InsertableWeighingProcedureModelAttributes::Extension(
                        InsertableWeighingProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    )
                })
            })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.created_by` column from table
    /// `weighing_procedure_models`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
    > {
        self.procedure_model = self.procedure_model.created_by(created_by).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableWeighingProcedureModelAttributes::Extension(
                    InsertableWeighingProcedureModelExtensionAttributes::ProcedureModel(attribute),
                )
            })
        })?;
        self = self.updated_by(created_by)?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.deprecated` column from table
    /// `weighing_procedure_models`.
    pub fn deprecated<Deprecated>(
        mut self,
        deprecated: Deprecated,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
    >
    where
        Deprecated: TryInto<bool>,
        <Deprecated as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.deprecated(deprecated).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableWeighingProcedureModelAttributes::Extension(
                    InsertableWeighingProcedureModelExtensionAttributes::ProcedureModel(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.description` column from table
    /// `weighing_procedure_models`.
    pub fn description<Description>(
        mut self,
        description: Description,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
    >
    where
        Description: TryInto<String>,
        <Description as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.description(description).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableWeighingProcedureModelAttributes::Extension(
                    InsertableWeighingProcedureModelExtensionAttributes::ProcedureModel(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.icon` column from table
    /// `weighing_procedure_models`.
    pub fn icon<Icon>(
        mut self,
        icon: Icon,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
    >
    where
        Icon: TryInto<String>,
        <Icon as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.icon(icon).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableWeighingProcedureModelAttributes::Extension(
                    InsertableWeighingProcedureModelExtensionAttributes::ProcedureModel(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.name` column from table
    /// `weighing_procedure_models`.
    pub fn name<Name>(
        mut self,
        name: Name,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
    >
    where
        Name: TryInto<String>,
        <Name as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.name(name).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableWeighingProcedureModelAttributes::Extension(
                    InsertableWeighingProcedureModelExtensionAttributes::ProcedureModel(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.name`,
    /// `procedure_models.description` columns from table
    /// `weighing_procedure_models`.
    pub fn name_and_description<Name, Description>(
        mut self,
        name: Name,
        description: Description,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
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
                    InsertableWeighingProcedureModelAttributes::Extension(
                        InsertableWeighingProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    )
                })
            })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.photograph_id` column from table
    /// `weighing_procedure_models`.
    pub fn photograph(
        mut self,
        photograph_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
    > {
        self.procedure_model = self.procedure_model.photograph(photograph_id).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableWeighingProcedureModelAttributes::Extension(
                    InsertableWeighingProcedureModelExtensionAttributes::ProcedureModel(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.updated_at` column from table
    /// `weighing_procedure_models`.
    pub fn updated_at<UpdatedAt>(
        mut self,
        updated_at: UpdatedAt,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
    >
    where
        UpdatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.updated_at(updated_at).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableWeighingProcedureModelAttributes::Extension(
                    InsertableWeighingProcedureModelExtensionAttributes::ProcedureModel(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.updated_by` column from table
    /// `weighing_procedure_models`.
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingProcedureModelAttributes>,
    > {
        self.procedure_model = self.procedure_model.updated_by(updated_by).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableWeighingProcedureModelAttributes::Extension(
                    InsertableWeighingProcedureModelExtensionAttributes::ProcedureModel(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl<ProcedureModel, C> web_common_traits::database::TryInsertGeneric<C>
for InsertableWeighingProcedureModelBuilder<ProcedureModel>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel,
        Error = web_common_traits::database::InsertError<
            InsertableWeighingProcedureModelAttributes,
        >,
    >,
    ProcedureModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::TryInsertGeneric<
        C,
    >,
{
    type Attributes = InsertableWeighingProcedureModelAttributes;
    fn is_complete(&self) -> bool {
        self.procedure_model.is_complete() && self.weighed_with.is_some()
            && self.procedure_weighed_with.is_complete()
            && self.sample_container_id.is_some()
            && self.procedure_sample_container.is_complete()
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
        let insertable: crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
