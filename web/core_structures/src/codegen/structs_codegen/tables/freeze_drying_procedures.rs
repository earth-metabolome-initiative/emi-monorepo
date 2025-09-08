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
#[diesel(belongs_to(crate::VolumetricContainer, foreign_key = freeze_dried_container))]
#[diesel(
    belongs_to(
        crate::VolumetricContainerModel,
        foreign_key = freeze_dried_container_model
    )
)]
#[diesel(belongs_to(crate::FreezeDryer, foreign_key = freeze_dried_with))]
#[diesel(belongs_to(crate::FreezeDryerModel, foreign_key = freeze_dried_with_model))]
#[diesel(primary_key(procedure))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures
)]
pub struct FreezeDryingProcedure {
    pub procedure: ::rosetta_uuid::Uuid,
    pub procedure_template: i32,
    pub freeze_dried_container: ::rosetta_uuid::Uuid,
    pub freeze_dried_container_model: i32,
    pub procedure_template_freeze_dried_container_model: i32,
    pub procedure_freeze_dried_container: ::rosetta_uuid::Uuid,
    pub freeze_dried_with: Option<::rosetta_uuid::Uuid>,
    pub freeze_dried_with_model: i32,
    pub procedure_template_freeze_dried_with_model: i32,
    pub procedure_freeze_dried_with: ::rosetta_uuid::Uuid,
}
impl web_common_traits::prelude::TableName for FreezeDryingProcedure {
    const TABLE_NAME: &'static str = "freeze_drying_procedures";
}
impl web_common_traits::prelude::ExtensionTable<crate::Procedure> for FreezeDryingProcedure where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>
{
}
impl web_common_traits::prelude::ExtensionTable<crate::FreezeDryingProcedure>
    for FreezeDryingProcedure
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for FreezeDryingProcedure {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.procedure
    }
}
impl FreezeDryingProcedure {
    pub fn freeze_dried_container<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::VolumetricContainer, diesel::result::Error>
    where
        crate::VolumetricContainer: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::VolumetricContainer::read(self.freeze_dried_container, conn)
    }
    pub fn freeze_dried_container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::VolumetricContainerModel, diesel::result::Error>
    where
        crate::VolumetricContainerModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::VolumetricContainerModel::read(self.freeze_dried_container_model, conn)
    }
    pub fn freeze_dried_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<Option<crate::FreezeDryer>, diesel::result::Error>
    where
        crate::FreezeDryer: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        let Some(freeze_dried_with) = self.freeze_dried_with else {
            return Ok(None);
        };
        crate::FreezeDryer::read(freeze_dried_with, conn).map(Some)
    }
    #[cfg(feature = "postgres")]
    pub fn freeze_drying_procedures_freeze_dried_with_freeze_dried_wi_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<crate::Asset>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        let Some(freeze_dried_with) = self.freeze_dried_with else {
            return Ok(None);
        };
        crate::Asset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::assets::assets::dsl::id
                    .eq(freeze_dried_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::assets::assets::dsl::model
                            .eq(&self.freeze_dried_with_model),
                    ),
            )
            .first::<crate::Asset>(conn)
            .map(Some)
    }
    pub fn freeze_dried_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::FreezeDryerModel, diesel::result::Error>
    where
        crate::FreezeDryerModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::FreezeDryerModel::read(self.freeze_dried_with_model, conn)
    }
    pub fn freeze_drying_procedures_freeze_dried_with_model_freeze_dr_fkey<
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
            (self.freeze_dried_with_model, self.freeze_dried_container_model),
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
    #[cfg(feature = "postgres")]
    pub fn freeze_drying_procedures_procedure_freeze_dried_container_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_freeze_dried_container)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_freeze_dried_container_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    pub fn procedure_freeze_dried_container<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error>
    where
        crate::ProcedureAsset: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureAsset::read(self.procedure_freeze_dried_container, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn freeze_drying_procedures_procedure_freeze_dried_container_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_freeze_dried_container)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.freeze_dried_container_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn freeze_drying_procedures_procedure_freeze_dried_container_fkey2(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_freeze_dried_container)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.freeze_dried_container),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    pub fn procedure_freeze_dried_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error>
    where
        crate::ProcedureAsset: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureAsset::read(self.procedure_freeze_dried_with, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn freeze_drying_procedures_procedure_freeze_dried_with_free_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<crate::ProcedureAsset>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        let Some(freeze_dried_with) = self.freeze_dried_with else {
            return Ok(None);
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_freeze_dried_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(freeze_dried_with),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub fn freeze_drying_procedures_procedure_freeze_dried_with_freez_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_freeze_dried_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.freeze_dried_with_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn freeze_drying_procedures_procedure_freeze_dried_with_proce_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_freeze_dried_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_freeze_dried_with_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn freeze_drying_procedures_procedure_procedure_template_fkey(
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
    ) -> Result<crate::FreezeDryingProcedureTemplate, diesel::result::Error>
    where
        crate::FreezeDryingProcedureTemplate: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::FreezeDryingProcedureTemplate::read(self.procedure_template, conn)
    }
    pub fn procedure_template_freeze_dried_container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(
            self.procedure_template_freeze_dried_container_model,
            conn,
        )
    }
    pub fn procedure_template_freeze_dried_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(
            self.procedure_template_freeze_dried_with_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn freeze_drying_procedures_procedure_template_procedure_tem_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::FreezeDryingProcedureTemplate, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::FreezeDryingProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::freeze_drying_procedure_templates::freeze_drying_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::freeze_drying_procedure_templates::freeze_drying_procedure_templates::dsl::procedure_template_freeze_dried_container_model
                            .eq(&self.procedure_template_freeze_dried_container_model),
                    ),
            )
            .first::<crate::FreezeDryingProcedureTemplate>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn freeze_drying_procedures_procedure_template_procedure_temp_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::FreezeDryingProcedureTemplate, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::FreezeDryingProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::freeze_drying_procedure_templates::freeze_drying_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::freeze_drying_procedure_templates::freeze_drying_procedure_templates::dsl::procedure_template_freeze_dried_with_model
                            .eq(&self.procedure_template_freeze_dried_with_model),
                    ),
            )
            .first::<crate::FreezeDryingProcedureTemplate>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_freeze_dried_with_and_freeze_dried_with_model(
        freeze_dried_with: ::rosetta_uuid::Uuid,
        freeze_dried_with_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures;
        Self::table()
            .filter(
                freeze_drying_procedures::freeze_dried_with.eq(freeze_dried_with).and(
                    freeze_drying_procedures::freeze_dried_with_model.eq(freeze_dried_with_model),
                ),
            )
            .order_by(freeze_drying_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_freeze_dried_with_model_and_freeze_dried_container_model(
        freeze_dried_with_model: i32,
        freeze_dried_container_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures;
        Self::table()
            .filter(
                freeze_drying_procedures::freeze_dried_with_model.eq(freeze_dried_with_model).and(
                    freeze_drying_procedures::freeze_dried_container_model
                        .eq(freeze_dried_container_model),
                ),
            )
            .order_by(freeze_drying_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure<C>(
        procedure: ::rosetta_uuid::Uuid,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures;
        Self::table()
            .filter(freeze_drying_procedures::procedure.eq(procedure))
            .order_by(freeze_drying_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_freeze_dried_container_and_procedure_template_freeze_dried_container_model(
        procedure_freeze_dried_container: ::rosetta_uuid::Uuid,
        procedure_template_freeze_dried_container_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures;
        Self::table()
            .filter(
                freeze_drying_procedures::procedure_freeze_dried_container
                    .eq(procedure_freeze_dried_container)
                    .and(
                        freeze_drying_procedures::procedure_template_freeze_dried_container_model
                            .eq(procedure_template_freeze_dried_container_model),
                    ),
            )
            .order_by(freeze_drying_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_freeze_dried_container<C>(
        procedure_freeze_dried_container: ::rosetta_uuid::Uuid,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure_freeze_dried_container as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure_freeze_dried_container as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure_freeze_dried_container as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures;
        Self::table()
            .filter(
                freeze_drying_procedures::procedure_freeze_dried_container
                    .eq(procedure_freeze_dried_container),
            )
            .order_by(freeze_drying_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_freeze_dried_container_and_freeze_dried_container_model(
        procedure_freeze_dried_container: ::rosetta_uuid::Uuid,
        freeze_dried_container_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures;
        Self::table()
            .filter(
                freeze_drying_procedures::procedure_freeze_dried_container
                    .eq(procedure_freeze_dried_container)
                    .and(
                        freeze_drying_procedures::freeze_dried_container_model
                            .eq(freeze_dried_container_model),
                    ),
            )
            .order_by(freeze_drying_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_freeze_dried_container_and_freeze_dried_container(
        procedure_freeze_dried_container: ::rosetta_uuid::Uuid,
        freeze_dried_container: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures;
        Self::table()
            .filter(
                freeze_drying_procedures::procedure_freeze_dried_container
                    .eq(procedure_freeze_dried_container)
                    .and(
                        freeze_drying_procedures::freeze_dried_container.eq(freeze_dried_container),
                    ),
            )
            .order_by(freeze_drying_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_freeze_dried_with<C>(
        procedure_freeze_dried_with: ::rosetta_uuid::Uuid,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure_freeze_dried_with as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure_freeze_dried_with as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure_freeze_dried_with as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures;
        Self::table()
            .filter(
                freeze_drying_procedures::procedure_freeze_dried_with
                    .eq(procedure_freeze_dried_with),
            )
            .order_by(freeze_drying_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_freeze_dried_with_and_freeze_dried_with(
        procedure_freeze_dried_with: ::rosetta_uuid::Uuid,
        freeze_dried_with: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures;
        Self::table()
            .filter(
                freeze_drying_procedures::procedure_freeze_dried_with
                    .eq(procedure_freeze_dried_with)
                    .and(freeze_drying_procedures::freeze_dried_with.eq(freeze_dried_with)),
            )
            .order_by(freeze_drying_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_freeze_dried_with_and_freeze_dried_with_model(
        procedure_freeze_dried_with: ::rosetta_uuid::Uuid,
        freeze_dried_with_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures;
        Self::table()
            .filter(
                freeze_drying_procedures::procedure_freeze_dried_with
                    .eq(procedure_freeze_dried_with)
                    .and(
                        freeze_drying_procedures::freeze_dried_with_model
                            .eq(freeze_dried_with_model),
                    ),
            )
            .order_by(freeze_drying_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_freeze_dried_with_and_procedure_template_freeze_dried_with_model(
        procedure_freeze_dried_with: ::rosetta_uuid::Uuid,
        procedure_template_freeze_dried_with_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures;
        Self::table()
            .filter(
                freeze_drying_procedures::procedure_freeze_dried_with
                    .eq(procedure_freeze_dried_with)
                    .and(
                        freeze_drying_procedures::procedure_template_freeze_dried_with_model
                            .eq(procedure_template_freeze_dried_with_model),
                    ),
            )
            .order_by(freeze_drying_procedures::procedure.asc())
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

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures;
        Self::table()
            .filter(
                freeze_drying_procedures::procedure
                    .eq(procedure)
                    .and(freeze_drying_procedures::procedure_template.eq(procedure_template)),
            )
            .order_by(freeze_drying_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template<C>(
        procedure_template: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures;
        Self::table()
            .filter(freeze_drying_procedures::procedure_template.eq(procedure_template))
            .order_by(freeze_drying_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_freeze_dried_container_model<C>(
        procedure_template_freeze_dried_container_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure_template_freeze_dried_container_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure_template_freeze_dried_container_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure_template_freeze_dried_container_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures;
        Self::table()
            .filter(
                freeze_drying_procedures::procedure_template_freeze_dried_container_model
                    .eq(procedure_template_freeze_dried_container_model),
            )
            .order_by(freeze_drying_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_freeze_dried_with_model<C>(
        procedure_template_freeze_dried_with_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure_template_freeze_dried_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure_template_freeze_dried_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure_template_freeze_dried_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures;
        Self::table()
            .filter(
                freeze_drying_procedures::procedure_template_freeze_dried_with_model
                    .eq(procedure_template_freeze_dried_with_model),
            )
            .order_by(freeze_drying_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_freeze_dried_container_model(
        procedure_template: i32,
        procedure_template_freeze_dried_container_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures;
        Self::table()
            .filter(
                freeze_drying_procedures::procedure_template.eq(procedure_template).and(
                    freeze_drying_procedures::procedure_template_freeze_dried_container_model
                        .eq(procedure_template_freeze_dried_container_model),
                ),
            )
            .order_by(freeze_drying_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_freeze_dried_with_model(
        procedure_template: i32,
        procedure_template_freeze_dried_with_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures;
        Self::table()
            .filter(
                freeze_drying_procedures::procedure_template.eq(procedure_template).and(
                    freeze_drying_procedures::procedure_template_freeze_dried_with_model
                        .eq(procedure_template_freeze_dried_with_model),
                ),
            )
            .order_by(freeze_drying_procedures::procedure.asc())
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
            freeze_drying_procedures::freeze_drying_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(freeze_drying_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(
                procedures::procedure_template
                    .eq(procedure_template)
                    .and(procedures::procedure.eq(procedure)),
            )
            .order_by(freeze_drying_procedures::procedure.asc())
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
            freeze_drying_procedures::freeze_drying_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(freeze_drying_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::parent_procedure.eq(parent_procedure))
            .order_by(freeze_drying_procedures::procedure.asc())
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
            freeze_drying_procedures::freeze_drying_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(freeze_drying_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::parent_procedure_template.eq(parent_procedure_template))
            .order_by(freeze_drying_procedures::procedure.asc())
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
            freeze_drying_procedures::freeze_drying_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(freeze_drying_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::most_concrete_table.eq(most_concrete_table))
            .order_by(freeze_drying_procedures::procedure.asc())
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
            freeze_drying_procedures::freeze_drying_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(freeze_drying_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::created_by.eq(created_by))
            .order_by(freeze_drying_procedures::procedure.asc())
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
            freeze_drying_procedures::freeze_drying_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(freeze_drying_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::created_at.eq(created_at))
            .order_by(freeze_drying_procedures::procedure.asc())
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
            freeze_drying_procedures::freeze_drying_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(freeze_drying_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::updated_by.eq(updated_by))
            .order_by(freeze_drying_procedures::procedure.asc())
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
            freeze_drying_procedures::freeze_drying_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(freeze_drying_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::updated_at.eq(updated_at))
            .order_by(freeze_drying_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<FreezeDryingProcedure> for FreezeDryingProcedure {
    fn as_ref(&self) -> &FreezeDryingProcedure {
        self
    }
}
