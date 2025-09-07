#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrganismTaxonForeignKeys {
    pub created_by: Option<crate::User>,
    pub organism: Option<crate::Organism>,
    pub taxon: Option<crate::Taxon>,
}
impl web_common_traits::prelude::HasForeignKeys for crate::OrganismTaxon {
    type ForeignKeys = OrganismTaxonForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(self.created_by),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Organism(self.organism_id),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Taxon(self.taxon_id),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.created_by.is_some()
            && foreign_keys.organism.is_some()
            && foreign_keys.taxon.is_some()
    }
    fn update(
        &self,
        foreign_keys: &mut Self::ForeignKeys,
        row: Self::Row,
        crud: web_common_traits::crud::CRUD,
    ) -> bool {
        let mut updated = false;
        match (row, crud) {
            (
                crate::codegen::tables::row::Row::Organism(organisms),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.organism_id == organisms.id {
                    foreign_keys.organism = Some(organisms);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Organism(organisms),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.organism_id == organisms.id {
                    foreign_keys.organism = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Taxon(taxa),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.taxon_id == taxa.id {
                    foreign_keys.taxon = Some(taxa);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Taxon(taxa),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.taxon_id == taxa.id {
                    foreign_keys.taxon = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::User(users),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.created_by == users.id {
                    foreign_keys.created_by = Some(users);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::User(users),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.created_by == users.id {
                    foreign_keys.created_by = None;
                    updated = true;
                }
            }
            (_, crud) => {
                unreachable!("Unexpected row type with operation {crud}");
            }
        }
        updated
    }
}
impl web_common_traits::prelude::ForeignKeys for OrganismTaxonForeignKeys {}
