#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::spectra::spectra)]
pub struct Spectrum {
    pub id: i32,
    pub spectra_collection_id: i32,
}
impl diesel::Identifiable for Spectrum {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl Spectrum {
    #[cfg(feature = "postgres")]
    pub async fn spectra_collection(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection::table()
            .filter(
                crate::codegen::diesel_codegen::tables::spectra_collections::spectra_collections::dsl::id
                    .eq(&self.spectra_collection_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_spectra_collection_id(
        conn: &mut diesel_async::AsyncPgConnection,
        spectra_collection_id: &crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::spectra::spectra::dsl::spectra_collection_id
                    .eq(spectra_collection_id.id),
            )
            .load::<Self>(conn)
            .await
    }
}
