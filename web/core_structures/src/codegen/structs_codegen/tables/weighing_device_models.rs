#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Insertable, diesel::Queryable, diesel::Identifiable)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::weighing_device_models::weighing_device_models
)]
pub struct WeighingDeviceModel {
    pub id: i32,
}
impl web_common_traits::prelude::TableName for WeighingDeviceModel {
    const TABLE_NAME: &'static str = "weighing_device_models";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
    > for WeighingDeviceModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
    > for WeighingDeviceModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel,
    > for WeighingDeviceModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl diesel::Identifiable for WeighingDeviceModel {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl WeighingDeviceModel {
    pub fn id<C: diesel::connection::LoadConnection>(
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
                self.id,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_model(
        parent_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            physical_asset_models::physical_asset_models,
            weighing_device_models::weighing_device_models,
        };
        Self::table()
            .inner_join(
                physical_asset_models::table
                    .on(weighing_device_models::id.eq(physical_asset_models::id)),
            )
            .filter(physical_asset_models::parent_model.eq(parent_model))
            .order_by(weighing_device_models::id.asc())
            .select(Self::as_select())
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
            asset_models::asset_models, weighing_device_models::weighing_device_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(weighing_device_models::id.eq(asset_models::id)))
            .filter(asset_models::name.eq(name))
            .order_by(weighing_device_models::id.asc())
            .select(Self::as_select())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_model_and_id(
        parent_model: &i32,
        id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl,
            SelectableHelper, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            asset_models::asset_models, weighing_device_models::weighing_device_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(weighing_device_models::id.eq(asset_models::id)))
            .filter(asset_models::parent_model.eq(parent_model).and(asset_models::id.eq(id)))
            .order_by(weighing_device_models::id.asc())
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
            asset_models::asset_models, weighing_device_models::weighing_device_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(weighing_device_models::id.eq(asset_models::id)))
            .filter(asset_models::most_concrete_table.eq(most_concrete_table))
            .order_by(weighing_device_models::id.asc())
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
            asset_models::asset_models, weighing_device_models::weighing_device_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(weighing_device_models::id.eq(asset_models::id)))
            .filter(asset_models::description.eq(description))
            .order_by(weighing_device_models::id.asc())
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
            asset_models::asset_models, weighing_device_models::weighing_device_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(weighing_device_models::id.eq(asset_models::id)))
            .filter(asset_models::created_by.eq(created_by))
            .order_by(weighing_device_models::id.asc())
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
            asset_models::asset_models, weighing_device_models::weighing_device_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(weighing_device_models::id.eq(asset_models::id)))
            .filter(asset_models::created_at.eq(created_at))
            .order_by(weighing_device_models::id.asc())
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
            asset_models::asset_models, weighing_device_models::weighing_device_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(weighing_device_models::id.eq(asset_models::id)))
            .filter(asset_models::updated_by.eq(updated_by))
            .order_by(weighing_device_models::id.asc())
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
            asset_models::asset_models, weighing_device_models::weighing_device_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(weighing_device_models::id.eq(asset_models::id)))
            .filter(asset_models::updated_at.eq(updated_at))
            .order_by(weighing_device_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<WeighingDeviceModel> for WeighingDeviceModel {
    fn as_ref(&self) -> &WeighingDeviceModel {
        self
    }
}
