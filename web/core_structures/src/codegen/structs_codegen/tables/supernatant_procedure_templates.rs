#[derive(Debug, Clone, PartialEq, Copy, PartialOrd)]
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
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
        foreign_key = pipette_tip_model
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::pipette_models::PipetteModel,
        foreign_key = transferred_with_model
    )
)]
#[diesel(primary_key(procedure_template))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates
)]
pub struct SupernatantProcedureTemplate {
    pub procedure_template: i32,
    pub liters: f32,
    pub stratified_source_model: i32,
    pub procedure_template_stratified_source_model: i32,
    pub supernatant_destination_model: i32,
    pub procedure_template_supernatant_destination_model: i32,
    pub transferred_with_model: i32,
    pub procedure_template_transferred_with_model: i32,
    pub pipette_tip_model: i32,
    pub procedure_template_pipette_tip_model: i32,
}
impl web_common_traits::prelude::TableName for SupernatantProcedureTemplate {
    const TABLE_NAME: &'static str = "supernatant_procedure_templates";
}
impl<'a> From<&'a SupernatantProcedureTemplate>
for web_common_traits::database::IdOrBuilder<
    i32,
    crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureTemplateBuilder,
> {
    fn from(value: &'a SupernatantProcedureTemplate) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.procedure_template)
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    > for SupernatantProcedureTemplate
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl web_common_traits::prelude::ExtensionTable<
    crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
> for SupernatantProcedureTemplate
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{}
impl diesel::Identifiable for SupernatantProcedureTemplate {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.procedure_template
    }
}
impl web_common_traits::database::PrimaryKeyLike for SupernatantProcedureTemplate {
    type PrimaryKey = i32;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.procedure_template
    }
}
impl SupernatantProcedureTemplate {
    pub fn supernatant_pm_compatibility_rules<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule::read(
            (self.transferred_with_model, self.pipette_tip_model),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn supernatant_procedure_templa_procedure_template_pipette_t_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(&self.procedure_template_pipette_tip_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.pipette_tip_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn supernatant_procedure_templa_procedure_template_stratifie_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(&self.procedure_template_stratified_source_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.stratified_source_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn supernatant_procedure_templa_procedure_template_supernata_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(&self.procedure_template_supernatant_destination_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.supernatant_destination_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn supernatant_procedure_templa_procedure_template_transferr_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(&self.procedure_template_transferred_with_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.transferred_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    pub fn procedure_template_pipette_tip_model<C: diesel::connection::LoadConnection>(
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
            self.procedure_template_pipette_tip_model,
            conn,
        )
    }
    pub fn procedure_template_stratified_source_model<
        C: diesel::connection::LoadConnection,
    >(
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
            self.procedure_template_stratified_source_model,
            conn,
        )
    }
    pub fn procedure_template_supernatant_destination_model<
        C: diesel::connection::LoadConnection,
    >(
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
            self.procedure_template_supernatant_destination_model,
            conn,
        )
    }
    pub fn procedure_template_transferred_with_model<
        C: diesel::connection::LoadConnection,
    >(
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
            self.procedure_template_transferred_with_model,
            conn,
        )
    }
    pub fn supernatant_destination_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel::read(
            self.supernatant_destination_model,
            conn,
        )
    }
    pub fn pipette_tip_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel::read(
            self.pipette_tip_model,
            conn,
        )
    }
    pub fn stratified_source_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel::read(
            self.stratified_source_model,
            conn,
        )
    }
    pub fn transferred_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::pipette_models::PipetteModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::pipette_models::PipetteModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::pipette_models::PipetteModel::read(
            self.transferred_with_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_transferred_with_model_and_pipette_tip_model(
        transferred_with_model: i32,
        pipette_tip_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates;
        Self::table()
            .filter(
                supernatant_procedure_templates::transferred_with_model
                    .eq(transferred_with_model)
                    .and(supernatant_procedure_templates::pipette_tip_model.eq(pipette_tip_model)),
            )
            .order_by(supernatant_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_pipette_tip_model_and_pipette_tip_model(
        procedure_template_pipette_tip_model: i32,
        pipette_tip_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates;
        Self::table()
            .filter(
                supernatant_procedure_templates::procedure_template_pipette_tip_model
                    .eq(procedure_template_pipette_tip_model)
                    .and(supernatant_procedure_templates::pipette_tip_model.eq(pipette_tip_model)),
            )
            .order_by(supernatant_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_stratified_source_model_and_stratified_source_model(
        procedure_template_stratified_source_model: i32,
        stratified_source_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates;
        Self::table()
            .filter(
                supernatant_procedure_templates::procedure_template_stratified_source_model
                    .eq(procedure_template_stratified_source_model)
                    .and(
                        supernatant_procedure_templates::stratified_source_model
                            .eq(stratified_source_model),
                    ),
            )
            .order_by(supernatant_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_supernatant_destination_model_and_supernatant_destination_model(
        procedure_template_supernatant_destination_model: i32,
        supernatant_destination_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates;
        Self::table()
            .filter(
                supernatant_procedure_templates::procedure_template_supernatant_destination_model
                    .eq(procedure_template_supernatant_destination_model)
                    .and(
                        supernatant_procedure_templates::supernatant_destination_model
                            .eq(supernatant_destination_model),
                    ),
            )
            .order_by(supernatant_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_transferred_with_model_and_transferred_with_model(
        procedure_template_transferred_with_model: i32,
        transferred_with_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates;
        Self::table()
            .filter(
                supernatant_procedure_templates::procedure_template_transferred_with_model
                    .eq(procedure_template_transferred_with_model)
                    .and(
                        supernatant_procedure_templates::transferred_with_model
                            .eq(transferred_with_model),
                    ),
            )
            .order_by(supernatant_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_pipette_tip_model<C>(
        procedure_template_pipette_tip_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template_pipette_tip_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template_pipette_tip_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template_pipette_tip_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates;
        Self::table()
            .filter(
                supernatant_procedure_templates::procedure_template_pipette_tip_model
                    .eq(procedure_template_pipette_tip_model),
            )
            .order_by(supernatant_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_stratified_source_model<C>(
        procedure_template_stratified_source_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template_stratified_source_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template_stratified_source_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template_stratified_source_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates;
        Self::table()
            .filter(
                supernatant_procedure_templates::procedure_template_stratified_source_model
                    .eq(procedure_template_stratified_source_model),
            )
            .order_by(supernatant_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_supernatant_destination_model<C>(
        procedure_template_supernatant_destination_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template_supernatant_destination_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template_supernatant_destination_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template_supernatant_destination_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates;
        Self::table()
            .filter(
                supernatant_procedure_templates::procedure_template_supernatant_destination_model
                    .eq(procedure_template_supernatant_destination_model),
            )
            .order_by(supernatant_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_transferred_with_model<C>(
        procedure_template_transferred_with_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template_transferred_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template_transferred_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template_transferred_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates;
        Self::table()
            .filter(
                supernatant_procedure_templates::procedure_template_transferred_with_model
                    .eq(procedure_template_transferred_with_model),
            )
            .order_by(supernatant_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    pub fn from_supernatant_destination_model<C>(
        supernatant_destination_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::supernatant_destination_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::supernatant_destination_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::supernatant_destination_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates;
        Self::table()
            .filter(
                supernatant_procedure_templates::supernatant_destination_model
                    .eq(supernatant_destination_model),
            )
            .order_by(supernatant_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    pub fn from_stratified_source_model<C>(
        stratified_source_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::stratified_source_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::stratified_source_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::stratified_source_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::procedure_template,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates;
        Self::table()
            .filter(
                supernatant_procedure_templates::stratified_source_model
                    .eq(stratified_source_model),
            )
            .order_by(supernatant_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            procedure_templates::procedure_templates,
            supernatant_procedure_templates::supernatant_procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(supernatant_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::name.eq(name))
            .order_by(supernatant_procedure_templates::procedure_template.asc())
            .select(Self::as_select())
            .first::<Self>(conn)
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
            procedure_templates::procedure_templates,
            supernatant_procedure_templates::supernatant_procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(supernatant_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::description.eq(description))
            .order_by(supernatant_procedure_templates::procedure_template.asc())
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
            procedure_templates::procedure_templates,
            supernatant_procedure_templates::supernatant_procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(supernatant_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::created_by.eq(created_by))
            .order_by(supernatant_procedure_templates::procedure_template.asc())
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
            procedure_templates::procedure_templates,
            supernatant_procedure_templates::supernatant_procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(supernatant_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::updated_by.eq(updated_by))
            .order_by(supernatant_procedure_templates::procedure_template.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_deprecated(
        deprecated: bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            procedure_templates::procedure_templates,
            supernatant_procedure_templates::supernatant_procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(supernatant_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::deprecated.eq(deprecated))
            .order_by(supernatant_procedure_templates::procedure_template.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<SupernatantProcedureTemplate> for SupernatantProcedureTemplate {
    fn as_ref(&self) -> &SupernatantProcedureTemplate {
        self
    }
}
