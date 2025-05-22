#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[diesel(table_name = crate::codegen::diesel_codegen::tables::taxa::taxa)]
pub struct Taxon {
    pub id: i32,
    pub name: String,
    pub parent_id: Option<i32>,
    pub rank_id: i16,
}
impl diesel::Identifiable for Taxon {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl Taxon {
    #[cfg(feature = "postgres")]
    pub async fn rank(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::ranks::Rank, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::ranks::Rank::table()
            .filter(crate::codegen::diesel_codegen::tables::ranks::ranks::dsl::id.eq(&self.rank_id))
            .first::<crate::codegen::structs_codegen::tables::ranks::Rank>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_rank_id(
        conn: &mut diesel_async::AsyncPgConnection,
        rank_id: &crate::codegen::structs_codegen::tables::ranks::Rank,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(crate::codegen::diesel_codegen::tables::taxa::taxa::dsl::rank_id.eq(rank_id.id))
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_name(
        name: &String,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;

        use crate::codegen::diesel_codegen::tables::taxa::taxa;
        Self::table().filter(taxa::name.eq(name)).order_by(taxa::id.asc()).load::<Self>(conn).await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_parent_id(
        parent_id: &Option<i32>,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;

        use crate::codegen::diesel_codegen::tables::taxa::taxa;
        Self::table()
            .filter(taxa::parent_id.eq(parent_id))
            .order_by(taxa::id.asc())
            .load::<Self>(conn)
            .await
    }
}
