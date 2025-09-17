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
impl<'a> From<&'a OrganismTaxon>
    for web_common_traits::database::IdOrBuilder<
        (::rosetta_uuid::Uuid, i32),
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxonBuilder,
    >
{
    fn from(value: &'a OrganismTaxon) -> Self {
        web_common_traits::database::IdOrBuilder::Id((value.organism_id, value.taxon_id))
    }
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
impl web_common_traits::database::PrimaryKeyLike for OrganismTaxon {
    type PrimaryKey = (::rosetta_uuid::Uuid, i32);
    fn primary_key(&self) -> Self::PrimaryKey {
        (self.organism_id, self.taxon_id)
    }
}
impl OrganismTaxon {
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::users::User: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::users::User::read(self.created_by, conn)
    }
    pub fn organism<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::organisms::Organism, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::organisms::Organism:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::organisms::Organism::read(self.organism_id, conn)
    }
    pub fn taxon<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::taxa::Taxon, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::taxa::Taxon: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::taxa::Taxon::read(self.taxon_id, conn)
    }
}
impl AsRef<OrganismTaxon> for OrganismTaxon {
    fn as_ref(&self) -> &OrganismTaxon {
        self
    }
}
