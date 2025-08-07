#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertablePackagingProcedureModelExtensionAttributes {
    ProcedureModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
    ),
}
impl core::fmt::Display for InsertablePackagingProcedureModelExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ProcedureModel(e) => write!(f, "ProcedureModel.{e}"),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertablePackagingProcedureModelAttributes {
    Extension(InsertablePackagingProcedureModelExtensionAttributes),
    ProcedureModelId,
    PackagedWith,
    ProcedurePackagedWith(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
    ProcedureSampleId(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
}
impl core::fmt::Display for InsertablePackagingProcedureModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureModelId => write!(f, "procedure_model_id"),
            Self::PackagedWith => write!(f, "packaged_with"),
            Self::ProcedurePackagedWith(e) => write!(f, "{e}"),
            Self::ProcedureSampleId(e) => write!(f, "{e}"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::packaging_procedure_models::packaging_procedure_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertablePackagingProcedureModel {
    pub(crate) procedure_model_id: i32,
    pub(crate) packaged_with: ::rosetta_uuid::Uuid,
    pub(crate) procedure_packaged_with: i32,
    pub(crate) procedure_sample_id: i32,
}
impl InsertablePackagingProcedureModel {
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
    pub fn packaged_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::container_models::ContainerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::container_models::ContainerModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::container_models::ContainerModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::container_models::ContainerModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::container_models::ContainerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::container_models::ContainerModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::container_models::ContainerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::container_models::ContainerModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::container_models::ContainerModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::container_models::ContainerModel::table(),
                self.packaged_with,
            ),
            conn,
        )
    }
    pub fn procedure_packaged_with<C: diesel::connection::LoadConnection>(
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
                self.procedure_packaged_with,
            ),
            conn,
        )
    }
    pub fn procedure_sample<C: diesel::connection::LoadConnection>(
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
                self.procedure_sample_id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertablePackagingProcedureModelBuilder<
    ProcedureModel
        = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    pub(crate) packaged_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_packaged_with: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    pub(crate) procedure_sample_id: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    pub(crate) procedure_model: ProcedureModel,
}
impl<ProcedureModel> web_common_traits::database::ExtendableBuilder
for InsertablePackagingProcedureModelBuilder<ProcedureModel>
where
    ProcedureModel: web_common_traits::database::ExtendableBuilder<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
    >,
{
    type Attributes = InsertablePackagingProcedureModelAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure_model = self
            .procedure_model
            .extend_builder(other.procedure_model)
            .map_err(|err| {
                err.into_field_name(|attribute| InsertablePackagingProcedureModelAttributes::Extension(
                    InsertablePackagingProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                ))
            })?;
        if let Some(packaged_with) = other.packaged_with {
            self = self.packaged_with(packaged_with)?;
        }
        self = self.procedure_packaged_with(other.procedure_packaged_with)?;
        self = self.procedure_sample(other.procedure_sample_id)?;
        Ok(self)
    }
}
impl<ProcedureModel> web_common_traits::prelude::SetPrimaryKey
    for InsertablePackagingProcedureModelBuilder<ProcedureModel>
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
    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the `packaging_procedure_models.packaged_with` column
    /// from table `packaging_procedure_models`.
    pub fn packaged_with(
        mut self,
        packaged_with: ::rosetta_uuid::Uuid,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePackagingProcedureModelAttributes>,
    > {
        self.packaged_with = Some(packaged_with);
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the
    /// `packaging_procedure_models.procedure_packaged_with` column from table
    /// `packaging_procedure_models`.
    pub fn procedure_packaged_with(
        mut self,
        procedure_packaged_with: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertablePackagingProcedureModelAttributes,
        >,
    >
    where
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::ExtendableBuilder<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        >,
    {
        use web_common_traits::database::ExtendableBuilder;
        self.procedure_packaged_with =
            self.procedure_packaged_with.extend_builder(procedure_packaged_with).map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertablePackagingProcedureModelAttributes::ProcedurePackagedWith(attribute)
                })
            })?;
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the `packaging_procedure_models.procedure_sample_id`
    /// column from table `packaging_procedure_models`.
    pub fn procedure_sample(
        mut self,
        procedure_sample_id: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertablePackagingProcedureModelAttributes,
        >,
    >
    where
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::ExtendableBuilder<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        >,
    {
        use web_common_traits::database::ExtendableBuilder;
        self.procedure_sample_id =
            self.procedure_sample_id.extend_builder(procedure_sample_id).map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertablePackagingProcedureModelAttributes::ProcedureSampleId(attribute)
                })
            })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.created_at` column from table
    /// `packaging_procedure_models`.
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePackagingProcedureModelAttributes>,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.created_at(created_at).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertablePackagingProcedureModelAttributes::Extension(
                    InsertablePackagingProcedureModelExtensionAttributes::ProcedureModel(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.created_by` column from table
    /// `packaging_procedure_models`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePackagingProcedureModelAttributes>,
    > {
        self.procedure_model = self.procedure_model.created_by(created_by).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertablePackagingProcedureModelAttributes::Extension(
                    InsertablePackagingProcedureModelExtensionAttributes::ProcedureModel(attribute),
                )
            })
        })?;
        self = self.updated_by(created_by)?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.deprecated` column from table
    /// `packaging_procedure_models`.
    pub fn deprecated<P>(
        mut self,
        deprecated: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePackagingProcedureModelAttributes>,
    >
    where
        P: TryInto<bool>,
        <P as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.deprecated(deprecated).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertablePackagingProcedureModelAttributes::Extension(
                    InsertablePackagingProcedureModelExtensionAttributes::ProcedureModel(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.description` column from table
    /// `packaging_procedure_models`.
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePackagingProcedureModelAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.description(description).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertablePackagingProcedureModelAttributes::Extension(
                    InsertablePackagingProcedureModelExtensionAttributes::ProcedureModel(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.icon` column from table
    /// `packaging_procedure_models`.
    pub fn icon<P>(
        mut self,
        icon: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePackagingProcedureModelAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.icon(icon).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertablePackagingProcedureModelAttributes::Extension(
                    InsertablePackagingProcedureModelExtensionAttributes::ProcedureModel(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.name` column from table
    /// `packaging_procedure_models`.
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePackagingProcedureModelAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.name(name).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertablePackagingProcedureModelAttributes::Extension(
                    InsertablePackagingProcedureModelExtensionAttributes::ProcedureModel(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.photograph_id` column from table
    /// `packaging_procedure_models`.
    pub fn photograph(
        mut self,
        photograph_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePackagingProcedureModelAttributes>,
    > {
        self.procedure_model = self.procedure_model.photograph(photograph_id).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertablePackagingProcedureModelAttributes::Extension(
                    InsertablePackagingProcedureModelExtensionAttributes::ProcedureModel(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.updated_at` column from table
    /// `packaging_procedure_models`.
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePackagingProcedureModelAttributes>,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.updated_at(updated_at).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertablePackagingProcedureModelAttributes::Extension(
                    InsertablePackagingProcedureModelExtensionAttributes::ProcedureModel(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.updated_by` column from table
    /// `packaging_procedure_models`.
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePackagingProcedureModelAttributes>,
    > {
        self.procedure_model = self.procedure_model.updated_by(updated_by).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertablePackagingProcedureModelAttributes::Extension(
                    InsertablePackagingProcedureModelExtensionAttributes::ProcedureModel(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl<ProcedureModel, C> web_common_traits::database::TryInsertGeneric<C>
for InsertablePackagingProcedureModelBuilder<ProcedureModel>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::packaging_procedure_models::PackagingProcedureModel,
        Error = web_common_traits::database::InsertError<
            InsertablePackagingProcedureModelAttributes,
        >,
    >,
    ProcedureModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::TryInsertGeneric<
        C,
    >,
{
    type Attributes = InsertablePackagingProcedureModelAttributes;
    fn is_complete(&self) -> bool {
        self.procedure_model.is_complete() && self.packaged_with.is_some()
            && self.procedure_packaged_with.is_complete()
            && self.procedure_sample_id.is_complete()
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
        let insertable: crate::codegen::structs_codegen::tables::packaging_procedure_models::PackagingProcedureModel = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
