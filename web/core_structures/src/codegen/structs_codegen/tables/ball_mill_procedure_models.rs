#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(primary_key(procedure_model_id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::ball_mill_procedure_models::ball_mill_procedure_models
)]
pub struct BallMillProcedureModel {
    pub procedure_model_id: i32,
    pub seconds: f32,
    pub hertz: f32,
    pub milled_with: ::rosetta_uuid::Uuid,
}
impl web_common_traits::prelude::TableName for BallMillProcedureModel {
    const TABLE_NAME: &'static str = "ball_mill_procedure_models";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
    > for BallMillProcedureModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel,
    > for BallMillProcedureModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl diesel::Identifiable for BallMillProcedureModel {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.procedure_model_id
    }
}
impl BallMillProcedureModel {
    pub fn procedure_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel::table(),
                self.procedure_model_id,
            ),
            conn,
        )
    }
    pub fn milled_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel::table(),
                self.milled_with,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_milled_with(
        milled_with: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ball_mill_procedure_models::ball_mill_procedure_models;
        Self::table()
            .filter(ball_mill_procedure_models::milled_with.eq(milled_with))
            .order_by(ball_mill_procedure_models::procedure_model_id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_model_id_and_milled_with(
        procedure_model_id: &i32,
        milled_with: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedure_models::ball_mill_procedure_models;
        Self::table()
            .filter(
                ball_mill_procedure_models::procedure_model_id
                    .eq(procedure_model_id)
                    .and(ball_mill_procedure_models::milled_with.eq(milled_with)),
            )
            .order_by(ball_mill_procedure_models::procedure_model_id.asc())
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
            ball_mill_procedure_models::ball_mill_procedure_models,
            procedure_models::procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(ball_mill_procedure_models::procedure_model_id.eq(procedure_models::id)),
            )
            .filter(procedure_models::name.eq(name))
            .order_by(ball_mill_procedure_models::procedure_model_id.asc())
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
            ball_mill_procedure_models::ball_mill_procedure_models,
            procedure_models::procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(ball_mill_procedure_models::procedure_model_id.eq(procedure_models::id)),
            )
            .filter(procedure_models::description.eq(description))
            .order_by(ball_mill_procedure_models::procedure_model_id.asc())
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
            ball_mill_procedure_models::ball_mill_procedure_models,
            procedure_models::procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(ball_mill_procedure_models::procedure_model_id.eq(procedure_models::id)),
            )
            .filter(procedure_models::deprecated.eq(deprecated))
            .order_by(ball_mill_procedure_models::procedure_model_id.asc())
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
            ball_mill_procedure_models::ball_mill_procedure_models,
            procedure_models::procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(ball_mill_procedure_models::procedure_model_id.eq(procedure_models::id)),
            )
            .filter(procedure_models::photograph_id.eq(photograph_id))
            .order_by(ball_mill_procedure_models::procedure_model_id.asc())
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
            ball_mill_procedure_models::ball_mill_procedure_models,
            procedure_models::procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(ball_mill_procedure_models::procedure_model_id.eq(procedure_models::id)),
            )
            .filter(procedure_models::icon.eq(icon))
            .order_by(ball_mill_procedure_models::procedure_model_id.asc())
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
            ball_mill_procedure_models::ball_mill_procedure_models,
            procedure_models::procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(ball_mill_procedure_models::procedure_model_id.eq(procedure_models::id)),
            )
            .filter(procedure_models::created_by.eq(created_by))
            .order_by(ball_mill_procedure_models::procedure_model_id.asc())
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
            ball_mill_procedure_models::ball_mill_procedure_models,
            procedure_models::procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(ball_mill_procedure_models::procedure_model_id.eq(procedure_models::id)),
            )
            .filter(procedure_models::created_at.eq(created_at))
            .order_by(ball_mill_procedure_models::procedure_model_id.asc())
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
            ball_mill_procedure_models::ball_mill_procedure_models,
            procedure_models::procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(ball_mill_procedure_models::procedure_model_id.eq(procedure_models::id)),
            )
            .filter(procedure_models::updated_by.eq(updated_by))
            .order_by(ball_mill_procedure_models::procedure_model_id.asc())
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
            ball_mill_procedure_models::ball_mill_procedure_models,
            procedure_models::procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(ball_mill_procedure_models::procedure_model_id.eq(procedure_models::id)),
            )
            .filter(procedure_models::updated_at.eq(updated_at))
            .order_by(ball_mill_procedure_models::procedure_model_id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_model_id_and_parent_container_id(
        procedure_model_id: &i32,
        parent_container_id: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, JoinOnDsl, OptionalExtension, QueryDsl,
            RunQueryDsl, SelectableHelper, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            ball_mill_procedure_models::ball_mill_procedure_models,
            storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                storage_procedure_models::table.on(ball_mill_procedure_models::procedure_model_id
                    .eq(storage_procedure_models::procedure_model_id)),
            )
            .filter(
                storage_procedure_models::procedure_model_id
                    .eq(procedure_model_id)
                    .and(storage_procedure_models::parent_container_id.eq(parent_container_id)),
            )
            .order_by(ball_mill_procedure_models::procedure_model_id.asc())
            .select(Self::as_select())
            .first::<Self>(conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_parent_container_id(
        procedure_parent_container_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            ball_mill_procedure_models::ball_mill_procedure_models,
            storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                storage_procedure_models::table.on(ball_mill_procedure_models::procedure_model_id
                    .eq(storage_procedure_models::procedure_model_id)),
            )
            .filter(
                storage_procedure_models::procedure_parent_container_id
                    .eq(procedure_parent_container_id),
            )
            .order_by(ball_mill_procedure_models::procedure_model_id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_child_container_id(
        child_container_id: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            ball_mill_procedure_models::ball_mill_procedure_models,
            storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                storage_procedure_models::table.on(ball_mill_procedure_models::procedure_model_id
                    .eq(storage_procedure_models::procedure_model_id)),
            )
            .filter(storage_procedure_models::child_container_id.eq(child_container_id))
            .order_by(ball_mill_procedure_models::procedure_model_id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_child_container_id(
        procedure_child_container_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            ball_mill_procedure_models::ball_mill_procedure_models,
            storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                storage_procedure_models::table.on(ball_mill_procedure_models::procedure_model_id
                    .eq(storage_procedure_models::procedure_model_id)),
            )
            .filter(
                storage_procedure_models::procedure_child_container_id
                    .eq(procedure_child_container_id),
            )
            .order_by(ball_mill_procedure_models::procedure_model_id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_parent_container_id_and_procedure_model_id(
        procedure_parent_container_id: &i32,
        procedure_model_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl,
            SelectableHelper, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            ball_mill_procedure_models::ball_mill_procedure_models,
            storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                storage_procedure_models::table.on(ball_mill_procedure_models::procedure_model_id
                    .eq(storage_procedure_models::procedure_model_id)),
            )
            .filter(
                storage_procedure_models::procedure_parent_container_id
                    .eq(procedure_parent_container_id)
                    .and(storage_procedure_models::procedure_model_id.eq(procedure_model_id)),
            )
            .order_by(ball_mill_procedure_models::procedure_model_id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_child_container_id_and_procedure_model_id(
        procedure_child_container_id: &i32,
        procedure_model_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl,
            SelectableHelper, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            ball_mill_procedure_models::ball_mill_procedure_models,
            storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                storage_procedure_models::table.on(ball_mill_procedure_models::procedure_model_id
                    .eq(storage_procedure_models::procedure_model_id)),
            )
            .filter(
                storage_procedure_models::procedure_child_container_id
                    .eq(procedure_child_container_id)
                    .and(storage_procedure_models::procedure_model_id.eq(procedure_model_id)),
            )
            .order_by(ball_mill_procedure_models::procedure_model_id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_parent_container_id_and_parent_container_id(
        procedure_parent_container_id: &i32,
        parent_container_id: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl,
            SelectableHelper, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            ball_mill_procedure_models::ball_mill_procedure_models,
            storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                storage_procedure_models::table.on(ball_mill_procedure_models::procedure_model_id
                    .eq(storage_procedure_models::procedure_model_id)),
            )
            .filter(
                storage_procedure_models::procedure_parent_container_id
                    .eq(procedure_parent_container_id)
                    .and(storage_procedure_models::parent_container_id.eq(parent_container_id)),
            )
            .order_by(ball_mill_procedure_models::procedure_model_id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_child_container_id_and_child_container_id(
        procedure_child_container_id: &i32,
        child_container_id: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl,
            SelectableHelper, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            ball_mill_procedure_models::ball_mill_procedure_models,
            storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                storage_procedure_models::table.on(ball_mill_procedure_models::procedure_model_id
                    .eq(storage_procedure_models::procedure_model_id)),
            )
            .filter(
                storage_procedure_models::procedure_child_container_id
                    .eq(procedure_child_container_id)
                    .and(storage_procedure_models::child_container_id.eq(child_container_id)),
            )
            .order_by(ball_mill_procedure_models::procedure_model_id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_container_id_and_child_container_id(
        parent_container_id: &::rosetta_uuid::Uuid,
        child_container_id: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl,
            SelectableHelper, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            ball_mill_procedure_models::ball_mill_procedure_models,
            storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                storage_procedure_models::table.on(ball_mill_procedure_models::procedure_model_id
                    .eq(storage_procedure_models::procedure_model_id)),
            )
            .filter(
                storage_procedure_models::parent_container_id
                    .eq(parent_container_id)
                    .and(storage_procedure_models::child_container_id.eq(child_container_id)),
            )
            .order_by(ball_mill_procedure_models::procedure_model_id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<BallMillProcedureModel> for BallMillProcedureModel {
    fn as_ref(&self) -> &BallMillProcedureModel {
        self
    }
}
