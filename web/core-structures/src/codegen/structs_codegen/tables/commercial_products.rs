#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::commercial_products::commercial_products
)]
pub struct CommercialProduct {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub photograph_id: rosetta_uuid::Uuid,
    pub deprecation_date: Option<rosetta_timestamp::TimestampUTC>,
    pub brand_id: i32,
    pub created_by: i32,
    pub created_at: rosetta_timestamp::TimestampUTC,
    pub updated_by: i32,
    pub updated_at: rosetta_timestamp::TimestampUTC,
}
impl CommercialProduct {
    #[cfg(feature = "postgres")]
    pub async fn photograph(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::photographs::Photograph,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::photographs::Photograph::table()
            .filter(
                crate::codegen::diesel_codegen::tables::photographs::photographs::dsl::id
                    .eq(&self.photograph_id),
            )
            .first::<crate::codegen::structs_codegen::tables::photographs::Photograph>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn brand(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::brands::Brand, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::brands::Brand::table()
            .filter(
                crate::codegen::diesel_codegen::tables::brands::brands::dsl::id.eq(&self.brand_id),
            )
            .first::<crate::codegen::structs_codegen::tables::brands::Brand>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn created_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .filter(
                crate::codegen::diesel_codegen::tables::users::users::dsl::id.eq(&self.created_by),
            )
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn updated_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .filter(
                crate::codegen::diesel_codegen::tables::users::users::dsl::id.eq(&self.updated_by),
            )
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_photograph_id(
        conn: &mut diesel_async::AsyncPgConnection,
        photograph_id: &crate::codegen::structs_codegen::tables::photographs::Photograph,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::commercial_products::commercial_products::dsl::photograph_id
                    .eq(photograph_id.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_brand_id(
        conn: &mut diesel_async::AsyncPgConnection,
        brand_id: &crate::codegen::structs_codegen::tables::brands::Brand,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::commercial_products::commercial_products::dsl::brand_id
                    .eq(brand_id.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_created_by(
        conn: &mut diesel_async::AsyncPgConnection,
        created_by: &crate::codegen::structs_codegen::tables::users::User,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::commercial_products::commercial_products::dsl::created_by
                    .eq(created_by.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_updated_by(
        conn: &mut diesel_async::AsyncPgConnection,
        updated_by: &crate::codegen::structs_codegen::tables::users::User,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::commercial_products::commercial_products::dsl::updated_by
                    .eq(updated_by.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_name(
        name: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                diesel::ExpressionMethods::eq(
                    crate::codegen::diesel_codegen::tables::commercial_products::commercial_products::name,
                    name,
                ),
            )
            .first::<Self>(conn)
            .await
            .optional()
    }
}
