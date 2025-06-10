#[derive(Debug, Clone, PartialEq, Copy)]
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
    table_name = crate::codegen::diesel_codegen::tables::packaging_models::packaging_models
)]
pub struct PackagingModel {
    pub id: ::rosetta_uuid::Uuid,
    pub kilograms: f32,
}
impl web_common_traits::prelude::TableName for PackagingModel {
    const TABLE_NAME: &'static str = "packaging_models";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::trackables::Trackable,
    > for PackagingModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
    > for PackagingModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for PackagingModel {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl PackagingModel {
    pub fn id<C: diesel::connection::LoadConnection>(
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
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct::table(),
                self.id,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::packaging_models::packaging_models;
        use crate::codegen::diesel_codegen::tables::trackables::trackables;
        use diesel::OptionalExtension;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
        Self::table()
            .inner_join(trackables::table.on(packaging_models::id.eq(trackables::id)))
            .filter(trackables::name.eq(name))
            .order_by(packaging_models::id.asc())
            .select(Self::as_select())
            .first::<Self>(conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn from_description(
        description: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::packaging_models::packaging_models;
        use crate::codegen::diesel_codegen::tables::trackables::trackables;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
        Self::table()
            .inner_join(trackables::table.on(packaging_models::id.eq(trackables::id)))
            .filter(trackables::description.eq(description))
            .order_by(packaging_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_photograph_id(
        photograph_id: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::packaging_models::packaging_models;
        use crate::codegen::diesel_codegen::tables::trackables::trackables;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
        Self::table()
            .inner_join(trackables::table.on(packaging_models::id.eq(trackables::id)))
            .filter(trackables::photograph_id.eq(photograph_id))
            .order_by(packaging_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_id(
        parent_id: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::packaging_models::packaging_models;
        use crate::codegen::diesel_codegen::tables::trackables::trackables;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
        Self::table()
            .inner_join(trackables::table.on(packaging_models::id.eq(trackables::id)))
            .filter(trackables::parent_id.eq(parent_id))
            .order_by(packaging_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_by(
        created_by: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::packaging_models::packaging_models;
        use crate::codegen::diesel_codegen::tables::trackables::trackables;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
        Self::table()
            .inner_join(trackables::table.on(packaging_models::id.eq(trackables::id)))
            .filter(trackables::created_by.eq(created_by))
            .order_by(packaging_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_at(
        created_at: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::packaging_models::packaging_models;
        use crate::codegen::diesel_codegen::tables::trackables::trackables;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
        Self::table()
            .inner_join(trackables::table.on(packaging_models::id.eq(trackables::id)))
            .filter(trackables::created_at.eq(created_at))
            .order_by(packaging_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_updated_by(
        updated_by: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::packaging_models::packaging_models;
        use crate::codegen::diesel_codegen::tables::trackables::trackables;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
        Self::table()
            .inner_join(trackables::table.on(packaging_models::id.eq(trackables::id)))
            .filter(trackables::updated_by.eq(updated_by))
            .order_by(packaging_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_updated_at(
        updated_at: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::packaging_models::packaging_models;
        use crate::codegen::diesel_codegen::tables::trackables::trackables;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
        Self::table()
            .inner_join(trackables::table.on(packaging_models::id.eq(trackables::id)))
            .filter(trackables::updated_at.eq(updated_at))
            .order_by(packaging_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_deprecation_date(
        deprecation_date: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::commercial_products::commercial_products;
        use crate::codegen::diesel_codegen::tables::packaging_models::packaging_models;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
        Self::table()
            .inner_join(
                commercial_products::table.on(packaging_models::id.eq(commercial_products::id)),
            )
            .filter(commercial_products::deprecation_date.eq(deprecation_date))
            .order_by(packaging_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_brand_id(
        brand_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::commercial_products::commercial_products;
        use crate::codegen::diesel_codegen::tables::packaging_models::packaging_models;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
        Self::table()
            .inner_join(
                commercial_products::table.on(packaging_models::id.eq(commercial_products::id)),
            )
            .filter(commercial_products::brand_id.eq(brand_id))
            .order_by(packaging_models::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<PackagingModel> for PackagingModel {
    fn as_ref(&self) -> &PackagingModel {
        self
    }
}
