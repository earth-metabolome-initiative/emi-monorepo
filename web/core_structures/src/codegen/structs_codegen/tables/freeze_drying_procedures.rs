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
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    > for FreezeDryingProcedure
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure,
    > for FreezeDryingProcedure
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
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer::table(),
                self.freeze_dried_container,
            ),
            conn,
        )
    }
    pub fn freeze_dried_container_model<C: diesel::connection::LoadConnection>(
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
                self.freeze_dried_container_model,
            ),
            conn,
        )
    }
    pub fn freeze_dried_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        let Some(freeze_dried_with) = self.freeze_dried_with else {
            return Ok(None);
        };
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer::table(),
                freeze_dried_with,
            ),
            conn,
        )
        .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub fn freeze_drying_procedures_freeze_dried_with_freeze_dried_wi_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<crate::codegen::structs_codegen::tables::assets::Asset>, diesel::result::Error>
    {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        let Some(freeze_dried_with) = self.freeze_dried_with else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::assets::Asset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::assets::assets::dsl::id
                    .eq(freeze_dried_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::assets::assets::dsl::model
                            .eq(&self.freeze_dried_with_model),
                    ),
            )
            .first::<crate::codegen::structs_codegen::tables::assets::Asset>(conn)
            .map(Some)
    }
    pub fn freeze_dried_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel::table(),
                self.freeze_dried_with_model,
            ),
            conn,
        )
    }
    pub fn freeze_drying_procedures_freeze_dried_with_model_freeze_dr_fkey<
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
                (self.freeze_dried_with_model, self.freeze_dried_container_model),
            ),
            conn,
        )
    }
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
    #[cfg(feature = "postgres")]
    pub fn freeze_drying_procedures_procedure_freeze_dried_container_fkey(
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
                    .eq(&self.procedure_freeze_dried_container)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_freeze_dried_container_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn procedure_freeze_dried_container<C: diesel::connection::LoadConnection>(
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
                self.procedure_freeze_dried_container,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn freeze_drying_procedures_procedure_freeze_dried_container_fkey1(
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
                    .eq(&self.procedure_freeze_dried_container)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.freeze_dried_container_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn freeze_drying_procedures_procedure_freeze_dried_container_fkey2(
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
                    .eq(&self.procedure_freeze_dried_container)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.freeze_dried_container),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn procedure_freeze_dried_with<C: diesel::connection::LoadConnection>(
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
                self.procedure_freeze_dried_with,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn freeze_drying_procedures_procedure_freeze_dried_with_free_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset>,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        let Some(freeze_dried_with) = self.freeze_dried_with else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_freeze_dried_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(freeze_dried_with),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub fn freeze_drying_procedures_procedure_freeze_dried_with_freez_fkey(
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
                    .eq(&self.procedure_freeze_dried_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.freeze_dried_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn freeze_drying_procedures_procedure_freeze_dried_with_proce_fkey(
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
                    .eq(&self.procedure_freeze_dried_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_freeze_dried_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn freeze_drying_procedures_procedure_procedure_template_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::procedures::Procedure, diesel::result::Error>
    {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedures::Procedure::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure
                    .eq(&self.procedure)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure_template
                            .eq(&self.procedure_template),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedures::Procedure,
            >(conn)
    }
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate::table(),
                self.procedure_template,
            ),
            conn,
        )
    }
    pub fn procedure_template_freeze_dried_container_model<
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
                self.procedure_template_freeze_dried_container_model,
            ),
            conn,
        )
    }
    pub fn procedure_template_freeze_dried_with_model<
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
                self.procedure_template_freeze_dried_with_model,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn freeze_drying_procedures_procedure_template_procedure_tem_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::freeze_drying_procedure_templates::freeze_drying_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::freeze_drying_procedure_templates::freeze_drying_procedure_templates::dsl::procedure_template_freeze_dried_container_model
                            .eq(&self.procedure_template_freeze_dried_container_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn freeze_drying_procedures_procedure_template_procedure_temp_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::freeze_drying_procedure_templates::freeze_drying_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::freeze_drying_procedure_templates::freeze_drying_procedure_templates::dsl::procedure_template_freeze_dried_with_model
                            .eq(&self.procedure_template_freeze_dried_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template(
        procedure_template: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures;
        Self::table()
            .filter(freeze_drying_procedures::procedure_template.eq(procedure_template))
            .order_by(freeze_drying_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_freeze_dried_container(
        freeze_dried_container: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures;
        Self::table()
            .filter(freeze_drying_procedures::freeze_dried_container.eq(freeze_dried_container))
            .order_by(freeze_drying_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_freeze_dried_container_model(
        freeze_dried_container_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures;
        Self::table()
            .filter(
                freeze_drying_procedures::freeze_dried_container_model
                    .eq(freeze_dried_container_model),
            )
            .order_by(freeze_drying_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_freeze_dried_container_model(
        procedure_template_freeze_dried_container_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
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
    #[cfg(feature = "postgres")]
    pub fn from_procedure_freeze_dried_container(
        procedure_freeze_dried_container: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
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
    pub fn from_freeze_dried_with(
        freeze_dried_with: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures;
        Self::table()
            .filter(freeze_drying_procedures::freeze_dried_with.eq(freeze_dried_with))
            .order_by(freeze_drying_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_freeze_dried_with_model(
        freeze_dried_with_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures;
        Self::table()
            .filter(freeze_drying_procedures::freeze_dried_with_model.eq(freeze_dried_with_model))
            .order_by(freeze_drying_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_freeze_dried_with_model(
        procedure_template_freeze_dried_with_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
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
    pub fn from_procedure_freeze_dried_with(
        procedure_freeze_dried_with: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
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
    pub fn from_freeze_dried_with_and_freeze_dried_with_model(
        freeze_dried_with: &::rosetta_uuid::Uuid,
        freeze_dried_with_model: &i32,
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
        freeze_dried_with_model: &i32,
        freeze_dried_container_model: &i32,
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
    #[cfg(feature = "postgres")]
    pub fn from_procedure_freeze_dried_container_and_procedure_template_freeze_dried_container_model(
        procedure_freeze_dried_container: &::rosetta_uuid::Uuid,
        procedure_template_freeze_dried_container_model: &i32,
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
    #[cfg(feature = "postgres")]
    pub fn from_procedure_freeze_dried_container_and_freeze_dried_container_model(
        procedure_freeze_dried_container: &::rosetta_uuid::Uuid,
        freeze_dried_container_model: &i32,
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
        procedure_freeze_dried_container: &::rosetta_uuid::Uuid,
        freeze_dried_container: &::rosetta_uuid::Uuid,
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
    #[cfg(feature = "postgres")]
    pub fn from_procedure_freeze_dried_with_and_freeze_dried_with(
        procedure_freeze_dried_with: &::rosetta_uuid::Uuid,
        freeze_dried_with: &::rosetta_uuid::Uuid,
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
        procedure_freeze_dried_with: &::rosetta_uuid::Uuid,
        freeze_dried_with_model: &i32,
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
        procedure_freeze_dried_with: &::rosetta_uuid::Uuid,
        procedure_template_freeze_dried_with_model: &i32,
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
        procedure: &::rosetta_uuid::Uuid,
        procedure_template: &i32,
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
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_freeze_dried_container_model(
        procedure_template: &i32,
        procedure_template_freeze_dried_container_model: &i32,
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
        procedure_template: &i32,
        procedure_template_freeze_dried_with_model: &i32,
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
        procedure_template: &i32,
        procedure: &::rosetta_uuid::Uuid,
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
        parent_procedure: &::rosetta_uuid::Uuid,
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
        parent_procedure_template: &i32,
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
        created_by: &i32,
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
        created_at: &::rosetta_timestamp::TimestampUTC,
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
        updated_by: &i32,
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
        updated_at: &::rosetta_timestamp::TimestampUTC,
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
