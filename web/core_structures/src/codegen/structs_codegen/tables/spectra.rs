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
#[diesel(table_name = crate::codegen::diesel_codegen::tables::spectra::spectra)]
pub struct Spectrum {
    pub id: i32,
    pub spectra_collection_id: i32,
}
impl web_common_traits::prelude::TableName for Spectrum {
    const TABLE_NAME: &'static str = "spectra";
}
impl diesel::Identifiable for Spectrum {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl Spectrum {
    pub fn spectra_collection<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
        >,
    {
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection::table(),
                self.spectra_collection_id,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_spectra_collection_id(
        spectra_collection_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::spectra::spectra;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, QueryDsl};
        Self::table()
            .filter(spectra::spectra_collection_id.eq(spectra_collection_id))
            .order_by(spectra::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<Spectrum> for Spectrum {
    fn as_ref(&self) -> &Spectrum {
        self
    }
}
