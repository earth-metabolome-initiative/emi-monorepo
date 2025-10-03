#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Associations,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        foreign_key = procedure_template_ppe_asset_model
    )
)]
#[diesel(primary_key(procedure))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::ppe_reminder_procedures::ppe_reminder_procedures
)]
pub struct PpeReminderProcedure {
    pub procedure: ::rosetta_uuid::Uuid,
    pub procedure_template: i32,
    pub procedure_template_ppe_asset_model: i32,
    pub procedure_ppe_asset: ::rosetta_uuid::Uuid,
}
impl web_common_traits::prelude::TableName for PpeReminderProcedure {
    const TABLE_NAME: &'static str = "ppe_reminder_procedures";
}
impl<'a> From<&'a PpeReminderProcedure>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertablePpeReminderProcedureBuilder,
    >
{
    fn from(value: &'a PpeReminderProcedure) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.procedure)
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    > for PpeReminderProcedure
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::ppe_reminder_procedures::PpeReminderProcedure,
    > for PpeReminderProcedure
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for PpeReminderProcedure {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.procedure
    }
}
impl web_common_traits::database::PrimaryKeyLike for PpeReminderProcedure {
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.procedure
    }
}
impl PpeReminderProcedure {
    pub fn procedure_ppe_asset<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
            self.procedure_ppe_asset,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn ppe_reminder_procedures_procedure_ppe_asset_procedure_temp_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_ppe_asset)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_ppe_asset_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::ppe_reminder_procedure_templates::PpeReminderProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::ppe_reminder_procedure_templates::PpeReminderProcedureTemplate: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::ppe_reminder_procedure_templates::PpeReminderProcedureTemplate::read(
            self.procedure_template,
            conn,
        )
    }
    pub fn procedure_template_ppe_asset_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
            self.procedure_template_ppe_asset_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn ppe_reminder_procedures_procedure_template_procedure_templ_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::ppe_reminder_procedure_templates::PpeReminderProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::ppe_reminder_procedure_templates::PpeReminderProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::ppe_reminder_procedure_templates::ppe_reminder_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::ppe_reminder_procedure_templates::ppe_reminder_procedure_templates::dsl::procedure_template_ppe_asset_model
                            .eq(&self.procedure_template_ppe_asset_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::ppe_reminder_procedure_templates::PpeReminderProcedureTemplate,
            >(conn)
    }
    pub fn from_procedure_ppe_asset<C>(
        procedure_ppe_asset: ::rosetta_uuid::Uuid,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::ppe_reminder_procedures::ppe_reminder_procedures::procedure_ppe_asset as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::ppe_reminder_procedures::ppe_reminder_procedures::procedure_ppe_asset as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::ppe_reminder_procedures::ppe_reminder_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::ppe_reminder_procedures::ppe_reminder_procedures::procedure_ppe_asset as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::ppe_reminder_procedures::ppe_reminder_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ppe_reminder_procedures::ppe_reminder_procedures;
        Self::table()
            .filter(ppe_reminder_procedures::procedure_ppe_asset.eq(procedure_ppe_asset))
            .order_by(ppe_reminder_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_ppe_asset_and_procedure_template_ppe_asset_model(
        procedure_ppe_asset: ::rosetta_uuid::Uuid,
        procedure_template_ppe_asset_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ppe_reminder_procedures::ppe_reminder_procedures;
        Self::table()
            .filter(
                ppe_reminder_procedures::procedure_ppe_asset.eq(procedure_ppe_asset).and(
                    ppe_reminder_procedures::procedure_template_ppe_asset_model
                        .eq(procedure_template_ppe_asset_model),
                ),
            )
            .order_by(ppe_reminder_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template<C>(
        procedure_template: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::ppe_reminder_procedures::ppe_reminder_procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::ppe_reminder_procedures::ppe_reminder_procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::ppe_reminder_procedures::ppe_reminder_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::ppe_reminder_procedures::ppe_reminder_procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::ppe_reminder_procedures::ppe_reminder_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ppe_reminder_procedures::ppe_reminder_procedures;
        Self::table()
            .filter(ppe_reminder_procedures::procedure_template.eq(procedure_template))
            .order_by(ppe_reminder_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_ppe_asset_model(
        procedure_template: i32,
        procedure_template_ppe_asset_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ppe_reminder_procedures::ppe_reminder_procedures;
        Self::table()
            .filter(
                ppe_reminder_procedures::procedure_template.eq(procedure_template).and(
                    ppe_reminder_procedures::procedure_template_ppe_asset_model
                        .eq(procedure_template_ppe_asset_model),
                ),
            )
            .order_by(ppe_reminder_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_procedure(
        parent_procedure: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            ppe_reminder_procedures::ppe_reminder_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(ppe_reminder_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::parent_procedure.eq(parent_procedure))
            .order_by(ppe_reminder_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_procedure_template(
        parent_procedure_template: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            ppe_reminder_procedures::ppe_reminder_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(ppe_reminder_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::parent_procedure_template.eq(parent_procedure_template))
            .order_by(ppe_reminder_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_predecessor_procedure(
        predecessor_procedure: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            ppe_reminder_procedures::ppe_reminder_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(ppe_reminder_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::predecessor_procedure.eq(predecessor_procedure))
            .order_by(ppe_reminder_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_predecessor_procedure_template(
        predecessor_procedure_template: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            ppe_reminder_procedures::ppe_reminder_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(ppe_reminder_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::predecessor_procedure_template.eq(predecessor_procedure_template))
            .order_by(ppe_reminder_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_by(
        created_by: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            ppe_reminder_procedures::ppe_reminder_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(ppe_reminder_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::created_by.eq(created_by))
            .order_by(ppe_reminder_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_updated_by(
        updated_by: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            ppe_reminder_procedures::ppe_reminder_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(ppe_reminder_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::updated_by.eq(updated_by))
            .order_by(ppe_reminder_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<PpeReminderProcedure> for PpeReminderProcedure {
    fn as_ref(&self) -> &PpeReminderProcedure {
        self
    }
}
