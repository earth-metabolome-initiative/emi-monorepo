#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommercialPipetteModelForeignKeys {
    pub commercial_pipette_models_id_fkey:
        Option<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel>,
    pub commercial_pipette_models_id_fkey1:
        Option<crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct>,
    pub pipette_model:
        Option<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::commercial_pipette_models::CommercialPipetteModel
{
    type ForeignKeys = CommercialPipetteModelForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PipetteModel(self.id),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialProduct(self.id),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PipetteModel(
                self.pipette_model,
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.commercial_pipette_models_id_fkey.is_some()
            && foreign_keys.commercial_pipette_models_id_fkey1.is_some()
            && foreign_keys.pipette_model.is_some()
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
                crate::codegen::tables::row::Row::CommercialProduct(commercial_products),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.id == commercial_products.id {
                    foreign_keys.commercial_pipette_models_id_fkey1 = Some(commercial_products);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CommercialProduct(commercial_products),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.id == commercial_products.id {
                    foreign_keys.commercial_pipette_models_id_fkey1 = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PipetteModel(pipette_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.id == pipette_models.id {
                    foreign_keys.commercial_pipette_models_id_fkey = Some(pipette_models);
                    updated = true;
                }
                if self.pipette_model == pipette_models.id {
                    foreign_keys.pipette_model = Some(pipette_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PipetteModel(pipette_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.id == pipette_models.id {
                    foreign_keys.commercial_pipette_models_id_fkey = None;
                    updated = true;
                }
                if self.pipette_model == pipette_models.id {
                    foreign_keys.pipette_model = None;
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
impl web_common_traits::prelude::ForeignKeys for CommercialPipetteModelForeignKeys {}
