#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssetModelForeignKeys {
    pub created_by: Option<crate::codegen::structs_codegen::tables::users::User>,
    pub parent_model: Option<crate::codegen::structs_codegen::tables::asset_models::AssetModel>,
    pub updated_by: Option<crate::codegen::structs_codegen::tables::users::User>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::asset_models::AssetModel
{
    type ForeignKeys = AssetModelForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(self.created_by),
        ));
        if let Some(parent_model) = self.parent_model {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetModel(
                    parent_model,
                ),
            ));
        }
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(self.updated_by),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.created_by.is_some()
            && (foreign_keys.parent_model.is_some() || self.parent_model.is_some())
            && foreign_keys.updated_by.is_some()
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
                if self.parent_model.is_some_and(|parent_model| parent_model == asset_models.id) {
                    foreign_keys.parent_model = Some(asset_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::AssetModel(asset_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.parent_model.is_some_and(|parent_model| parent_model == asset_models.id) {
                    foreign_keys.parent_model = None;
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
                    foreign_keys.created_by = Some(users.clone());
                    updated = true;
                }
                if self.updated_by == users.id {
                    foreign_keys.updated_by = Some(users.clone());
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
                if self.updated_by == users.id {
                    foreign_keys.updated_by = None;
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
impl web_common_traits::prelude::ForeignKeys for AssetModelForeignKeys {}
