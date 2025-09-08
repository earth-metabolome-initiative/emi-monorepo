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
#[diesel(belongs_to(crate::User, foreign_key = created_by))]
#[diesel(belongs_to(crate::Organism, foreign_key = organism_id))]
#[diesel(belongs_to(crate::Taxon, foreign_key = taxon_id))]
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
impl web_common_traits::prelude::ExtensionTable<crate::OrganismTaxon> for OrganismTaxon where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a (::rosetta_uuid::Uuid, i32)>
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
    ) -> Result<crate::User, diesel::result::Error>
    where
        crate::User: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::User::read(self.created_by, conn)
    }
    pub fn organism<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::Organism, diesel::result::Error>
    where
        crate::Organism: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::Organism::read(self.organism_id, conn)
    }
    pub fn taxon<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::Taxon, diesel::result::Error>
    where
        crate::Taxon: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::Taxon::read(self.taxon_id, conn)
    }
    pub fn from_created_at<C>(
        created_at: ::rosetta_timestamp::TimestampUTC,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::created_at as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::created_at as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::organism_id,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::taxon_id,
                >,
            ),
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::created_at as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::organism_id,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa::taxon_id,
                >,
            ),
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
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
