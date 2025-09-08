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
#[diesel(belongs_to(crate::CommercialFreezerModel, foreign_key = product_model))]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::commercial_freezer_lots::commercial_freezer_lots
)]
pub struct CommercialFreezerLot {
    pub id: i32,
    pub product_model: i32,
}
impl web_common_traits::prelude::TableName for CommercialFreezerLot {
    const TABLE_NAME: &'static str = "commercial_freezer_lots";
}
impl<'a> From<&'a CommercialFreezerLot>
    for web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerLotBuilder,
    >
{
    fn from(value: &'a CommercialFreezerLot) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.id)
    }
}
impl web_common_traits::prelude::ExtensionTable<crate::AssetModel> for CommercialFreezerLot where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>
{
}
impl web_common_traits::prelude::ExtensionTable<crate::CommercialProductLot>
    for CommercialFreezerLot
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl web_common_traits::prelude::ExtensionTable<crate::FreezerModel> for CommercialFreezerLot where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>
{
}
impl web_common_traits::prelude::ExtensionTable<crate::PhysicalAssetModel> for CommercialFreezerLot where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>
{
}
impl web_common_traits::prelude::ExtensionTable<crate::CommercialFreezerLot>
    for CommercialFreezerLot
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl diesel::Identifiable for CommercialFreezerLot {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl CommercialFreezerLot {
    pub fn commercial_freezer_lots_id_fkey<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::CommercialProductLot, diesel::result::Error>
    where
        crate::CommercialProductLot: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::CommercialProductLot::read(self.id, conn)
    }
    pub fn commercial_freezer_lots_id_fkey1<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::FreezerModel, diesel::result::Error>
    where
        crate::FreezerModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::FreezerModel::read(self.id, conn)
    }
    pub fn product_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::CommercialFreezerModel, diesel::result::Error>
    where
        crate::CommercialFreezerModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::CommercialFreezerModel::read(self.product_model, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn commercial_freezer_lots_id_product_model_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::AssetModel, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::AssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::asset_models::asset_models::dsl::id
                    .eq(&self.id)
                    .and(
                        crate::codegen::diesel_codegen::tables::asset_models::asset_models::dsl::parent_model
                            .eq(&self.product_model),
                    ),
            )
            .first::<crate::AssetModel>(conn)
    }
    pub fn from_id<C>(id: i32, conn: &mut C) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::commercial_freezer_lots::commercial_freezer_lots::id as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::commercial_freezer_lots::commercial_freezer_lots::id as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::commercial_freezer_lots::commercial_freezer_lots::id,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::commercial_freezer_lots::commercial_freezer_lots::id as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::commercial_freezer_lots::commercial_freezer_lots::id,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::commercial_freezer_lots::commercial_freezer_lots;
        Self::table()
            .filter(commercial_freezer_lots::id.eq(id))
            .order_by(commercial_freezer_lots::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_id_and_product_model(
        id: i32,
        product_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::commercial_freezer_lots::commercial_freezer_lots;
        Self::table()
            .filter(
                commercial_freezer_lots::id
                    .eq(id)
                    .and(commercial_freezer_lots::product_model.eq(product_model)),
            )
            .order_by(commercial_freezer_lots::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_lot_and_product_model(
        lot: &str,
        product_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl,
            SelectableHelper, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            commercial_freezer_lots::commercial_freezer_lots,
            commercial_product_lots::commercial_product_lots,
        };
        Self::table()
            .inner_join(
                commercial_product_lots::table
                    .on(commercial_freezer_lots::id.eq(commercial_product_lots::id)),
            )
            .filter(
                commercial_product_lots::lot
                    .eq(lot)
                    .and(commercial_product_lots::product_model.eq(product_model)),
            )
            .order_by(commercial_freezer_lots::id.asc())
            .select(Self::as_select())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_lot(
        lot: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            commercial_freezer_lots::commercial_freezer_lots,
            commercial_product_lots::commercial_product_lots,
        };
        Self::table()
            .inner_join(
                commercial_product_lots::table
                    .on(commercial_freezer_lots::id.eq(commercial_product_lots::id)),
            )
            .filter(commercial_product_lots::lot.eq(lot))
            .order_by(commercial_freezer_lots::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_product_model(
        product_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            commercial_freezer_lots::commercial_freezer_lots,
            commercial_product_lots::commercial_product_lots,
        };
        Self::table()
            .inner_join(
                commercial_product_lots::table
                    .on(commercial_freezer_lots::id.eq(commercial_product_lots::id)),
            )
            .filter(commercial_product_lots::product_model.eq(product_model))
            .order_by(commercial_freezer_lots::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_model(
        parent_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            commercial_freezer_lots::commercial_freezer_lots,
            physical_asset_models::physical_asset_models,
        };
        Self::table()
            .inner_join(
                physical_asset_models::table
                    .on(commercial_freezer_lots::id.eq(physical_asset_models::id)),
            )
            .filter(physical_asset_models::parent_model.eq(parent_model))
            .order_by(commercial_freezer_lots::id.asc())
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
            asset_models::asset_models, commercial_freezer_lots::commercial_freezer_lots,
        };
        Self::table()
            .inner_join(asset_models::table.on(commercial_freezer_lots::id.eq(asset_models::id)))
            .filter(asset_models::name.eq(name))
            .order_by(commercial_freezer_lots::id.asc())
            .select(Self::as_select())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_model_and_id(
        parent_model: i32,
        id: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl,
            SelectableHelper, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            asset_models::asset_models, commercial_freezer_lots::commercial_freezer_lots,
        };
        Self::table()
            .inner_join(asset_models::table.on(commercial_freezer_lots::id.eq(asset_models::id)))
            .filter(asset_models::parent_model.eq(parent_model).and(asset_models::id.eq(id)))
            .order_by(commercial_freezer_lots::id.asc())
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
            asset_models::asset_models, commercial_freezer_lots::commercial_freezer_lots,
        };
        Self::table()
            .inner_join(asset_models::table.on(commercial_freezer_lots::id.eq(asset_models::id)))
            .filter(asset_models::most_concrete_table.eq(most_concrete_table))
            .order_by(commercial_freezer_lots::id.asc())
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
            asset_models::asset_models, commercial_freezer_lots::commercial_freezer_lots,
        };
        Self::table()
            .inner_join(asset_models::table.on(commercial_freezer_lots::id.eq(asset_models::id)))
            .filter(asset_models::description.eq(description))
            .order_by(commercial_freezer_lots::id.asc())
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
            asset_models::asset_models, commercial_freezer_lots::commercial_freezer_lots,
        };
        Self::table()
            .inner_join(asset_models::table.on(commercial_freezer_lots::id.eq(asset_models::id)))
            .filter(asset_models::created_by.eq(created_by))
            .order_by(commercial_freezer_lots::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_at(
        created_at: ::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            asset_models::asset_models, commercial_freezer_lots::commercial_freezer_lots,
        };
        Self::table()
            .inner_join(asset_models::table.on(commercial_freezer_lots::id.eq(asset_models::id)))
            .filter(asset_models::created_at.eq(created_at))
            .order_by(commercial_freezer_lots::id.asc())
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
            asset_models::asset_models, commercial_freezer_lots::commercial_freezer_lots,
        };
        Self::table()
            .inner_join(asset_models::table.on(commercial_freezer_lots::id.eq(asset_models::id)))
            .filter(asset_models::updated_by.eq(updated_by))
            .order_by(commercial_freezer_lots::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_updated_at(
        updated_at: ::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            asset_models::asset_models, commercial_freezer_lots::commercial_freezer_lots,
        };
        Self::table()
            .inner_join(asset_models::table.on(commercial_freezer_lots::id.eq(asset_models::id)))
            .filter(asset_models::updated_at.eq(updated_at))
            .order_by(commercial_freezer_lots::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<CommercialFreezerLot> for CommercialFreezerLot {
    fn as_ref(&self) -> &CommercialFreezerLot {
        self
    }
}
