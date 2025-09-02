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
    table_name = crate::codegen::diesel_codegen::tables::digital_asset_models::digital_asset_models
)]
pub struct DigitalAssetModel {
    pub id: i32,
    pub parent_model_id: Option<i32>,
}
impl web_common_traits::prelude::TableName for DigitalAssetModel {
    const TABLE_NAME: &'static str = "digital_asset_models";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
    > for DigitalAssetModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl<C> web_common_traits::prelude::Ancestor<C> for DigitalAssetModel
where
    Self: web_common_traits::prelude::TableName + Sized,
    C: diesel::connection::LoadConnection,
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization
        + diesel::sql_types::HasSqlType<diesel::sql_types::Integer>
        + 'static,
    web_common_traits::prelude::AncestorExists: diesel::deserialize::FromSqlRow<
            diesel::sql_types::Untyped,
            <C as diesel::Connection>::Backend,
        >,
    for<'a> &'a Self: diesel::Identifiable,
    for<'a> <&'a Self as diesel::Identifiable>::Id:
        diesel::serialize::ToSql<diesel::sql_types::Integer, C::Backend>,
{
    const PARENT_ID: &'static str = "parent_model_id";
    const ID: &'static str = "id";
    type SqlType = diesel::sql_types::Integer;
}
impl web_common_traits::prelude::Descendant<DigitalAssetModel> for DigitalAssetModel {
    fn parent(&self) -> Option<<&Self as diesel::Identifiable>::Id> {
        self.parent_model_id.as_ref()
    }
}
impl diesel::Identifiable for DigitalAssetModel {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl DigitalAssetModel {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_models::AssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::asset_models::AssetModel::table(),
                self.id,
            ),
            conn,
        )
    }
    pub fn parent_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<
            crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel,
        >,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        let Some(parent_model_id) = self.parent_model_id else {
            return Ok(None);
        };
        RunQueryDsl::first(
                QueryDsl::find(
                    crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel::table(),
                    parent_model_id,
                ),
                conn,
            )
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_model_id(
        parent_model_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::digital_asset_models::digital_asset_models;
        Self::table()
            .filter(digital_asset_models::parent_model_id.eq(parent_model_id))
            .order_by(digital_asset_models::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_id_and_parent_model_id(
        id: &i32,
        parent_model_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::digital_asset_models::digital_asset_models;
        Self::table()
            .filter(
                digital_asset_models::id
                    .eq(id)
                    .and(digital_asset_models::parent_model_id.eq(parent_model_id)),
            )
            .order_by(digital_asset_models::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_model_id_and_id(
        parent_model_id: &i32,
        id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl,
            SelectableHelper, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            asset_models::asset_models, digital_asset_models::digital_asset_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(digital_asset_models::id.eq(asset_models::id)))
            .filter(asset_models::parent_model_id.eq(parent_model_id).and(asset_models::id.eq(id)))
            .order_by(digital_asset_models::id.asc())
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
            asset_models::asset_models, digital_asset_models::digital_asset_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(digital_asset_models::id.eq(asset_models::id)))
            .filter(asset_models::most_concrete_table.eq(most_concrete_table))
            .order_by(digital_asset_models::id.asc())
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
            asset_models::asset_models, digital_asset_models::digital_asset_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(digital_asset_models::id.eq(asset_models::id)))
            .filter(asset_models::description.eq(description))
            .order_by(digital_asset_models::id.asc())
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
            asset_models::asset_models, digital_asset_models::digital_asset_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(digital_asset_models::id.eq(asset_models::id)))
            .filter(asset_models::created_by.eq(created_by))
            .order_by(digital_asset_models::id.asc())
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
            asset_models::asset_models, digital_asset_models::digital_asset_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(digital_asset_models::id.eq(asset_models::id)))
            .filter(asset_models::created_at.eq(created_at))
            .order_by(digital_asset_models::id.asc())
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
            asset_models::asset_models, digital_asset_models::digital_asset_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(digital_asset_models::id.eq(asset_models::id)))
            .filter(asset_models::updated_by.eq(updated_by))
            .order_by(digital_asset_models::id.asc())
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
            asset_models::asset_models, digital_asset_models::digital_asset_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(digital_asset_models::id.eq(asset_models::id)))
            .filter(asset_models::updated_at.eq(updated_at))
            .order_by(digital_asset_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<DigitalAssetModel> for DigitalAssetModel {
    fn as_ref(&self) -> &DigitalAssetModel {
        self
    }
}
