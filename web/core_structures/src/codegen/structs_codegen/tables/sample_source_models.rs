#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Insertable, diesel::Queryable, diesel::Identifiable)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::sample_source_models::sample_source_models
)]
pub struct SampleSourceModel {
    pub id: i32,
}
impl web_common_traits::prelude::TableName for SampleSourceModel {
    const TABLE_NAME: &'static str = "sample_source_models";
}
impl<'a> From<&'a SampleSourceModel>
    for web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceModelBuilder,
    >
{
    fn from(value: &'a SampleSourceModel) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.id)
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
    > for SampleSourceModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
    > for SampleSourceModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel,
    > for SampleSourceModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl diesel::Identifiable for SampleSourceModel {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for SampleSourceModel {
    type PrimaryKey = i32;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl SampleSourceModel {
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
            physical_asset_models::physical_asset_models,
            sample_source_models::sample_source_models,
        };
        Self::table()
            .inner_join(
                physical_asset_models::table
                    .on(sample_source_models::id.eq(physical_asset_models::id)),
            )
            .filter(physical_asset_models::parent_model.eq(parent_model))
            .order_by(sample_source_models::id.asc())
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
            asset_models::asset_models, sample_source_models::sample_source_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(sample_source_models::id.eq(asset_models::id)))
            .filter(asset_models::name.eq(name))
            .order_by(sample_source_models::id.asc())
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
            asset_models::asset_models, sample_source_models::sample_source_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(sample_source_models::id.eq(asset_models::id)))
            .filter(asset_models::description.eq(description))
            .order_by(sample_source_models::id.asc())
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
            asset_models::asset_models, sample_source_models::sample_source_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(sample_source_models::id.eq(asset_models::id)))
            .filter(asset_models::created_by.eq(created_by))
            .order_by(sample_source_models::id.asc())
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
            asset_models::asset_models, sample_source_models::sample_source_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(sample_source_models::id.eq(asset_models::id)))
            .filter(asset_models::updated_by.eq(updated_by))
            .order_by(sample_source_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<SampleSourceModel> for SampleSourceModel {
    fn as_ref(&self) -> &SampleSourceModel {
        self
    }
}
