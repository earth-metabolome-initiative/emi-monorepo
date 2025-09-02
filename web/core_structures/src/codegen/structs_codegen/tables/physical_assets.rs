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
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::physical_assets::physical_assets
)]
pub struct PhysicalAsset {
    pub id: ::rosetta_uuid::Uuid,
    pub model_id: i32,
}
impl web_common_traits::prelude::TableName for PhysicalAsset {
    const TABLE_NAME: &'static str = "physical_assets";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::assets::Asset,
    > for PhysicalAsset
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for PhysicalAsset {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl PhysicalAsset {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::assets::Asset,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::assets::Asset: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::assets::Asset as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::assets::Asset as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::assets::Asset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::assets::Asset as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::assets::Asset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::assets::Asset as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::assets::Asset,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::assets::Asset::table(),
                self.id,
            ),
            conn,
        )
    }
    pub fn model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel::table(),
                self.model_id,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_model_id(
        model_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
        Self::table()
            .filter(physical_assets::model_id.eq(model_id))
            .order_by(physical_assets::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_id_and_model_id(
        id: &::rosetta_uuid::Uuid,
        model_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
        Self::table()
            .filter(physical_assets::id.eq(id).and(physical_assets::model_id.eq(model_id)))
            .order_by(physical_assets::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_model_id_and_id(
        model_id: &i32,
        id: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl,
            SelectableHelper, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            assets::assets, physical_assets::physical_assets,
        };
        Self::table()
            .inner_join(assets::table.on(physical_assets::id.eq(assets::id)))
            .filter(assets::model_id.eq(model_id).and(assets::id.eq(id)))
            .order_by(physical_assets::id.asc())
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
            assets::assets, physical_assets::physical_assets,
        };
        Self::table()
            .inner_join(assets::table.on(physical_assets::id.eq(assets::id)))
            .filter(assets::most_concrete_table.eq(most_concrete_table))
            .order_by(physical_assets::id.asc())
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
            assets::assets, physical_assets::physical_assets,
        };
        Self::table()
            .inner_join(assets::table.on(physical_assets::id.eq(assets::id)))
            .filter(assets::description.eq(description))
            .order_by(physical_assets::id.asc())
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
            assets::assets, physical_assets::physical_assets,
        };
        Self::table()
            .inner_join(assets::table.on(physical_assets::id.eq(assets::id)))
            .filter(assets::created_by.eq(created_by))
            .order_by(physical_assets::id.asc())
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
            assets::assets, physical_assets::physical_assets,
        };
        Self::table()
            .inner_join(assets::table.on(physical_assets::id.eq(assets::id)))
            .filter(assets::created_at.eq(created_at))
            .order_by(physical_assets::id.asc())
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
            assets::assets, physical_assets::physical_assets,
        };
        Self::table()
            .inner_join(assets::table.on(physical_assets::id.eq(assets::id)))
            .filter(assets::updated_by.eq(updated_by))
            .order_by(physical_assets::id.asc())
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
            assets::assets, physical_assets::physical_assets,
        };
        Self::table()
            .inner_join(assets::table.on(physical_assets::id.eq(assets::id)))
            .filter(assets::updated_at.eq(updated_at))
            .order_by(physical_assets::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<PhysicalAsset> for PhysicalAsset {
    fn as_ref(&self) -> &PhysicalAsset {
        self
    }
}
