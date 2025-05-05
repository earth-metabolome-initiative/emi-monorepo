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
#[diesel(primary_key(organism_id, taxon_id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa
)]
pub struct OrganismTaxon {
    pub created_by: i32,
    pub created_at: rosetta_timestamp::TimestampUTC,
    pub organism_id: rosetta_uuid::Uuid,
    pub taxon_id: i32,
}
impl diesel::Identifiable for OrganismTaxon {
    type Id = (rosetta_uuid::Uuid, i32);
    fn id(self) -> Self::Id {
        (self.organism_id, self.taxon_id)
    }
}
impl OrganismTaxon {
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
    pub async fn organism(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::organisms::Organism, diesel::result::Error>
    {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::organisms::Organism::table()
            .filter(
                crate::codegen::diesel_codegen::tables::organisms::organisms::dsl::id
                    .eq(&self.organism_id),
            )
            .first::<crate::codegen::structs_codegen::tables::organisms::Organism>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn taxon(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::taxa::Taxon, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::taxa::Taxon::table()
            .filter(crate::codegen::diesel_codegen::tables::taxa::taxa::dsl::id.eq(&self.taxon_id))
            .first::<crate::codegen::structs_codegen::tables::taxa::Taxon>(conn)
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
                crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::dsl::created_by
                    .eq(created_by.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_organism_id(
        conn: &mut diesel_async::AsyncPgConnection,
        organism_id: &crate::codegen::structs_codegen::tables::organisms::Organism,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::dsl::organism_id
                    .eq(organism_id.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_taxon_id(
        conn: &mut diesel_async::AsyncPgConnection,
        taxon_id: &crate::codegen::structs_codegen::tables::taxa::Taxon,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::dsl::taxon_id
                    .eq(taxon_id.id),
            )
            .load::<Self>(conn)
            .await
    }
}
