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
#[diesel(belongs_to(crate::PhysicalAsset, foreign_key = stored_asset))]
#[diesel(belongs_to(crate::PhysicalAssetModel, foreign_key = stored_asset_model))]
#[diesel(belongs_to(crate::Container, foreign_key = stored_into))]
#[diesel(belongs_to(crate::ContainerModel, foreign_key = stored_into_model))]
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
impl web_common_traits::prelude::ExtensionTable<crate::Procedure> for StorageProcedure where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>
{
}
impl web_common_traits::prelude::ExtensionTable<crate::StorageProcedure> for StorageProcedure where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>
{
}
impl diesel::Identifiable for StorageProcedure {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.procedure
    }
}
impl StorageProcedure {
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
    pub fn storage_procedures_procedure_procedure_template_fkey(
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
    pub fn procedure_stored_asset<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error>
    where
        crate::ProcedureAsset: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureAsset::read(self.procedure_stored_asset, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_stored_asset_procedure_templa_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_stored_asset)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_stored_asset_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_stored_asset_stored_asset_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_stored_asset)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.stored_asset),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_stored_asset_stored_asset_mod_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_stored_asset)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.stored_asset_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    pub fn procedure_stored_into<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error>
    where
        crate::ProcedureAsset: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureAsset::read(self.procedure_stored_into, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_stored_into_procedure_templat_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_stored_into)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_stored_into_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_stored_into_stored_into_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_stored_into)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.stored_into),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_stored_into_stored_into_model_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_stored_into)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.stored_into_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::StorageProcedureTemplate, diesel::result::Error>
    where
        crate::StorageProcedureTemplate: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::StorageProcedureTemplate::read(self.procedure_template, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_template_procedure_template_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::StorageProcedureTemplate, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::StorageProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::storage_procedure_templates::storage_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::storage_procedure_templates::storage_procedure_templates::dsl::procedure_template_stored_into_model
                            .eq(&self.procedure_template_stored_into_model),
                    ),
            )
            .first::<crate::StorageProcedureTemplate>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_template_procedure_template_s_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::StorageProcedureTemplate, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::StorageProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::storage_procedure_templates::storage_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::storage_procedure_templates::storage_procedure_templates::dsl::procedure_template_stored_asset_model
                            .eq(&self.procedure_template_stored_asset_model),
                    ),
            )
            .first::<crate::StorageProcedureTemplate>(conn)
    }
    pub fn procedure_template_stored_asset_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(self.procedure_template_stored_asset_model, conn)
    }
    pub fn procedure_template_stored_into_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(self.procedure_template_stored_into_model, conn)
    }
    pub fn stored_asset<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::PhysicalAsset, diesel::result::Error>
    where
        crate::PhysicalAsset: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::PhysicalAsset::read(self.stored_asset, conn)
    }
    pub fn stored_asset_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::PhysicalAssetModel, diesel::result::Error>
    where
        crate::PhysicalAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::PhysicalAssetModel::read(self.stored_asset_model, conn)
    }
    pub fn stored_into<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::Container, diesel::result::Error>
    where
        crate::Container: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::Container::read(self.stored_into, conn)
    }
    pub fn stored_into_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ContainerModel, diesel::result::Error>
    where
        crate::ContainerModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ContainerModel::read(self.stored_into_model, conn)
    }
    pub fn storage_procedures_stored_into_model_stored_asset_model_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<crate::ContainerCompatibilityRule, diesel::result::Error>
    where
        crate::ContainerCompatibilityRule: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ContainerCompatibilityRule::read(
            (self.stored_into_model, self.stored_asset_model),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure(
        procedure: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures;
        Self::table()
            .filter(storage_procedures::procedure.eq(procedure))
            .order_by(storage_procedures::procedure.asc())
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

        use crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures;
        Self::table()
            .filter(
                storage_procedures::procedure
                    .eq(procedure)
                    .and(storage_procedures::procedure_template.eq(procedure_template)),
            )
            .order_by(storage_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_stored_asset(
        procedure_stored_asset: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures;
        Self::table()
            .filter(storage_procedures::procedure_stored_asset.eq(procedure_stored_asset))
            .order_by(storage_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_stored_asset_and_procedure_template_stored_asset_model(
        procedure_stored_asset: &::rosetta_uuid::Uuid,
        procedure_template_stored_asset_model: &i32,
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
        procedure_stored_asset: &::rosetta_uuid::Uuid,
        stored_asset: &::rosetta_uuid::Uuid,
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
        procedure_stored_asset: &::rosetta_uuid::Uuid,
        stored_asset_model: &i32,
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
    #[cfg(feature = "postgres")]
    pub fn from_procedure_stored_into(
        procedure_stored_into: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures;
        Self::table()
            .filter(storage_procedures::procedure_stored_into.eq(procedure_stored_into))
            .order_by(storage_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_stored_into_and_procedure_template_stored_into_model(
        procedure_stored_into: &::rosetta_uuid::Uuid,
        procedure_template_stored_into_model: &i32,
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
        procedure_stored_into: &::rosetta_uuid::Uuid,
        stored_into: &::rosetta_uuid::Uuid,
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
        procedure_stored_into: &::rosetta_uuid::Uuid,
        stored_into_model: &i32,
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
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template(
        procedure_template: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures;
        Self::table()
            .filter(storage_procedures::procedure_template.eq(procedure_template))
            .order_by(storage_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_stored_into_model(
        procedure_template: &i32,
        procedure_template_stored_into_model: &i32,
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
        procedure_template: &i32,
        procedure_template_stored_asset_model: &i32,
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
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_stored_asset_model(
        procedure_template_stored_asset_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
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
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_stored_into_model(
        procedure_template_stored_into_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
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
        stored_into_model: &i32,
        stored_asset_model: &i32,
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
            procedures::procedures, storage_procedures::storage_procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(storage_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(
                procedures::procedure_template
                    .eq(procedure_template)
                    .and(procedures::procedure.eq(procedure)),
            )
            .order_by(storage_procedures::procedure.asc())
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
        parent_procedure_template: &i32,
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
    pub fn from_most_concrete_table(
        most_concrete_table: &str,
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
            .filter(procedures::most_concrete_table.eq(most_concrete_table))
            .order_by(storage_procedures::procedure.asc())
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
    pub fn from_created_at(
        created_at: &::rosetta_timestamp::TimestampUTC,
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
            .filter(procedures::created_at.eq(created_at))
            .order_by(storage_procedures::procedure.asc())
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
            procedures::procedures, storage_procedures::storage_procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(storage_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::updated_at.eq(updated_at))
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
