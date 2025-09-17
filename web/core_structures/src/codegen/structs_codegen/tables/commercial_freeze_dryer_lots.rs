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
        crate::codegen::structs_codegen::tables::commercial_freeze_dryer_models::CommercialFreezeDryerModel,
        foreign_key = product_model
    )
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::commercial_freeze_dryer_lots::commercial_freeze_dryer_lots
)]
pub struct CommercialFreezeDryerLot {
    pub id: i32,
    pub product_model: i32,
}
impl web_common_traits::prelude::TableName for CommercialFreezeDryerLot {
    const TABLE_NAME: &'static str = "commercial_freeze_dryer_lots";
}
impl<'a> From<&'a CommercialFreezeDryerLot>
for web_common_traits::database::IdOrBuilder<
    i32,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLotBuilder,
> {
    fn from(value: &'a CommercialFreezeDryerLot) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.id)
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
    > for CommercialFreezeDryerLot
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
    > for CommercialFreezeDryerLot
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel,
    > for CommercialFreezeDryerLot
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
    > for CommercialFreezeDryerLot
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl web_common_traits::prelude::ExtensionTable<
    crate::codegen::structs_codegen::tables::commercial_freeze_dryer_lots::CommercialFreezeDryerLot,
> for CommercialFreezeDryerLot
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{}
impl diesel::Identifiable for CommercialFreezeDryerLot {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for CommercialFreezeDryerLot {
    type PrimaryKey = i32;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl CommercialFreezeDryerLot {
    pub fn product_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::commercial_freeze_dryer_models::CommercialFreezeDryerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::commercial_freeze_dryer_models::CommercialFreezeDryerModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::commercial_freeze_dryer_models::CommercialFreezeDryerModel::read(
            self.product_model,
            conn,
        )
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
            commercial_freeze_dryer_lots::commercial_freeze_dryer_lots,
            commercial_product_lots::commercial_product_lots,
        };
        Self::table()
            .inner_join(
                commercial_product_lots::table
                    .on(commercial_freeze_dryer_lots::id.eq(commercial_product_lots::id)),
            )
            .filter(
                commercial_product_lots::lot
                    .eq(lot)
                    .and(commercial_product_lots::product_model.eq(product_model)),
            )
            .order_by(commercial_freeze_dryer_lots::id.asc())
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
            commercial_freeze_dryer_lots::commercial_freeze_dryer_lots,
            commercial_product_lots::commercial_product_lots,
        };
        Self::table()
            .inner_join(
                commercial_product_lots::table
                    .on(commercial_freeze_dryer_lots::id.eq(commercial_product_lots::id)),
            )
            .filter(commercial_product_lots::lot.eq(lot))
            .order_by(commercial_freeze_dryer_lots::id.asc())
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
            commercial_freeze_dryer_lots::commercial_freeze_dryer_lots,
            commercial_product_lots::commercial_product_lots,
        };
        Self::table()
            .inner_join(
                commercial_product_lots::table
                    .on(commercial_freeze_dryer_lots::id.eq(commercial_product_lots::id)),
            )
            .filter(commercial_product_lots::product_model.eq(product_model))
            .order_by(commercial_freeze_dryer_lots::id.asc())
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
            commercial_freeze_dryer_lots::commercial_freeze_dryer_lots,
            physical_asset_models::physical_asset_models,
        };
        Self::table()
            .inner_join(
                physical_asset_models::table
                    .on(commercial_freeze_dryer_lots::id.eq(physical_asset_models::id)),
            )
            .filter(physical_asset_models::parent_model.eq(parent_model))
            .order_by(commercial_freeze_dryer_lots::id.asc())
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
            asset_models::asset_models, commercial_freeze_dryer_lots::commercial_freeze_dryer_lots,
        };
        Self::table()
            .inner_join(
                asset_models::table.on(commercial_freeze_dryer_lots::id.eq(asset_models::id)),
            )
            .filter(asset_models::name.eq(name))
            .order_by(commercial_freeze_dryer_lots::id.asc())
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
            asset_models::asset_models, commercial_freeze_dryer_lots::commercial_freeze_dryer_lots,
        };
        Self::table()
            .inner_join(
                asset_models::table.on(commercial_freeze_dryer_lots::id.eq(asset_models::id)),
            )
            .filter(asset_models::description.eq(description))
            .order_by(commercial_freeze_dryer_lots::id.asc())
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
            asset_models::asset_models, commercial_freeze_dryer_lots::commercial_freeze_dryer_lots,
        };
        Self::table()
            .inner_join(
                asset_models::table.on(commercial_freeze_dryer_lots::id.eq(asset_models::id)),
            )
            .filter(asset_models::created_by.eq(created_by))
            .order_by(commercial_freeze_dryer_lots::id.asc())
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
            asset_models::asset_models, commercial_freeze_dryer_lots::commercial_freeze_dryer_lots,
        };
        Self::table()
            .inner_join(
                asset_models::table.on(commercial_freeze_dryer_lots::id.eq(asset_models::id)),
            )
            .filter(asset_models::updated_by.eq(updated_by))
            .order_by(commercial_freeze_dryer_lots::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<CommercialFreezeDryerLot> for CommercialFreezeDryerLot {
    fn as_ref(&self) -> &CommercialFreezeDryerLot {
        self
    }
}
