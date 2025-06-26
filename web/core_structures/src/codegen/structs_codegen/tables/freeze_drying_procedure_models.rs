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
    table_name = crate::codegen::diesel_codegen::tables::freeze_drying_procedure_models::freeze_drying_procedure_models
)]
pub struct FreezeDryingProcedureModel {
    pub procedure_model_id: i32,
    pub pascal: f32,
    pub seconds: f32,
    pub freeze_dried_with: ::rosetta_uuid::Uuid,
}
impl web_common_traits::prelude::TableName for FreezeDryingProcedureModel {
    const TABLE_NAME: &'static str = "freeze_drying_procedure_models";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
    > for FreezeDryingProcedureModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel,
    > for FreezeDryingProcedureModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl diesel::Identifiable for FreezeDryingProcedureModel {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.procedure_model_id
    }
}
impl FreezeDryingProcedureModel {
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
    pub fn freeze_dried_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::freeze_drier_models::FreezeDrierModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::freeze_drier_models::FreezeDrierModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::freeze_drier_models::FreezeDrierModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freeze_drier_models::FreezeDrierModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::freeze_drier_models::FreezeDrierModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freeze_drier_models::FreezeDrierModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::freeze_drier_models::FreezeDrierModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freeze_drier_models::FreezeDrierModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::freeze_drier_models::FreezeDrierModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::freeze_drier_models::FreezeDrierModel::table(),
                self.freeze_dried_with,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_freeze_dried_with(
        freeze_dried_with: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedure_models::freeze_drying_procedure_models;
        Self::table()
            .filter(freeze_drying_procedure_models::freeze_dried_with.eq(freeze_dried_with))
            .order_by(freeze_drying_procedure_models::procedure_model_id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_model_id_and_freeze_dried_with(
        procedure_model_id: &i32,
        freeze_dried_with: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedure_models::freeze_drying_procedure_models;
        Self::table()
            .filter(
                freeze_drying_procedure_models::procedure_model_id
                    .eq(procedure_model_id)
                    .and(freeze_drying_procedure_models::freeze_dried_with.eq(freeze_dried_with)),
            )
            .order_by(freeze_drying_procedure_models::procedure_model_id.asc())
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
            freeze_drying_procedure_models::freeze_drying_procedure_models,
            procedure_models::procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(
                        freeze_drying_procedure_models::procedure_model_id
                            .eq(procedure_models::id),
                    ),
            )
            .filter(procedure_models::name.eq(name))
            .order_by(freeze_drying_procedure_models::procedure_model_id.asc())
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
            freeze_drying_procedure_models::freeze_drying_procedure_models,
            procedure_models::procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(
                        freeze_drying_procedure_models::procedure_model_id
                            .eq(procedure_models::id),
                    ),
            )
            .filter(procedure_models::description.eq(description))
            .order_by(freeze_drying_procedure_models::procedure_model_id.asc())
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
            freeze_drying_procedure_models::freeze_drying_procedure_models,
            procedure_models::procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(
                        freeze_drying_procedure_models::procedure_model_id
                            .eq(procedure_models::id),
                    ),
            )
            .filter(procedure_models::deprecated.eq(deprecated))
            .order_by(freeze_drying_procedure_models::procedure_model_id.asc())
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
            freeze_drying_procedure_models::freeze_drying_procedure_models,
            procedure_models::procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(
                        freeze_drying_procedure_models::procedure_model_id
                            .eq(procedure_models::id),
                    ),
            )
            .filter(procedure_models::photograph_id.eq(photograph_id))
            .order_by(freeze_drying_procedure_models::procedure_model_id.asc())
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
            freeze_drying_procedure_models::freeze_drying_procedure_models,
            procedure_models::procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(
                        freeze_drying_procedure_models::procedure_model_id
                            .eq(procedure_models::id),
                    ),
            )
            .filter(procedure_models::icon.eq(icon))
            .order_by(freeze_drying_procedure_models::procedure_model_id.asc())
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
            freeze_drying_procedure_models::freeze_drying_procedure_models,
            procedure_models::procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(
                        freeze_drying_procedure_models::procedure_model_id
                            .eq(procedure_models::id),
                    ),
            )
            .filter(procedure_models::created_by.eq(created_by))
            .order_by(freeze_drying_procedure_models::procedure_model_id.asc())
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
            freeze_drying_procedure_models::freeze_drying_procedure_models,
            procedure_models::procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(
                        freeze_drying_procedure_models::procedure_model_id
                            .eq(procedure_models::id),
                    ),
            )
            .filter(procedure_models::created_at.eq(created_at))
            .order_by(freeze_drying_procedure_models::procedure_model_id.asc())
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
            freeze_drying_procedure_models::freeze_drying_procedure_models,
            procedure_models::procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(
                        freeze_drying_procedure_models::procedure_model_id
                            .eq(procedure_models::id),
                    ),
            )
            .filter(procedure_models::updated_by.eq(updated_by))
            .order_by(freeze_drying_procedure_models::procedure_model_id.asc())
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
            freeze_drying_procedure_models::freeze_drying_procedure_models,
            procedure_models::procedure_models,
        };
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(
                        freeze_drying_procedure_models::procedure_model_id
                            .eq(procedure_models::id),
                    ),
            )
            .filter(procedure_models::updated_at.eq(updated_at))
            .order_by(freeze_drying_procedure_models::procedure_model_id.asc())
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
            freeze_drying_procedure_models::freeze_drying_procedure_models,
            storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                storage_procedure_models::table
                    .on(freeze_drying_procedure_models::procedure_model_id
                        .eq(storage_procedure_models::procedure_model_id)),
            )
            .filter(
                storage_procedure_models::procedure_model_id
                    .eq(procedure_model_id)
                    .and(storage_procedure_models::parent_container_id.eq(parent_container_id)),
            )
            .order_by(freeze_drying_procedure_models::procedure_model_id.asc())
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
            freeze_drying_procedure_models::freeze_drying_procedure_models,
            storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                storage_procedure_models::table
                    .on(freeze_drying_procedure_models::procedure_model_id
                        .eq(storage_procedure_models::procedure_model_id)),
            )
            .filter(
                storage_procedure_models::procedure_parent_container_id
                    .eq(procedure_parent_container_id),
            )
            .order_by(freeze_drying_procedure_models::procedure_model_id.asc())
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
            freeze_drying_procedure_models::freeze_drying_procedure_models,
            storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                storage_procedure_models::table
                    .on(freeze_drying_procedure_models::procedure_model_id
                        .eq(storage_procedure_models::procedure_model_id)),
            )
            .filter(storage_procedure_models::child_container_id.eq(child_container_id))
            .order_by(freeze_drying_procedure_models::procedure_model_id.asc())
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
            freeze_drying_procedure_models::freeze_drying_procedure_models,
            storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                storage_procedure_models::table
                    .on(freeze_drying_procedure_models::procedure_model_id
                        .eq(storage_procedure_models::procedure_model_id)),
            )
            .filter(
                storage_procedure_models::procedure_child_container_id
                    .eq(procedure_child_container_id),
            )
            .order_by(freeze_drying_procedure_models::procedure_model_id.asc())
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
            freeze_drying_procedure_models::freeze_drying_procedure_models,
            storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                storage_procedure_models::table
                    .on(freeze_drying_procedure_models::procedure_model_id
                        .eq(storage_procedure_models::procedure_model_id)),
            )
            .filter(
                storage_procedure_models::procedure_parent_container_id
                    .eq(procedure_parent_container_id)
                    .and(storage_procedure_models::procedure_model_id.eq(procedure_model_id)),
            )
            .order_by(freeze_drying_procedure_models::procedure_model_id.asc())
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
            freeze_drying_procedure_models::freeze_drying_procedure_models,
            storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                storage_procedure_models::table
                    .on(freeze_drying_procedure_models::procedure_model_id
                        .eq(storage_procedure_models::procedure_model_id)),
            )
            .filter(
                storage_procedure_models::procedure_child_container_id
                    .eq(procedure_child_container_id)
                    .and(storage_procedure_models::procedure_model_id.eq(procedure_model_id)),
            )
            .order_by(freeze_drying_procedure_models::procedure_model_id.asc())
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
            freeze_drying_procedure_models::freeze_drying_procedure_models,
            storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                storage_procedure_models::table
                    .on(freeze_drying_procedure_models::procedure_model_id
                        .eq(storage_procedure_models::procedure_model_id)),
            )
            .filter(
                storage_procedure_models::procedure_parent_container_id
                    .eq(procedure_parent_container_id)
                    .and(storage_procedure_models::parent_container_id.eq(parent_container_id)),
            )
            .order_by(freeze_drying_procedure_models::procedure_model_id.asc())
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
            freeze_drying_procedure_models::freeze_drying_procedure_models,
            storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                storage_procedure_models::table
                    .on(freeze_drying_procedure_models::procedure_model_id
                        .eq(storage_procedure_models::procedure_model_id)),
            )
            .filter(
                storage_procedure_models::procedure_child_container_id
                    .eq(procedure_child_container_id)
                    .and(storage_procedure_models::child_container_id.eq(child_container_id)),
            )
            .order_by(freeze_drying_procedure_models::procedure_model_id.asc())
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
            freeze_drying_procedure_models::freeze_drying_procedure_models,
            storage_procedure_models::storage_procedure_models,
        };
        Self::table()
            .inner_join(
                storage_procedure_models::table
                    .on(freeze_drying_procedure_models::procedure_model_id
                        .eq(storage_procedure_models::procedure_model_id)),
            )
            .filter(
                storage_procedure_models::parent_container_id
                    .eq(parent_container_id)
                    .and(storage_procedure_models::child_container_id.eq(child_container_id)),
            )
            .order_by(freeze_drying_procedure_models::procedure_model_id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<FreezeDryingProcedureModel> for FreezeDryingProcedureModel {
    fn as_ref(&self) -> &FreezeDryingProcedureModel {
        self
    }
}
