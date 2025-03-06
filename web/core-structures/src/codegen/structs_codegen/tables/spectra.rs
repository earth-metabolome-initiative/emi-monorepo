#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "diesel", derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable))]
#[cfg_attr(feature = "diesel", diesel(primary_key(id)))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: spectra :: spectra))]
pub struct Spectra {
    pub id: i32,
    pub spectra_collection_id: i32,
}
impl Spectra {
    #[cfg(feature = "postgres")]
    pub async fn spectra_collection(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
        diesel::result::Error,
    > {
        use diesel::{associations::HasTable, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate :: codegen :: structs_codegen :: tables :: spectra_collections :: SpectraCollection :: table () . find (& self . spectra_collection_id) . first :: < crate :: codegen :: structs_codegen :: tables :: spectra_collections :: SpectraCollection > (conn) . await
    }
}
