#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableSharedProcedureModelTrackableAttributes {
    ParentId,
    ParentTrackableId,
    ParentProcedureModelId,
    ChildId,
    ChildTrackableId,
    ChildProcedureModelId,
    CreatedBy,
    CreatedAt,
}
impl core::fmt::Display for InsertableSharedProcedureModelTrackableAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableSharedProcedureModelTrackableAttributes::ParentId => {
                write!(f, "parent_id")
            }
            InsertableSharedProcedureModelTrackableAttributes::ParentTrackableId => {
                write!(f, "parent_trackable_id")
            }
            InsertableSharedProcedureModelTrackableAttributes::ParentProcedureModelId => {
                write!(f, "parent_procedure_model_id")
            }
            InsertableSharedProcedureModelTrackableAttributes::ChildId => {
                write!(f, "child_id")
            }
            InsertableSharedProcedureModelTrackableAttributes::ChildTrackableId => {
                write!(f, "child_trackable_id")
            }
            InsertableSharedProcedureModelTrackableAttributes::ChildProcedureModelId => {
                write!(f, "child_procedure_model_id")
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
    parent_trackable_id: ::rosetta_uuid::Uuid,
    parent_procedure_model_id: i32,
    child_id: i32,
    child_trackable_id: ::rosetta_uuid::Uuid,
    child_procedure_model_id: i32,
    created_by: i32,
    created_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableSharedProcedureModelTrackable {
    #[cfg(feature = "postgres")]
    pub fn shared_procedure_model_tracka_child_id_child_procedure_mod_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        diesel::result::Error,
    >{
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, QueryDsl};
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables::dsl::id
                    .eq(&self.child_id)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables::dsl::procedure_model_id
                            .eq(&self.child_procedure_model_id),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn shared_procedure_model_tracka_parent_id_parent_procedure_m_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        diesel::result::Error,
    >{
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, QueryDsl};
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables::dsl::id
                    .eq(&self.parent_id)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables::dsl::procedure_model_id
                            .eq(&self.parent_procedure_model_id),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn shared_procedure_model_tracka_parent_id_parent_trackable_i_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        diesel::result::Error,
    >{
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, QueryDsl};
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables::dsl::id
                    .eq(&self.parent_id)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables::dsl::trackable_id
                            .eq(&self.parent_trackable_id),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
            >(conn)
    }
    pub fn shared_procedure_model_tracka_parent_procedure_model_id_ch_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel,
        >,
    {
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel::table(),
                (self.parent_procedure_model_id, self.child_procedure_model_id),
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn shared_procedure_model_trackab_child_id_child_trackable_id_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        diesel::result::Error,
    >{
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, QueryDsl};
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables::dsl::id
                    .eq(&self.child_id)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables::dsl::trackable_id
                            .eq(&self.child_trackable_id),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
            >(conn)
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
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel::table(),
                self.parent_procedure_model_id,
            ),
            conn,
        )
    }
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
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable::table(),
                self.child_id,
            ),
            conn,
        )
    }
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
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel::table(),
                self.child_procedure_model_id,
            ),
            conn,
        )
    }
    pub fn child_trackable<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::trackables::Trackable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::trackables::Trackable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::trackables::Trackable,
        >,
    {
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::trackables::Trackable::table(),
                self.child_trackable_id,
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
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
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
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable::table(),
                self.parent_id,
            ),
            conn,
        )
    }
    pub fn parent_trackable<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::trackables::Trackable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::trackables::Trackable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::trackables::Trackable,
        >,
    {
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::trackables::Trackable::table(),
                self.parent_trackable_id,
            ),
            conn,
        )
    }
}
pub struct InsertableSharedProcedureModelTrackableBuilder {
    pub(crate) parent_id: Option<i32>,
    pub(crate) parent_trackable_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) parent_procedure_model_id: Option<i32>,
    pub(crate) child_id: Option<i32>,
    pub(crate) child_trackable_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) child_procedure_model_id: Option<i32>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableSharedProcedureModelTrackableBuilder {
    fn default() -> Self {
        Self {
            parent_id: Default::default(),
            parent_trackable_id: Default::default(),
            parent_procedure_model_id: Default::default(),
            child_id: Default::default(),
            child_trackable_id: Default::default(),
            child_procedure_model_id: Default::default(),
            created_by: Default::default(),
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
    pub fn parent_trackable_id<P>(
        mut self,
        parent_trackable_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableSharedProcedureModelTrackableAttributes>,
    >
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let parent_trackable_id = parent_trackable_id.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(
                    InsertableSharedProcedureModelTrackableAttributes::ParentTrackableId,
                )
            },
        )?;
        self.parent_trackable_id = Some(parent_trackable_id);
        Ok(self)
    }
    pub fn parent_procedure_model_id<P>(
        mut self,
        parent_procedure_model_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableSharedProcedureModelTrackableAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let parent_procedure_model_id =
            parent_procedure_model_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
                Into::into(err).rename_field(
                    InsertableSharedProcedureModelTrackableAttributes::ParentProcedureModelId,
                )
            })?;
        self.parent_procedure_model_id = Some(parent_procedure_model_id);
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
    pub fn child_trackable_id<P>(
        mut self,
        child_trackable_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableSharedProcedureModelTrackableAttributes>,
    >
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let child_trackable_id = child_trackable_id.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(
                    InsertableSharedProcedureModelTrackableAttributes::ChildTrackableId,
                )
            },
        )?;
        self.child_trackable_id = Some(child_trackable_id);
        Ok(self)
    }
    pub fn child_procedure_model_id<P>(
        mut self,
        child_procedure_model_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableSharedProcedureModelTrackableAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let child_procedure_model_id =
            child_procedure_model_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
                Into::into(err).rename_field(
                    InsertableSharedProcedureModelTrackableAttributes::ChildProcedureModelId,
                )
            })?;
        self.child_procedure_model_id = Some(child_procedure_model_id);
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
            parent_trackable_id: builder.parent_trackable_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSharedProcedureModelTrackableAttributes::ParentTrackableId,
                ),
            )?,
            parent_procedure_model_id: builder.parent_procedure_model_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSharedProcedureModelTrackableAttributes::ParentProcedureModelId,
                ),
            )?,
            child_id: builder.child_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSharedProcedureModelTrackableAttributes::ChildId,
                ),
            )?,
            child_trackable_id: builder.child_trackable_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSharedProcedureModelTrackableAttributes::ChildTrackableId,
                ),
            )?,
            child_procedure_model_id: builder.child_procedure_model_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSharedProcedureModelTrackableAttributes::ChildProcedureModelId,
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
