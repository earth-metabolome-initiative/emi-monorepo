#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrganismSamplingStepModelForeignKeys {
    pub id:
        Option<crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel>,
    pub organism: Option<crate::codegen::structs_codegen::tables::organisms::Organism>,
}
impl web_common_traits::prelude::HasForeignKeys
for crate::codegen::structs_codegen::tables::organism_sampling_step_models::OrganismSamplingStepModel {
    type ForeignKeys = OrganismSamplingStepModelForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::SamplingStepModel(
                        self.id,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::Organism(
                        self.organism_id,
                    ),
                ),
            );
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.id.is_some() && foreign_keys.organism.is_some()
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
                if organisms.id == self.organism_id {
                    foreign_keys.organism = Some(organisms);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Organism(organisms),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if organisms.id == self.organism_id {
                    foreign_keys.organism = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::SamplingStepModel(
                    sampling_step_models,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if sampling_step_models.id == self.id {
                    foreign_keys.id = Some(sampling_step_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::SamplingStepModel(
                    sampling_step_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if sampling_step_models.id == self.id {
                    foreign_keys.id = None;
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
impl web_common_traits::prelude::ForeignKeys for OrganismSamplingStepModelForeignKeys {}
