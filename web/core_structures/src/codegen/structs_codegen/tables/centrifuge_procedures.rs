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
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
        foreign_key = centrifuged_container
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        foreign_key = centrifuged_container_model
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::centrifuges::Centrifuge,
        foreign_key = centrifuged_with
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel,
        foreign_key = centrifuged_with_model
    )
)]
#[diesel(primary_key(procedure))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures
)]
pub struct CentrifugeProcedure {
    pub procedure: ::rosetta_uuid::Uuid,
    pub procedure_template: i32,
    pub centrifuged_container: ::rosetta_uuid::Uuid,
    pub centrifuged_container_model: i32,
    pub procedure_template_centrifuged_container_model: i32,
    pub procedure_centrifuged_container: ::rosetta_uuid::Uuid,
    pub centrifuged_with_model: i32,
    pub centrifuged_with: Option<::rosetta_uuid::Uuid>,
    pub procedure_template_centrifuged_with_model: i32,
    pub procedure_centrifuged_with: ::rosetta_uuid::Uuid,
}
impl web_common_traits::prelude::TableName for CentrifugeProcedure {
    const TABLE_NAME: &'static str = "centrifuge_procedures";
}
impl<'a> From<&'a CentrifugeProcedure>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder,
    >
{
    fn from(value: &'a CentrifugeProcedure) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.procedure)
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    > for CentrifugeProcedure
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure,
    > for CentrifugeProcedure
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for CentrifugeProcedure {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.procedure
    }
}
impl web_common_traits::database::PrimaryKeyLike for CentrifugeProcedure {
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.procedure
    }
}
impl CentrifugeProcedure {
    pub fn centrifuged_container<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer::read(
            self.centrifuged_container,
            conn,
        )
    }
    pub fn centrifuged_container_model<C: diesel::connection::LoadConnection>(
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
            self.centrifuged_container_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn centrifuge_procedures_centrifuged_with_centrifuged_with_mo_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<crate::codegen::structs_codegen::tables::assets::Asset>, diesel::result::Error>
    {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
            associations::HasTable,
        };
        let Some(centrifuged_with) = self.centrifuged_with else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::assets::Asset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::assets::assets::dsl::id
                    .eq(centrifuged_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::assets::assets::dsl::model
                            .eq(&self.centrifuged_with_model),
                    ),
            )
            .first::<crate::codegen::structs_codegen::tables::assets::Asset>(conn)
            .optional()
    }
    pub fn centrifuged_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::centrifuges::Centrifuge>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::centrifuges::Centrifuge:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(centrifuged_with) = self.centrifuged_with else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::centrifuges::Centrifuge::read(
            centrifuged_with,
            conn,
        )
        .optional()
    }
    pub fn centrifuge_procedures_centrifuged_with_model_centrifuged_c_fkey<
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
            (self.centrifuged_with_model, self.centrifuged_container_model),
            conn,
        )
    }
    pub fn centrifuged_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel::read(
            self.centrifuged_with_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn centrifuge_procedures_procedure_centrifuged_container_cen_fkey1(
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
                    .eq(&self.procedure_centrifuged_container)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.centrifuged_container),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn centrifuge_procedures_procedure_centrifuged_container_cent_fkey(
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
                    .eq(&self.procedure_centrifuged_container)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.centrifuged_container_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn procedure_centrifuged_container<C: diesel::connection::LoadConnection>(
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
            self.procedure_centrifuged_container,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn centrifuge_procedures_procedure_centrifuged_container_proc_fkey(
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
                    .eq(&self.procedure_centrifuged_container)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_centrifuged_container_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn centrifuge_procedures_procedure_centrifuged_with_centrifu_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset>,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
            associations::HasTable,
        };
        let Some(centrifuged_with) = self.centrifuged_with else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_centrifuged_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(centrifuged_with),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn centrifuge_procedures_procedure_centrifuged_with_centrifug_fkey(
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
                    .eq(&self.procedure_centrifuged_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.centrifuged_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn procedure_centrifuged_with<C: diesel::connection::LoadConnection>(
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
            self.procedure_centrifuged_with,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn centrifuge_procedures_procedure_centrifuged_with_procedure_fkey(
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
                    .eq(&self.procedure_centrifuged_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_centrifuged_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn procedure_template_centrifuged_container_model<
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
            self.procedure_template_centrifuged_container_model,
            conn,
        )
    }
    pub fn procedure_template_centrifuged_with_model<
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
            self.procedure_template_centrifuged_with_model,
            conn,
        )
    }
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate::read(
            self.procedure_template,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn centrifuge_procedures_procedure_template_procedure_templa_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::centrifuge_procedure_templates::centrifuge_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::centrifuge_procedure_templates::centrifuge_procedure_templates::dsl::procedure_template_centrifuged_with_model
                            .eq(&self.procedure_template_centrifuged_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn centrifuge_procedures_procedure_template_procedure_templat_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::centrifuge_procedure_templates::centrifuge_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::centrifuge_procedure_templates::centrifuge_procedure_templates::dsl::procedure_template_centrifuged_container_model
                            .eq(&self.procedure_template_centrifuged_container_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_centrifuged_with_and_centrifuged_with_model(
        centrifuged_with: ::rosetta_uuid::Uuid,
        centrifuged_with_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures;
        Self::table()
            .filter(
                centrifuge_procedures::centrifuged_with
                    .eq(centrifuged_with)
                    .and(centrifuge_procedures::centrifuged_with_model.eq(centrifuged_with_model)),
            )
            .order_by(centrifuge_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_centrifuged_with_model_and_centrifuged_container_model(
        centrifuged_with_model: i32,
        centrifuged_container_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures;
        Self::table()
            .filter(centrifuge_procedures::centrifuged_with_model.eq(centrifuged_with_model).and(
                centrifuge_procedures::centrifuged_container_model.eq(centrifuged_container_model),
            ))
            .order_by(centrifuge_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_centrifuged_container_and_centrifuged_container(
        procedure_centrifuged_container: ::rosetta_uuid::Uuid,
        centrifuged_container: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures;
        Self::table()
            .filter(
                centrifuge_procedures::procedure_centrifuged_container
                    .eq(procedure_centrifuged_container)
                    .and(centrifuge_procedures::centrifuged_container.eq(centrifuged_container)),
            )
            .order_by(centrifuge_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_centrifuged_container_and_centrifuged_container_model(
        procedure_centrifuged_container: ::rosetta_uuid::Uuid,
        centrifuged_container_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures;
        Self::table()
            .filter(
                centrifuge_procedures::procedure_centrifuged_container
                    .eq(procedure_centrifuged_container)
                    .and(
                        centrifuge_procedures::centrifuged_container_model
                            .eq(centrifuged_container_model),
                    ),
            )
            .order_by(centrifuge_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_centrifuged_container<C>(
        procedure_centrifuged_container: ::rosetta_uuid::Uuid,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure_centrifuged_container as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure_centrifuged_container as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure_centrifuged_container as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures;
        Self::table()
            .filter(
                centrifuge_procedures::procedure_centrifuged_container
                    .eq(procedure_centrifuged_container),
            )
            .order_by(centrifuge_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_centrifuged_container_and_procedure_template_centrifuged_container_model(
        procedure_centrifuged_container: ::rosetta_uuid::Uuid,
        procedure_template_centrifuged_container_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures;
        Self::table()
            .filter(
                centrifuge_procedures::procedure_centrifuged_container
                    .eq(procedure_centrifuged_container)
                    .and(
                        centrifuge_procedures::procedure_template_centrifuged_container_model
                            .eq(procedure_template_centrifuged_container_model),
                    ),
            )
            .order_by(centrifuge_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_centrifuged_with_and_centrifuged_with(
        procedure_centrifuged_with: ::rosetta_uuid::Uuid,
        centrifuged_with: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures;
        Self::table()
            .filter(
                centrifuge_procedures::procedure_centrifuged_with
                    .eq(procedure_centrifuged_with)
                    .and(centrifuge_procedures::centrifuged_with.eq(centrifuged_with)),
            )
            .order_by(centrifuge_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_centrifuged_with_and_centrifuged_with_model(
        procedure_centrifuged_with: ::rosetta_uuid::Uuid,
        centrifuged_with_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures;
        Self::table()
            .filter(
                centrifuge_procedures::procedure_centrifuged_with
                    .eq(procedure_centrifuged_with)
                    .and(centrifuge_procedures::centrifuged_with_model.eq(centrifuged_with_model)),
            )
            .order_by(centrifuge_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_centrifuged_with<C>(
        procedure_centrifuged_with: ::rosetta_uuid::Uuid,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure_centrifuged_with as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure_centrifuged_with as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure_centrifuged_with as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures;
        Self::table()
            .filter(
                centrifuge_procedures::procedure_centrifuged_with.eq(procedure_centrifuged_with),
            )
            .order_by(centrifuge_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_centrifuged_with_and_procedure_template_centrifuged_with_model(
        procedure_centrifuged_with: ::rosetta_uuid::Uuid,
        procedure_template_centrifuged_with_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures;
        Self::table()
            .filter(
                centrifuge_procedures::procedure_centrifuged_with
                    .eq(procedure_centrifuged_with)
                    .and(
                        centrifuge_procedures::procedure_template_centrifuged_with_model
                            .eq(procedure_template_centrifuged_with_model),
                    ),
            )
            .order_by(centrifuge_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_centrifuged_container_model<C>(
        procedure_template_centrifuged_container_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure_template_centrifuged_container_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure_template_centrifuged_container_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure_template_centrifuged_container_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures;
        Self::table()
            .filter(
                centrifuge_procedures::procedure_template_centrifuged_container_model
                    .eq(procedure_template_centrifuged_container_model),
            )
            .order_by(centrifuge_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_centrifuged_with_model<C>(
        procedure_template_centrifuged_with_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure_template_centrifuged_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure_template_centrifuged_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure_template_centrifuged_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures;
        Self::table()
            .filter(
                centrifuge_procedures::procedure_template_centrifuged_with_model
                    .eq(procedure_template_centrifuged_with_model),
            )
            .order_by(centrifuge_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template<C>(
        procedure_template: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures;
        Self::table()
            .filter(centrifuge_procedures::procedure_template.eq(procedure_template))
            .order_by(centrifuge_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_centrifuged_with_model(
        procedure_template: i32,
        procedure_template_centrifuged_with_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures;
        Self::table()
            .filter(
                centrifuge_procedures::procedure_template.eq(procedure_template).and(
                    centrifuge_procedures::procedure_template_centrifuged_with_model
                        .eq(procedure_template_centrifuged_with_model),
                ),
            )
            .order_by(centrifuge_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_centrifuged_container_model(
        procedure_template: i32,
        procedure_template_centrifuged_container_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::centrifuge_procedures::centrifuge_procedures;
        Self::table()
            .filter(
                centrifuge_procedures::procedure_template.eq(procedure_template).and(
                    centrifuge_procedures::procedure_template_centrifuged_container_model
                        .eq(procedure_template_centrifuged_container_model),
                ),
            )
            .order_by(centrifuge_procedures::procedure.asc())
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
            centrifuge_procedures::centrifuge_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(centrifuge_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::parent_procedure.eq(parent_procedure))
            .order_by(centrifuge_procedures::procedure.asc())
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
            centrifuge_procedures::centrifuge_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(centrifuge_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::parent_procedure_template.eq(parent_procedure_template))
            .order_by(centrifuge_procedures::procedure.asc())
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
            centrifuge_procedures::centrifuge_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(centrifuge_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::predecessor_procedure.eq(predecessor_procedure))
            .order_by(centrifuge_procedures::procedure.asc())
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
            centrifuge_procedures::centrifuge_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(centrifuge_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::predecessor_procedure_template.eq(predecessor_procedure_template))
            .order_by(centrifuge_procedures::procedure.asc())
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
            centrifuge_procedures::centrifuge_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(centrifuge_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::created_by.eq(created_by))
            .order_by(centrifuge_procedures::procedure.asc())
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
            centrifuge_procedures::centrifuge_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(centrifuge_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::updated_by.eq(updated_by))
            .order_by(centrifuge_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<CentrifugeProcedure> for CentrifugeProcedure {
    fn as_ref(&self) -> &CentrifugeProcedure {
        self
    }
}
