#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableBinaryQuestionProcedureModelExtensionAttributes {
    ProcedureModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
    ),
}
impl core::fmt::Display for InsertableBinaryQuestionProcedureModelExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ProcedureModel(e) => write!(f, "{e}"),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableBinaryQuestionProcedureModelAttributes {
    Extension(InsertableBinaryQuestionProcedureModelExtensionAttributes),
    ProcedureModelId,
    TrackableId,
}
impl core::fmt::Display for InsertableBinaryQuestionProcedureModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureModelId => write!(f, "procedure_model_id"),
            Self::TrackableId => write!(f, "trackable_id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::binary_question_procedure_models::binary_question_procedure_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableBinaryQuestionProcedureModel {
    pub(crate) procedure_model_id: i32,
    pub(crate) trackable_id: i32,
}
impl InsertableBinaryQuestionProcedureModel {
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
    pub fn trackable<C: diesel::connection::LoadConnection>(
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
                self.trackable_id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableBinaryQuestionProcedureModelBuilder<
    ProcedureModel
        = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    pub(crate) trackable_id: Option<i32>,
    pub(crate) procedure_model: ProcedureModel,
}
impl<ProcedureModel> web_common_traits::database::ExtendableBuilder
for InsertableBinaryQuestionProcedureModelBuilder<ProcedureModel>
where
    ProcedureModel: web_common_traits::database::ExtendableBuilder<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
    >,
{
    type Attributes = InsertableBinaryQuestionProcedureModelAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure_model = self
            .procedure_model
            .extend_builder(other.procedure_model)
            .map_err(|err| {
                err.into_field_name(|attribute| InsertableBinaryQuestionProcedureModelAttributes::Extension(
                    InsertableBinaryQuestionProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                ))
            })?;
        if let Some(trackable_id) = other.trackable_id {
            self = self.trackable(trackable_id)?;
        }
        Ok(self)
    }
}
impl<ProcedureModel> web_common_traits::prelude::SetPrimaryKey
    for InsertableBinaryQuestionProcedureModelBuilder<ProcedureModel>
where
    ProcedureModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.procedure_model = self.procedure_model.set_primary_key(primary_key);
        self
    }
}
impl<
    ProcedureModel,
> crate::codegen::structs_codegen::tables::insertables::InsertableBinaryQuestionProcedureModelBuilder<
    ProcedureModel,
> {
    ///Sets the value of the `binary_question_procedure_models.trackable_id` column from table `binary_question_procedure_models`.
    pub fn trackable(
        mut self,
        trackable_id: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableBinaryQuestionProcedureModelAttributes,
        >,
    > {
        self.trackable_id = Some(trackable_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableBinaryQuestionProcedureModelBuilder<
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    ///Sets the value of the `procedure_models.created_at` column from table `binary_question_procedure_models`.
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableBinaryQuestionProcedureModelAttributes,
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
                    .into_field_name(|attribute| InsertableBinaryQuestionProcedureModelAttributes::Extension(
                        InsertableBinaryQuestionProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    ))
            })?;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableBinaryQuestionProcedureModelBuilder<
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    ///Sets the value of the `procedure_models.created_by` column from table `binary_question_procedure_models`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableBinaryQuestionProcedureModelAttributes,
        >,
    > {
        self.procedure_model = self
            .procedure_model
            .created_by(created_by)
            .map_err(|e| {
                e
                    .into_field_name(|attribute| InsertableBinaryQuestionProcedureModelAttributes::Extension(
                        InsertableBinaryQuestionProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    ))
            })?;
        self = self.updated_by(created_by)?;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableBinaryQuestionProcedureModelBuilder<
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    ///Sets the value of the `procedure_models.deprecated` column from table `binary_question_procedure_models`.
    pub fn deprecated<P>(
        mut self,
        deprecated: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableBinaryQuestionProcedureModelAttributes,
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
                    .into_field_name(|attribute| InsertableBinaryQuestionProcedureModelAttributes::Extension(
                        InsertableBinaryQuestionProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    ))
            })?;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableBinaryQuestionProcedureModelBuilder<
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    ///Sets the value of the `procedure_models.description` column from table `binary_question_procedure_models`.
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableBinaryQuestionProcedureModelAttributes,
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
                    .into_field_name(|attribute| InsertableBinaryQuestionProcedureModelAttributes::Extension(
                        InsertableBinaryQuestionProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    ))
            })?;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableBinaryQuestionProcedureModelBuilder<
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    ///Sets the value of the `procedure_models.icon` column from table `binary_question_procedure_models`.
    pub fn icon<P>(
        mut self,
        icon: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableBinaryQuestionProcedureModelAttributes,
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
                    .into_field_name(|attribute| InsertableBinaryQuestionProcedureModelAttributes::Extension(
                        InsertableBinaryQuestionProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    ))
            })?;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableBinaryQuestionProcedureModelBuilder<
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    ///Sets the value of the `procedure_models.name` column from table `binary_question_procedure_models`.
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableBinaryQuestionProcedureModelAttributes,
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
                    .into_field_name(|attribute| InsertableBinaryQuestionProcedureModelAttributes::Extension(
                        InsertableBinaryQuestionProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    ))
            })?;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableBinaryQuestionProcedureModelBuilder<
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    ///Sets the value of the `procedure_models.photograph_id` column from table `binary_question_procedure_models`.
    pub fn photograph(
        mut self,
        photograph_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableBinaryQuestionProcedureModelAttributes,
        >,
    > {
        self.procedure_model = self
            .procedure_model
            .photograph(photograph_id)
            .map_err(|e| {
                e
                    .into_field_name(|attribute| InsertableBinaryQuestionProcedureModelAttributes::Extension(
                        InsertableBinaryQuestionProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    ))
            })?;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableBinaryQuestionProcedureModelBuilder<
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    ///Sets the value of the `procedure_models.updated_at` column from table `binary_question_procedure_models`.
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableBinaryQuestionProcedureModelAttributes,
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
                    .into_field_name(|attribute| InsertableBinaryQuestionProcedureModelAttributes::Extension(
                        InsertableBinaryQuestionProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    ))
            })?;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableBinaryQuestionProcedureModelBuilder<
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    ///Sets the value of the `procedure_models.updated_by` column from table `binary_question_procedure_models`.
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableBinaryQuestionProcedureModelAttributes,
        >,
    > {
        self.procedure_model = self
            .procedure_model
            .updated_by(updated_by)
            .map_err(|e| {
                e
                    .into_field_name(|attribute| InsertableBinaryQuestionProcedureModelAttributes::Extension(
                        InsertableBinaryQuestionProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    ))
            })?;
        Ok(self)
    }
}
impl<ProcedureModel, C> web_common_traits::database::TryInsertGeneric<C>
for InsertableBinaryQuestionProcedureModelBuilder<ProcedureModel>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::binary_question_procedure_models::BinaryQuestionProcedureModel,
        Error = web_common_traits::database::InsertError<
            InsertableBinaryQuestionProcedureModelAttributes,
        >,
    >,
    ProcedureModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
{
    type Attributes = InsertableBinaryQuestionProcedureModelAttributes;
    fn is_complete(&self) -> bool {
        self.procedure_model.is_complete() && self.trackable_id.is_some()
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
        let insertable: crate::codegen::structs_codegen::tables::binary_question_procedure_models::BinaryQuestionProcedureModel = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
