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
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::aliquoting_procedure_models::aliquoting_procedure_models
)]
pub struct AliquotingProcedureModel {
    pub id: i32,
    pub liters: f32,
    pub error: f32,
}
impl web_common_traits::prelude::TableName for AliquotingProcedureModel {
    const TABLE_NAME: &'static str = "aliquoting_procedure_models";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
    > for AliquotingProcedureModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl diesel::Identifiable for AliquotingProcedureModel {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl AliquotingProcedureModel {
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
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel::table(),
                self.id,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_models::aliquoting_procedure_models;
        use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
        use diesel::OptionalExtension;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(aliquoting_procedure_models::id.eq(procedure_models::id)),
            )
            .filter(procedure_models::name.eq(name))
            .order_by(aliquoting_procedure_models::id.asc())
            .select(Self::as_select())
            .first::<Self>(conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn from_description(
        description: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_models::aliquoting_procedure_models;
        use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(aliquoting_procedure_models::id.eq(procedure_models::id)),
            )
            .filter(procedure_models::description.eq(description))
            .order_by(aliquoting_procedure_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_deprecated(
        deprecated: &bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_models::aliquoting_procedure_models;
        use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(aliquoting_procedure_models::id.eq(procedure_models::id)),
            )
            .filter(procedure_models::deprecated.eq(deprecated))
            .order_by(aliquoting_procedure_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_photograph_id(
        photograph_id: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_models::aliquoting_procedure_models;
        use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(aliquoting_procedure_models::id.eq(procedure_models::id)),
            )
            .filter(procedure_models::photograph_id.eq(photograph_id))
            .order_by(aliquoting_procedure_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_icon(
        icon: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_models::aliquoting_procedure_models;
        use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(aliquoting_procedure_models::id.eq(procedure_models::id)),
            )
            .filter(procedure_models::icon.eq(icon))
            .order_by(aliquoting_procedure_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_by(
        created_by: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_models::aliquoting_procedure_models;
        use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(aliquoting_procedure_models::id.eq(procedure_models::id)),
            )
            .filter(procedure_models::created_by.eq(created_by))
            .order_by(aliquoting_procedure_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_at(
        created_at: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_models::aliquoting_procedure_models;
        use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(aliquoting_procedure_models::id.eq(procedure_models::id)),
            )
            .filter(procedure_models::created_at.eq(created_at))
            .order_by(aliquoting_procedure_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_updated_by(
        updated_by: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_models::aliquoting_procedure_models;
        use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(aliquoting_procedure_models::id.eq(procedure_models::id)),
            )
            .filter(procedure_models::updated_by.eq(updated_by))
            .order_by(aliquoting_procedure_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_updated_at(
        updated_at: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_models::aliquoting_procedure_models;
        use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
        Self::table()
            .inner_join(
                procedure_models::table
                    .on(aliquoting_procedure_models::id.eq(procedure_models::id)),
            )
            .filter(procedure_models::updated_at.eq(updated_at))
            .order_by(aliquoting_procedure_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<AliquotingProcedureModel> for AliquotingProcedureModel {
    fn as_ref(&self) -> &AliquotingProcedureModel {
        self
    }
}
