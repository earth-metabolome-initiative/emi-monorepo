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
    table_name = crate::codegen::diesel_codegen::tables::commercial_pipette_tip_models::commercial_pipette_tip_models
)]
pub struct CommercialPipetteTipModel {
    pub id: i32,
    pub pipette_tip_model: i32,
}
impl web_common_traits::prelude::TableName for CommercialPipetteTipModel {
    const TABLE_NAME: &'static str = "commercial_pipette_tip_models";
}
impl<'a> From<&'a CommercialPipetteTipModel>
for web_common_traits::database::IdOrBuilder<
    i32,
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelBuilder,
> {
    fn from(value: &'a CommercialPipetteTipModel) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.id)
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
    > for CommercialPipetteTipModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
    > for CommercialPipetteTipModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
    > for CommercialPipetteTipModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
    > for CommercialPipetteTipModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl web_common_traits::prelude::ExtensionTable<
    crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel,
> for CommercialPipetteTipModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{}
impl diesel::Identifiable for CommercialPipetteTipModel {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for CommercialPipetteTipModel {
    type PrimaryKey = i32;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl CommercialPipetteTipModel {
    pub fn pipette_tip_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel::read(
            self.pipette_tip_model,
            conn,
        )
    }
    pub fn from_pipette_tip_model<C>(
        pipette_tip_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::commercial_pipette_tip_models::commercial_pipette_tip_models::pipette_tip_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::commercial_pipette_tip_models::commercial_pipette_tip_models::pipette_tip_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::commercial_pipette_tip_models::commercial_pipette_tip_models::id,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::commercial_pipette_tip_models::commercial_pipette_tip_models::pipette_tip_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::commercial_pipette_tip_models::commercial_pipette_tip_models::id,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::commercial_pipette_tip_models::commercial_pipette_tip_models;
        Self::table()
            .filter(commercial_pipette_tip_models::pipette_tip_model.eq(pipette_tip_model))
            .order_by(commercial_pipette_tip_models::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_brand_id(
        brand_id: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            commercial_pipette_tip_models::commercial_pipette_tip_models,
            commercial_products::commercial_products,
        };
        Self::table()
            .inner_join(
                commercial_products::table
                    .on(commercial_pipette_tip_models::id.eq(commercial_products::id)),
            )
            .filter(commercial_products::brand_id.eq(brand_id))
            .order_by(commercial_pipette_tip_models::id.asc())
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
            commercial_pipette_tip_models::commercial_pipette_tip_models,
            physical_asset_models::physical_asset_models,
        };
        Self::table()
            .inner_join(
                physical_asset_models::table
                    .on(commercial_pipette_tip_models::id.eq(physical_asset_models::id)),
            )
            .filter(physical_asset_models::parent_model.eq(parent_model))
            .order_by(commercial_pipette_tip_models::id.asc())
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
            commercial_pipette_tip_models::commercial_pipette_tip_models,
        };
        Self::table()
            .inner_join(
                asset_models::table.on(commercial_pipette_tip_models::id.eq(asset_models::id)),
            )
            .filter(asset_models::name.eq(name))
            .order_by(commercial_pipette_tip_models::id.asc())
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
            asset_models::asset_models,
            commercial_pipette_tip_models::commercial_pipette_tip_models,
        };
        Self::table()
            .inner_join(
                asset_models::table.on(commercial_pipette_tip_models::id.eq(asset_models::id)),
            )
            .filter(asset_models::description.eq(description))
            .order_by(commercial_pipette_tip_models::id.asc())
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
            asset_models::asset_models,
            commercial_pipette_tip_models::commercial_pipette_tip_models,
        };
        Self::table()
            .inner_join(
                asset_models::table.on(commercial_pipette_tip_models::id.eq(asset_models::id)),
            )
            .filter(asset_models::created_by.eq(created_by))
            .order_by(commercial_pipette_tip_models::id.asc())
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
            asset_models::asset_models,
            commercial_pipette_tip_models::commercial_pipette_tip_models,
        };
        Self::table()
            .inner_join(
                asset_models::table.on(commercial_pipette_tip_models::id.eq(asset_models::id)),
            )
            .filter(asset_models::updated_by.eq(updated_by))
            .order_by(commercial_pipette_tip_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<CommercialPipetteTipModel> for CommercialPipetteTipModel {
    fn as_ref(&self) -> &CommercialPipetteTipModel {
        self
    }
}
