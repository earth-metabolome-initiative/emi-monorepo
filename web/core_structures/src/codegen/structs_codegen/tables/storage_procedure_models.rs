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
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::storage_procedure_models::storage_procedure_models
)]
pub struct StorageProcedureModel {
    pub id: i32,
    pub child_container_id: i32,
    pub parent_container_id: i32,
}
impl web_common_traits::prelude::TableName for StorageProcedureModel {
    const TABLE_NAME: &'static str = "storage_procedure_models";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
    > for StorageProcedureModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl diesel::Identifiable for StorageProcedureModel {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl StorageProcedureModel {
    pub fn child_container<C: diesel::connection::LoadConnection>(
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
                self.child_container_id,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedure_models_child_container_id_id_fkey(
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
                    .eq(&self.child_container_id)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables::dsl::procedure_model_id
                            .eq(&self.id),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
            >(conn)
    }
    pub fn id<C: diesel::connection::LoadConnection>(
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
                self.id,
            ),
            conn,
        )
    }
    pub fn parent_container<C: diesel::connection::LoadConnection>(
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
                self.parent_container_id,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedure_models_parent_container_id_id_fkey(
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
                    .eq(&self.parent_container_id)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables::dsl::procedure_model_id
                            .eq(&self.id),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_child_container_id(
        child_container_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::storage_procedure_models::storage_procedure_models;
        Self::table()
            .filter(storage_procedure_models::child_container_id.eq(child_container_id))
            .order_by(storage_procedure_models::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_container_id(
        parent_container_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::storage_procedure_models::storage_procedure_models;
        Self::table()
            .filter(storage_procedure_models::parent_container_id.eq(parent_container_id))
            .order_by(storage_procedure_models::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_child_container_id_and_id(
        child_container_id: &i32,
        id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::storage_procedure_models::storage_procedure_models;
        Self::table()
            .filter(
                storage_procedure_models::child_container_id
                    .eq(child_container_id)
                    .and(storage_procedure_models::id.eq(id)),
            )
            .order_by(storage_procedure_models::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_container_id_and_id(
        parent_container_id: &i32,
        id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::storage_procedure_models::storage_procedure_models;
        Self::table()
            .filter(
                storage_procedure_models::parent_container_id
                    .eq(parent_container_id)
                    .and(storage_procedure_models::id.eq(id)),
            )
            .order_by(storage_procedure_models::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, OptionalExtension, QueryDsl, RunQueryDsl,
            SelectableHelper, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            procedure_models::procedure_models, storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table.on(storage_procedure_models::id.eq(procedure_models::id)),
            )
            .filter(procedure_models::name.eq(name))
            .order_by(storage_procedure_models::id.asc())
            .select(Self::as_select())
            .first::<Self>(conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn from_description(
        description: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            procedure_models::procedure_models, storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table.on(storage_procedure_models::id.eq(procedure_models::id)),
            )
            .filter(procedure_models::description.eq(description))
            .order_by(storage_procedure_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_deprecated(
        deprecated: &bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            procedure_models::procedure_models, storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table.on(storage_procedure_models::id.eq(procedure_models::id)),
            )
            .filter(procedure_models::deprecated.eq(deprecated))
            .order_by(storage_procedure_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_photograph_id(
        photograph_id: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            procedure_models::procedure_models, storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table.on(storage_procedure_models::id.eq(procedure_models::id)),
            )
            .filter(procedure_models::photograph_id.eq(photograph_id))
            .order_by(storage_procedure_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_icon(
        icon: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            procedure_models::procedure_models, storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table.on(storage_procedure_models::id.eq(procedure_models::id)),
            )
            .filter(procedure_models::icon.eq(icon))
            .order_by(storage_procedure_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_by(
        created_by: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            procedure_models::procedure_models, storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table.on(storage_procedure_models::id.eq(procedure_models::id)),
            )
            .filter(procedure_models::created_by.eq(created_by))
            .order_by(storage_procedure_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_at(
        created_at: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            procedure_models::procedure_models, storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table.on(storage_procedure_models::id.eq(procedure_models::id)),
            )
            .filter(procedure_models::created_at.eq(created_at))
            .order_by(storage_procedure_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_updated_by(
        updated_by: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            procedure_models::procedure_models, storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table.on(storage_procedure_models::id.eq(procedure_models::id)),
            )
            .filter(procedure_models::updated_by.eq(updated_by))
            .order_by(storage_procedure_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_updated_at(
        updated_at: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            procedure_models::procedure_models, storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table.on(storage_procedure_models::id.eq(procedure_models::id)),
            )
            .filter(procedure_models::updated_at.eq(updated_at))
            .order_by(storage_procedure_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<StorageProcedureModel> for StorageProcedureModel {
    fn as_ref(&self) -> &StorageProcedureModel {
        self
    }
}
