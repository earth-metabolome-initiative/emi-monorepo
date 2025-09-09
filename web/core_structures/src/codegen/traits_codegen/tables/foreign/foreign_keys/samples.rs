#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SampleForeignKeys {
    pub id: Option<crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset>,
    pub model: Option<crate::codegen::structs_codegen::tables::sample_models::SampleModel>,
    pub sample_source:
        Option<crate::codegen::structs_codegen::tables::sample_sources::SampleSource>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::samples::Sample
{
    type ForeignKeys = SampleForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhysicalAsset(self.id),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::SampleModel(self.model),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::SampleSource(
                self.sample_source,
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.id.is_some()
            && foreign_keys.model.is_some()
            && foreign_keys.sample_source.is_some()
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
                crate::codegen::tables::row::Row::PhysicalAsset(physical_assets),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.id == physical_assets.id {
                    foreign_keys.id = Some(physical_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PhysicalAsset(physical_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.id == physical_assets.id {
                    foreign_keys.id = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::SampleModel(sample_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.model == sample_models.id {
                    foreign_keys.model = Some(sample_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::SampleModel(sample_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.model == sample_models.id {
                    foreign_keys.model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::SampleSource(sample_sources),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.sample_source == sample_sources.id {
                    foreign_keys.sample_source = Some(sample_sources);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::SampleSource(sample_sources),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.sample_source == sample_sources.id {
                    foreign_keys.sample_source = None;
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
impl web_common_traits::prelude::ForeignKeys for SampleForeignKeys {}
