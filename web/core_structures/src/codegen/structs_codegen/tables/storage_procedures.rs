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
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
        foreign_key = stored_asset
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
        foreign_key = stored_asset_model
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::containers::Container,
        foreign_key = stored_into
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::container_models::ContainerModel,
        foreign_key = stored_into_model
    )
)]
#[diesel(primary_key(procedure))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures
)]
pub struct StorageProcedure {
    pub procedure: ::rosetta_uuid::Uuid,
    pub procedure_template: i32,
    pub stored_asset: ::rosetta_uuid::Uuid,
    pub stored_asset_model: i32,
    pub procedure_template_stored_asset_model: i32,
    pub procedure_stored_asset: ::rosetta_uuid::Uuid,
    pub stored_into: ::rosetta_uuid::Uuid,
    pub stored_into_model: i32,
    pub procedure_template_stored_into_model: i32,
    pub procedure_stored_into: ::rosetta_uuid::Uuid,
}
impl web_common_traits::prelude::TableName for StorageProcedure {
    const TABLE_NAME: &'static str = "storage_procedures";
}
impl<'a> From<&'a StorageProcedure>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureBuilder,
    >
{
    fn from(value: &'a StorageProcedure) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.procedure)
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    > for StorageProcedure
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure,
    > for StorageProcedure
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for StorageProcedure {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.procedure
    }
}
impl web_common_traits::database::PrimaryKeyLike for StorageProcedure {
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.procedure
    }
}
impl StorageProcedure {
    pub fn procedure_stored_asset<C: diesel::connection::LoadConnection>(
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
            self.procedure_stored_asset,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_stored_asset_procedure_templa_fkey(
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
                    .eq(&self.procedure_stored_asset)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_stored_asset_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_stored_asset_stored_asset_fkey(
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
                    .eq(&self.procedure_stored_asset)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.stored_asset),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_stored_asset_stored_asset_mod_fkey(
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
                    .eq(&self.procedure_stored_asset)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.stored_asset_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn procedure_stored_into<C: diesel::connection::LoadConnection>(
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
            self.procedure_stored_into,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_stored_into_procedure_templat_fkey(
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
                    .eq(&self.procedure_stored_into)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_stored_into_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_stored_into_stored_into_fkey(
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
                    .eq(&self.procedure_stored_into)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.stored_into),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_stored_into_stored_into_model_fkey(
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
                    .eq(&self.procedure_stored_into)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.stored_into_model),
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
        crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate::read(
            self.procedure_template,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_template_procedure_template_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::storage_procedure_templates::storage_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::storage_procedure_templates::storage_procedure_templates::dsl::procedure_template_stored_into_model
                            .eq(&self.procedure_template_stored_into_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_template_procedure_template_s_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::storage_procedure_templates::storage_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::storage_procedure_templates::storage_procedure_templates::dsl::procedure_template_stored_asset_model
                            .eq(&self.procedure_template_stored_asset_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate,
            >(conn)
    }
    pub fn procedure_template_stored_asset_model<C: diesel::connection::LoadConnection>(
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
            self.procedure_template_stored_asset_model,
            conn,
        )
    }
    pub fn procedure_template_stored_into_model<C: diesel::connection::LoadConnection>(
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
            self.procedure_template_stored_into_model,
            conn,
        )
    }
    pub fn stored_asset<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset::read(
            self.stored_asset,
            conn,
        )
    }
    pub fn stored_asset_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel::read(
            self.stored_asset_model,
            conn,
        )
    }
    pub fn stored_into<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::containers::Container, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::containers::Container:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::containers::Container::read(self.stored_into, conn)
    }
    pub fn stored_into_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::container_models::ContainerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::container_models::ContainerModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::container_models::ContainerModel::read(
            self.stored_into_model,
            conn,
        )
    }
    pub fn storage_procedures_stored_into_model_stored_asset_model_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule::read(
            (self.stored_into_model, self.stored_asset_model),
            conn,
        )
    }
    pub fn from_procedure_stored_asset<C>(
        procedure_stored_asset: ::rosetta_uuid::Uuid,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure_stored_asset as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure_stored_asset as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure_stored_asset as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures;
        Self::table()
            .filter(storage_procedures::procedure_stored_asset.eq(procedure_stored_asset))
            .order_by(storage_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_stored_asset_and_procedure_template_stored_asset_model(
        procedure_stored_asset: ::rosetta_uuid::Uuid,
        procedure_template_stored_asset_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures;
        Self::table()
            .filter(
                storage_procedures::procedure_stored_asset.eq(procedure_stored_asset).and(
                    storage_procedures::procedure_template_stored_asset_model
                        .eq(procedure_template_stored_asset_model),
                ),
            )
            .order_by(storage_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_stored_asset_and_stored_asset(
        procedure_stored_asset: ::rosetta_uuid::Uuid,
        stored_asset: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures;
        Self::table()
            .filter(
                storage_procedures::procedure_stored_asset
                    .eq(procedure_stored_asset)
                    .and(storage_procedures::stored_asset.eq(stored_asset)),
            )
            .order_by(storage_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_stored_asset_and_stored_asset_model(
        procedure_stored_asset: ::rosetta_uuid::Uuid,
        stored_asset_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures;
        Self::table()
            .filter(
                storage_procedures::procedure_stored_asset
                    .eq(procedure_stored_asset)
                    .and(storage_procedures::stored_asset_model.eq(stored_asset_model)),
            )
            .order_by(storage_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_stored_into<C>(
        procedure_stored_into: ::rosetta_uuid::Uuid,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure_stored_into as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure_stored_into as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure_stored_into as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures;
        Self::table()
            .filter(storage_procedures::procedure_stored_into.eq(procedure_stored_into))
            .order_by(storage_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_stored_into_and_procedure_template_stored_into_model(
        procedure_stored_into: ::rosetta_uuid::Uuid,
        procedure_template_stored_into_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures;
        Self::table()
            .filter(
                storage_procedures::procedure_stored_into.eq(procedure_stored_into).and(
                    storage_procedures::procedure_template_stored_into_model
                        .eq(procedure_template_stored_into_model),
                ),
            )
            .order_by(storage_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_stored_into_and_stored_into(
        procedure_stored_into: ::rosetta_uuid::Uuid,
        stored_into: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures;
        Self::table()
            .filter(
                storage_procedures::procedure_stored_into
                    .eq(procedure_stored_into)
                    .and(storage_procedures::stored_into.eq(stored_into)),
            )
            .order_by(storage_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_stored_into_and_stored_into_model(
        procedure_stored_into: ::rosetta_uuid::Uuid,
        stored_into_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures;
        Self::table()
            .filter(
                storage_procedures::procedure_stored_into
                    .eq(procedure_stored_into)
                    .and(storage_procedures::stored_into_model.eq(stored_into_model)),
            )
            .order_by(storage_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template<C>(
        procedure_template: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures;
        Self::table()
            .filter(storage_procedures::procedure_template.eq(procedure_template))
            .order_by(storage_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_stored_into_model(
        procedure_template: i32,
        procedure_template_stored_into_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures;
        Self::table()
            .filter(
                storage_procedures::procedure_template.eq(procedure_template).and(
                    storage_procedures::procedure_template_stored_into_model
                        .eq(procedure_template_stored_into_model),
                ),
            )
            .order_by(storage_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_stored_asset_model(
        procedure_template: i32,
        procedure_template_stored_asset_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures;
        Self::table()
            .filter(
                storage_procedures::procedure_template.eq(procedure_template).and(
                    storage_procedures::procedure_template_stored_asset_model
                        .eq(procedure_template_stored_asset_model),
                ),
            )
            .order_by(storage_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_stored_asset_model<C>(
        procedure_template_stored_asset_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure_template_stored_asset_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure_template_stored_asset_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure_template_stored_asset_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures;
        Self::table()
            .filter(
                storage_procedures::procedure_template_stored_asset_model
                    .eq(procedure_template_stored_asset_model),
            )
            .order_by(storage_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_stored_into_model<C>(
        procedure_template_stored_into_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure_template_stored_into_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure_template_stored_into_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure_template_stored_into_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures;
        Self::table()
            .filter(
                storage_procedures::procedure_template_stored_into_model
                    .eq(procedure_template_stored_into_model),
            )
            .order_by(storage_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_stored_into_model_and_stored_asset_model(
        stored_into_model: i32,
        stored_asset_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures;
        Self::table()
            .filter(
                storage_procedures::stored_into_model
                    .eq(stored_into_model)
                    .and(storage_procedures::stored_asset_model.eq(stored_asset_model)),
            )
            .order_by(storage_procedures::procedure.asc())
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
            procedures::procedures, storage_procedures::storage_procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(storage_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::parent_procedure.eq(parent_procedure))
            .order_by(storage_procedures::procedure.asc())
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
            procedures::procedures, storage_procedures::storage_procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(storage_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::parent_procedure_template.eq(parent_procedure_template))
            .order_by(storage_procedures::procedure.asc())
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
            procedures::procedures, storage_procedures::storage_procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(storage_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::predecessor_procedure.eq(predecessor_procedure))
            .order_by(storage_procedures::procedure.asc())
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
            procedures::procedures, storage_procedures::storage_procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(storage_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::predecessor_procedure_template.eq(predecessor_procedure_template))
            .order_by(storage_procedures::procedure.asc())
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
            procedures::procedures, storage_procedures::storage_procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(storage_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::created_by.eq(created_by))
            .order_by(storage_procedures::procedure.asc())
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
            procedures::procedures, storage_procedures::storage_procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(storage_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::updated_by.eq(updated_by))
            .order_by(storage_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<StorageProcedure> for StorageProcedure {
    fn as_ref(&self) -> &StorageProcedure {
        self
    }
}
