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
    table_name = crate::codegen::diesel_codegen::tables::commercial_cap_models::commercial_cap_models
)]
pub struct CommercialCapModel {
    pub id: i32,
    pub cap_model: i32,
}
impl web_common_traits::prelude::TableName for CommercialCapModel {
    const TABLE_NAME: &'static str = "commercial_cap_models";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
    > for CommercialCapModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::cap_models::CapModel,
    > for CommercialCapModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
    > for CommercialCapModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
    > for CommercialCapModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::commercial_cap_models::CommercialCapModel,
    > for CommercialCapModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl diesel::Identifiable for CommercialCapModel {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl CommercialCapModel {
    pub fn cap_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::cap_models::CapModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::cap_models::CapModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::cap_models::CapModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::cap_models::CapModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::cap_models::CapModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::cap_models::CapModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::cap_models::CapModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::cap_models::CapModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::cap_models::CapModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::cap_models::CapModel::table(),
                self.cap_model,
            ),
            conn,
        )
    }
    pub fn commercial_cap_models_id_fkey<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::cap_models::CapModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::cap_models::CapModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::cap_models::CapModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::cap_models::CapModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::cap_models::CapModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::cap_models::CapModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::cap_models::CapModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::cap_models::CapModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::cap_models::CapModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::cap_models::CapModel::table(),
                self.id,
            ),
            conn,
        )
    }
    pub fn commercial_cap_models_id_fkey1<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct::table(),
                self.id,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn commercial_cap_models_id_cap_model_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::asset_models::AssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::asset_models::asset_models::dsl::id
                    .eq(&self.id)
                    .and(
                        crate::codegen::diesel_codegen::tables::asset_models::asset_models::dsl::parent_model
                            .eq(&self.cap_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::asset_models::AssetModel,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_cap_model(
        cap_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::commercial_cap_models::commercial_cap_models;
        Self::table()
            .filter(commercial_cap_models::cap_model.eq(cap_model))
            .order_by(commercial_cap_models::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_id_and_cap_model(
        id: &i32,
        cap_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::commercial_cap_models::commercial_cap_models;
        Self::table()
            .filter(
                commercial_cap_models::id
                    .eq(id)
                    .and(commercial_cap_models::cap_model.eq(cap_model)),
            )
            .order_by(commercial_cap_models::id.asc())
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
            commercial_cap_models::commercial_cap_models, commercial_products::commercial_products,
        };
        Self::table()
            .inner_join(
                commercial_products::table
                    .on(commercial_cap_models::id.eq(commercial_products::id)),
            )
            .filter(commercial_products::deprecation_date.eq(deprecation_date))
            .order_by(commercial_cap_models::id.asc())
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
            commercial_cap_models::commercial_cap_models, commercial_products::commercial_products,
        };
        Self::table()
            .inner_join(
                commercial_products::table
                    .on(commercial_cap_models::id.eq(commercial_products::id)),
            )
            .filter(commercial_products::brand_id.eq(brand_id))
            .order_by(commercial_cap_models::id.asc())
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
            commercial_cap_models::commercial_cap_models,
            physical_asset_models::physical_asset_models,
        };
        Self::table()
            .inner_join(
                physical_asset_models::table
                    .on(commercial_cap_models::id.eq(physical_asset_models::id)),
            )
            .filter(physical_asset_models::parent_model.eq(parent_model))
            .order_by(commercial_cap_models::id.asc())
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
            asset_models::asset_models, commercial_cap_models::commercial_cap_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(commercial_cap_models::id.eq(asset_models::id)))
            .filter(asset_models::name.eq(name))
            .order_by(commercial_cap_models::id.asc())
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
            asset_models::asset_models, commercial_cap_models::commercial_cap_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(commercial_cap_models::id.eq(asset_models::id)))
            .filter(asset_models::parent_model.eq(parent_model).and(asset_models::id.eq(id)))
            .order_by(commercial_cap_models::id.asc())
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
            asset_models::asset_models, commercial_cap_models::commercial_cap_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(commercial_cap_models::id.eq(asset_models::id)))
            .filter(asset_models::most_concrete_table.eq(most_concrete_table))
            .order_by(commercial_cap_models::id.asc())
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
            asset_models::asset_models, commercial_cap_models::commercial_cap_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(commercial_cap_models::id.eq(asset_models::id)))
            .filter(asset_models::description.eq(description))
            .order_by(commercial_cap_models::id.asc())
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
            asset_models::asset_models, commercial_cap_models::commercial_cap_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(commercial_cap_models::id.eq(asset_models::id)))
            .filter(asset_models::created_by.eq(created_by))
            .order_by(commercial_cap_models::id.asc())
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
            asset_models::asset_models, commercial_cap_models::commercial_cap_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(commercial_cap_models::id.eq(asset_models::id)))
            .filter(asset_models::created_at.eq(created_at))
            .order_by(commercial_cap_models::id.asc())
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
            asset_models::asset_models, commercial_cap_models::commercial_cap_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(commercial_cap_models::id.eq(asset_models::id)))
            .filter(asset_models::updated_by.eq(updated_by))
            .order_by(commercial_cap_models::id.asc())
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
            asset_models::asset_models, commercial_cap_models::commercial_cap_models,
        };
        Self::table()
            .inner_join(asset_models::table.on(commercial_cap_models::id.eq(asset_models::id)))
            .filter(asset_models::updated_at.eq(updated_at))
            .order_by(commercial_cap_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<CommercialCapModel> for CommercialCapModel {
    fn as_ref(&self) -> &CommercialCapModel {
        self
    }
}
