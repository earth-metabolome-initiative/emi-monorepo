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
#[diesel(primary_key(procedure))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::packaging_procedures::packaging_procedures
)]
pub struct PackagingProcedure {
    pub procedure: ::rosetta_uuid::Uuid,
    pub procedure_template: i32,
    pub packaged_with_model: i32,
}
impl web_common_traits::prelude::TableName for PackagingProcedure {
    const TABLE_NAME: &'static str = "packaging_procedures";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    > for PackagingProcedure
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for PackagingProcedure {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.procedure
    }
}
impl PackagingProcedure {
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
                self.procedure,
            ),
            conn,
        )
    }
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate::table(),
                self.procedure_template,
            ),
            conn,
        )
    }
    pub fn packaged_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::packaging_models::PackagingModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::packaging_models::PackagingModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::packaging_models::PackagingModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::packaging_models::PackagingModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::packaging_models::PackagingModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::packaging_models::PackagingModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::packaging_models::PackagingModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::packaging_models::PackagingModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::packaging_models::PackagingModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::packaging_models::PackagingModel::table(),
                self.packaged_with_model,
            ),
            conn,
        )
    }
    pub fn packaging_procedures_procedure_packaged_with_model_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table(),
                (self.procedure, self.packaged_with_model),
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template(
        procedure_template: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::packaging_procedures::packaging_procedures;
        Self::table()
            .filter(packaging_procedures::procedure_template.eq(procedure_template))
            .order_by(packaging_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_packaged_with_model(
        packaged_with_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::packaging_procedures::packaging_procedures;
        Self::table()
            .filter(packaging_procedures::packaged_with_model.eq(packaged_with_model))
            .order_by(packaging_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_and_procedure_template(
        procedure: &::rosetta_uuid::Uuid,
        procedure_template: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::packaging_procedures::packaging_procedures;
        Self::table()
            .filter(
                packaging_procedures::procedure
                    .eq(procedure)
                    .and(packaging_procedures::procedure_template.eq(procedure_template)),
            )
            .order_by(packaging_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_and_packaged_with_model(
        procedure: &::rosetta_uuid::Uuid,
        packaged_with_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::packaging_procedures::packaging_procedures;
        Self::table()
            .filter(
                packaging_procedures::procedure
                    .eq(procedure)
                    .and(packaging_procedures::packaged_with_model.eq(packaged_with_model)),
            )
            .order_by(packaging_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure(
        procedure_template: &i32,
        procedure: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl,
            SelectableHelper, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            packaging_procedures::packaging_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(packaging_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(
                procedures::procedure_template
                    .eq(procedure_template)
                    .and(procedures::procedure.eq(procedure)),
            )
            .order_by(packaging_procedures::procedure.asc())
            .select(Self::as_select())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_most_concrete_table(
        most_concrete_table: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            packaging_procedures::packaging_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(packaging_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::most_concrete_table.eq(most_concrete_table))
            .order_by(packaging_procedures::procedure.asc())
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
            packaging_procedures::packaging_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(packaging_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::created_by.eq(created_by))
            .order_by(packaging_procedures::procedure.asc())
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
            packaging_procedures::packaging_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(packaging_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::created_at.eq(created_at))
            .order_by(packaging_procedures::procedure.asc())
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
            packaging_procedures::packaging_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(packaging_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::updated_by.eq(updated_by))
            .order_by(packaging_procedures::procedure.asc())
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
            packaging_procedures::packaging_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(packaging_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::updated_at.eq(updated_at))
            .order_by(packaging_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<PackagingProcedure> for PackagingProcedure {
    fn as_ref(&self) -> &PackagingProcedure {
        self
    }
}
