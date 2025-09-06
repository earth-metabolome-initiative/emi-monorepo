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
#[diesel(primary_key(procedure_template))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates
)]
pub struct BallMillProcedureTemplate {
    pub procedure_template: i32,
    pub kelvin: f32,
    pub kelvin_tolerance_percentage: f32,
    pub seconds: f32,
    pub hertz: f32,
    pub bead_model: i32,
    pub procedure_template_bead_model: i32,
    pub number_of_beads: i16,
    pub milled_with_model: i32,
    pub procedure_template_milled_with_model: i32,
    pub milled_container_model: i32,
    pub procedure_template_milled_container_model: i32,
}
impl web_common_traits::prelude::TableName for BallMillProcedureTemplate {
    const TABLE_NAME: &'static str = "ball_mill_procedure_templates";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    > for BallMillProcedureTemplate
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl web_common_traits::prelude::ExtensionTable<
    crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate,
> for BallMillProcedureTemplate
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{}
impl diesel::Identifiable for BallMillProcedureTemplate {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.procedure_template
    }
}
impl BallMillProcedureTemplate {
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::table(),
                self.procedure_template,
            ),
            conn,
        )
    }
    pub fn bead_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::bead_models::BeadModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::bead_models::BeadModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::bead_models::BeadModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::bead_models::BeadModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::bead_models::BeadModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::bead_models::BeadModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::bead_models::BeadModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::bead_models::BeadModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::bead_models::BeadModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::bead_models::BeadModel::table(),
                self.bead_model,
            ),
            conn,
        )
    }
    pub fn procedure_template_bead_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table(),
                self.procedure_template_bead_model,
            ),
            conn,
        )
    }
    pub fn milled_with_model<C: diesel::connection::LoadConnection>(
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
                self.milled_with_model,
            ),
            conn,
        )
    }
    pub fn procedure_template_milled_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table(),
                self.procedure_template_milled_with_model,
            ),
            conn,
        )
    }
    pub fn milled_container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel::table(),
                self.milled_container_model,
            ),
            conn,
        )
    }
    pub fn procedure_template_milled_container_model<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table(),
                self.procedure_template_milled_container_model,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedure_template_procedure_template_bead_mode_fkey1(
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
                    .eq(&self.procedure_template_bead_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.bead_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedure_template_procedure_template_milled_wi_fkey1(
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
                    .eq(&self.procedure_template_milled_with_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.milled_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedure_template_procedure_template_milled_co_fkey1(
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
                    .eq(&self.procedure_template_milled_container_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.milled_container_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    pub fn ball_mill_procedure_templates_milled_with_model_milled_con_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule::table(),
                (self.milled_with_model, self.milled_container_model),
            ),
            conn,
        )
    }
    pub fn ball_mill_procedure_templates_milled_with_model_bead_model_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule::table(),
                (self.milled_with_model, self.bead_model),
            ),
            conn,
        )
    }
    pub fn ball_mill_procedure_templates_bead_model_milled_container_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule::table(),
                (self.bead_model, self.milled_container_model),
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_bead_model(
        bead_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates;
        Self::table()
            .filter(ball_mill_procedure_templates::bead_model.eq(bead_model))
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_bead_model(
        procedure_template_bead_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates;
        Self::table()
            .filter(
                ball_mill_procedure_templates::procedure_template_bead_model
                    .eq(procedure_template_bead_model),
            )
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_number_of_beads(
        number_of_beads: &i16,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates;
        Self::table()
            .filter(ball_mill_procedure_templates::number_of_beads.eq(number_of_beads))
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_milled_with_model(
        milled_with_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates;
        Self::table()
            .filter(ball_mill_procedure_templates::milled_with_model.eq(milled_with_model))
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_milled_with_model(
        procedure_template_milled_with_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates;
        Self::table()
            .filter(
                ball_mill_procedure_templates::procedure_template_milled_with_model
                    .eq(procedure_template_milled_with_model),
            )
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_milled_container_model(
        milled_container_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates;
        Self::table()
            .filter(
                ball_mill_procedure_templates::milled_container_model.eq(milled_container_model),
            )
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_milled_container_model(
        procedure_template_milled_container_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates;
        Self::table()
            .filter(
                ball_mill_procedure_templates::procedure_template_milled_container_model
                    .eq(procedure_template_milled_container_model),
            )
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_bead_model(
        procedure_template: &i32,
        procedure_template_bead_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates;
        Self::table()
            .filter(
                ball_mill_procedure_templates::procedure_template.eq(procedure_template).and(
                    ball_mill_procedure_templates::procedure_template_bead_model
                        .eq(procedure_template_bead_model),
                ),
            )
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_milled_with_model(
        procedure_template: &i32,
        procedure_template_milled_with_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates;
        Self::table()
            .filter(
                ball_mill_procedure_templates::procedure_template.eq(procedure_template).and(
                    ball_mill_procedure_templates::procedure_template_milled_with_model
                        .eq(procedure_template_milled_with_model),
                ),
            )
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_milled_container_model(
        procedure_template: &i32,
        procedure_template_milled_container_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates;
        Self::table()
            .filter(
                ball_mill_procedure_templates::procedure_template.eq(procedure_template).and(
                    ball_mill_procedure_templates::procedure_template_milled_container_model
                        .eq(procedure_template_milled_container_model),
                ),
            )
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_bead_model_and_bead_model(
        procedure_template_bead_model: &i32,
        bead_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates;
        Self::table()
            .filter(
                ball_mill_procedure_templates::procedure_template_bead_model
                    .eq(procedure_template_bead_model)
                    .and(ball_mill_procedure_templates::bead_model.eq(bead_model)),
            )
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_milled_with_model_and_milled_with_model(
        procedure_template_milled_with_model: &i32,
        milled_with_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates;
        Self::table()
            .filter(
                ball_mill_procedure_templates::procedure_template_milled_with_model
                    .eq(procedure_template_milled_with_model)
                    .and(ball_mill_procedure_templates::milled_with_model.eq(milled_with_model)),
            )
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_milled_container_model_and_milled_container_model(
        procedure_template_milled_container_model: &i32,
        milled_container_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates;
        Self::table()
            .filter(
                ball_mill_procedure_templates::procedure_template_milled_container_model
                    .eq(procedure_template_milled_container_model)
                    .and(
                        ball_mill_procedure_templates::milled_container_model
                            .eq(milled_container_model),
                    ),
            )
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_milled_with_model_and_milled_container_model(
        milled_with_model: &i32,
        milled_container_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates;
        Self::table()
            .filter(ball_mill_procedure_templates::milled_with_model.eq(milled_with_model).and(
                ball_mill_procedure_templates::milled_container_model.eq(milled_container_model),
            ))
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_milled_with_model_and_bead_model(
        milled_with_model: &i32,
        bead_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates;
        Self::table()
            .filter(
                ball_mill_procedure_templates::milled_with_model
                    .eq(milled_with_model)
                    .and(ball_mill_procedure_templates::bead_model.eq(bead_model)),
            )
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_bead_model_and_milled_container_model(
        bead_model: &i32,
        milled_container_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates;
        Self::table()
            .filter(ball_mill_procedure_templates::bead_model.eq(bead_model).and(
                ball_mill_procedure_templates::milled_container_model.eq(milled_container_model),
            ))
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
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
            ball_mill_procedure_templates::ball_mill_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(ball_mill_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::name.eq(name))
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
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
            ball_mill_procedure_templates::ball_mill_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(ball_mill_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::most_concrete_table.eq(most_concrete_table))
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
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
            ball_mill_procedure_templates::ball_mill_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(ball_mill_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::description.eq(description))
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
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
            ball_mill_procedure_templates::ball_mill_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(ball_mill_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::deprecated.eq(deprecated))
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
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
            ball_mill_procedure_templates::ball_mill_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(ball_mill_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::icon.eq(icon))
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
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
            ball_mill_procedure_templates::ball_mill_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(ball_mill_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::created_by.eq(created_by))
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
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
            ball_mill_procedure_templates::ball_mill_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(ball_mill_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::created_at.eq(created_at))
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
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
            ball_mill_procedure_templates::ball_mill_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(ball_mill_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::updated_by.eq(updated_by))
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
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
            ball_mill_procedure_templates::ball_mill_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(ball_mill_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::updated_at.eq(updated_at))
            .order_by(ball_mill_procedure_templates::procedure_template.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<BallMillProcedureTemplate> for BallMillProcedureTemplate {
    fn as_ref(&self) -> &BallMillProcedureTemplate {
        self
    }
}
