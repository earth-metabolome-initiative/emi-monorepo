#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableParentProcedureModelAttributes {
    ParentProcedureModelId,
    ChildProcedureModelId,
    Snoozable,
    Copiable,
    Repeatable,
    Skippable,
    CreatedBy,
    CreatedAt,
}
impl core::fmt::Display for InsertableParentProcedureModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableParentProcedureModelAttributes::ParentProcedureModelId => {
                write!(f, "parent_procedure_model_id")
            }
            InsertableParentProcedureModelAttributes::ChildProcedureModelId => {
                write!(f, "child_procedure_model_id")
            }
            InsertableParentProcedureModelAttributes::Snoozable => write!(f, "snoozable"),
            InsertableParentProcedureModelAttributes::Copiable => write!(f, "copiable"),
            InsertableParentProcedureModelAttributes::Repeatable => {
                write!(f, "repeatable")
            }
            InsertableParentProcedureModelAttributes::Skippable => write!(f, "skippable"),
            InsertableParentProcedureModelAttributes::CreatedBy => {
                write!(f, "created_by")
            }
            InsertableParentProcedureModelAttributes::CreatedAt => {
                write!(f, "created_at")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::parent_procedure_models::parent_procedure_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableParentProcedureModel {
    parent_procedure_model_id: i32,
    child_procedure_model_id: i32,
    snoozable: bool,
    copiable: bool,
    repeatable: bool,
    skippable: bool,
    created_by: i32,
    created_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableParentProcedureModel {
    pub fn child_procedure_model<C: diesel::connection::LoadConnection>(
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
                self.child_procedure_model_id,
            ),
            conn,
        )
    }
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::users::User,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::users::User: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::users::User,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.created_by,
            ),
            conn,
        )
    }
    pub fn parent_procedure_model<C: diesel::connection::LoadConnection>(
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
                self.parent_procedure_model_id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableParentProcedureModelBuilder {
    pub(crate) parent_procedure_model_id: Option<i32>,
    pub(crate) child_procedure_model_id: Option<i32>,
    pub(crate) snoozable: Option<bool>,
    pub(crate) copiable: Option<bool>,
    pub(crate) repeatable: Option<bool>,
    pub(crate) skippable: Option<bool>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableParentProcedureModelBuilder {
    fn default() -> Self {
        Self {
            parent_procedure_model_id: Default::default(),
            child_procedure_model_id: Default::default(),
            snoozable: Some(false),
            copiable: Some(false),
            repeatable: Some(false),
            skippable: Some(false),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableParentProcedureModelBuilder {
    pub fn parent_procedure_model_id<P>(
        mut self,
        parent_procedure_model_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableParentProcedureModelAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let parent_procedure_model_id =
            parent_procedure_model_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
                Into::into(err)
                    .rename_field(InsertableParentProcedureModelAttributes::ParentProcedureModelId)
            })?;
        if let Some(child_procedure_model_id) = self.child_procedure_model_id {
            pgrx_validation::must_be_distinct_i32(
                parent_procedure_model_id,
                child_procedure_model_id,
            )
            .map_err(|e| {
                e.rename_fields(
                    InsertableParentProcedureModelAttributes::ParentProcedureModelId,
                    InsertableParentProcedureModelAttributes::ChildProcedureModelId,
                )
            })?;
        }
        self.parent_procedure_model_id = Some(parent_procedure_model_id);
        Ok(self)
    }
    pub fn child_procedure_model_id<P>(
        mut self,
        child_procedure_model_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableParentProcedureModelAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let child_procedure_model_id =
            child_procedure_model_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
                Into::into(err)
                    .rename_field(InsertableParentProcedureModelAttributes::ChildProcedureModelId)
            })?;
        if let Some(parent_procedure_model_id) = self.parent_procedure_model_id {
            pgrx_validation::must_be_distinct_i32(
                parent_procedure_model_id,
                child_procedure_model_id,
            )
            .map_err(|e| {
                e.rename_fields(
                    InsertableParentProcedureModelAttributes::ParentProcedureModelId,
                    InsertableParentProcedureModelAttributes::ChildProcedureModelId,
                )
            })?;
        }
        self.child_procedure_model_id = Some(child_procedure_model_id);
        Ok(self)
    }
    pub fn snoozable<P>(
        mut self,
        snoozable: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableParentProcedureModelAttributes>,
    >
    where
        P: TryInto<bool>,
        <P as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let snoozable = snoozable.try_into().map_err(|err: <P as TryInto<bool>>::Error| {
            Into::into(err).rename_field(InsertableParentProcedureModelAttributes::Snoozable)
        })?;
        self.snoozable = Some(snoozable);
        Ok(self)
    }
    pub fn copiable<P>(
        mut self,
        copiable: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableParentProcedureModelAttributes>,
    >
    where
        P: TryInto<bool>,
        <P as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let copiable = copiable.try_into().map_err(|err: <P as TryInto<bool>>::Error| {
            Into::into(err).rename_field(InsertableParentProcedureModelAttributes::Copiable)
        })?;
        self.copiable = Some(copiable);
        Ok(self)
    }
    pub fn repeatable<P>(
        mut self,
        repeatable: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableParentProcedureModelAttributes>,
    >
    where
        P: TryInto<bool>,
        <P as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let repeatable = repeatable.try_into().map_err(|err: <P as TryInto<bool>>::Error| {
            Into::into(err).rename_field(InsertableParentProcedureModelAttributes::Repeatable)
        })?;
        self.repeatable = Some(repeatable);
        Ok(self)
    }
    pub fn skippable<P>(
        mut self,
        skippable: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableParentProcedureModelAttributes>,
    >
    where
        P: TryInto<bool>,
        <P as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let skippable = skippable.try_into().map_err(|err: <P as TryInto<bool>>::Error| {
            Into::into(err).rename_field(InsertableParentProcedureModelAttributes::Skippable)
        })?;
        self.skippable = Some(skippable);
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableParentProcedureModelAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let created_by = created_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableParentProcedureModelAttributes::CreatedBy)
        })?;
        self.created_by = Some(created_by);
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableParentProcedureModelAttributes>,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableParentProcedureModelAttributes::CreatedAt)
            },
        )?;
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl TryFrom<InsertableParentProcedureModelBuilder> for InsertableParentProcedureModel {
    type Error = common_traits::prelude::BuilderError<InsertableParentProcedureModelAttributes>;
    fn try_from(
        builder: InsertableParentProcedureModelBuilder,
    ) -> Result<InsertableParentProcedureModel, Self::Error> {
        Ok(Self {
            parent_procedure_model_id: builder.parent_procedure_model_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableParentProcedureModelAttributes::ParentProcedureModelId,
                ),
            )?,
            child_procedure_model_id: builder.child_procedure_model_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableParentProcedureModelAttributes::ChildProcedureModelId,
                ),
            )?,
            snoozable: builder.snoozable.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableParentProcedureModelAttributes::Snoozable,
                ),
            )?,
            copiable: builder.copiable.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableParentProcedureModelAttributes::Copiable,
                ),
            )?,
            repeatable: builder.repeatable.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableParentProcedureModelAttributes::Repeatable,
                ),
            )?,
            skippable: builder.skippable.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableParentProcedureModelAttributes::Skippable,
                ),
            )?,
            created_by: builder.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableParentProcedureModelAttributes::CreatedBy,
                ),
            )?,
            created_at: builder.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableParentProcedureModelAttributes::CreatedAt,
                ),
            )?,
        })
    }
}
