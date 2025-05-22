#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommercialProductForeignKeys {
    pub photograph: Option<crate::codegen::structs_codegen::tables::documents::Document>,
    pub brand: Option<crate::codegen::structs_codegen::tables::brands::Brand>,
    pub created_by: Option<crate::codegen::structs_codegen::tables::users::User>,
    pub updated_by: Option<crate::codegen::structs_codegen::tables::users::User>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct
{
    type ForeignKeys = CommercialProductForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Document(
                self.photograph_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Brand(self.brand_id),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(self.created_by),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(self.updated_by),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.photograph.is_some()
            && foreign_keys.brand.is_some()
            && foreign_keys.created_by.is_some()
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
                crate::codegen::tables::row::Row::Brand(brands),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if brands.id == self.brand_id {
                    foreign_keys.brand = Some(brands);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Brand(brands),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if brands.id == self.brand_id {
                    foreign_keys.brand = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Document(documents),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if documents.id == self.photograph_id {
                    foreign_keys.photograph = Some(documents);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Document(documents),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if documents.id == self.photograph_id {
                    foreign_keys.photograph = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::User(users),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if users.id == self.created_by {
                    foreign_keys.created_by = Some(users.clone());
                    updated = true;
                }
                if users.id == self.updated_by {
                    foreign_keys.updated_by = Some(users.clone());
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::User(users),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if users.id == self.created_by {
                    foreign_keys.created_by = None;
                    updated = true;
                }
                if users.id == self.updated_by {
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
impl web_common_traits::prelude::ForeignKeys for CommercialProductForeignKeys {}
