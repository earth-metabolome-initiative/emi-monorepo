#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableSharedProcedureModelTrackableAttributes {
    ParentId,
    ChildId,
    CreatedBy,
    CreatedAt,
}
impl core::fmt::Display for InsertableSharedProcedureModelTrackableAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableSharedProcedureModelTrackableAttributes::ParentId => {
                write!(f, "parent_id")
            }
            InsertableSharedProcedureModelTrackableAttributes::ChildId => {
                write!(f, "child_id")
            }
            InsertableSharedProcedureModelTrackableAttributes::CreatedBy => {
                write!(f, "created_by")
            }
            InsertableSharedProcedureModelTrackableAttributes::CreatedAt => {
                write!(f, "created_at")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::shared_procedure_model_trackables::shared_procedure_model_trackables
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSharedProcedureModelTrackable {
    parent_id: i32,
    child_id: i32,
    created_by: i32,
    created_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableSharedProcedureModelTrackable {
    pub fn child<C: diesel::connection::LoadConnection>(
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
                self.child_id,
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
    pub fn parent<C: diesel::connection::LoadConnection>(
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
                self.parent_id,
            ),
            conn,
        )
    }
}
pub struct InsertableSharedProcedureModelTrackableBuilder {
    pub(crate) parent_id: Option<i32>,
    pub(crate) child_id: Option<i32>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableSharedProcedureModelTrackableBuilder {
    fn default() -> Self {
        Self {
            parent_id: None,
            child_id: None,
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableSharedProcedureModelTrackableBuilder {
    pub fn parent_id<P>(
        mut self,
        parent_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableSharedProcedureModelTrackableAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let parent_id = parent_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err)
                .rename_field(InsertableSharedProcedureModelTrackableAttributes::ParentId)
        })?;
        if let Some(child_id) = self.child_id {
            pgrx_validation::must_be_distinct_i32(parent_id, child_id).map_err(|e| {
                e.rename_fields(
                    InsertableSharedProcedureModelTrackableAttributes::ParentId,
                    InsertableSharedProcedureModelTrackableAttributes::ChildId,
                )
            })?;
        }
        self.parent_id = Some(parent_id);
        Ok(self)
    }
    pub fn child_id<P>(
        mut self,
        child_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableSharedProcedureModelTrackableAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let child_id = child_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableSharedProcedureModelTrackableAttributes::ChildId)
        })?;
        if let Some(parent_id) = self.parent_id {
            pgrx_validation::must_be_distinct_i32(parent_id, child_id).map_err(|e| {
                e.rename_fields(
                    InsertableSharedProcedureModelTrackableAttributes::ParentId,
                    InsertableSharedProcedureModelTrackableAttributes::ChildId,
                )
            })?;
        }
        self.child_id = Some(child_id);
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableSharedProcedureModelTrackableAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let created_by = created_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err)
                .rename_field(InsertableSharedProcedureModelTrackableAttributes::CreatedBy)
        })?;
        self.created_by = Some(created_by);
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableSharedProcedureModelTrackableAttributes>,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err)
                    .rename_field(InsertableSharedProcedureModelTrackableAttributes::CreatedAt)
            },
        )?;
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl TryFrom<InsertableSharedProcedureModelTrackableBuilder>
    for InsertableSharedProcedureModelTrackable
{
    type Error =
        common_traits::prelude::BuilderError<InsertableSharedProcedureModelTrackableAttributes>;
    fn try_from(
        builder: InsertableSharedProcedureModelTrackableBuilder,
    ) -> Result<InsertableSharedProcedureModelTrackable, Self::Error> {
        Ok(Self {
            parent_id: builder.parent_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSharedProcedureModelTrackableAttributes::ParentId,
                ),
            )?,
            child_id: builder.child_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSharedProcedureModelTrackableAttributes::ChildId,
                ),
            )?,
            created_by: builder.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSharedProcedureModelTrackableAttributes::CreatedBy,
                ),
            )?,
            created_at: builder.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSharedProcedureModelTrackableAttributes::CreatedAt,
                ),
            )?,
        })
    }
}
