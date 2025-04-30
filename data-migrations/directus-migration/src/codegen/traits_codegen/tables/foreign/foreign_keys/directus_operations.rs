#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DirectusOperationForeignKeys {
    pub resolve: Option<
        std::rc::Rc<
            crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation,
        >,
    >,
    pub reject: Option<
        std::rc::Rc<
            crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation,
        >,
    >,
    pub flow:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow>>,
    pub user_created:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation
{
    type ForeignKeys = DirectusOperationForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        if let Some(resolve) = self.resolve {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusOperation(
                    resolve,
                ),
            ));
        }
        if let Some(reject) = self.reject {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusOperation(
                    reject,
                ),
            ));
        }
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusFlow(self.flow),
        ));
        if let Some(user_created) = self.user_created {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusUser(
                    user_created,
                ),
            ));
        }
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        (foreign_keys.resolve.is_some() || self.resolve.is_none())
            && (foreign_keys.reject.is_some() || self.reject.is_none())
            && foreign_keys.flow.is_some()
            && (foreign_keys.user_created.is_some() || self.user_created.is_none())
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
                crate::codegen::tables::row::Row::DirectusFlow(directus_flows),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if directus_flows.id == self.flow {
                    foreign_keys.flow = Some(directus_flows);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusFlow(directus_flows),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if directus_flows.id == self.flow {
                    foreign_keys.flow = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusOperation(directus_operations),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(resolve) = self.resolve {
                    if directus_operations.id == resolve {
                        foreign_keys.resolve = Some(directus_operations.clone());
                        updated = true;
                    }
                }
                if let Some(reject) = self.reject {
                    if directus_operations.id == reject {
                        foreign_keys.reject = Some(directus_operations.clone());
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusOperation(directus_operations),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(resolve) = self.resolve {
                    if directus_operations.id == resolve {
                        foreign_keys.resolve = None;
                        updated = true;
                    }
                }
                if let Some(reject) = self.reject {
                    if directus_operations.id == reject {
                        foreign_keys.reject = None;
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusUser(directus_users),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(user_created) = self.user_created {
                    if directus_users.id == user_created {
                        foreign_keys.user_created = Some(directus_users);
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusUser(directus_users),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(user_created) = self.user_created {
                    if directus_users.id == user_created {
                        foreign_keys.user_created = None;
                        updated = true;
                    }
                }
            }
            (_, crud) => {
                unreachable!("Unexpected row type with operation {crud}");
            }
        }
        updated
    }
}
impl web_common_traits::prelude::ForeignKeys for DirectusOperationForeignKeys {}
