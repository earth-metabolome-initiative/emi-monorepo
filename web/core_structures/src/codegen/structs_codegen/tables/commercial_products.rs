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
    table_name = crate::codegen::diesel_codegen::tables::commercial_products::commercial_products
)]
pub struct CommercialProduct {
    pub id: ::rosetta_uuid::Uuid,
    pub deprecation_date: Option<::rosetta_timestamp::TimestampUTC>,
    pub brand_id: i32,
}
impl web_common_traits::prelude::TableName for CommercialProduct {
    const TABLE_NAME: &'static str = "commercial_products";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::trackables::Trackable,
    > for CommercialProduct
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for CommercialProduct {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl CommercialProduct {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::trackables::Trackable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::trackables::Trackable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::trackables::Trackable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::trackables::Trackable::table(),
                self.id,
            ),
            conn,
        )
    }
    pub fn brand<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::brands::Brand,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::brands::Brand: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::brands::Brand as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::brands::Brand as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::brands::Brand as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::brands::Brand as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::brands::Brand as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::brands::Brand as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::brands::Brand,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::brands::Brand::table(),
                self.brand_id,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_deprecation_date(
        deprecation_date: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::commercial_products::commercial_products;
        Self::table()
            .filter(commercial_products::deprecation_date.eq(deprecation_date))
            .order_by(commercial_products::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_brand_id(
        brand_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::commercial_products::commercial_products;
        Self::table()
            .filter(commercial_products::brand_id.eq(brand_id))
            .order_by(commercial_products::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, OptionalExtension, QueryDsl, RunQueryDsl,
            SelectableHelper, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            commercial_products::commercial_products, trackables::trackables,
        };
        Self::table()
            .inner_join(trackables::table.on(commercial_products::id.eq(trackables::id)))
            .filter(trackables::name.eq(name))
            .order_by(commercial_products::id.asc())
            .select(Self::as_select())
            .first::<Self>(conn)
            .optional()
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
            commercial_products::commercial_products, trackables::trackables,
        };
        Self::table()
            .inner_join(trackables::table.on(commercial_products::id.eq(trackables::id)))
            .filter(trackables::description.eq(description))
            .order_by(commercial_products::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_photograph_id(
        photograph_id: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            commercial_products::commercial_products, trackables::trackables,
        };
        Self::table()
            .inner_join(trackables::table.on(commercial_products::id.eq(trackables::id)))
            .filter(trackables::photograph_id.eq(photograph_id))
            .order_by(commercial_products::id.asc())
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
            commercial_products::commercial_products, trackables::trackables,
        };
        Self::table()
            .inner_join(trackables::table.on(commercial_products::id.eq(trackables::id)))
            .filter(trackables::created_by.eq(created_by))
            .order_by(commercial_products::id.asc())
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
            commercial_products::commercial_products, trackables::trackables,
        };
        Self::table()
            .inner_join(trackables::table.on(commercial_products::id.eq(trackables::id)))
            .filter(trackables::created_at.eq(created_at))
            .order_by(commercial_products::id.asc())
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
            commercial_products::commercial_products, trackables::trackables,
        };
        Self::table()
            .inner_join(trackables::table.on(commercial_products::id.eq(trackables::id)))
            .filter(trackables::updated_by.eq(updated_by))
            .order_by(commercial_products::id.asc())
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
            commercial_products::commercial_products, trackables::trackables,
        };
        Self::table()
            .inner_join(trackables::table.on(commercial_products::id.eq(trackables::id)))
            .filter(trackables::updated_at.eq(updated_at))
            .order_by(commercial_products::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<CommercialProduct> for CommercialProduct {
    fn as_ref(&self) -> &CommercialProduct {
        self
    }
}
