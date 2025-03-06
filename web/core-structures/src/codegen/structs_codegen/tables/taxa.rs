#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "diesel", derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable))]
#[cfg_attr(feature = "diesel", diesel(primary_key(id)))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: taxa :: taxa))]
pub struct Taxa {
    pub id: i32,
    pub name: String,
    pub parent_id: Option<i32>,
    pub rank_id: i16,
}
impl Taxa {
    #[cfg(feature = "postgres")]
    pub async fn rank(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::ranks::Rank, diesel::result::Error> {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::ranks::Rank::table()
            .find(&self.rank_id)
            .first::<crate::codegen::structs_codegen::tables::ranks::Rank>(conn)
            .await
    }
}
