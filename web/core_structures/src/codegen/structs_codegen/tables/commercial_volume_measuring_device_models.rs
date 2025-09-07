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
    table_name = crate::codegen::diesel_codegen::tables::commercial_volume_measuring_device_models::commercial_volume_measuring_device_models
)]
pub struct CommercialVolumeMeasuringDeviceModel {
    pub id: i32,
    pub volume_measuring_device_model: i32,
}
impl web_common_traits::prelude::TableName for CommercialVolumeMeasuringDeviceModel {
    const TABLE_NAME: &'static str = "commercial_volume_measuring_device_models";
}
impl web_common_traits::prelude::ExtensionTable<crate::AssetModel>
    for CommercialVolumeMeasuringDeviceModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl web_common_traits::prelude::ExtensionTable<crate::CommercialProduct>
    for CommercialVolumeMeasuringDeviceModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl web_common_traits::prelude::ExtensionTable<crate::PhysicalAssetModel>
    for CommercialVolumeMeasuringDeviceModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl web_common_traits::prelude::ExtensionTable<crate::VolumeMeasuringDeviceModel>
    for CommercialVolumeMeasuringDeviceModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl web_common_traits::prelude::ExtensionTable<crate::CommercialVolumeMeasuringDeviceModel>
    for CommercialVolumeMeasuringDeviceModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl diesel::Identifiable for CommercialVolumeMeasuringDeviceModel {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl CommercialVolumeMeasuringDeviceModel {
    pub fn volume_measuring_device_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::VolumeMeasuringDeviceModel, diesel::result::Error>
    where
        crate::VolumeMeasuringDeviceModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::VolumeMeasuringDeviceModel::read(self.volume_measuring_device_model, conn)
    }
    pub fn commercial_volume_measuring_device_models_id_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<crate::VolumeMeasuringDeviceModel, diesel::result::Error>
    where
        crate::VolumeMeasuringDeviceModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::VolumeMeasuringDeviceModel::read(self.id, conn)
    }
    pub fn commercial_volume_measuring_device_models_id_fkey1<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<crate::CommercialProduct, diesel::result::Error>
    where
        crate::CommercialProduct: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::CommercialProduct::read(self.id, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn commercial_volume_measuring_d_id_volume_measuring_device_m_fkey(
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
                            .eq(&self.volume_measuring_device_model),
                    ),
            )
            .first::<crate::AssetModel>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_volume_measuring_device_model(
        volume_measuring_device_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::commercial_volume_measuring_device_models::commercial_volume_measuring_device_models;
        Self::table()
            .filter(
                commercial_volume_measuring_device_models::volume_measuring_device_model
                    .eq(volume_measuring_device_model),
            )
            .order_by(commercial_volume_measuring_device_models::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_id(
        id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::commercial_volume_measuring_device_models::commercial_volume_measuring_device_models;
        Self::table()
            .filter(commercial_volume_measuring_device_models::id.eq(id))
            .order_by(commercial_volume_measuring_device_models::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_id_and_volume_measuring_device_model(
        id: &i32,
        volume_measuring_device_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::commercial_volume_measuring_device_models::commercial_volume_measuring_device_models;
        Self::table()
            .filter(
                commercial_volume_measuring_device_models::id.eq(id).and(
                    commercial_volume_measuring_device_models::volume_measuring_device_model
                        .eq(volume_measuring_device_model),
                ),
            )
            .order_by(commercial_volume_measuring_device_models::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_deprecation_date(
        deprecation_date: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            commercial_products::commercial_products,
            commercial_volume_measuring_device_models::commercial_volume_measuring_device_models,
        };
        Self::table()
            .inner_join(
                commercial_products::table
                    .on(commercial_volume_measuring_device_models::id.eq(commercial_products::id)),
            )
            .filter(commercial_products::deprecation_date.eq(deprecation_date))
            .order_by(commercial_volume_measuring_device_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_brand_id(
        brand_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            commercial_products::commercial_products,
            commercial_volume_measuring_device_models::commercial_volume_measuring_device_models,
        };
        Self::table()
            .inner_join(
                commercial_products::table
                    .on(commercial_volume_measuring_device_models::id.eq(commercial_products::id)),
            )
            .filter(commercial_products::brand_id.eq(brand_id))
            .order_by(commercial_volume_measuring_device_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
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
            commercial_volume_measuring_device_models::commercial_volume_measuring_device_models,
            physical_asset_models::physical_asset_models,
        };
        Self::table()
            .inner_join(
                physical_asset_models::table
                    .on(
                        commercial_volume_measuring_device_models::id
                            .eq(physical_asset_models::id),
                    ),
            )
            .filter(physical_asset_models::parent_model.eq(parent_model))
            .order_by(commercial_volume_measuring_device_models::id.asc())
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
            asset_models::asset_models,
            commercial_volume_measuring_device_models::commercial_volume_measuring_device_models,
        };
        Self::table()
            .inner_join(
                asset_models::table
                    .on(commercial_volume_measuring_device_models::id.eq(asset_models::id)),
            )
            .filter(asset_models::name.eq(name))
            .order_by(commercial_volume_measuring_device_models::id.asc())
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
            asset_models::asset_models,
            commercial_volume_measuring_device_models::commercial_volume_measuring_device_models,
        };
        Self::table()
            .inner_join(
                asset_models::table
                    .on(commercial_volume_measuring_device_models::id.eq(asset_models::id)),
            )
            .filter(asset_models::parent_model.eq(parent_model).and(asset_models::id.eq(id)))
            .order_by(commercial_volume_measuring_device_models::id.asc())
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
            asset_models::asset_models,
            commercial_volume_measuring_device_models::commercial_volume_measuring_device_models,
        };
        Self::table()
            .inner_join(
                asset_models::table
                    .on(commercial_volume_measuring_device_models::id.eq(asset_models::id)),
            )
            .filter(asset_models::most_concrete_table.eq(most_concrete_table))
            .order_by(commercial_volume_measuring_device_models::id.asc())
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
            asset_models::asset_models,
            commercial_volume_measuring_device_models::commercial_volume_measuring_device_models,
        };
        Self::table()
            .inner_join(
                asset_models::table
                    .on(commercial_volume_measuring_device_models::id.eq(asset_models::id)),
            )
            .filter(asset_models::description.eq(description))
            .order_by(commercial_volume_measuring_device_models::id.asc())
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
            asset_models::asset_models,
            commercial_volume_measuring_device_models::commercial_volume_measuring_device_models,
        };
        Self::table()
            .inner_join(
                asset_models::table
                    .on(commercial_volume_measuring_device_models::id.eq(asset_models::id)),
            )
            .filter(asset_models::created_by.eq(created_by))
            .order_by(commercial_volume_measuring_device_models::id.asc())
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
            asset_models::asset_models,
            commercial_volume_measuring_device_models::commercial_volume_measuring_device_models,
        };
        Self::table()
            .inner_join(
                asset_models::table
                    .on(commercial_volume_measuring_device_models::id.eq(asset_models::id)),
            )
            .filter(asset_models::created_at.eq(created_at))
            .order_by(commercial_volume_measuring_device_models::id.asc())
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
            asset_models::asset_models,
            commercial_volume_measuring_device_models::commercial_volume_measuring_device_models,
        };
        Self::table()
            .inner_join(
                asset_models::table
                    .on(commercial_volume_measuring_device_models::id.eq(asset_models::id)),
            )
            .filter(asset_models::updated_by.eq(updated_by))
            .order_by(commercial_volume_measuring_device_models::id.asc())
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
            asset_models::asset_models,
            commercial_volume_measuring_device_models::commercial_volume_measuring_device_models,
        };
        Self::table()
            .inner_join(
                asset_models::table
                    .on(commercial_volume_measuring_device_models::id.eq(asset_models::id)),
            )
            .filter(asset_models::updated_at.eq(updated_at))
            .order_by(commercial_volume_measuring_device_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<CommercialVolumeMeasuringDeviceModel> for CommercialVolumeMeasuringDeviceModel {
    fn as_ref(&self) -> &CommercialVolumeMeasuringDeviceModel {
        self
    }
}
