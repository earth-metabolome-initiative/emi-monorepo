#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(primary_key(parent_id, child_id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::shared_procedure_model_trackables::shared_procedure_model_trackables
)]
pub struct SharedProcedureModelTrackable {
    pub parent_id: i32,
    pub parent_trackable_id: ::rosetta_uuid::Uuid,
    pub parent_procedure_model_id: i32,
    pub child_id: i32,
    pub child_trackable_id: ::rosetta_uuid::Uuid,
    pub child_procedure_model_id: i32,
    pub created_by: i32,
    pub created_at: ::rosetta_timestamp::TimestampUTC,
}
impl web_common_traits::prelude::TableName for SharedProcedureModelTrackable {
    const TABLE_NAME: &'static str = "shared_procedure_model_trackables";
}
impl diesel::Identifiable for SharedProcedureModelTrackable {
    type Id = (i32, i32);
    fn id(self) -> Self::Id {
        (self.parent_id, self.child_id)
    }
}
impl SharedProcedureModelTrackable {
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
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::trackables::Trackable::table(),
                self.child_trackable_id,
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
    #[cfg(feature = "postgres")]
    pub fn from_parent_id(
        parent_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::shared_procedure_model_trackables::shared_procedure_model_trackables;
        Self::table()
            .filter(shared_procedure_model_trackables::parent_id.eq(parent_id))
            .order_by((
                shared_procedure_model_trackables::parent_id.asc(),
                shared_procedure_model_trackables::child_id.asc(),
            ))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_trackable_id(
        parent_trackable_id: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::shared_procedure_model_trackables::shared_procedure_model_trackables;
        Self::table()
            .filter(shared_procedure_model_trackables::parent_trackable_id.eq(parent_trackable_id))
            .order_by((
                shared_procedure_model_trackables::parent_id.asc(),
                shared_procedure_model_trackables::child_id.asc(),
            ))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_procedure_model_id(
        parent_procedure_model_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::shared_procedure_model_trackables::shared_procedure_model_trackables;
        Self::table()
            .filter(
                shared_procedure_model_trackables::parent_procedure_model_id
                    .eq(parent_procedure_model_id),
            )
            .order_by((
                shared_procedure_model_trackables::parent_id.asc(),
                shared_procedure_model_trackables::child_id.asc(),
            ))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_child_id(
        child_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::shared_procedure_model_trackables::shared_procedure_model_trackables;
        Self::table()
            .filter(shared_procedure_model_trackables::child_id.eq(child_id))
            .order_by((
                shared_procedure_model_trackables::parent_id.asc(),
                shared_procedure_model_trackables::child_id.asc(),
            ))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_child_trackable_id(
        child_trackable_id: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::shared_procedure_model_trackables::shared_procedure_model_trackables;
        Self::table()
            .filter(shared_procedure_model_trackables::child_trackable_id.eq(child_trackable_id))
            .order_by((
                shared_procedure_model_trackables::parent_id.asc(),
                shared_procedure_model_trackables::child_id.asc(),
            ))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_child_procedure_model_id(
        child_procedure_model_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::shared_procedure_model_trackables::shared_procedure_model_trackables;
        Self::table()
            .filter(
                shared_procedure_model_trackables::child_procedure_model_id
                    .eq(child_procedure_model_id),
            )
            .order_by((
                shared_procedure_model_trackables::parent_id.asc(),
                shared_procedure_model_trackables::child_id.asc(),
            ))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_by(
        created_by: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::shared_procedure_model_trackables::shared_procedure_model_trackables;
        Self::table()
            .filter(shared_procedure_model_trackables::created_by.eq(created_by))
            .order_by((
                shared_procedure_model_trackables::parent_id.asc(),
                shared_procedure_model_trackables::child_id.asc(),
            ))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_at(
        created_at: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::shared_procedure_model_trackables::shared_procedure_model_trackables;
        Self::table()
            .filter(shared_procedure_model_trackables::created_at.eq(created_at))
            .order_by((
                shared_procedure_model_trackables::parent_id.asc(),
                shared_procedure_model_trackables::child_id.asc(),
            ))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_id_and_parent_trackable_id(
        parent_id: &i32,
        parent_trackable_id: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::shared_procedure_model_trackables::shared_procedure_model_trackables;
        Self::table()
            .filter(shared_procedure_model_trackables::parent_id.eq(parent_id).and(
                shared_procedure_model_trackables::parent_trackable_id.eq(parent_trackable_id),
            ))
            .order_by((
                shared_procedure_model_trackables::parent_id.asc(),
                shared_procedure_model_trackables::child_id.asc(),
            ))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_child_id_and_child_trackable_id(
        child_id: &i32,
        child_trackable_id: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::shared_procedure_model_trackables::shared_procedure_model_trackables;
        Self::table()
            .filter(
                shared_procedure_model_trackables::child_id.eq(child_id).and(
                    shared_procedure_model_trackables::child_trackable_id.eq(child_trackable_id),
                ),
            )
            .order_by((
                shared_procedure_model_trackables::parent_id.asc(),
                shared_procedure_model_trackables::child_id.asc(),
            ))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_id_and_parent_procedure_model_id(
        parent_id: &i32,
        parent_procedure_model_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::shared_procedure_model_trackables::shared_procedure_model_trackables;
        Self::table()
            .filter(
                shared_procedure_model_trackables::parent_id.eq(parent_id).and(
                    shared_procedure_model_trackables::parent_procedure_model_id
                        .eq(parent_procedure_model_id),
                ),
            )
            .order_by((
                shared_procedure_model_trackables::parent_id.asc(),
                shared_procedure_model_trackables::child_id.asc(),
            ))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_child_id_and_child_procedure_model_id(
        child_id: &i32,
        child_procedure_model_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::shared_procedure_model_trackables::shared_procedure_model_trackables;
        Self::table()
            .filter(
                shared_procedure_model_trackables::child_id.eq(child_id).and(
                    shared_procedure_model_trackables::child_procedure_model_id
                        .eq(child_procedure_model_id),
                ),
            )
            .order_by((
                shared_procedure_model_trackables::parent_id.asc(),
                shared_procedure_model_trackables::child_id.asc(),
            ))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_procedure_model_id_and_child_procedure_model_id(
        parent_procedure_model_id: &i32,
        child_procedure_model_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::shared_procedure_model_trackables::shared_procedure_model_trackables;
        Self::table()
            .filter(
                shared_procedure_model_trackables::parent_procedure_model_id
                    .eq(parent_procedure_model_id)
                    .and(
                        shared_procedure_model_trackables::child_procedure_model_id
                            .eq(child_procedure_model_id),
                    ),
            )
            .order_by((
                shared_procedure_model_trackables::parent_id.asc(),
                shared_procedure_model_trackables::child_id.asc(),
            ))
            .load::<Self>(conn)
    }
}
impl AsRef<SharedProcedureModelTrackable> for SharedProcedureModelTrackable {
    fn as_ref(&self) -> &SharedProcedureModelTrackable {
        self
    }
}
