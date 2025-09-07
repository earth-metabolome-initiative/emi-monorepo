#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Associations,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::users::User,
        foreign_key = created_by
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::organisms::Organism,
        foreign_key = organism_id
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::taxa::Taxon,
        foreign_key = taxon_id
    )
)]
#[diesel(primary_key(organism_id, taxon_id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa
)]
pub struct OrganismTaxon {
    pub created_by: i32,
    pub created_at: ::rosetta_timestamp::TimestampUTC,
    pub organism_id: ::rosetta_uuid::Uuid,
    pub taxon_id: i32,
}
impl web_common_traits::prelude::TableName for OrganismTaxon {
    const TABLE_NAME: &'static str = "organism_taxa";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon,
    > for OrganismTaxon
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a (::rosetta_uuid::Uuid, i32)>,
{
}
impl diesel::Identifiable for OrganismTaxon {
    type Id = (::rosetta_uuid::Uuid, i32);
    fn id(self) -> Self::Id {
        (self.organism_id, self.taxon_id)
    }
}
impl OrganismTaxon {
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::users::User,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::users::User: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::users::User,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.created_by,
            ),
            conn,
        )
    }
    pub fn organism<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::organisms::Organism,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::organisms::Organism: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::organisms::Organism as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::organisms::Organism as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::organisms::Organism as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::organisms::Organism as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::organisms::Organism as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::organisms::Organism as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::organisms::Organism,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::organisms::Organism::table(),
                self.organism_id,
            ),
            conn,
        )
    }
    pub fn taxon<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::taxa::Taxon,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::taxa::Taxon: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::taxa::Taxon as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::taxa::Taxon as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::taxa::Taxon as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::taxa::Taxon as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::taxa::Taxon as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::taxa::Taxon as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::taxa::Taxon,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::taxa::Taxon::table(),
                self.taxon_id,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_at(
        created_at: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa;
        Self::table()
            .filter(organism_taxa::created_at.eq(created_at))
            .order_by((organism_taxa::organism_id.asc(), organism_taxa::taxon_id.asc()))
            .load::<Self>(conn)
    }
}
impl AsRef<OrganismTaxon> for OrganismTaxon {
    fn as_ref(&self) -> &OrganismTaxon {
        self
    }
}
