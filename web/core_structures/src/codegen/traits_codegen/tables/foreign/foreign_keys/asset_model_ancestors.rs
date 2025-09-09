#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssetModelAncestorForeignKeys {
    pub descendant_model: Option<crate::codegen::structs_codegen::tables::asset_models::AssetModel>,
    pub ancestor_model: Option<crate::codegen::structs_codegen::tables::asset_models::AssetModel>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor
{
    type ForeignKeys = AssetModelAncestorForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetModel(
                self.descendant_model,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetModel(
                self.ancestor_model,
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.descendant_model.is_some() && foreign_keys.ancestor_model.is_some()
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
                crate::codegen::tables::row::Row::AssetModel(asset_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.descendant_model == asset_models.id {
                    foreign_keys.descendant_model = Some(asset_models.clone());
                    updated = true;
                }
                if self.ancestor_model == asset_models.id {
                    foreign_keys.ancestor_model = Some(asset_models.clone());
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::AssetModel(asset_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.descendant_model == asset_models.id {
                    foreign_keys.descendant_model = None;
                    updated = true;
                }
                if self.ancestor_model == asset_models.id {
                    foreign_keys.ancestor_model = None;
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
impl web_common_traits::prelude::ForeignKeys for AssetModelAncestorForeignKeys {}
