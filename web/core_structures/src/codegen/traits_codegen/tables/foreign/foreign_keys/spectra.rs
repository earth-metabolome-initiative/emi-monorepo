#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpectrumForeignKeys {
    pub id: Option<crate::DigitalAsset>,
    pub spectra_collection: Option<crate::SpectraCollection>,
}
impl web_common_traits::prelude::HasForeignKeys for crate::Spectrum {
    type ForeignKeys = SpectrumForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::DigitalAsset(self.id),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::SpectraCollection(
                self.spectra_collection_id,
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.id.is_some() && foreign_keys.spectra_collection.is_some()
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
                crate::codegen::tables::row::Row::DigitalAsset(digital_assets),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.id == digital_assets.id {
                    foreign_keys.id = Some(digital_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::DigitalAsset(digital_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.id == digital_assets.id {
                    foreign_keys.id = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::SpectraCollection(spectra_collections),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.spectra_collection_id == spectra_collections.id {
                    foreign_keys.spectra_collection = Some(spectra_collections);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::SpectraCollection(spectra_collections),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.spectra_collection_id == spectra_collections.id {
                    foreign_keys.spectra_collection = None;
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
impl web_common_traits::prelude::ForeignKeys for SpectrumForeignKeys {}
