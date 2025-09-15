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
        crate::codegen::structs_codegen::tables::pipette_models::PipetteModel,
        foreign_key = aliquoted_with_model
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
        foreign_key = pipette_tip_model
    )
)]
#[diesel(primary_key(procedure_template))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates
)]
pub struct AliquotingProcedureTemplate {
    pub procedure_template: i32,
    pub liters: f32,
    pub aliquoted_from_model: i32,
    pub procedure_template_aliquoted_from_model: i32,
    pub aliquoted_into_model: i32,
    pub procedure_template_aliquoted_into_model: i32,
    pub aliquoted_with_model: i32,
    pub procedure_template_aliquoted_with_model: i32,
    pub pipette_tip_model: i32,
    pub procedure_template_pipette_tip_model: i32,
}
impl web_common_traits::prelude::TableName for AliquotingProcedureTemplate {
    const TABLE_NAME: &'static str = "aliquoting_procedure_templates";
}
impl<'a> From<&'a AliquotingProcedureTemplate>
for web_common_traits::database::IdOrBuilder<
    i32,
    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureTemplateBuilder,
> {
    fn from(value: &'a AliquotingProcedureTemplate) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.procedure_template)
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    > for AliquotingProcedureTemplate
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl web_common_traits::prelude::ExtensionTable<
    crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
> for AliquotingProcedureTemplate
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{}
impl diesel::Identifiable for AliquotingProcedureTemplate {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.procedure_template
    }
}
impl web_common_traits::database::PrimaryKeyLike for AliquotingProcedureTemplate {
    type PrimaryKey = i32;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.procedure_template
    }
}
impl AliquotingProcedureTemplate {
    pub fn procedure_template_aliquoted_into_model<
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
            self.procedure_template_aliquoted_into_model,
            conn,
        )
    }
    pub fn procedure_template_aliquoted_with_model<
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
            self.procedure_template_aliquoted_with_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedure_templat_procedure_template_aliquoted_fkey3(
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
                    .eq(&self.procedure_template_aliquoted_with_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.aliquoted_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedure_templat_procedure_template_aliquoted_fkey4(
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
                    .eq(&self.procedure_template_aliquoted_from_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.aliquoted_from_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedure_templat_procedure_template_aliquoted_fkey5(
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
                    .eq(&self.procedure_template_aliquoted_into_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.aliquoted_into_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedure_templat_procedure_template_pipette_t_fkey1(
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
    pub fn aliquoting_procedure_template_aliquoted_with_model_pipette_fkey<
        C: diesel::connection::LoadConnection,
    >(
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
            (self.aliquoted_with_model, self.pipette_tip_model),
            conn,
        )
    }
    pub fn procedure_template_aliquoted_from_model<
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
            self.procedure_template_aliquoted_from_model,
            conn,
        )
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
    pub fn aliquoted_from_model<C: diesel::connection::LoadConnection>(
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
            self.aliquoted_from_model,
            conn,
        )
    }
    pub fn aliquoted_into_model<C: diesel::connection::LoadConnection>(
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
            self.aliquoted_into_model,
            conn,
        )
    }
    pub fn aliquoted_with_model<C: diesel::connection::LoadConnection>(
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
            self.aliquoted_with_model,
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
    pub fn from_procedure_template_aliquoted_into_model<C>(
        procedure_template_aliquoted_into_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template_aliquoted_into_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template_aliquoted_into_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template_aliquoted_into_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates;
        Self::table()
            .filter(
                aliquoting_procedure_templates::procedure_template_aliquoted_into_model
                    .eq(procedure_template_aliquoted_into_model),
            )
            .order_by(aliquoting_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_aliquoted_with_model<C>(
        procedure_template_aliquoted_with_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template_aliquoted_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template_aliquoted_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template_aliquoted_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates;
        Self::table()
            .filter(
                aliquoting_procedure_templates::procedure_template_aliquoted_with_model
                    .eq(procedure_template_aliquoted_with_model),
            )
            .order_by(aliquoting_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_aliquoted_with_model_and_aliquoted_with_model(
        procedure_template_aliquoted_with_model: i32,
        aliquoted_with_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates;
        Self::table()
            .filter(
                aliquoting_procedure_templates::procedure_template_aliquoted_with_model
                    .eq(procedure_template_aliquoted_with_model)
                    .and(
                        aliquoting_procedure_templates::aliquoted_with_model
                            .eq(aliquoted_with_model),
                    ),
            )
            .order_by(aliquoting_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_aliquoted_from_model_and_aliquoted_from_model(
        procedure_template_aliquoted_from_model: i32,
        aliquoted_from_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates;
        Self::table()
            .filter(
                aliquoting_procedure_templates::procedure_template_aliquoted_from_model
                    .eq(procedure_template_aliquoted_from_model)
                    .and(
                        aliquoting_procedure_templates::aliquoted_from_model
                            .eq(aliquoted_from_model),
                    ),
            )
            .order_by(aliquoting_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_aliquoted_into_model_and_aliquoted_into_model(
        procedure_template_aliquoted_into_model: i32,
        aliquoted_into_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates;
        Self::table()
            .filter(
                aliquoting_procedure_templates::procedure_template_aliquoted_into_model
                    .eq(procedure_template_aliquoted_into_model)
                    .and(
                        aliquoting_procedure_templates::aliquoted_into_model
                            .eq(aliquoted_into_model),
                    ),
            )
            .order_by(aliquoting_procedure_templates::procedure_template.asc())
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

        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates;
        Self::table()
            .filter(
                aliquoting_procedure_templates::procedure_template_pipette_tip_model
                    .eq(procedure_template_pipette_tip_model)
                    .and(aliquoting_procedure_templates::pipette_tip_model.eq(pipette_tip_model)),
            )
            .order_by(aliquoting_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_aliquoted_with_model_and_pipette_tip_model(
        aliquoted_with_model: i32,
        pipette_tip_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates;
        Self::table()
            .filter(
                aliquoting_procedure_templates::aliquoted_with_model
                    .eq(aliquoted_with_model)
                    .and(aliquoting_procedure_templates::pipette_tip_model.eq(pipette_tip_model)),
            )
            .order_by(aliquoting_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_aliquoted_from_model<C>(
        procedure_template_aliquoted_from_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template_aliquoted_from_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template_aliquoted_from_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template_aliquoted_from_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates;
        Self::table()
            .filter(
                aliquoting_procedure_templates::procedure_template_aliquoted_from_model
                    .eq(procedure_template_aliquoted_from_model),
            )
            .order_by(aliquoting_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_pipette_tip_model<C>(
        procedure_template_pipette_tip_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template_pipette_tip_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template_pipette_tip_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template_pipette_tip_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates;
        Self::table()
            .filter(
                aliquoting_procedure_templates::procedure_template_pipette_tip_model
                    .eq(procedure_template_pipette_tip_model),
            )
            .order_by(aliquoting_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    pub fn from_aliquoted_from_model<C>(
        aliquoted_from_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::aliquoted_from_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::aliquoted_from_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::aliquoted_from_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates;
        Self::table()
            .filter(aliquoting_procedure_templates::aliquoted_from_model.eq(aliquoted_from_model))
            .order_by(aliquoting_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    pub fn from_aliquoted_into_model<C>(
        aliquoted_into_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::aliquoted_into_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::aliquoted_into_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::aliquoted_into_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::procedure_template,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates;
        Self::table()
            .filter(aliquoting_procedure_templates::aliquoted_into_model.eq(aliquoted_into_model))
            .order_by(aliquoting_procedure_templates::procedure_template.asc())
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
            aliquoting_procedure_templates::aliquoting_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(aliquoting_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::name.eq(name))
            .order_by(aliquoting_procedure_templates::procedure_template.asc())
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
            aliquoting_procedure_templates::aliquoting_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(aliquoting_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::description.eq(description))
            .order_by(aliquoting_procedure_templates::procedure_template.asc())
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
            aliquoting_procedure_templates::aliquoting_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(aliquoting_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::created_by.eq(created_by))
            .order_by(aliquoting_procedure_templates::procedure_template.asc())
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
            aliquoting_procedure_templates::aliquoting_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(aliquoting_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::updated_by.eq(updated_by))
            .order_by(aliquoting_procedure_templates::procedure_template.asc())
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
            aliquoting_procedure_templates::aliquoting_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(aliquoting_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::deprecated.eq(deprecated))
            .order_by(aliquoting_procedure_templates::procedure_template.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<AliquotingProcedureTemplate> for AliquotingProcedureTemplate {
    fn as_ref(&self) -> &AliquotingProcedureTemplate {
        self
    }
}
