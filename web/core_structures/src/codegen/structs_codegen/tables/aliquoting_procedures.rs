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
    table_name = crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures
)]
pub struct AliquotingProcedure {
    pub procedure: ::rosetta_uuid::Uuid,
    pub procedure_template: i32,
    pub aliquoted_with: Option<::rosetta_uuid::Uuid>,
    pub aliquoted_with_model: i32,
    pub procedure_template_aliquoted_with_model: i32,
    pub procedure_aliquoted_with: ::rosetta_uuid::Uuid,
    pub pipette_tip_model: i32,
    pub procedure_template_pipette_tip_model: i32,
    pub procedure_pipette_tip: ::rosetta_uuid::Uuid,
    pub aliquoted_from: ::rosetta_uuid::Uuid,
    pub procedure_template_aliquoted_from_model: i32,
    pub procedure_aliquoted_from: ::rosetta_uuid::Uuid,
    pub aliquoted_into: ::rosetta_uuid::Uuid,
    pub procedure_template_aliquoted_into_model: i32,
    pub procedure_aliquoted_into: ::rosetta_uuid::Uuid,
}
impl web_common_traits::prelude::TableName for AliquotingProcedure {
    const TABLE_NAME: &'static str = "aliquoting_procedures";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    > for AliquotingProcedure
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure,
    > for AliquotingProcedure
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for AliquotingProcedure {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.procedure
    }
}
impl AliquotingProcedure {
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
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate::table(),
                self.procedure_template,
            ),
            conn,
        )
    }
    pub fn aliquoted_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::pipettes::Pipette>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::pipettes::Pipette: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::pipettes::Pipette as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipettes::Pipette as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::pipettes::Pipette as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipettes::Pipette as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::pipettes::Pipette as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipettes::Pipette as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::pipettes::Pipette,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        let Some(aliquoted_with) = self.aliquoted_with else {
            return Ok(None);
        };
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::pipettes::Pipette::table(),
                aliquoted_with,
            ),
            conn,
        )
        .map(Some)
    }
    pub fn aliquoted_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::pipette_models::PipetteModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::pipette_models::PipetteModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::pipette_models::PipetteModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::pipette_models::PipetteModel::table(),
                self.aliquoted_with_model,
            ),
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
                self.procedure_template_aliquoted_with_model,
            ),
            conn,
        )
    }
    pub fn procedure_aliquoted_with<C: diesel::connection::LoadConnection>(
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
                self.procedure_aliquoted_with,
            ),
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
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel::table(
                ),
                self.pipette_tip_model,
            ),
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
                self.procedure_template_pipette_tip_model,
            ),
            conn,
        )
    }
    pub fn procedure_pipette_tip<C: diesel::connection::LoadConnection>(
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
                self.procedure_pipette_tip,
            ),
            conn,
        )
    }
    pub fn aliquoted_from<C: diesel::connection::LoadConnection>(
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
                self.aliquoted_from,
            ),
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
                self.procedure_template_aliquoted_from_model,
            ),
            conn,
        )
    }
    pub fn procedure_aliquoted_from<C: diesel::connection::LoadConnection>(
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
                self.procedure_aliquoted_from,
            ),
            conn,
        )
    }
    pub fn aliquoted_into<C: diesel::connection::LoadConnection>(
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
                self.aliquoted_into,
            ),
            conn,
        )
    }
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
                self.procedure_template_aliquoted_into_model,
            ),
            conn,
        )
    }
    pub fn procedure_aliquoted_into<C: diesel::connection::LoadConnection>(
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
                self.procedure_aliquoted_into,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_procedure_template_fkey(
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
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_template_procedure_templat_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::dsl::procedure_template_aliquoted_with_model
                            .eq(&self.procedure_template_aliquoted_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_template_procedure_templa_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::dsl::procedure_template_pipette_tip_model
                            .eq(&self.procedure_template_pipette_tip_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_template_procedure_templa_fkey2(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::dsl::procedure_template_aliquoted_from_model
                            .eq(&self.procedure_template_aliquoted_from_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_template_procedure_templa_fkey3(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::dsl::procedure_template_aliquoted_into_model
                            .eq(&self.procedure_template_aliquoted_into_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_aliquoted_with_procedure_t_fkey(
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
                    .eq(&self.procedure_aliquoted_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_aliquoted_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_pipette_tip_procedure_temp_fkey(
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
                    .eq(&self.procedure_pipette_tip)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_pipette_tip_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_aliquoted_from_procedure_t_fkey(
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
                    .eq(&self.procedure_aliquoted_from)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_aliquoted_from_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_aliquoted_into_procedure_t_fkey(
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
                    .eq(&self.procedure_aliquoted_into)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_aliquoted_into_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_aliquoted_with_aliquoted_w_fkey(
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
                    .eq(&self.procedure_aliquoted_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.aliquoted_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_aliquoted_with_aliquoted_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset>,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        let Some(aliquoted_with) = self.aliquoted_with else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_aliquoted_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(aliquoted_with),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_aliquoted_from_aliquoted_f_fkey(
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
                    .eq(&self.procedure_aliquoted_from)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.aliquoted_from),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_aliquoted_into_aliquoted_i_fkey(
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
                    .eq(&self.procedure_aliquoted_into)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.aliquoted_into),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_pipette_tip_pipette_tip_mo_fkey(
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
                    .eq(&self.procedure_pipette_tip)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.pipette_tip_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn aliquoting_procedures_aliquoted_with_model_pipette_tip_mod_fkey<
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
                (self.aliquoted_with_model, self.pipette_tip_model),
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

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(aliquoting_procedures::procedure_template.eq(procedure_template))
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_aliquoted_with(
        aliquoted_with: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(aliquoting_procedures::aliquoted_with.eq(aliquoted_with))
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_aliquoted_with_model(
        aliquoted_with_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(aliquoting_procedures::aliquoted_with_model.eq(aliquoted_with_model))
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_aliquoted_with_model(
        procedure_template_aliquoted_with_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(
                aliquoting_procedures::procedure_template_aliquoted_with_model
                    .eq(procedure_template_aliquoted_with_model),
            )
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_aliquoted_with(
        procedure_aliquoted_with: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(aliquoting_procedures::procedure_aliquoted_with.eq(procedure_aliquoted_with))
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_pipette_tip_model(
        pipette_tip_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(aliquoting_procedures::pipette_tip_model.eq(pipette_tip_model))
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_pipette_tip_model(
        procedure_template_pipette_tip_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(
                aliquoting_procedures::procedure_template_pipette_tip_model
                    .eq(procedure_template_pipette_tip_model),
            )
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_pipette_tip(
        procedure_pipette_tip: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(aliquoting_procedures::procedure_pipette_tip.eq(procedure_pipette_tip))
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_aliquoted_from(
        aliquoted_from: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(aliquoting_procedures::aliquoted_from.eq(aliquoted_from))
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_aliquoted_from_model(
        procedure_template_aliquoted_from_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(
                aliquoting_procedures::procedure_template_aliquoted_from_model
                    .eq(procedure_template_aliquoted_from_model),
            )
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_aliquoted_from(
        procedure_aliquoted_from: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(aliquoting_procedures::procedure_aliquoted_from.eq(procedure_aliquoted_from))
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_aliquoted_into(
        aliquoted_into: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(aliquoting_procedures::aliquoted_into.eq(aliquoted_into))
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_aliquoted_into_model(
        procedure_template_aliquoted_into_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(
                aliquoting_procedures::procedure_template_aliquoted_into_model
                    .eq(procedure_template_aliquoted_into_model),
            )
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_aliquoted_into(
        procedure_aliquoted_into: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(aliquoting_procedures::procedure_aliquoted_into.eq(procedure_aliquoted_into))
            .order_by(aliquoting_procedures::procedure.asc())
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

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(
                aliquoting_procedures::procedure
                    .eq(procedure)
                    .and(aliquoting_procedures::procedure_template.eq(procedure_template)),
            )
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_aliquoted_with_model(
        procedure_template: &i32,
        procedure_template_aliquoted_with_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(
                aliquoting_procedures::procedure_template.eq(procedure_template).and(
                    aliquoting_procedures::procedure_template_aliquoted_with_model
                        .eq(procedure_template_aliquoted_with_model),
                ),
            )
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_pipette_tip_model(
        procedure_template: &i32,
        procedure_template_pipette_tip_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(
                aliquoting_procedures::procedure_template.eq(procedure_template).and(
                    aliquoting_procedures::procedure_template_pipette_tip_model
                        .eq(procedure_template_pipette_tip_model),
                ),
            )
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_aliquoted_from_model(
        procedure_template: &i32,
        procedure_template_aliquoted_from_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(
                aliquoting_procedures::procedure_template.eq(procedure_template).and(
                    aliquoting_procedures::procedure_template_aliquoted_from_model
                        .eq(procedure_template_aliquoted_from_model),
                ),
            )
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_aliquoted_into_model(
        procedure_template: &i32,
        procedure_template_aliquoted_into_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(
                aliquoting_procedures::procedure_template.eq(procedure_template).and(
                    aliquoting_procedures::procedure_template_aliquoted_into_model
                        .eq(procedure_template_aliquoted_into_model),
                ),
            )
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_aliquoted_with_and_procedure_template_aliquoted_with_model(
        procedure_aliquoted_with: &::rosetta_uuid::Uuid,
        procedure_template_aliquoted_with_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(
                aliquoting_procedures::procedure_aliquoted_with.eq(procedure_aliquoted_with).and(
                    aliquoting_procedures::procedure_template_aliquoted_with_model
                        .eq(procedure_template_aliquoted_with_model),
                ),
            )
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_pipette_tip_and_procedure_template_pipette_tip_model(
        procedure_pipette_tip: &::rosetta_uuid::Uuid,
        procedure_template_pipette_tip_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(
                aliquoting_procedures::procedure_pipette_tip.eq(procedure_pipette_tip).and(
                    aliquoting_procedures::procedure_template_pipette_tip_model
                        .eq(procedure_template_pipette_tip_model),
                ),
            )
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_aliquoted_from_and_procedure_template_aliquoted_from_model(
        procedure_aliquoted_from: &::rosetta_uuid::Uuid,
        procedure_template_aliquoted_from_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(
                aliquoting_procedures::procedure_aliquoted_from.eq(procedure_aliquoted_from).and(
                    aliquoting_procedures::procedure_template_aliquoted_from_model
                        .eq(procedure_template_aliquoted_from_model),
                ),
            )
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_aliquoted_into_and_procedure_template_aliquoted_into_model(
        procedure_aliquoted_into: &::rosetta_uuid::Uuid,
        procedure_template_aliquoted_into_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(
                aliquoting_procedures::procedure_aliquoted_into.eq(procedure_aliquoted_into).and(
                    aliquoting_procedures::procedure_template_aliquoted_into_model
                        .eq(procedure_template_aliquoted_into_model),
                ),
            )
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_aliquoted_with_and_aliquoted_with_model(
        procedure_aliquoted_with: &::rosetta_uuid::Uuid,
        aliquoted_with_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(
                aliquoting_procedures::procedure_aliquoted_with
                    .eq(procedure_aliquoted_with)
                    .and(aliquoting_procedures::aliquoted_with_model.eq(aliquoted_with_model)),
            )
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_aliquoted_with_and_aliquoted_with(
        procedure_aliquoted_with: &::rosetta_uuid::Uuid,
        aliquoted_with: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(
                aliquoting_procedures::procedure_aliquoted_with
                    .eq(procedure_aliquoted_with)
                    .and(aliquoting_procedures::aliquoted_with.eq(aliquoted_with)),
            )
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_aliquoted_from_and_aliquoted_from(
        procedure_aliquoted_from: &::rosetta_uuid::Uuid,
        aliquoted_from: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(
                aliquoting_procedures::procedure_aliquoted_from
                    .eq(procedure_aliquoted_from)
                    .and(aliquoting_procedures::aliquoted_from.eq(aliquoted_from)),
            )
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_aliquoted_into_and_aliquoted_into(
        procedure_aliquoted_into: &::rosetta_uuid::Uuid,
        aliquoted_into: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(
                aliquoting_procedures::procedure_aliquoted_into
                    .eq(procedure_aliquoted_into)
                    .and(aliquoting_procedures::aliquoted_into.eq(aliquoted_into)),
            )
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_pipette_tip_and_pipette_tip_model(
        procedure_pipette_tip: &::rosetta_uuid::Uuid,
        pipette_tip_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(
                aliquoting_procedures::procedure_pipette_tip
                    .eq(procedure_pipette_tip)
                    .and(aliquoting_procedures::pipette_tip_model.eq(pipette_tip_model)),
            )
            .order_by(aliquoting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_aliquoted_with_model_and_pipette_tip_model(
        aliquoted_with_model: &i32,
        pipette_tip_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures;
        Self::table()
            .filter(
                aliquoting_procedures::aliquoted_with_model
                    .eq(aliquoted_with_model)
                    .and(aliquoting_procedures::pipette_tip_model.eq(pipette_tip_model)),
            )
            .order_by(aliquoting_procedures::procedure.asc())
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
            aliquoting_procedures::aliquoting_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(aliquoting_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(
                procedures::procedure_template
                    .eq(procedure_template)
                    .and(procedures::procedure.eq(procedure)),
            )
            .order_by(aliquoting_procedures::procedure.asc())
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
            aliquoting_procedures::aliquoting_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(aliquoting_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::parent_procedure.eq(parent_procedure))
            .order_by(aliquoting_procedures::procedure.asc())
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
            aliquoting_procedures::aliquoting_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(aliquoting_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::parent_procedure_template.eq(parent_procedure_template))
            .order_by(aliquoting_procedures::procedure.asc())
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
            aliquoting_procedures::aliquoting_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(aliquoting_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::most_concrete_table.eq(most_concrete_table))
            .order_by(aliquoting_procedures::procedure.asc())
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
            aliquoting_procedures::aliquoting_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(aliquoting_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::created_by.eq(created_by))
            .order_by(aliquoting_procedures::procedure.asc())
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
            aliquoting_procedures::aliquoting_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(aliquoting_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::created_at.eq(created_at))
            .order_by(aliquoting_procedures::procedure.asc())
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
            aliquoting_procedures::aliquoting_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(aliquoting_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::updated_by.eq(updated_by))
            .order_by(aliquoting_procedures::procedure.asc())
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
            aliquoting_procedures::aliquoting_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(aliquoting_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::updated_at.eq(updated_at))
            .order_by(aliquoting_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<AliquotingProcedure> for AliquotingProcedure {
    fn as_ref(&self) -> &AliquotingProcedure {
        self
    }
}
