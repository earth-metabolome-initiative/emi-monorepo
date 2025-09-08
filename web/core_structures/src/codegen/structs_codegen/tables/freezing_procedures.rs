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
#[diesel(belongs_to(crate::VolumetricContainer, foreign_key = frozen_container))]
#[diesel(
    belongs_to(crate::VolumetricContainerModel, foreign_key = frozen_container_model)
)]
#[diesel(belongs_to(crate::Freezer, foreign_key = frozen_with))]
#[diesel(belongs_to(crate::FreezerModel, foreign_key = frozen_with_model))]
#[diesel(primary_key(procedure))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures
)]
pub struct FreezingProcedure {
    pub procedure: ::rosetta_uuid::Uuid,
    pub procedure_template: i32,
    pub frozen_container: ::rosetta_uuid::Uuid,
    pub frozen_container_model: i32,
    pub procedure_template_frozen_container_model: i32,
    pub procedure_frozen_container: ::rosetta_uuid::Uuid,
    pub frozen_with: Option<::rosetta_uuid::Uuid>,
    pub frozen_with_model: i32,
    pub procedure_template_frozen_with_model: i32,
    pub procedure_frozen_with: ::rosetta_uuid::Uuid,
}
impl web_common_traits::prelude::TableName for FreezingProcedure {
    const TABLE_NAME: &'static str = "freezing_procedures";
}
impl<'a> From<&'a FreezingProcedure>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureBuilder,
    >
{
    fn from(value: &'a FreezingProcedure) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.procedure)
    }
}
impl web_common_traits::prelude::ExtensionTable<crate::Procedure> for FreezingProcedure where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>
{
}
impl web_common_traits::prelude::ExtensionTable<crate::FreezingProcedure> for FreezingProcedure where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>
{
}
impl diesel::Identifiable for FreezingProcedure {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.procedure
    }
}
impl FreezingProcedure {
    pub fn frozen_container<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::VolumetricContainer, diesel::result::Error>
    where
        crate::VolumetricContainer: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::VolumetricContainer::read(self.frozen_container, conn)
    }
    pub fn frozen_container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::VolumetricContainerModel, diesel::result::Error>
    where
        crate::VolumetricContainerModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::VolumetricContainerModel::read(self.frozen_container_model, conn)
    }
    pub fn frozen_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<Option<crate::Freezer>, diesel::result::Error>
    where
        crate::Freezer: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        let Some(frozen_with) = self.frozen_with else {
            return Ok(None);
        };
        crate::Freezer::read(frozen_with, conn).map(Some)
    }
    pub fn frozen_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::FreezerModel, diesel::result::Error>
    where
        crate::FreezerModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::FreezerModel::read(self.frozen_with_model, conn)
    }
    pub fn freezing_procedures_frozen_with_model_frozen_container_mod_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<crate::AssetCompatibilityRule, diesel::result::Error>
    where
        crate::AssetCompatibilityRule: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::AssetCompatibilityRule::read(
            (self.frozen_with_model, self.frozen_container_model),
            conn,
        )
    }
    pub fn procedure<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::Procedure, diesel::result::Error>
    where
        crate::Procedure: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::Procedure::read(self.procedure, conn)
    }
    pub fn procedure_frozen_container<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error>
    where
        crate::ProcedureAsset: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureAsset::read(self.procedure_frozen_container, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn freezing_procedures_procedure_frozen_container_frozen_con_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_frozen_container)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.frozen_container),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn freezing_procedures_procedure_frozen_container_frozen_cont_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_frozen_container)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.frozen_container_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn freezing_procedures_procedure_frozen_container_procedure_t_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_frozen_container)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_frozen_container_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    pub fn procedure_frozen_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error>
    where
        crate::ProcedureAsset: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureAsset::read(self.procedure_frozen_with, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn freezing_procedures_procedure_frozen_with_frozen_with_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<crate::ProcedureAsset>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        let Some(frozen_with) = self.frozen_with else {
            return Ok(None);
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_frozen_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(frozen_with),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub fn freezing_procedures_procedure_frozen_with_frozen_with_mode_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_frozen_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.frozen_with_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn freezing_procedures_procedure_frozen_with_procedure_templa_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_frozen_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_frozen_with_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn freezing_procedures_procedure_procedure_template_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::Procedure, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::Procedure::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure
                    .eq(&self.procedure)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure_template
                            .eq(&self.procedure_template),
                    ),
            )
            .first::<crate::Procedure>(conn)
    }
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::FreezingProcedureTemplate, diesel::result::Error>
    where
        crate::FreezingProcedureTemplate: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::FreezingProcedureTemplate::read(self.procedure_template, conn)
    }
    pub fn procedure_template_frozen_container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(
            self.procedure_template_frozen_container_model,
            conn,
        )
    }
    pub fn procedure_template_frozen_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(self.procedure_template_frozen_with_model, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn freezing_procedures_procedure_template_procedure_template_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::FreezingProcedureTemplate, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::FreezingProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::freezing_procedure_templates::freezing_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::freezing_procedure_templates::freezing_procedure_templates::dsl::procedure_template_frozen_with_model
                            .eq(&self.procedure_template_frozen_with_model),
                    ),
            )
            .first::<crate::FreezingProcedureTemplate>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn freezing_procedures_procedure_template_procedure_template_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::FreezingProcedureTemplate, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::FreezingProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::freezing_procedure_templates::freezing_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::freezing_procedure_templates::freezing_procedure_templates::dsl::procedure_template_frozen_container_model
                            .eq(&self.procedure_template_frozen_container_model),
                    ),
            )
            .first::<crate::FreezingProcedureTemplate>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_frozen_with_model_and_frozen_container_model(
        frozen_with_model: i32,
        frozen_container_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures;
        Self::table()
            .filter(
                freezing_procedures::frozen_with_model
                    .eq(frozen_with_model)
                    .and(freezing_procedures::frozen_container_model.eq(frozen_container_model)),
            )
            .order_by(freezing_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure<C>(
        procedure: ::rosetta_uuid::Uuid,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures;
        Self::table()
            .filter(freezing_procedures::procedure.eq(procedure))
            .order_by(freezing_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_frozen_container<C>(
        procedure_frozen_container: ::rosetta_uuid::Uuid,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure_frozen_container as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure_frozen_container as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure_frozen_container as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures;
        Self::table()
            .filter(freezing_procedures::procedure_frozen_container.eq(procedure_frozen_container))
            .order_by(freezing_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_frozen_container_and_frozen_container(
        procedure_frozen_container: ::rosetta_uuid::Uuid,
        frozen_container: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures;
        Self::table()
            .filter(
                freezing_procedures::procedure_frozen_container
                    .eq(procedure_frozen_container)
                    .and(freezing_procedures::frozen_container.eq(frozen_container)),
            )
            .order_by(freezing_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_frozen_container_and_frozen_container_model(
        procedure_frozen_container: ::rosetta_uuid::Uuid,
        frozen_container_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures;
        Self::table()
            .filter(
                freezing_procedures::procedure_frozen_container
                    .eq(procedure_frozen_container)
                    .and(freezing_procedures::frozen_container_model.eq(frozen_container_model)),
            )
            .order_by(freezing_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_frozen_container_and_procedure_template_frozen_container_model(
        procedure_frozen_container: ::rosetta_uuid::Uuid,
        procedure_template_frozen_container_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures;
        Self::table()
            .filter(
                freezing_procedures::procedure_frozen_container.eq(procedure_frozen_container).and(
                    freezing_procedures::procedure_template_frozen_container_model
                        .eq(procedure_template_frozen_container_model),
                ),
            )
            .order_by(freezing_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_frozen_with<C>(
        procedure_frozen_with: ::rosetta_uuid::Uuid,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure_frozen_with as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure_frozen_with as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure_frozen_with as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures;
        Self::table()
            .filter(freezing_procedures::procedure_frozen_with.eq(procedure_frozen_with))
            .order_by(freezing_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_frozen_with_and_frozen_with(
        procedure_frozen_with: ::rosetta_uuid::Uuid,
        frozen_with: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures;
        Self::table()
            .filter(
                freezing_procedures::procedure_frozen_with
                    .eq(procedure_frozen_with)
                    .and(freezing_procedures::frozen_with.eq(frozen_with)),
            )
            .order_by(freezing_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_frozen_with_and_frozen_with_model(
        procedure_frozen_with: ::rosetta_uuid::Uuid,
        frozen_with_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures;
        Self::table()
            .filter(
                freezing_procedures::procedure_frozen_with
                    .eq(procedure_frozen_with)
                    .and(freezing_procedures::frozen_with_model.eq(frozen_with_model)),
            )
            .order_by(freezing_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_frozen_with_and_procedure_template_frozen_with_model(
        procedure_frozen_with: ::rosetta_uuid::Uuid,
        procedure_template_frozen_with_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures;
        Self::table()
            .filter(
                freezing_procedures::procedure_frozen_with.eq(procedure_frozen_with).and(
                    freezing_procedures::procedure_template_frozen_with_model
                        .eq(procedure_template_frozen_with_model),
                ),
            )
            .order_by(freezing_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_and_procedure_template(
        procedure: ::rosetta_uuid::Uuid,
        procedure_template: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures;
        Self::table()
            .filter(
                freezing_procedures::procedure
                    .eq(procedure)
                    .and(freezing_procedures::procedure_template.eq(procedure_template)),
            )
            .order_by(freezing_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template<C>(
        procedure_template: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures;
        Self::table()
            .filter(freezing_procedures::procedure_template.eq(procedure_template))
            .order_by(freezing_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_frozen_container_model<C>(
        procedure_template_frozen_container_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure_template_frozen_container_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure_template_frozen_container_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure_template_frozen_container_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures;
        Self::table()
            .filter(
                freezing_procedures::procedure_template_frozen_container_model
                    .eq(procedure_template_frozen_container_model),
            )
            .order_by(freezing_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_frozen_with_model<C>(
        procedure_template_frozen_with_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure_template_frozen_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure_template_frozen_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure_template_frozen_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures;
        Self::table()
            .filter(
                freezing_procedures::procedure_template_frozen_with_model
                    .eq(procedure_template_frozen_with_model),
            )
            .order_by(freezing_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_frozen_with_model(
        procedure_template: i32,
        procedure_template_frozen_with_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures;
        Self::table()
            .filter(
                freezing_procedures::procedure_template.eq(procedure_template).and(
                    freezing_procedures::procedure_template_frozen_with_model
                        .eq(procedure_template_frozen_with_model),
                ),
            )
            .order_by(freezing_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_frozen_container_model(
        procedure_template: i32,
        procedure_template_frozen_container_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures;
        Self::table()
            .filter(
                freezing_procedures::procedure_template.eq(procedure_template).and(
                    freezing_procedures::procedure_template_frozen_container_model
                        .eq(procedure_template_frozen_container_model),
                ),
            )
            .order_by(freezing_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure(
        procedure_template: i32,
        procedure: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl,
            SelectableHelper, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            freezing_procedures::freezing_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(freezing_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(
                procedures::procedure_template
                    .eq(procedure_template)
                    .and(procedures::procedure.eq(procedure)),
            )
            .order_by(freezing_procedures::procedure.asc())
            .select(Self::as_select())
            .first::<Self>(conn)
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
            freezing_procedures::freezing_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(freezing_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::parent_procedure.eq(parent_procedure))
            .order_by(freezing_procedures::procedure.asc())
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
            freezing_procedures::freezing_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(freezing_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::parent_procedure_template.eq(parent_procedure_template))
            .order_by(freezing_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
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
            freezing_procedures::freezing_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(freezing_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::most_concrete_table.eq(most_concrete_table))
            .order_by(freezing_procedures::procedure.asc())
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
            freezing_procedures::freezing_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(freezing_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::created_by.eq(created_by))
            .order_by(freezing_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_at(
        created_at: ::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            freezing_procedures::freezing_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(freezing_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::created_at.eq(created_at))
            .order_by(freezing_procedures::procedure.asc())
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
            freezing_procedures::freezing_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(freezing_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::updated_by.eq(updated_by))
            .order_by(freezing_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_updated_at(
        updated_at: ::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            freezing_procedures::freezing_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(freezing_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::updated_at.eq(updated_at))
            .order_by(freezing_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<FreezingProcedure> for FreezingProcedure {
    fn as_ref(&self) -> &FreezingProcedure {
        self
    }
}
