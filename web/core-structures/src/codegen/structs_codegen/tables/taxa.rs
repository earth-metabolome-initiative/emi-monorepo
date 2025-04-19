#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::taxa::taxa)]
pub struct Taxon {
    pub id: i32,
    pub name: String,
    pub parent_id: Option<i32>,
    pub rank_id: i16,
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
}
