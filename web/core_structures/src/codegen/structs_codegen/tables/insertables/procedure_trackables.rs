#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableProcedureTrackableAttributes {
    ProcedureId,
    ProcedureModelId,
    TrackableId,
    ProcedureModelTrackableId,
    AncestorTrackableId,
    ParentTrackableId,
    CreatedBy,
    CreatedAt,
}
impl core::fmt::Display for InsertableProcedureTrackableAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableProcedureTrackableAttributes::ProcedureId => {
                write!(f, "procedure_id")
            }
            InsertableProcedureTrackableAttributes::ProcedureModelId => {
                write!(f, "procedure_model_id")
            }
            InsertableProcedureTrackableAttributes::TrackableId => {
                write!(f, "trackable_id")
            }
            InsertableProcedureTrackableAttributes::ProcedureModelTrackableId => {
                write!(f, "procedure_model_trackable_id")
            }
            InsertableProcedureTrackableAttributes::AncestorTrackableId => {
                write!(f, "ancestor_trackable_id")
            }
            InsertableProcedureTrackableAttributes::ParentTrackableId => {
                write!(f, "parent_trackable_id")
            }
            InsertableProcedureTrackableAttributes::CreatedBy => write!(f, "created_by"),
            InsertableProcedureTrackableAttributes::CreatedAt => write!(f, "created_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::procedure_trackables::procedure_trackables
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableProcedureTrackable {
    procedure_id: ::rosetta_uuid::Uuid,
    procedure_model_id: i32,
    trackable_id: ::rosetta_uuid::Uuid,
    procedure_model_trackable_id: i32,
    ancestor_trackable_id: ::rosetta_uuid::Uuid,
    parent_trackable_id: ::rosetta_uuid::Uuid,
    created_by: i32,
    created_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableProcedureTrackable {
    pub fn ancestor_trackable<C: diesel::connection::LoadConnection>(
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
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::trackables::Trackable::table(),
                self.ancestor_trackable_id,
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
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::trackables::Trackable::table(),
                self.parent_trackable_id,
            ),
            conn,
        )
    }
    pub fn procedure<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedures::Procedure: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedures::Procedure,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedures::Procedure::table(),
                self.procedure_id,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn procedure_trackables_procedure_id_procedure_model_id_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::procedures::Procedure, diesel::result::Error>
    {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedures::Procedure::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::id
                    .eq(&self.procedure_id)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure_model_id
                            .eq(&self.procedure_model_id),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedures::Procedure,
            >(conn)
    }
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
    #[cfg(feature = "postgres")]
    pub fn procedure_trackables_procedure_model_trackable_id_ancestor_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables::dsl::id
                    .eq(&self.procedure_model_trackable_id)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables::dsl::trackable_id
                            .eq(&self.ancestor_trackable_id),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
            >(conn)
    }
    pub fn procedure_model_trackable<C: diesel::connection::LoadConnection>(
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
                self.procedure_model_trackable_id,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn procedure_trackables_procedure_model_trackable_id_procedur_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables::dsl::id
                    .eq(&self.procedure_model_trackable_id)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables::dsl::procedure_model_id
                            .eq(&self.procedure_model_id),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
            >(conn)
    }
    pub fn trackable<C: diesel::connection::LoadConnection>(
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
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::trackables::Trackable::table(),
                self.trackable_id,
            ),
            conn,
        )
    }
}
pub struct InsertableProcedureTrackableBuilder {
    pub(crate) procedure_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_model_id: Option<i32>,
    pub(crate) trackable_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_model_trackable_id: Option<i32>,
    pub(crate) ancestor_trackable_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) parent_trackable_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableProcedureTrackableBuilder {
    fn default() -> Self {
        Self {
            procedure_id: Default::default(),
            procedure_model_id: Default::default(),
            trackable_id: Default::default(),
            procedure_model_trackable_id: Default::default(),
            ancestor_trackable_id: Default::default(),
            parent_trackable_id: Default::default(),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableProcedureTrackableBuilder {
    pub fn procedure_id<P>(
        mut self,
        procedure_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcedureTrackableAttributes>,
    >
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let procedure_id = procedure_id.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(InsertableProcedureTrackableAttributes::ProcedureId)
            },
        )?;
        self.procedure_id = Some(procedure_id);
        Ok(self)
    }
    pub fn procedure_model_id<P>(
        mut self,
        procedure_model_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcedureTrackableAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let procedure_model_id =
            procedure_model_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
                Into::into(err)
                    .rename_field(InsertableProcedureTrackableAttributes::ProcedureModelId)
            })?;
        self.procedure_model_id = Some(procedure_model_id);
        Ok(self)
    }
    pub fn trackable_id<P>(
        mut self,
        trackable_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcedureTrackableAttributes>,
    >
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let trackable_id = trackable_id.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(InsertableProcedureTrackableAttributes::TrackableId)
            },
        )?;
        self.trackable_id = Some(trackable_id);
        Ok(self)
    }
    pub fn procedure_model_trackable_id<P>(
        mut self,
        procedure_model_trackable_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcedureTrackableAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let procedure_model_trackable_id = procedure_model_trackable_id.try_into().map_err(
            |err: <P as TryInto<i32>>::Error| {
                Into::into(err)
                    .rename_field(InsertableProcedureTrackableAttributes::ProcedureModelTrackableId)
            },
        )?;
        self.procedure_model_trackable_id = Some(procedure_model_trackable_id);
        Ok(self)
    }
    pub fn ancestor_trackable_id<P>(
        mut self,
        ancestor_trackable_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcedureTrackableAttributes>,
    >
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let ancestor_trackable_id = ancestor_trackable_id.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err)
                    .rename_field(InsertableProcedureTrackableAttributes::AncestorTrackableId)
            },
        )?;
        self.ancestor_trackable_id = Some(ancestor_trackable_id);
        Ok(self)
    }
    pub fn parent_trackable_id<P>(
        mut self,
        parent_trackable_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcedureTrackableAttributes>,
    >
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let parent_trackable_id = parent_trackable_id.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err)
                    .rename_field(InsertableProcedureTrackableAttributes::ParentTrackableId)
            },
        )?;
        self.parent_trackable_id = Some(parent_trackable_id);
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcedureTrackableAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let created_by = created_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableProcedureTrackableAttributes::CreatedBy)
        })?;
        self.created_by = Some(created_by);
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcedureTrackableAttributes>,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableProcedureTrackableAttributes::CreatedAt)
            },
        )?;
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl TryFrom<InsertableProcedureTrackableBuilder> for InsertableProcedureTrackable {
    type Error = common_traits::prelude::BuilderError<InsertableProcedureTrackableAttributes>;
    fn try_from(
        builder: InsertableProcedureTrackableBuilder,
    ) -> Result<InsertableProcedureTrackable, Self::Error> {
        Ok(Self {
            procedure_id: builder.procedure_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureTrackableAttributes::ProcedureId,
                ),
            )?,
            procedure_model_id: builder.procedure_model_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureTrackableAttributes::ProcedureModelId,
                ),
            )?,
            trackable_id: builder.trackable_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureTrackableAttributes::TrackableId,
                ),
            )?,
            procedure_model_trackable_id: builder.procedure_model_trackable_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureTrackableAttributes::ProcedureModelTrackableId,
                ),
            )?,
            ancestor_trackable_id: builder.ancestor_trackable_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureTrackableAttributes::AncestorTrackableId,
                ),
            )?,
            parent_trackable_id: builder.parent_trackable_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureTrackableAttributes::ParentTrackableId,
                ),
            )?,
            created_by: builder.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureTrackableAttributes::CreatedBy,
                ),
            )?,
            created_at: builder.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureTrackableAttributes::CreatedAt,
                ),
            )?,
        })
    }
}
